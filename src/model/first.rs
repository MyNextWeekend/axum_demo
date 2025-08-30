use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize,Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub salt: u32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
