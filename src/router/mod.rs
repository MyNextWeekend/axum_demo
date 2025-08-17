pub(crate) mod hello;
pub(crate) mod user;

// pub fn init() -> Router {
//     tracing::info!("Initializing router...");
//     // 注册路由 注册中间件
//     Router::new()
//         .route("/", get(hello::hello_world))
//         .route("/users", get(user::create_user))
// }
