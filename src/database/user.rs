use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    // Encrypted using AES-256
    // Not using bcrypt because we need to be able to decrypt the password
    pub password: String,
    pub librus_access_token: String,
    pub next_check_at: time::OffsetDateTime,
    pub is_test_account: bool,

    pub created_at: time::OffsetDateTime,
}