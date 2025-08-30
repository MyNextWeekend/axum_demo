use crate::{Error, model::first::User};
use sqlx::MySqlPool;

pub struct UserDao;

impl UserDao {
    pub async fn query_by_id(db: &MySqlPool, id: u64) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await?;
        Ok(user)
    }

    pub async fn query(db: &MySqlPool, page: u64, page_size: u64) -> Result<Vec<User>, Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM user limit ? offset ?")
            .bind(page * page_size)
            .bind(page_size)
            .fetch_all(db)
            .await?;
        Ok(user)
    }

    pub async fn insert(db: &MySqlPool, username: &str) -> Result<u64, Error> {
        let rec = sqlx::query("INSERT INTO user (username) VALUES (?)")
            .bind(username)
            .execute(db)
            .await?;
        Ok(rec.last_insert_id())
    }

    pub async fn update(db: &MySqlPool, id: u64, username: &str) -> Result<u64, Error> {
        let rec = sqlx::query("UPDATE user SET username = ? WHERE id = ?")
            .bind(username)
            .bind(id)
            .execute(db)
            .await?;
        Ok(rec.rows_affected())
    }

    pub async fn delete(db: &MySqlPool, id: u64) -> Result<u64, Error> {
        let rec = sqlx::query("DELETE FROM user WHERE id = ?")
            .bind(id)
            .execute(db)
            .await?;
        Ok(rec.rows_affected())
    }
}
