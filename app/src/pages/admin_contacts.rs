use leptos::prelude::*;
use leptos_meta::Title;

use crate::admin_auth::friendly_error_message;
use crate::components::AdminNav;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContactSubmission {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub learning_stage: Option<String>,
    pub concern: Option<String>,
    pub message: Option<String>,
    pub is_read: bool,
    pub created_at: String,
}

/// 後台取得所有聯絡表單留言，對照 spec.md 6.5 節
#[server(GetAllContacts, "/api")]
pub async fn get_all_contacts() -> Result<Vec<ContactSubmission>, ServerFnError> {
    crate::admin_auth::require_admin().await?;
    let pool = expect_context::<sqlx::SqlitePool>();

    let rows: Vec<(i64, String, String, Option<String>, Option<String>, Option<String>, i64, String)> =
        sqlx::query_as(
            "SELECT id, name, phone, learning_stage, concern, message, is_read, created_at
             FROM contact_submissions ORDER BY created_at DESC",
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|(id, name, phone, learning_stage, concern, message, is_read, created_at)| ContactSubmission {
            id,
            name,
            phone,
            learning_stage,
            concern,
            message,
            is_read: is_read != 0,
            created_at,
        })
        .collect())
}

#[server(MarkContactRead, "/api")]
pub async fn mark_contact_read(id: i64) -> Result<(), ServerFnError> {
    crate::admin_auth::require_admin().await?;
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query("UPDATE contact_submissions SET is_read = 1 WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server(DeleteContact, "/api")]
pub async fn delete_contact(id: i64) -> Result<(), ServerFnError> {
    crate::admin_auth::require_admin().await?;
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query("DELETE FROM contact_submissions WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[component]
pub fn AdminContactsPage() -> impl IntoView {
    let (refresh_key, set_refresh_key) = signal(0u32);
    let contacts = Resource::new(move || refresh_key.get(), |_| get_all_contacts());
    let (expanded_id, set_expanded_id) = signal::<Option<i64>>(None);

    view! {
        <Title text="聯絡表單管理｜知暖學習工作室"/>
        <AdminNav/>

        <section class="bg-white">
            <div class="mx-auto max-w-5xl px-6 py-12">
                <h1 class="text-2xl font-bold text-brand-blue">"聯絡表單留言"</h1>

                <Suspense fallback=move || view! { <p class="mt-8 text-slate-gray">"載入中…"</p> }>
                    {move || {
                        contacts
                            .get()
                            .map(|result| match result {
                                Ok(items) if items.is_empty() => {
                                    view! { <p class="mt-8 text-slate-gray">"目前尚無聯絡表單留言"</p> }.into_any()
                                }
                                Ok(items) => {
                                    view! {
                                        <div class="mt-8 flex flex-col gap-3">
                                            {items
                                                .into_iter()
                                                .map(|item| {
                                                    let id = item.id;
                                                    let is_read = item.is_read;
                                                    view! {
                                                        <div class="rounded-2xl border border-border-gray p-4">
                                                            <div
                                                                class="flex cursor-pointer flex-wrap items-center justify-between gap-2"
                                                                on:click=move |_| {
                                                                    set_expanded_id
                                                                        .update(|cur| {
                                                                            *cur = if *cur == Some(id) { None } else { Some(id) };
                                                                        });
                                                                }
                                                            >
                                                                <div class="flex flex-wrap items-center gap-3">
                                                                    <Show when=move || !is_read>
                                                                        <span class="rounded-full bg-warm-amber px-2 py-0.5 text-xs font-medium text-ink">
                                                                            "未讀"
                                                                        </span>
                                                                    </Show>
                                                                    <span class="font-medium text-ink">{item.name.clone()}</span>
                                                                    <span class="text-sm text-slate-gray">{item.phone.clone()}</span>
                                                                    <span class="text-sm text-slate-gray">
                                                                        {item.learning_stage.clone().unwrap_or_else(|| "未填寫".to_string())}
                                                                    </span>
                                                                    <span class="text-sm text-slate-gray">
                                                                        {item.concern.clone().unwrap_or_else(|| "未填寫".to_string())}
                                                                    </span>
                                                                </div>
                                                                <span class="text-xs text-slate-gray">{item.created_at.clone()}</span>
                                                            </div>

                                                            <Show when=move || expanded_id.get() == Some(id)>
                                                                <div class="mt-3 flex flex-col gap-3 border-t border-border-gray pt-3">
                                                                    <p class="text-sm leading-[1.7] text-ink">
                                                                        {item
                                                                            .message
                                                                            .clone()
                                                                            .filter(|m| !m.is_empty())
                                                                            .unwrap_or_else(|| "（無留言內容）".to_string())}
                                                                    </p>
                                                                    <div class="flex gap-3">
                                                                        <Show when=move || !is_read>
                                                                            <button
                                                                                type="button"
                                                                                class="rounded-full bg-brand-blue px-4 py-1.5 text-xs font-medium text-white hover:bg-[#14294F]"
                                                                                on:click=move |ev| {
                                                                                    ev.stop_propagation();
                                                                                    leptos::task::spawn_local(async move {
                                                                                        if mark_contact_read(id).await.is_ok() {
                                                                                            set_refresh_key.update(|n| *n += 1);
                                                                                        }
                                                                                    });
                                                                                }
                                                                            >
                                                                                "標記已讀"
                                                                            </button>
                                                                        </Show>
                                                                        <button
                                                                            type="button"
                                                                            class="rounded-full border border-error-red px-4 py-1.5 text-xs font-medium text-error-red hover:bg-error-red/10"
                                                                            on:click=move |ev| {
                                                                                ev.stop_propagation();
                                                                                let confirmed = leptos::web_sys::window()
                                                                                    .and_then(|w| w.confirm_with_message("確定要刪除這筆留言嗎？").ok())
                                                                                    .unwrap_or(false);
                                                                                if confirmed {
                                                                                    leptos::task::spawn_local(async move {
                                                                                        if delete_contact(id).await.is_ok() {
                                                                                            set_refresh_key.update(|n| *n += 1);
                                                                                        }
                                                                                    });
                                                                                }
                                                                            }
                                                                        >
                                                                            "刪除"
                                                                        </button>
                                                                    </div>
                                                                </div>
                                                            </Show>
                                                        </div>
                                                    }
                                                })
                                                .collect_view()}
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
