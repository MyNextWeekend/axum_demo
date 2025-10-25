use crate::{Error, entity, vo::user_vo};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

pub struct UserDao;

impl UserDao {
    pub async fn insert(
        db: &sea_orm::DbConn,
        parm: user_vo::InsertReq,
    ) -> Result<entity::user::Model, Error> {
        let user = entity::user::ActiveModel {
            username: Set(parm.username),
            password: Set(parm.password),
            salt: Set(parm.salt),
            role: Set(1),
            enable_flag: Set(1),
            ..Default::default()
        };
        Ok(user.insert(db).await?)
    }

    pub async fn update_by_id(
        db: &sea_orm::DbConn,
        parm: &user_vo::UpdateReq,
    ) -> Result<entity::user::Model, Error> {
        let model = entity::user::Entity::find_by_id(parm.id)
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound(format!("ID {} 不存在", parm.id)))?;
        let mut model: entity::user::ActiveModel = model.into();

        // 遍历字段，只更新有值的字段
        if let Some(username) = &parm.username {
            model.username = Set(username.clone());
        }
        if let Some(password) = &parm.password {
            model.password = Set(password.clone());
        }
        if let Some(salt) = &parm.salt {
            model.salt = Set(Some(salt.clone()));
        }
        if let Some(role) = &parm.role {
            model.role = Set(role.clone());
        }

        let result = model.update(db).await?;
        Ok(result)
    }

    pub async fn query_by_username(
        db: &sea_orm::DbConn,
        username: &str,
    ) -> Result<Option<entity::user::Model>, Error> {
        Ok(entity::user::Entity::find()
            .filter(entity::user::Column::Username.eq(username))
            .one(db)
            .await?)
    }
}
