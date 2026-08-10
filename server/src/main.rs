mod auth;
mod db;
mod routes;

use app::{shell, App};
use axum::extract::Extension;
use axum::routing::get;
use axum::Router;
use leptos::prelude::provide_context;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::catch_panic::CatchPanicLayer;

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

    let conf = leptos::config::get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/sitemap.xml", get(routes::sitemap_handler))
        .layer(Extension(pool.clone()))
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
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(CatchPanicLayer::custom(routes::handle_panic))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("無法綁定伺服器位址");
    tracing::info!("伺服器啟動於 http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
