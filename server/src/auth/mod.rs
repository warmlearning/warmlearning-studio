use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use argon2::Argon2;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use sqlx::SqlitePool;
use tower_sessions::Session;

/// 保護 /admin/* 路由（登入頁除外）：未登入的 session 直接導向 /admin/login，
/// 防止未授權訪客繞過前端直接開啟後台網址（spec.md 9.8 節）
pub async fn admin_guard(session: Session, req: Request, next: Next) -> Response {
    let path = req.uri().path();
    if path.starts_with("/admin") && path != "/admin/login" {
        let admin_id: Option<i64> = session
            .get(app::admin_auth::ADMIN_SESSION_KEY)
            .await
            .unwrap_or(None);
        if admin_id.is_none() {
            return Redirect::to("/admin/login").into_response();
        }
    }
    next.run(req).await
}

/// 若 admin_users 資料表尚無任何帳號，建立預設管理者帳號 warm/learning
/// （spec.md 6.1 節；密碼僅存 argon2 雜湊值，正式上線前務必更換）
pub async fn seed_default_admin(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM admin_users")
        .fetch_one(pool)
        .await?;
    if count > 0 {
        return Ok(());
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(b"learning", &salt)
        .expect("預設管理者密碼雜湊失敗")
        .to_string();

    sqlx::query(
        "INSERT INTO admin_users (username, password_hash, created_at, failed_login_attempts)
         VALUES (?, ?, datetime('now'), 0)",
    )
    .bind("warm")
    .bind(password_hash)
    .execute(pool)
    .await?;

    tracing::warn!("已建立預設管理者帳號 warm/learning，正式上線前請務必更換密碼");

    Ok(())
}
