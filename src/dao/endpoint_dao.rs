use crate::{Error, entity, vo::endpoint_vo};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

pub struct EndpointDao;

impl EndpointDao {
    pub async fn insert(
        db: &sea_orm::DbConn,
        parm: &endpoint_vo::InsertReq,
    ) -> Result<entity::endpoint::Model, Error> {
        let ment = entity::endpoint::ActiveModel {
            name: Set(parm.name.clone()),
            code: Set(parm.code.clone()),
            method: Set(parm.method.clone()),
            domain_code: Set(parm.domain_code.clone()),
            path: Set(parm.path.clone()),
            description: Set(parm.description.clone()),
            is_active: Set(parm.is_active.clone()),
            ..Default::default()
        };
        Ok(ment.insert(db).await?)
    }

    pub async fn update_by_id(
        db: &sea_orm::DbConn,
        parm: &endpoint_vo::UpdateReq,
    ) -> Result<entity::endpoint::Model, Error> {
        let model = entity::endpoint::Entity::find_by_id(parm.id)
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound(format!("ID {} 不存在", parm.id)))?;
        let mut model: entity::endpoint::ActiveModel = model.into();
        // 遍历字段，只更新有值的字段
        if let Some(name) = &parm.name {
            model.name = Set(name.clone());
        }
        if let Some(code) = &parm.code {
            model.code = Set(code.clone());
        }
        if let Some(method) = &parm.method {
            model.method = Set(method.clone());
        }
        if let Some(domain_code) = &parm.domain_code {
            model.domain_code = Set(domain_code.clone());
        }
        if let Some(path) = &parm.path {
            model.path = Set(path.clone());
        }
        if let Some(description) = &parm.description {
            model.description = Set(Some(description.clone()));
        }
        if let Some(is_active) = &parm.is_active {
            model.is_active = Set(Some(is_active.clone()));
        }

        let result = model.update(db).await?;
        Ok(result)
    }
}
