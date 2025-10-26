use sea_orm::DbConn;
use serde::Serialize;

use crate::{Error, dao, entity, vo};

#[derive(Debug, Serialize)]
pub struct MenuNode {
    pub id: i64,
    pub label: String,
    pub menu: entity::menu::Model,
    pub children: Vec<MenuNode>,
}

pub async fn get_menu_tree(db: &DbConn, parm: &vo::QueryReq) -> Result<Vec<MenuNode>, Error> {
    // 此处全量查询，不分页
    let all_menus = dao::query_all::<entity::menu::Entity>(db, parm).await?;
    let tree = build_tree(&all_menus, 0);
    Ok(tree)
}

fn build_tree(all_menus: &[entity::menu::Model], parent_id: i64) -> Vec<MenuNode> {
    all_menus
        .iter()
        .filter(|m| m.parent_id == parent_id)
        .map(|m| MenuNode {
            id: m.id,
            label: m.name.clone(),
            menu: m.clone(),
            children: build_tree(all_menus, m.id),
        })
        .collect()
}
