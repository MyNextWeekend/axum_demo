use crate::{Error, entity, vo::menu_vo};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

pub struct MenuDao;

impl MenuDao {
    pub async fn insert(
        db: &sea_orm::DbConn,
        parm: &menu_vo::InsertReq,
    ) -> Result<entity::menu::Model, Error> {
        let ment = entity::menu::ActiveModel {
            parent_id: Set(parm.parent_id),
            path: Set(parm.path.clone()),
            name: Set(parm.name.clone()),
            component: Set(parm.component.clone()),
            redirect: Set(parm.redirect.clone()),
            sort: Set(parm.sort),
            meta: Set(parm.meta.clone()),
            status: Set(parm.status),
            remark: Set(parm.remark.clone()),
            ..Default::default()
        };
        Ok(ment.insert(db).await?)
    }

    pub async fn update_by_id(
        db: &sea_orm::DbConn,
        parm: &menu_vo::UpdateReq,
    ) -> Result<entity::menu::Model, Error> {
        let model = entity::menu::Entity::find_by_id(parm.id)
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound(format!("ID {} 不存在", parm.id)))?;
        let mut model: entity::menu::ActiveModel = model.into();
        // 遍历字段，只更新有值的字段
        if let Some(parent_id) = &parm.parent_id {
            model.parent_id = Set(parent_id.clone());
        }
        if let Some(path) = &parm.path {
            model.path = Set(path.clone());
        }
        if let Some(name) = &parm.name {
            model.name = Set(name.clone());
        }
        if let Some(component) = &parm.component {
            model.component = Set(component.clone());
        }
        if let Some(redirect) = &parm.redirect {
            model.redirect = Set(Some(redirect.clone()));
        }
        if let Some(sort) = &parm.sort {
            model.sort = Set(sort.clone());
        }
        if let Some(meta) = &parm.meta {
            model.meta = Set(Some(meta.clone()));
        }
        if let Some(status) = &parm.status {
            model.status = Set(status.clone());
        }
        if let Some(remark) = &parm.remark {
            model.remark = Set(Some(remark.clone()));
        }

        let result = model.update(db).await?;
        Ok(result)
    }
}
