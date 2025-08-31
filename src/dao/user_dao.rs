use crate::{Error, model::first::User};
use sqlx::MySqlPool;

pub struct UserDao;

impl UserDao {
    pub async fn query_by_id(db: &MySqlPool, id: u64) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ? ")
            .bind(id)
            .fetch_optional(db)
            .await?;
        Ok(user)
    }
    
    pub async fn query_by_username(db: &MySqlPool, username: &str) -> Result<Option<User>, Error> {
        let user =
            sqlx::query_as::<_, User>("SELECT * FROM user WHERE username = ? and enable_flag = 1")
                .bind(username)
                .fetch_optional(db)
                .await?;
        Ok(user)
    }

    pub async fn query(db: &MySqlPool, page: u64, page_size: u64) -> Result<Vec<User>, Error> {
        let user =
            sqlx::query_as::<_, User>("SELECT * FROM user where enable_flag=1 limit ? offset ?")
                .bind(page * page_size)
                .bind(page_size)
                .fetch_all(db)
                .await?;
        Ok(user)
    }

    pub async fn insert(db: &MySqlPool, user: User) -> Result<u64, Error> {
        let rec = sqlx::query(
            "INSERT INTO user (username,password,salt,role,created_at,updated_at) VALUES (?,?,?,?,?,?)",
        )
        .bind(user.username).bind(user.password).bind(user.salt).bind(user.role).bind(user.created_at).bind(user.updated_at )
        .execute(db)
        .await?;
        Ok(rec.last_insert_id())
    }

    pub async fn update_salt_by_id(db: &MySqlPool, id: u64, salt: &str) -> Result<u64, Error> {
        let rec = sqlx::query("UPDATE user SET salt = ? WHERE id = ?")
            .bind(salt)
            .bind(id)
            .execute(db)
            .await?;
        Ok(rec.rows_affected())
    }

    pub async fn delete(db: &MySqlPool, id: u64) -> Result<u64, Error> {
        let rec = sqlx::query("UPDATE user SET enable_flag = 0 WHERE id = ?")
            .bind(id)
            .execute(db)
            .await?;
        Ok(rec.rows_affected())
    }
}
