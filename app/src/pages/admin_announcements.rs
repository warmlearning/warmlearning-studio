use leptos::prelude::*;
use leptos_meta::Title;

use crate::admin_auth::friendly_error_message;
use crate::components::AdminNav;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnnouncementListItem {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub published_at: Option<String>,
}

/// 後台取得所有公告（含草稿），對照 spec.md 6.3、8 節
#[server(GetAllAnnouncements, "/api")]
pub async fn get_all_announcements() -> Result<Vec<AnnouncementListItem>, ServerFnError> {
    crate::admin_auth::require_admin().await?;
    let pool = expect_context::<sqlx::SqlitePool>();

    let rows: Vec<(i64, String, String, Option<String>)> =
        sqlx::query_as("SELECT id, title, status, published_at FROM announcements ORDER BY id DESC")
            .fetch_all(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|(id, title, status, published_at)| AnnouncementListItem { id, title, status, published_at })
        .collect())
}

#[server(DeleteAnnouncement, "/api")]
pub async fn delete_announcement(id: i64) -> Result<(), ServerFnError> {
    crate::admin_auth::require_admin().await?;
    let pool = expect_context::<sqlx::SqlitePool>();

    sqlx::query("DELETE FROM announcements WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./data/uploads".to_string());
    let _ = std::fs::remove_file(format!("{upload_dir}/announcements/{id}.jpg"));
    let _ = std::fs::remove_file(format!("{upload_dir}/announcements/{id}-thumb.jpg"));

    Ok(())
}

#[component]
pub fn AdminAnnouncementsListPage() -> impl IntoView {
    let (refresh_key, set_refresh_key) = signal(0u32);
    let announcements = Resource::new(move || refresh_key.get(), |_| get_all_announcements());

    view! {
        <Title text="公告管理｜知暖學習工作室"/>
        <AdminNav/>

        <section class="bg-white">
            <div class="mx-auto max-w-5xl px-6 py-12">
                <div class="flex flex-wrap items-center justify-between gap-4">
                    <h1 class="text-2xl font-bold text-brand-blue">"公告管理"</h1>
                    <a
                        href="/admin/announcements/new"
                        class="rounded-full bg-brand-blue px-6 py-2.5 text-sm font-medium text-white hover:bg-[#14294F]"
                    >
                        "新增公告"
                    </a>
                </div>

                <Suspense fallback=move || view! { <p class="mt-8 text-slate-gray">"載入中…"</p> }>
                    {move || {
                        announcements
                            .get()
                            .map(|result| match result {
                                Ok(items) if items.is_empty() => {
                                    view! { <p class="mt-8 text-slate-gray">"目前尚無公告"</p> }.into_any()
                                }
                                Ok(items) => {
                                    view! {
                                        <div class="mt-8 overflow-x-auto">
                                            <table class="w-full text-left text-sm">
                                                <thead>
                                                    <tr class="border-b border-border-gray text-slate-gray">
                                                        <th class="py-2 pr-4 font-medium">"標題"</th>
                                                        <th class="py-2 pr-4 font-medium">"狀態"</th>
                                                        <th class="py-2 pr-4 font-medium">"發布日期"</th>
                                                        <th class="py-2 font-medium">"操作"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {items
                                                        .into_iter()
                                                        .map(|item| {
                                                            let id = item.id;
                                                            let status_label = if item.status == "published" {
                                                                "已發布"
                                                            } else {
                                                                "草稿"
                                                            };
                                                            view! {
                                                                <tr class="border-b border-border-gray">
                                                                    <td class="py-3 pr-4 text-ink">{item.title.clone()}</td>
                                                                    <td class="py-3 pr-4">
                                                                        <span class=if item.status == "published" {
                                                                            "rounded-full bg-mist-blue px-2 py-0.5 text-xs text-brand-blue"
                                                                        } else {
                                                                            "rounded-full bg-border-gray px-2 py-0.5 text-xs text-slate-gray"
                                                                        }>{status_label}</span>
                                                                    </td>
                                                                    <td class="py-3 pr-4 text-slate-gray">
                                                                        {item.published_at.clone().unwrap_or_else(|| "—".to_string())}
                                                                    </td>
                                                                    <td class="py-3">
                                                                        <div class="flex gap-3">
                                                                            <a
                                                                                href=format!("/admin/announcements/{id}/edit")
                                                                                class="text-brand-blue hover:underline"
                                                                            >
                                                                                "編輯"
                                                                            </a>
                                                                            <button
                                                                                type="button"
                                                                                class="text-error-red hover:underline"
                                                                                on:click=move |_| {
                                                                                    let confirmed = leptos::web_sys::window()
                                                                                        .and_then(|w| w.confirm_with_message("確定要刪除這篇公告嗎？此動作無法復原。").ok())
                                                                                        .unwrap_or(false);
                                                                                    if confirmed {
                                                                                        leptos::task::spawn_local(async move {
                                                                                            if delete_announcement(id).await.is_ok() {
                                                                                                set_refresh_key.update(|n| *n += 1);
                                                                                            }
                                                                                        });
                                                                                    }
                                                                                }
                                                                            >
                                                                                "刪除"
                                                                            </button>
                                                                        </div>
                                                                    </td>
                                                                </tr>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </tbody>
                                            </table>
                                        </div>
                                    }
                                        .into_any()
                                }
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
