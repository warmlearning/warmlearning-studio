use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub const ADMIN_SESSION_KEY: &str = "admin_id";

/// 檢查目前 request 的 session 是否為已登入管理者，未登入回傳 Err，
/// 供每個受保護的 server function 開頭呼叫（spec.md 第 8 節權限檢查規則）
#[cfg(feature = "ssr")]
pub async fn require_admin() -> Result<i64, ServerFnError> {
    let session: tower_sessions::Session = leptos_axum::extract().await?;
    let admin_id: Option<i64> = session
        .get(ADMIN_SESSION_KEY)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    admin_id.ok_or_else(|| ServerFnError::new("UNAUTHORIZED"))
}

/// 從 ServerFnError 取出乾淨的錯誤訊息文字，避免畫面上顯示
/// "error running server function: " 這類技術性前綴
pub fn friendly_error_message(e: &ServerFnError) -> String {
    match e {
        ServerFnError::ServerError(msg) => msg.clone(),
        other => other.to_string(),
    }
}
