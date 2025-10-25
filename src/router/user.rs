use crate::{
    AppState, Result,
    core::extractor::UserInfo,
    dao::{self, UserDao},
    entity,
    vo::{self, user_vo},
};
use axum::{Json, extract::State};
use tracing::info;
use validator::Validate;

// 用户登录，成功返回 token
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<user_vo::LoginReq>,
) -> Result<user_vo::LoginResp> {
    payload.validate()?;
    info!("User login attempt: {:?}", payload.username);

    let user = UserDao::query_by_username(&state.db, &payload.username).await?;

    if let Some(user) = user
        && user.password == payload.password
    {
        let user = UserInfo::login(user, &state).await?;
        Ok(user_vo::LoginResp {
            user_id: user.user_db.id,
            token: user.token,
            username: user.user_db.username,
            role: user.user_db.role,
        }
        .into())
    } else {
        info!(
            "Invalid username or password for user: {}",
            payload.username
        );
        Err(crate::Error::Unauthorized("账号或密码错误".into()))
    }
}

// 获取用户权限信息
pub async fn permission(user: UserInfo) -> Result<user_vo::PermissionResp> {
    Ok(user_vo::PermissionResp {
        roles: vec![String::from("admin")],
        name: user.user_db.username,
        avatar: String::from(""),
        introduction: String::from(""),
    }
    .into())
}

// 用户登出，删除 redis 中的 session
pub async fn logout(State(state): State<AppState>, user: UserInfo) -> Result<String> {
    info!("User logout attempt: {:?}", user.user_db.username);
    user.logout(&state).await?;
    Ok(format!("Logout successful").into())
}

// 创建用户，返回新用户 ID
pub async fn create(
    user: UserInfo,
    State(state): State<AppState>,
    Json(parm): Json<user_vo::InsertReq>,
) -> Result<i64> {
    parm.validate()?;
    if user.is_admin() {
        return Err(crate::Error::Unauthorized("无权限操作".into()));
    }
    info!("Create user attempt by: {:?}", user.user_db.username);
    let u = UserDao::query_by_username(&state.db, &parm.username).await?;
    if u.is_some() {
        return Err(crate::Error::AlreadyExists("用户名已存在".into()));
    }
    let user = UserDao::insert(&state.db, parm).await?;
    Ok(user.id.into())
}

// 删除用户
pub async fn delete(State(state): State<AppState>, Json(parm): Json<vo::IdReq>) -> Result<u64> {
    let lins = dao::delete_by_id::<entity::user::Entity>(&state.db, parm.id).await?;
    Ok(lins.into())
}

// 动态修改用户信息
pub async fn update(
    State(state): State<AppState>,
    Json(parm): Json<user_vo::UpdateReq>,
) -> Result<i64> {
    parm.validate()?;
    let users_id = UserDao::update_by_id(&state.db, &parm).await?;
    Ok(users_id.id.into())
}

// 动态查询，返回用户列表
pub async fn query(
    State(state): State<AppState>,
    Json(parm): Json<vo::QueryReq>,
) -> Result<vo::PageResp<user_vo::SearchResp>> {
    parm.validate()?;
    info!("Get all users with params: {:?}", parm);
    let page_result = dao::query_by_page::<entity::user::Entity>(&state.db, &parm).await?;
    let page_result = page_result.map(|user| user_vo::SearchResp {
        id: user.id,
        username: user.username,
        password: user.password,
        salt: user.salt,
        role: user.role,
        enable_flag: user.enable_flag,
        created_at: user.created_at,
        updated_at: user.updated_at,
    });
    Ok(page_result.into())
}

// 查询详情信息
pub async fn info(
    State(state): State<AppState>,
    Json(parm): Json<vo::IdReq>,
) -> Result<Option<user_vo::SearchResp>> {
    let user = dao::query_by_id::<entity::user::Entity>(&state.db, parm.id)
        .await?
        .map(|user| user_vo::SearchResp {
            id: user.id,
            username: user.username,
            password: user.password,
            salt: user.salt,
            role: user.role,
            enable_flag: user.enable_flag,
            created_at: user.created_at,
            updated_at: user.updated_at,
        });
    Ok(user.into())
}
