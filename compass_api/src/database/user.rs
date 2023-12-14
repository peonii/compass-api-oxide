use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(Serialize, Deserialize, SimpleObject)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    // Encrypted using AES-256
    // Not using bcrypt because we need to be able to decrypt the password
    pub password: String,
    pub librus_access_token: String,
    pub next_check_at: time::OffsetDateTime,
    pub is_test_account: bool,

    pub first_name: String,
    pub last_name: String,

    pub created_at: time::OffsetDateTime,
}

pub struct NewUser {
    pub email: String,
    pub password: String,
    pub librus_access_token: String,
    pub next_check_at: time::OffsetDateTime,
    pub is_test_account: bool,

    pub first_name: String,
    pub last_name: String,
}

impl User {
    pub async fn find_by_id(pg: &sqlx::PgPool, id: uuid::Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT id, email, password, librus_access_token, next_check_at, is_test_account, first_name, last_name, created_at
                FROM users
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pg)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(pg: &sqlx::PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT id, email, password, librus_access_token, next_check_at, is_test_account, first_name, last_name, created_at
                FROM users
                WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pg)
        .await?;

        Ok(user)
    }

    pub async fn update_token(pg: &sqlx::PgPool, id: uuid::Uuid, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                UPDATE users
                SET librus_access_token = $1
                WHERE id = $2
            "#,
            token,
            id
        )
        .execute(pg)
        .await?;

        Ok(())
    }
}

impl NewUser {
    pub async fn create_or_update(&self, pg: &sqlx::PgPool) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (email, password, librus_access_token, next_check_at, is_test_account, first_name, last_name)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (email) DO UPDATE SET
                    password = $2,
                    librus_access_token = $3,
                    next_check_at = $4,
                    is_test_account = $5
                RETURNING id, email, password, librus_access_token, next_check_at, is_test_account, first_name, last_name, created_at
            "#,
            self.email,
            self.password,
            self.librus_access_token,
            self.next_check_at,
            self.is_test_account,
            self.first_name,
            self.last_name
        )
        .fetch_one(pg)
        .await?;

        Ok(user)
    }
}


// Stored in Redis
#[derive(Serialize, Deserialize, SimpleObject)]
pub struct UserSession {
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expires_at: time::OffsetDateTime,
    pub created_at: time::OffsetDateTime,
}

impl UserSession {
    pub async fn find_by_token(redis: &redis::Client, token: &str) -> anyhow::Result<Option<UserSession>> {
        let mut conn = redis.get_async_connection().await?;

        let session: Option<Vec<u8>> = redis::cmd("GET")
            .arg(token)
            .query_async(&mut conn)
            .await?;


        if let Some(session) = session {
            let session: UserSession = rmp_serde::from_slice(session.as_slice())?;

            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    pub async fn create(redis: &redis::Client, user_id: uuid::Uuid) -> anyhow::Result<UserSession> {
        let token = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(80)
            .map(char::from)
            .collect::<String>();

        let expires_at = time::OffsetDateTime::now_utc() + time::Duration::days(30);

        let session = UserSession {
            user_id,
            token: token.clone(),
            expires_at,
            created_at: time::OffsetDateTime::now_utc(),
        };

        let mut conn = redis.get_async_connection().await?;

        let session_enc = rmp_serde::to_vec(&session)?;

        redis::cmd("SET")
            .arg(token)
            .arg(session_enc)
            .arg("EX")
            .arg(60 * 60 * 24 * 30)
            .query_async(&mut conn)
            .await?;

        Ok(session)
    }

}