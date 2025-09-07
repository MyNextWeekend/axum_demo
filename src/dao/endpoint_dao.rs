use crate::{
    Error,
    model::first::Endpoint,
    vo::{
        PageReq, SortOrder,
        endpoint_vo::{SearchReq, UpdateReq},
    },
};
use sqlx::{Executor, MySql, QueryBuilder};

pub struct EndpointDao;

impl EndpointDao {
    pub async fn query_by_id<'e, E>(executor: E, id: u64) -> Result<Option<Endpoint>, Error>
    where
        E: Executor<'e, Database = MySql>,
    {
        let user = sqlx::query_as::<_, Endpoint>("SELECT * FROM endpoint WHERE id = ? ")
            .bind(id)
            .fetch_optional(executor)
            .await?;
        Ok(user)
    }

    pub async fn query<'e, E>(
        executor: E,
        parm: &PageReq<SearchReq>,
    ) -> Result<Vec<Endpoint>, Error>
    where
        E: Executor<'e, Database = MySql>,
    {
        // sqlx 提供的动态 SQL 构造器
        let mut builder = QueryBuilder::new("SELECT * FROM endpoint WHERE 1=1");
        if let Some(filter) = &parm.filter {
            if let Some(id) = &filter.id {
                builder.push("and id =").push_bind(id);
            }
            if let Some(name) = &filter.name {
                builder.push("and name =").push_bind(name);
            }
            if let Some(code) = &filter.code {
                builder.push("and code =").push_bind(code);
            }
            if let Some(method) = &filter.method {
                builder.push("and method =").push_bind(method);
            }
            if let Some(domain_code) = &filter.domain_code {
                builder.push("and domain_code =").push_bind(domain_code);
            }
            if let Some(path) = &filter.path {
                builder.push("and path =").push_bind(path);
            }
            if let Some(description) = &filter.description {
                builder.push("and description =").push_bind(description);
            }
            if let Some(is_active) = &filter.is_active {
                builder.push("and is_active =").push_bind(is_active);
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

    pub async fn insert<'e, E>(executor: E, parm: &Endpoint) -> Result<u64, Error>
    where
        E: Executor<'e, Database = MySql>,
    {
        let rec = sqlx::query(
            "INSERT INTO endpoint (name,code,method,domain_code,path,description,is_active) VALUES (?,?,?,?,?,?,?)",
        )
        .bind(&parm.name).bind(&parm.code).bind(&parm.method).bind(&parm.domain_code).bind(&parm.path).bind(&parm.description).bind(&parm.is_active)
        .execute(executor)
        .await?;
        Ok(rec.last_insert_id())
    }

    pub async fn update_by_id<'e, E>(executor: E, parm: &UpdateReq) -> Result<u64, Error>
    where
        E: Executor<'e, Database = MySql>,
    {
        // sqlx 提供的动态 SQL 构造器
        let mut builder = QueryBuilder::new("UPDATE endpoint SET ");
        // 自动添加逗号
        let mut separated = builder.separated(", ");
        if let Some(name) = &parm.name {
            separated.push("name =").push_bind(name);
        }
        if let Some(code) = &parm.code {
            separated.push("code =").push_bind(code);
        }
        if let Some(method) = &parm.method {
            separated.push("method =").push_bind(method);
        }
        if let Some(domain_code) = &parm.domain_code {
            separated.push("domain_code =").push_bind(domain_code);
        }
        if let Some(path) = &parm.path {
            separated.push("path =").push_bind(path);
        }
        if let Some(description) = &parm.description {
            separated.push("description =").push_bind(description);
        }
        if let Some(is_active) = &parm.is_active {
            separated.push("is_active =").push_bind(is_active);
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
        let rec = sqlx::query("UPDATE endpoint SET enable_flag = 0 WHERE id = ?")
            .bind(id)
            .execute(executor)
            .await?;
        Ok(rec.rows_affected())
    }
}
