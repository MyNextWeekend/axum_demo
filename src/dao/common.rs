//! 通用查询与分页请求体定义
//!
//! 该模块包含：
//! - 根据主键删除单条记录
//! - 根据主键查询单条记录
//! - 根据请求条件分页查询
//! - 根据请求条件查询全量数据（不分页）
//!
//! 为各业务模块复用。

use sea_orm::{EntityTrait, FromQueryResult, PaginatorTrait, PrimaryKeyTrait};

use crate::{dao::query_build::build_query, error::Error, vo};

/// 根据主键删除单条记录
///
/// # 参数
/// * `db` - 数据库连接对象
/// * `id` - 实体主键值
///
/// # 返回
/// * `Ok(u64)` - 影响的行数（一般为 0 或 1）
/// * `Err(Error)` - 执行过程中发生的错误
pub async fn delete_by_id<E>(
    db: &sea_orm::DbConn,
    id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
) -> Result<u64, Error>
where
    E: EntityTrait,
{
    let result = E::delete_by_id(id).exec(db).await?;
    Ok(result.rows_affected)
}

/// 根据主键查询单条记录
///
/// # 参数
/// * `db` - 数据库连接对象
/// * `id` - 实体主键值
///
/// # 返回
/// * `Ok(Some(E::Model))` - 查询到的模型
/// * `Ok(None)` - 未找到记录
/// * `Err(Error)` - 查询过程中发生的错误
pub async fn query_by_id<E>(
    db: &sea_orm::DbConn,
    id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
) -> Result<Option<E::Model>, Error>
where
    E: EntityTrait,
{
    Ok(E::find_by_id(id).one(db).await?)
}

/// 根据请求条件分页查询
///
/// # 参数
/// * `db` - 数据库连接对象
/// * `req` - 查询请求体（包含过滤条件、页码、页大小等）
///
/// # 返回
/// * `Ok(PageResp<E::Model>)` - 分页结果，含总数、页码、数据
/// * `Err(Error)` - 查询过程中发生的错误
pub async fn query_by_page<E>(
    db: &sea_orm::DbConn,
    req: &vo::QueryReq,
) -> Result<vo::PageResp<E::Model>, Error>
where
    E: EntityTrait,
    E::Model: FromQueryResult + Sized + Send + Sync,
{
    let query = build_query::<E>(req)?;
    let paginator = query.paginate(db, req.size);
    let total = paginator.num_items().await?;
    let data = paginator.fetch_page(req.page).await?;
    Ok(vo::PageResp::new(total, req.page, req.size, data))
}

/// 根据请求条件查询全量数据（不分页）
///
/// # 参数
/// * `db` - 数据库连接对象
/// * `req` - 查询请求体（包含过滤条件）
///
/// # 返回
/// * `Ok(Vec<E::Model>)` - 匹配的全部结果
/// * `Err(Error)` - 查询过程中发生的错误
pub async fn query_all<E>(db: &sea_orm::DbConn, req: &vo::QueryReq) -> Result<Vec<E::Model>, Error>
where
    E: EntityTrait,
{
    let result = build_query::<E>(req)?.all(db).await?;
    Ok(result)
}
