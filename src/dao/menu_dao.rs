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
            ..Default::default()
        };
        Ok(ment.insert(db).await?)
    }

    pub async fn update_by_id(
        db: &sea_orm::DbConn,
        parm: &menu_vo::UpdateReq,
    ) -> Result<entity::menu::Model, Error> {
        let pear = entity::menu::Entity::find_by_id(parm.id)
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound(format!("ID {} 不存在", parm.id)))?;
        let mut pear: entity::menu::ActiveModel = pear.into();
        // 遍历字段，只更新有值的字段
        if let Some(password) = &parm.password {
            pear.component = Set(password.clone());
        }

        let result = pear.update(db).await?;
        Ok(result)
    }
}
