mod auth;
mod db;
mod routes;

use app::{shell, App};
use axum::Router;
use leptos::prelude::provide_context;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_sessions::cookie::SameSite;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://./data/zhinuan.db".to_string());

    let pool = db::init_pool(&database_url)
        .await
        .expect("無法初始化資料庫連線池，請確認 DATABASE_URL 設定與 migrations 是否正確");
    tracing::info!("資料庫連線與 migrations 已就緒");

    auth::seed_default_admin(&pool)
        .await
        .expect("建立預設管理者帳號失敗");

    // tower-sessions-sqlx-store 目前鎖定 sqlx 0.8.x，與主要資料存取用的 sqlx 0.9.x
    // 是兩個不同版本的型別，因此 session store 另外開一條指向同一個資料庫檔案的連線池
    let session_pool = sqlx08::sqlite::SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("無法建立 session 用資料庫連線池");
    let session_store = SqliteStore::new(session_pool);
    session_store
        .migrate()
        .await
        .expect("session 資料表 migration 失敗");

    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./data/uploads".to_string());
    std::fs::create_dir_all(&upload_dir).expect("無法建立圖片上傳目錄");

    let conf = leptos::config::get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let is_prod = matches!(leptos_options.env, leptos::config::Env::PROD);

    // session cookie 安全性設定對照 spec.md 9.3 節；Secure 僅在正式環境（HTTPS）啟用，
    // 避免本機開發用 http:// 時瀏覽器直接丟棄 cookie
    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(7)))
        .with_http_only(true)
        .with_same_site(SameSite::Lax)
        .with_secure(is_prod);

    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let pool = pool.clone();
                move || provide_context(pool.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .nest_service("/uploads", tower_http::services::ServeDir::new(&upload_dir))
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(axum::middleware::from_fn(auth::admin_guard))
        .layer(session_layer)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("無法綁定伺服器位址");
    tracing::info!("伺服器啟動於 http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
