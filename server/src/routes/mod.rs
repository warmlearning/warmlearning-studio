use axum::extract::Extension;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};

use crate::db::DbPool;

const STATIC_PAGES: [&str; 6] = ["", "about", "courses", "news", "contact", "faq"];

fn xml_escape(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// 依目前 request 的 Host header 組出對外網址前綴，避免在網域尚未確定前寫死假網域
/// （spec.md 14 節：自訂網域待購買後才會確定）
fn base_url_from_headers(headers: &HeaderMap) -> String {
    let host = headers.get(header::HOST).and_then(|v| v.to_str().ok()).unwrap_or("localhost");
    let scheme = if host.starts_with("localhost") || host.starts_with("127.0.0.1") { "http" } else { "https" };
    format!("{scheme}://{host}")
}

/// 動態產生 /sitemap.xml，列出五個對外頁面與所有已發布公告的單篇網址（spec.md 10.3 節）
/// /privacy 因設定 noindex，不列入 sitemap
pub async fn sitemap_handler(headers: HeaderMap, Extension(pool): Extension<DbPool>) -> Response {
    let base = base_url_from_headers(&headers);

    let slugs: Vec<String> = sqlx::query_scalar(
        "SELECT slug FROM announcements
         WHERE status = 'published' AND published_at <= datetime('now')
         ORDER BY published_at DESC",
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for path in STATIC_PAGES {
        xml.push_str(&format!("  <url><loc>{base}/{path}</loc></url>\n"));
    }
    for slug in &slugs {
        xml.push_str(&format!("  <url><loc>{base}/news/{}</loc></url>\n", xml_escape(slug)));
    }
    xml.push_str("</urlset>\n");

    (StatusCode::OK, [(header::CONTENT_TYPE, "application/xml; charset=utf-8")], xml).into_response()
}

const SERVER_ERROR_HTML: &str = r#"<!doctype html>
<html lang="zh-Hant">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="robots" content="noindex">
<title>系統出了一點小狀況｜知暖學習工作室</title>
<link rel="stylesheet" href="/pkg/warmlearning-studio.css">
</head>
<body>
<section class="bg-white">
  <div class="mx-auto flex max-w-xl flex-col items-center gap-6 px-6 py-24 text-center">
    <img src="/img/illustration-404.png" alt="" class="w-48 sm:w-64">
    <h1 class="text-2xl font-bold text-brand-blue">系統出了一點小狀況</h1>
    <p class="text-slate-gray">我們正在盡快處理，請稍後再試一次，或直接透過 LINE 與我們聯繫。</p>
    <div class="flex flex-wrap justify-center gap-4">
      <a href="/" class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-full px-8 py-3 font-medium transition-all duration-200 bg-brand-blue text-white hover:bg-[#14294F]">返回首頁</a>
      <a href="https://line.me/R/ti/p/@891ivojl" target="_blank" rel="noopener noreferrer" class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-full px-8 py-3 font-medium transition-all duration-200 bg-line-green text-white hover:brightness-95">加入 LINE 諮詢</a>
    </div>
  </div>
</section>
</body>
</html>
"#;

/// 伺服器發生未預期錯誤（panic）時顯示的友善錯誤頁，對照 spec.md 11.2 節與
/// tech-implementation.md 第 5 節文案；技術錯誤細節僅記錄於後端 log，不顯示給訪客
pub fn handle_panic(err: Box<dyn std::any::Any + Send + 'static>) -> Response {
    let message = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        (*s).to_string()
    } else {
        "未知錯誤".to_string()
    };
    tracing::error!("伺服器發生未預期錯誤（panic）: {message}");

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        SERVER_ERROR_HTML,
    )
        .into_response()
}
