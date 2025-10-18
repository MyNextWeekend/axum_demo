use crate::{
    AppState, Result,
    core::extractor::UserInfo,
    dao::UserDao,
    model::first::User,
    vo::{
        IdReq, IdsReq, PageReq,
        user_vo::{
            InfoResp, InsertReq, LoginReq, LoginResp, PermissionResp, SearchReq, SearchResp,
            UpdateReq,
        },
    },
};
use axum::{Json, extract::State};
use tracing::info;
use validator::Validate;

// 用户登录，成功返回 token
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginReq>,
) -> Result<LoginResp> {
    payload.validate()?;
    info!("User login attempt: {:?}", payload.username);

    let user = UserDao::query_by_username(&state.db, &payload.username).await?;

    if let Some(user) = user
        && user.password == payload.password
    {
        let user = UserInfo::login(user, &state).await?;
        Ok(LoginResp {
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
pub async fn permission(user: UserInfo) -> Result<PermissionResp> {
    Ok(PermissionResp {
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
    Json(parm): Json<InsertReq>,
) -> Result<u64> {
    parm.validate()?;
    if user.is_admin() {
        return Err(crate::Error::Unauthorized("无权限操作".into()));
    }
    info!("Create user attempt by: {:?}", user.user_db.username);
    let u = UserDao::query_by_username(&state.db, &parm.username).await?;
    if u.is_some() {
        return Err(crate::Error::AlreadyExists("用户名已存在".into()));
    }
    let parm = User {
        id: 0,
        username: parm.username,
        password: parm.password,
        salt: None,
        role: 1,
        enable_flag: 1,
        created_at: Some(chrono::Local::now().naive_local()),
        updated_at: Some(chrono::Local::now().naive_local()),
    };
    let new_user_id = UserDao::insert(&state.db, parm).await?;
    Ok(new_user_id.into())
}

// 删除用户
pub async fn delete(State(state): State<AppState>, Json(parm): Json<IdsReq>) -> Result<u64> {
    let lins = UserDao::delete(&state.db, parm.ids).await?;
    Ok(lins.into())
}

// 动态修改用户信息
pub async fn update(State(state): State<AppState>, Json(parm): Json<UpdateReq>) -> Result<u64> {
    parm.validate()?;
    let users_id = UserDao::update_by_id(&state.db, &parm).await?;
    Ok(users_id.into())
}

// 动态查询，返回用户列表
pub async fn query(
    State(state): State<AppState>,
    Json(parm): Json<PageReq<SearchReq>>,
) -> Result<Vec<SearchResp>> {
    parm.validate()?;
    info!("Get all users with params: {:?}", parm);
    let users = UserDao::query(&state.db, &parm).await?;
    let users: Vec<SearchResp> = users
        .into_iter()
        .map(|u| SearchResp {
            user_id: u.id,
            username: u.username,
        })
        .collect();
    Ok(users.into())
}

// 查询详情信息
pub async fn info(
    State(state): State<AppState>,
    Json(parm): Json<IdReq>,
) -> Result<Option<InfoResp>> {
    let user = UserDao::query_by_id(&state.db, parm.id).await?;
    let user = user.map(|user| InfoResp {
        id: user.id,
        username: user.username,
        role: user.role,
        created_at: user.created_at,
        updated_at: user.updated_at,
    });
    Ok(user.into())
}
