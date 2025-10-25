use crate::{Error, entity, vo::endpoint_vo};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

pub struct EndpointDao;

impl EndpointDao {
    pub async fn insert(
        db: &sea_orm::DbConn,
        parm: &endpoint_vo::InsertReq,
    ) -> Result<entity::endpoint::Model, Error> {
        let ment = entity::endpoint::ActiveModel {
            code: Set(parm.code.clone()),
            ..Default::default()
        };
        Ok(ment.insert(db).await?)
    }

    pub async fn update_by_id(
        db: &sea_orm::DbConn,
        parm: &endpoint_vo::UpdateReq,
    ) -> Result<entity::endpoint::Model, Error> {
        let pear = entity::endpoint::Entity::find_by_id(parm.id)
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound(format!("ID {} 不存在", parm.id)))?;
        let mut pear: entity::endpoint::ActiveModel = pear.into();
        // 遍历字段，只更新有值的字段
        // if let Some(password) = &parm.password {
        //     pear.component = Set(password.clone());
        // }

        let result = pear.update(db).await?;
        Ok(result)
    }
}
