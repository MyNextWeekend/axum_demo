use sea_orm::{Condition, EntityTrait, ExprTrait, QueryFilter, QueryOrder, prelude::Expr};
use std::str::FromStr;

use crate::{error::Error, vo};

// ============================
//  复杂查询逻辑构建（条件、排序等）
// ============================

/// 根据 SearchReq 构造查询，失败则返回错误
pub fn build_query<E>(req: &vo::QueryReq) -> Result<sea_orm::Select<E>, Error>
where
    E: EntityTrait,
{
    let mut query = E::find();

    // 构造过滤条件
    if let Some(filters) = &req.filters {
        let cond = build_condition::<E>(filters, &req.logic)?;
        query = query.filter(cond);
    }

    // 构造排序条件
    if let Some(sorts) = &req.sorts {
        for sort in sorts {
            let column = match E::Column::from_str(&sort.field) {
                Ok(c) => c,
                Err(_) => {
                    return Err(Error::BuildQueryError(format!(
                        "排序字段 {} 不存在",
                        sort.field
                    )));
                }
            };

            match sort.order {
                vo::Order::Asc => query = query.order_by_asc(column),
                vo::Order::Desc => query = query.order_by_desc(column),
            }
        }
    }

    Ok(query)
}

/// 构造 SeaORM 的 Condition
fn build_condition<E>(
    filters: &[vo::Filter],
    logic: &vo::LogicOp,
) -> Result<sea_orm::Condition, Error>
where
    E: EntityTrait,
{
    let mut cond = match logic {
        vo::LogicOp::And => Condition::all(),
        vo::LogicOp::Or => Condition::any(),
    };

    for filter in filters {
        // 获取 column（安全地防注入）
        let column = match E::Column::from_str(&filter.field) {
            Ok(c) => c,
            Err(_) => {
                return Err(Error::InvalidQueryField(format!("{}", filter.field)));
            }
        };

        let field = Expr::col(column);

        match filter.op {
            vo::CompareOp::Eq => {
                if let Some(v) = filter.values.first() {
                    cond = cond.add(field.eq(v.clone()));
                }
            }
            vo::CompareOp::Ne => {
                if let Some(v) = filter.values.first() {
                    cond = cond.add(field.ne(v.clone()));
                }
            }
            vo::CompareOp::Gt => {
                if let Some(v) = filter.values.first() {
                    cond = cond.add(field.gt(v.clone()));
                }
            }
            vo::CompareOp::Ge => {
                if let Some(v) = filter.values.first() {
                    cond = cond.add(field.gte(v.clone()));
                }
            }
            vo::CompareOp::Lt => {
                if let Some(v) = filter.values.first() {
                    cond = cond.add(field.lt(v.clone()));
                }
            }
            vo::CompareOp::Le => {
                if let Some(v) = filter.values.first() {
                    cond = cond.add(field.lte(v.clone()));
                }
            }
            vo::CompareOp::Like => {
                if let Some(v) = filter.values.first() {
                    cond = cond.add(field.like(format!("%{}%", v)));
                }
            }
            vo::CompareOp::In => {
                cond = cond.add(field.is_in(filter.values.clone()));
            }
            vo::CompareOp::Between => {
                if filter.values.len() == 2 {
                    cond =
                        cond.add(field.between(filter.values[0].clone(), filter.values[1].clone()));
                }
            }
            vo::CompareOp::IsNull => cond = cond.add(field.is_null()),
            vo::CompareOp::IsNotNull => cond = cond.add(field.is_not_null()),
        };
    }

    Ok(cond)
}

#[cfg(test)]
mod test_query {

    use super::*;
    use crate::entity;
    use sea_orm::{DbBackend, QueryTrait};

    #[test]
    fn test_build_sql() {
        let req = vo::QueryReq {
            filters: Some(vec![
                vo::Filter {
                    field: "name".to_string(),
                    op: vo::CompareOp::Like,
                    values: vec!["Alice".to_string()],
                },
                vo::Filter {
                    field: "sort".to_string(),
                    op: vo::CompareOp::Gt,
                    values: vec!["20".to_string()],
                },
            ]),
            logic: vo::LogicOp::And,
            sorts: Some(vec![vo::SortField {
                field: "sort".to_string(),
                order: vo::Order::Desc,
            }]),
            page: 1,
            size: 50,
        };

        let query = build_query::<entity::menu::Entity>(&req).unwrap();
        let stmt = query.build(DbBackend::MySql);
        println!("Generated SQL: {}", stmt.to_string());
    }
}
