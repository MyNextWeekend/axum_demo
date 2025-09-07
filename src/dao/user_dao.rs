use crate::{
    Error,
    model::first::User,
    vo::{
        PageReq, SortOrder,
        user_vo::{SearchReq, UpdateReq},
    },
};
use sqlx::{Executor, MySql, QueryBuilder};

pub struct UserDao;

impl UserDao {
    pub async fn query_by_id<'e, E>(executor: E, id: u64) -> Result<Option<User>, Error>
    where
        E: Executor<'e, Database = MySql>,
    {
        let user = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ? ")
            .bind(id)
            .fetch_optional(executor)
            .await?;
        Ok(user)
    }

    pub async fn query_by_username<'e, E>(
        executor: E,
        username: &str,
    ) -> Result<Option<User>, Error>
    where
        E: Executor<'e, Database = MySql>,
    {
        let user =
            sqlx::query_as::<_, User>("SELECT * FROM user WHERE username = ? and enable_flag = 1")
                .bind(username)
                .fetch_optional(executor)
                .await?;
        Ok(user)
    }

    pub async fn query<'e, E>(executor: E, parm: &PageReq<SearchReq>) -> Result<Vec<User>, Error>
    where
        E: Executor<'e, Database = MySql>,
    {
        // sqlx 提供的动态 SQL 构造器
        let mut builder = QueryBuilder::new("SELECT * FROM user WHERE 1=1");
        if let Some(filter) = &parm.filter {
            if let Some(name) = &filter.name {
                builder.push("and name =").push_bind(name);
            }
            if let Some(age) = &filter.age {
                builder.push("and age =").push_bind(age);
            }
        }
        // ---------- 排序 ----------
        if let Some(sort_by) = &parm.sort_by {
            builder.push(" ORDER BY ").push(sort_by);
            if let Some(order) = &parm.sort_order {
                builder.push(" ").push(match order {
                    SortOrder::Asc => "ASC",
                    SortOrder::Desc => "DESC",
                });
            }
        }
        // ---------- 分页 ----------
        let offset = (parm.page.saturating_sub(1) * parm.page_size) as i64;
        builder.push(" limit ").push_bind(parm.page_size);
        builder.push(" offset ").push_bind(offset);
        let users = builder.build_query_as().fetch_all(executor).await?;
        Ok(users)
    }

    pub async fn insert<'e, E>(executor: E, user: User) -> Result<u64, Error>
    where
        E: Executor<'e, Database = MySql>,
    {
        let rec = sqlx::query(
            "INSERT INTO user (username,password,salt,role,created_at,updated_at) VALUES (?,?,?,?,?,?)",
        )
        .bind(user.username).bind(user.password).bind(user.salt).bind(user.role).bind(user.created_at).bind(user.updated_at )
        .execute(executor)
        .await?;
        Ok(rec.last_insert_id())
    }

    pub async fn update_by_id<'e, E>(executor: E, parm: &UpdateReq) -> Result<u64, Error>
    where
        E: Executor<'e, Database = MySql>,
    {
        // sqlx 提供的动态 SQL 构造器
        let mut builder = QueryBuilder::new("UPDATE users SET ");
        // 自动添加逗号
        let mut separated = builder.separated(", ");
        if let Some(username) = &parm.username {
            separated.push(" username = ").push_bind(username);
        }
        if let Some(password) = &parm.password {
            separated.push(" password = ").push_bind(password);
        }
        if let Some(salt) = &parm.salt {
            separated.push(" salt = ").push_bind(salt);
        }
        if let Some(role) = &parm.role {
            separated.push(" role = ").push_bind(role);
        }
        // WHERE 条件
        builder.push(" WHERE id = ").push_bind(&parm.id);

        let result = builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete<'e, E>(executor: E, id: u64) -> Result<u64, Error>
    where
        E: Executor<'e, Database = MySql>,
    {
        let rec = sqlx::query("UPDATE user SET enable_flag = 0 WHERE id = ?")
            .bind(id)
            .execute(executor)
            .await?;
        Ok(rec.rows_affected())
    }
}
