use std::time::Duration;

use crate::{
    AppState, Result,
    core::{constant, extractor::UserInfo},
    dao::UserDao,
    model::first::User,
    utils::RedisUtil,
    vo::user_vo::{UserFilter, UserFilterResp, UserInfoResp},
};
use axum::{Json, extract::State};
use tracing::info;
use validator::Validate;

// 用户登录，成功返回 token
pub async fn user_login(
    State(state): State<AppState>,
    Json(payload): Json<crate::vo::user_vo::UserLoginReq>,
) -> Result<crate::vo::user_vo::UserLoginResp> {
    payload.validate()?;
    info!("User login attempt: {:?}", payload.username);

    let user = UserDao::query_by_username(&state.db, &payload.username).await?;

    match user {
        Some(u) if u.password == payload.password => {
            // 生成随机值存放数据库
            let salt = chrono::Local::now().timestamp();
            // 登陆信息存放在 redis 中
            let redis = RedisUtil::new(state.redis.clone());
            let session_key = format!("{}:{}:{}", constant::SESSION_KEY, u.id, salt);
            redis
                .set_with_expire(
                    &session_key,
                    serde_json::to_string(&u).unwrap(),
                    Duration::from_secs(constant::EXPIRATION_SECS),
                )
                .await?;
            info!("User login successful: {:?}", payload.username);
            Ok(crate::vo::user_vo::UserLoginResp {
                user_id: u.id,
                token: session_key,
                username: u.username,
                role: u.role,
            }
            .into())
        }
        _ => {
            info!(
                "Invalid username or password for user: {}",
                payload.username
            );
            Err(crate::Error::Unauthorized("账号或密码错误".into()))
        }
    }
}

// 用户登出，删除 redis 中的 session
pub async fn user_logout(State(state): State<AppState>, user: UserInfo) -> Result<String> {
    info!("User logout attempt: {:?}", user.user_db.username);
    user.logout(&state).await?;
    Ok(format!("Logout successful").into())
}

// 动态查询，返回用户列表
pub async fn user_query(
    State(state): State<AppState>,
    Json(parm): Json<crate::vo::PageReq<UserFilter>>,
) -> Result<Vec<UserFilterResp>> {
    parm.validate()?;
    info!("Get all users with params: {:?}", parm);
    let users = UserDao::query(&state.db, parm).await?;
    let users: Vec<UserFilterResp> = users
        .into_iter()
        .map(|u| UserFilterResp {
            user_id: u.id,
            username: u.username,
        })
        .collect();
    Ok(users.into())
}

// 查询详情信息
pub async fn user_info(
    State(state): State<AppState>,
    Json(id): Json<u64>,
) -> Result<Option<UserInfoResp>> {
    let user = UserDao::query_by_id(&state.db, id).await?;
    let user = user.map(|user| UserInfoResp {
        id: user.id,
        username: user.username,
        role: user.role,
        created_at: user.created_at,
        updated_at: user.updated_at,
    });
    Ok(user.into())
}

// 创建用户，返回新用户 ID
pub async fn user_create(
    user: UserInfo,
    State(state): State<AppState>,
    Json(new_user): Json<crate::vo::user_vo::UserCreateReq>,
) -> Result<u64> {
    new_user.validate()?;
    if user.is_admin() {
        return Err(crate::Error::Unauthorized("无权限操作".into()));
    }
    info!("Create user attempt by: {:?}", user.user_db.username);
    let u = UserDao::query_by_username(&state.db, &new_user.username).await?;
    if u.is_some() {
        return Err(crate::Error::AlreadyExists("用户名已存在".into()));
    }
    let new_user = User {
        id: 0,
        username: new_user.username,
        password: new_user.password,
        salt: None,
        role: 1,
        enable_flag: 1,
        created_at: Some(chrono::Local::now().naive_local()),
        updated_at: Some(chrono::Local::now().naive_local()),
    };
    let new_user_id = UserDao::insert(&state.db, new_user).await?;
    Ok(new_user_id.into())
}

// 动态修改用户信息
pub async fn user_update(
    State(state): State<AppState>,
    Json(parm): Json<crate::vo::user_vo::UserUpdateReq>,
) -> Result<u64> {
    parm.validate()?;
    let users_id = UserDao::update_by_id(&state.db, parm).await?;
    Ok(users_id.into())
}

// 删除用户
pub async fn user_remove(State(state): State<AppState>, Json(id): Json<u64>) -> Result<u64> {
    let user_id = UserDao::delete(&state.db, id).await?;
    Ok(user_id.into())
}
