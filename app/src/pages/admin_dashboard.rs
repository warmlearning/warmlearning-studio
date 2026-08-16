use leptos::prelude::*;
use leptos_meta::Title;

use crate::admin_auth::friendly_error_message;
use crate::components::AdminNav;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DashboardStats {
    pub published_count: i64,
    pub draft_count: i64,
    pub unread_contacts_count: i64,
}

/// 後台儀表板統計數字，對照 spec.md 6.2 節
#[server(GetDashboardStats, "/api")]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    crate::admin_auth::require_admin().await?;
    let pool = expect_context::<sqlx::SqlitePool>();

    let published_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM announcements WHERE status = 'published'")
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let draft_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM announcements WHERE status = 'draft'")
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let unread_contacts_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM contact_submissions WHERE is_read = 0")
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(DashboardStats { published_count, draft_count, unread_contacts_count })
}

#[component]
pub fn AdminDashboardPage() -> impl IntoView {
    let stats = Resource::new(|| (), |_| get_dashboard_stats());

    view! {
        <Title text="後台儀表板｜知暖學習工作室"/>
        <AdminNav/>

        <section class="bg-white">
            <div class="mx-auto max-w-7xl px-6 py-12">
                <h1 class="text-3xl font-bold text-brand-blue">"儀表板"</h1>

                <Suspense fallback=move || view! { <p class="mt-8 text-slate-gray">"載入中…"</p> }>
                    {move || {
                        stats
                            .get()
                            .map(|result| match result {
                                Ok(s) => {
                                    view! {
                                        <div class="mt-8 grid grid-cols-1 gap-6 sm:grid-cols-3">
                                            <div class="rounded-2xl bg-mist-blue p-6 text-center">
                                                <p class="text-4xl font-bold text-brand-blue">{s.published_count}</p>
                                                <p class="mt-1 text-base text-slate-gray">"已發布公告"</p>
                                            </div>
                                            <div class="rounded-2xl bg-mist-blue p-6 text-center">
                                                <p class="text-4xl font-bold text-brand-blue">{s.draft_count}</p>
                                                <p class="mt-1 text-base text-slate-gray">"草稿公告"</p>
                                            </div>
                                            <div class="rounded-2xl bg-mist-blue p-6 text-center">
                                                <p class="text-4xl font-bold text-brand-blue">
                                                    {s.unread_contacts_count}
                                                </p>
                                                <p class="mt-1 text-base text-slate-gray">"未讀聯絡表單"</p>
                                            </div>
                                        </div>

                                        <div class="mt-10 flex flex-wrap gap-4">
                                            <a
                                                href="/admin/announcements/new"
                                                class="rounded-full bg-brand-blue px-6 py-3 text-base font-medium text-white hover:bg-[#14294F]"
                                            >
                                                "新增公告"
                                            </a>
                                            <a
                                                href="/admin/contacts"
                                                class="rounded-full border border-brand-blue px-6 py-3 text-base font-medium text-brand-blue hover:bg-mist-blue"
                                            >
                                                "查看聯絡表單"
                                            </a>
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <div class="mt-8">
                                            <p class="text-error-red">{friendly_error_message(&e)}</p>
                                            <a href="/admin/login" class="mt-2 inline-block text-base text-brand-blue hover:underline">
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
