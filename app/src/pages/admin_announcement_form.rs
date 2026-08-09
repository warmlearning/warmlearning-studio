use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::Form;
use leptos_router::hooks::use_params_map;

use crate::admin_auth::friendly_error_message;
use crate::components::AdminNav;

#[cfg(feature = "ssr")]
const MAX_IMAGE_BYTES: usize = 5 * 1024 * 1024;
#[cfg(feature = "ssr")]
const MAX_DISPLAY_DIMENSION: u32 = 1600;
#[cfg(feature = "ssr")]
const THUMB_DIMENSION: u32 = 400;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnnouncementEditData {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
    pub published_at: String,
    pub cover_image_path: Option<String>,
}

/// 後台取得單篇公告（含草稿），供編輯表單預填欄位
#[server(GetAnnouncementForEdit, "/api")]
pub async fn get_announcement_for_edit(id: i64) -> Result<Option<AnnouncementEditData>, ServerFnError> {
    crate::admin_auth::require_admin().await?;
    let pool = expect_context::<sqlx::SqlitePool>();

    let row: Option<(String, String, String, String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT title, slug, content, status, published_at, cover_image_path FROM announcements WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(row.map(|(title, slug, content, status, published_at, cover_image_path)| AnnouncementEditData {
        title,
        slug,
        content,
        status,
        published_at: published_at.unwrap_or_default(),
        cover_image_path,
    }))
}

#[cfg(feature = "ssr")]
fn process_cover_image(
    bytes: &[u8],
    upload_dir: &str,
    announcement_id: i64,
) -> Result<String, ServerFnError> {
    if bytes.len() > MAX_IMAGE_BYTES {
        return Err(ServerFnError::new("圖片檔案大小超過 5MB 限制"));
    }

    let img = image::load_from_memory(bytes)
        .map_err(|e| ServerFnError::new(format!("圖片格式不支援，僅接受 jpg、png、webp（{e}）")))?;

    let dir = format!("{upload_dir}/announcements");
    std::fs::create_dir_all(&dir).map_err(|e| ServerFnError::new(format!("無法建立圖片目錄: {e}")))?;

    let display = if img.width() > MAX_DISPLAY_DIMENSION || img.height() > MAX_DISPLAY_DIMENSION {
        img.resize(MAX_DISPLAY_DIMENSION, MAX_DISPLAY_DIMENSION, image::imageops::FilterType::Lanczos3)
    } else {
        img.clone()
    };
    let thumb = img.thumbnail(THUMB_DIMENSION, THUMB_DIMENSION);

    display
        .to_rgb8()
        .save_with_format(format!("{dir}/{announcement_id}.jpg"), image::ImageFormat::Jpeg)
        .map_err(|e| ServerFnError::new(format!("原圖儲存失敗: {e}")))?;
    thumb
        .to_rgb8()
        .save_with_format(format!("{dir}/{announcement_id}-thumb.jpg"), image::ImageFormat::Jpeg)
        .map_err(|e| ServerFnError::new(format!("縮圖儲存失敗: {e}")))?;

    Ok(format!("/uploads/announcements/{announcement_id}.jpg"))
}

/// 新增／編輯公告，含封面圖片上傳（壓縮並產生縮圖），對照 spec.md 6.4、9.5 節
#[server(SaveAnnouncement, "/api", input = leptos::server_fn::codec::MultipartFormData)]
pub async fn save_announcement(
    data: leptos::server_fn::codec::MultipartData,
) -> Result<(), ServerFnError> {
    crate::admin_auth::require_admin().await?;

    let mut multipart = data.into_inner().expect("multipart data on server");

    let mut id: Option<i64> = None;
    let mut title = String::new();
    let mut slug_input = String::new();
    let mut content = String::new();
    let mut status = String::from("draft");
    let mut published_at_input = String::new();
    let mut image_bytes: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "id" => {
                let bytes = field.bytes().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                id = String::from_utf8_lossy(&bytes).trim().parse().ok();
            }
            "title" => {
                let bytes = field.bytes().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                title = String::from_utf8_lossy(&bytes).trim().to_string();
            }
            "slug" => {
                let bytes = field.bytes().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                slug_input = String::from_utf8_lossy(&bytes).trim().to_string();
            }
            "content" => {
                let bytes = field.bytes().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                content = String::from_utf8_lossy(&bytes).to_string();
            }
            "status" => {
                let bytes = field.bytes().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                status = String::from_utf8_lossy(&bytes).trim().to_string();
            }
            "published_at" => {
                let bytes = field.bytes().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                published_at_input = String::from_utf8_lossy(&bytes).trim().to_string();
            }
            "cover_image" => {
                let bytes = field.bytes().await.map_err(|e| ServerFnError::new(e.to_string()))?;
                if !bytes.is_empty() {
                    image_bytes = Some(bytes.to_vec());
                }
            }
            _ => {
                let _ = field.bytes().await;
            }
        }
    }

    if title.is_empty() {
        return Err(ServerFnError::new("請填寫標題"));
    }
    if !matches!(status.as_str(), "draft" | "published") {
        status = "draft".to_string();
    }

    let pool = expect_context::<sqlx::SqlitePool>();

    let announcement_id: i64 = match id {
        Some(existing_id) => existing_id,
        None => {
            let result = sqlx::query(
                "INSERT INTO announcements
                 (slug, title, content, cover_image_path, status, published_at, created_at, updated_at)
                 VALUES (?, '', '', NULL, 'draft', datetime('now'), datetime('now'), datetime('now'))",
            )
            .bind(format!("draft-{}", uuid_like()))
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
            result.last_insert_rowid()
        }
    };

    let existing_cover_image_path: Option<String> =
        sqlx::query_scalar("SELECT cover_image_path FROM announcements WHERE id = ?")
            .bind(announcement_id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .flatten();

    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./data/uploads".to_string());
    let cover_image_path = match image_bytes {
        Some(bytes) => Some(process_cover_image(&bytes, &upload_dir, announcement_id)?),
        None => existing_cover_image_path,
    };

    let final_slug = if slug_input.is_empty() {
        announcement_id.to_string()
    } else {
        let normalized = slug_input
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' })
            .collect::<String>();
        let taken: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM announcements WHERE slug = ? AND id != ?")
            .bind(&normalized)
            .bind(announcement_id)
            .fetch_one(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        if taken > 0 {
            return Err(ServerFnError::new("此網址代稱已被使用，請更換一個"));
        }
        normalized
    };

    let published_at_expr = if published_at_input.is_empty() { "datetime('now')" } else { "?" };
    let sql = format!(
        "UPDATE announcements
         SET slug = ?, title = ?, content = ?, cover_image_path = ?, status = ?,
             published_at = {published_at_expr}, updated_at = datetime('now')
         WHERE id = ?"
    );
    // published_at_expr 只會是 "datetime('now')" 或 "?" 兩個固定字面值之一（非使用者輸入），
    // 故以 AssertSqlSafe 標示已人工確認無 SQL 注入風險
    let mut query = sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(final_slug)
        .bind(title)
        .bind(content)
        .bind(cover_image_path)
        .bind(status);
    if !published_at_input.is_empty() {
        query = query.bind(published_at_input.replace('T', " "));
    }
    query
        .bind(announcement_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect("/admin/announcements");
    Ok(())
}

#[cfg(feature = "ssr")]
fn uuid_like() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or_default();
    format!("{nanos:x}")
}

#[component]
pub fn AdminAnnouncementFormPage() -> impl IntoView {
    let params = use_params_map();
    let id = Memo::new(move |_| params.with(|p| p.get("id").and_then(|s| s.parse::<i64>().ok())));

    let initial = Resource::new(
        move || id.get(),
        |id| async move {
            match id {
                Some(id) => get_announcement_for_edit(id).await,
                None => Ok(None),
            }
        },
    );

    view! {
        <Title text="公告編輯｜知暖學習工作室"/>
        <AdminNav/>

        <section class="bg-white">
            <div class="mx-auto max-w-3xl px-6 py-12">
                <h1 class="text-2xl font-bold text-brand-blue">
                    {move || if id.get().is_some() { "編輯公告" } else { "新增公告" }}
                </h1>

                <Suspense fallback=move || view! { <p class="mt-8 text-slate-gray">"載入中…"</p> }>
                    {move || {
                        initial
                            .get()
                            .map(|result| match result {
                                Ok(data) => view! { <AnnouncementForm id=id.get() initial=data/> }.into_any(),
                                Err(e) => {
                                    view! {
                                        <div class="mt-8">
                                            <p class="text-error-red">{friendly_error_message(&e)}</p>
                                            <a href="/admin/login" class="mt-2 inline-block text-sm text-brand-blue hover:underline">
                                                "請重新登入"
                                            </a>
                                        </div>
                                    }
                                        .into_any()
                                }
                            })
                    }}
                </Suspense>
            </div>
        </section>
    }
}

#[component]
fn AnnouncementForm(id: Option<i64>, initial: Option<AnnouncementEditData>) -> impl IntoView {
    let initial_title = initial.as_ref().map(|d| d.title.clone()).unwrap_or_default();
    let initial_slug = initial.as_ref().map(|d| d.slug.clone()).unwrap_or_default();
    let initial_content = initial.as_ref().map(|d| d.content.clone()).unwrap_or_default();
    let initial_status = initial.as_ref().map(|d| d.status.clone()).unwrap_or_else(|| "draft".to_string());
    let initial_published_at = initial
        .as_ref()
        .map(|d| d.published_at.replace(' ', "T").chars().take(16).collect::<String>())
        .unwrap_or_default();
    let existing_cover_image = initial.as_ref().and_then(|d| d.cover_image_path.clone());

    let (content_html, set_content_html) = signal(initial_content.clone());
    let editor_ref = NodeRef::<leptos::html::Div>::new();

    let sync_content = move || {
        if let Some(el) = editor_ref.get() {
            set_content_html.set(el.inner_html());
        }
    };

    let exec = move |command: &'static str| {
        move |_| {
            use leptos::wasm_bindgen::JsCast;
            if let Some(document) =
                leptos::web_sys::window().and_then(|w| w.document()).and_then(|d| d.dyn_into::<leptos::web_sys::HtmlDocument>().ok())
            {
                let _ = document.exec_command(command);
            }
            sync_content();
        }
    };

    let insert_image = move |_| {
        use leptos::wasm_bindgen::JsCast;
        if let Some(window) = leptos::web_sys::window() {
            if let Ok(Some(url)) = window.prompt_with_message("輸入圖片網址（例如 /img/xxx.png）") {
                if !url.is_empty() {
                    if let Some(document) =
                        window.document().and_then(|d| d.dyn_into::<leptos::web_sys::HtmlDocument>().ok())
                    {
                        let _ = document.exec_command_with_show_ui_and_value("insertImage", false, &url);
                    }
                }
            }
        }
        sync_content();
    };

    let save_action_url = <SaveAnnouncement as leptos::server_fn::ServerFn>::url().to_string();

    view! {
        <Form
            action=save_action_url
            method="post"
            enctype="multipart/form-data".to_string()
            attr:class="mt-8 flex flex-col gap-5"
        >
            {id.map(|id| view! { <input type="hidden" name="id" value=id.to_string()/> })}

            <div class="flex flex-col gap-1.5">
                <label for="ann-title" class="text-sm font-medium text-ink">
                    "標題"
                </label>
                <input
                    id="ann-title"
                    type="text"
                    name="title"
                    required
                    value=initial_title
                    class="rounded-xl border border-border-gray px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                />
            </div>

            <div class="flex flex-col gap-1.5">
                <label for="ann-slug" class="text-sm font-medium text-ink">
                    "網址代稱（選填，留空則自動產生）"
                </label>
                <input
                    id="ann-slug"
                    type="text"
                    name="slug"
                    value=initial_slug
                    placeholder="例如 summer-camp-2026"
                    class="rounded-xl border border-border-gray px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                />
            </div>

            <div class="flex flex-col gap-1.5">
                <span class="text-sm font-medium text-ink">"內文"</span>
                <div class="flex flex-wrap gap-2 rounded-t-xl border border-b-0 border-border-gray bg-mist-blue p-2">
                    <button
                        type="button"
                        class="rounded-md bg-white px-3 py-1 text-sm font-bold shadow-sm"
                        on:click=exec("bold")
                    >
                        "B"
                    </button>
                    <button
                        type="button"
                        class="rounded-md bg-white px-3 py-1 text-sm shadow-sm"
                        on:click=exec("insertUnorderedList")
                    >
                        "• 項目符號"
                    </button>
                    <button
                        type="button"
                        class="rounded-md bg-white px-3 py-1 text-sm shadow-sm"
                        on:click=exec("insertLineBreak")
                    >
                        "換行"
                    </button>
                    <button
                        type="button"
                        class="rounded-md bg-white px-3 py-1 text-sm shadow-sm"
                        on:click=insert_image
                    >
                        "插入圖片"
                    </button>
                </div>
                <div
                    node_ref=editor_ref
                    contenteditable="true"
                    inner_html=initial_content
                    on:input=move |_| sync_content()
                    class="min-h-[220px] rounded-b-xl border border-border-gray px-4 py-3 text-sm leading-[1.7] text-ink outline-none focus:border-brand-blue"
                ></div>
                <input type="hidden" name="content" prop:value=move || content_html.get()/>
            </div>

            <div class="flex flex-col gap-1.5">
                <label for="ann-image" class="text-sm font-medium text-ink">
                    "封面圖片（選填，jpg／png／webp，上限 5MB）"
                </label>
                {existing_cover_image
                    .map(|src| {
                        view! {
                            <img src=src alt="目前封面圖片" class="h-32 w-32 rounded-xl object-cover"/>
                        }
                    })}
                <input
                    id="ann-image"
                    type="file"
                    name="cover_image"
                    accept="image/jpeg,image/png,image/webp"
                    class="text-sm text-ink"
                />
            </div>

            <div class="flex flex-col gap-1.5">
                <label for="ann-status" class="text-sm font-medium text-ink">
                    "發布狀態"
                </label>
                <select
                    id="ann-status"
                    name="status"
                    class="rounded-xl border border-border-gray bg-white px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                >
                    <option value="draft" selected=initial_status == "draft">
                        "草稿"
                    </option>
                    <option value="published" selected=initial_status == "published">
                        "發布"
                    </option>
                </select>
            </div>

            <div class="flex flex-col gap-1.5">
                <label for="ann-published-at" class="text-sm font-medium text-ink">
                    "發布日期（選填，留空為現在；可設定未來時間排程發布）"
                </label>
                <input
                    id="ann-published-at"
                    type="datetime-local"
                    name="published_at"
                    value=initial_published_at
                    class="rounded-xl border border-border-gray px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                />
            </div>

            <button
                type="submit"
                class="mt-2 rounded-full bg-brand-blue px-8 py-3 font-medium text-white transition-all hover:bg-[#14294F]"
            >
                "儲存"
            </button>
        </Form>
    }
}
