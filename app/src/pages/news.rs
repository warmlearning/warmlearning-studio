use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::{use_params_map, use_query_map};

use crate::admin_auth::friendly_error_message;
use crate::components::{Card, FadeIn, Reveal};

const SKELETON_CARD_COUNT: usize = 6;

/// 最新消息列表載入中的骨架屏卡片，對照 spec.md 5.8 節第 2 項
#[component]
fn NewsCardSkeleton() -> impl IntoView {
    view! {
        <div class="flex h-full flex-col overflow-hidden rounded-2xl bg-white shadow-md">
            <div class="aspect-[4/3] w-full animate-pulse bg-pale-blue"></div>
            <div class="flex flex-1 flex-col gap-3 p-5">
                <div class="h-3 w-16 animate-pulse rounded bg-pale-blue"></div>
                <div class="h-5 w-3/4 animate-pulse rounded bg-pale-blue"></div>
                <div class="h-3 w-full animate-pulse rounded bg-pale-blue"></div>
                <div class="h-3 w-5/6 animate-pulse rounded bg-pale-blue"></div>
            </div>
        </div>
    }
}

#[cfg(feature = "ssr")]
const PAGE_SIZE: i64 = 9;
#[cfg(feature = "ssr")]
const EXCERPT_MAX_CHARS: usize = 100;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnnouncementSummary {
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub cover_image_path: Option<String>,
    pub published_at: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnnouncementListPage {
    pub items: Vec<AnnouncementSummary>,
    pub page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnnouncementDetail {
    pub title: String,
    pub content: String,
    pub cover_image_path: Option<String>,
    pub published_at: String,
}

/// 去除 HTML 標籤並截取前 N 個字，用於列表頁摘要（spec.md 4.4 節）
#[cfg(feature = "ssr")]
fn make_excerpt(html: &str, max_chars: usize) -> String {
    let mut plain = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => plain.push(c),
            _ => {}
        }
    }
    let plain = plain.split_whitespace().collect::<Vec<_>>().join(" ");
    let char_count = plain.chars().count();
    let truncated: String = plain.chars().take(max_chars).collect();
    if char_count > max_chars {
        format!("{truncated}…")
    } else {
        truncated
    }
}

/// 取得已發布公告列表（分頁），對照 spec.md 4.4／8 節
#[server(GetPublishedAnnouncements, "/api")]
pub async fn get_published_announcements(page: i64) -> Result<AnnouncementListPage, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    let page = page.max(1);
    let offset = (page - 1) * PAGE_SIZE;

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM announcements WHERE status = 'published' AND published_at <= datetime('now')",
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let total_pages = ((total as f64) / (PAGE_SIZE as f64)).ceil().max(1.0) as i64;

    let rows: Vec<(String, String, String, Option<String>, String)> = sqlx::query_as(
        "SELECT slug, title, content, cover_image_path, published_at
         FROM announcements
         WHERE status = 'published' AND published_at <= datetime('now')
         ORDER BY published_at DESC
         LIMIT ? OFFSET ?",
    )
    .bind(PAGE_SIZE)
    .bind(offset)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let items = rows
        .into_iter()
        .map(|(slug, title, content, cover_image_path, published_at)| AnnouncementSummary {
            slug,
            title,
            excerpt: make_excerpt(&content, EXCERPT_MAX_CHARS),
            cover_image_path,
            published_at,
        })
        .collect();

    Ok(AnnouncementListPage { items, page, total_pages })
}

/// 取得單篇已發布公告，對照 spec.md 4.4／8 節
#[server(GetAnnouncementBySlug, "/api")]
pub async fn get_announcement_by_slug(slug: String) -> Result<Option<AnnouncementDetail>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();

    let row: Option<(String, String, Option<String>, String)> = sqlx::query_as(
        "SELECT title, content, cover_image_path, published_at
         FROM announcements
         WHERE slug = ? AND status = 'published' AND published_at <= datetime('now')",
    )
    .bind(&slug)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(row.map(|(title, content, cover_image_path, published_at)| AnnouncementDetail {
        title,
        content,
        cover_image_path,
        published_at,
    }))
}

fn format_date(published_at: &str) -> String {
    published_at.chars().take(10).collect()
}

#[component]
pub fn NewsListPage() -> impl IntoView {
    let query = use_query_map();
    let page_num = Memo::new(move |_| {
        query.with(|q| q.get("page").and_then(|p| p.parse::<i64>().ok()).unwrap_or(1).max(1))
    });
    let announcements = Resource::new(move || page_num.get(), get_published_announcements);

    view! {
        <Title text="知暖最新消息｜Warm Learning Studio 英文課程與學習分享"/>
        <Meta
            name="description"
            content="掌握知暖學習工作室（Warm Learning Studio）最新課程資訊、英文學習文章與活動公告，陪伴高雄幼兒到成人學習者持續提升英文能力與自主學習力。"
        />

        <section class="bg-mist-blue">
            <div class="mx-auto max-w-7xl px-6 py-16 lg:py-24">
                <h1 class="text-center text-4xl font-bold text-brand-blue">"最新消息"</h1>

                <Suspense fallback=move || {
                    view! {
                        <div class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
                            {(0..SKELETON_CARD_COUNT).map(|_| view! { <NewsCardSkeleton/> }).collect_view()}
                        </div>
                    }
                }>
                    {move || {
                        announcements
                            .get()
                            .map(|result| match result {
                                Ok(data) if data.items.is_empty() => {
                                    view! {
                                        <p class="mt-12 text-center text-slate-gray">"目前尚無公告"</p>
                                    }
                                        .into_any()
                                }
                                Ok(AnnouncementListPage { items, page, total_pages }) => {
                                    view! {
                                        <FadeIn class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3"
                                            .to_string()>
                                            {items
                                                .into_iter()
                                                .map(|item| {
                                                    view! {
                                                        <a href=format!("/news/{}", item.slug)>
                                                            <Card class="flex h-full flex-col overflow-hidden".to_string()>
                                                                {item
                                                                    .cover_image_path
                                                                    .clone()
                                                                    .map(|src| {
                                                                        view! {
                                                                            <img
                                                                                src=src
                                                                                alt=item.title.clone()
                                                                                class="aspect-[4/3] w-full rounded-t-2xl object-cover"
                                                                            />
                                                                        }
                                                                    })}
                                                                <div class="flex flex-1 flex-col gap-2 p-5">
                                                                    <p class="text-sm text-slate-gray">
                                                                        {format_date(&item.published_at)}
                                                                    </p>
                                                                    <h2 class="text-2xl font-bold text-ink">
                                                                        {item.title.clone()}
                                                                    </h2>
                                                                    <p class="flex-1 text-base leading-[1.7] text-slate-gray">
                                                                        {item.excerpt.clone()}
                                                                    </p>
                                                                </div>
                                                            </Card>
                                                        </a>
                                                    }
                                                })
                                                .collect_view()}
                                        </FadeIn>

                                        <div class="mt-12 flex items-center justify-center gap-6">
                                            {(page > 1)
                                                .then(|| {
                                                    view! {
                                                        <a
                                                            href=format!("/news?page={}", page - 1)
                                                            class="text-base font-medium text-brand-blue hover:underline"
                                                        >
                                                            "← 上一頁"
                                                        </a>
                                                    }
                                                })}
                                            <span class="text-base text-slate-gray">
                                                {format!("第 {} / {} 頁", page, total_pages)}
                                            </span>
                                            {(page < total_pages)
                                                .then(|| {
                                                    view! {
                                                        <a
                                                            href=format!("/news?page={}", page + 1)
                                                            class="text-base font-medium text-brand-blue hover:underline"
                                                        >
                                                            "下一頁 →"
                                                        </a>
                                                    }
                                                })}
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <p class="mt-12 text-center text-error-red">
                                            {friendly_error_message(&e)}
                                        </p>
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
pub fn NewsDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = Memo::new(move |_| params.with(|p| p.get("slug").unwrap_or_default()));
    let announcement = Resource::new(move || slug.get(), get_announcement_by_slug);

    view! {
        <section class="bg-white">
            <Reveal class="mx-auto max-w-3xl px-6 py-16 lg:py-24".to_string()>
                <Suspense fallback=move || view! { <p class="text-center text-slate-gray">"載入中…"</p> }>
                    {move || {
                        announcement
                            .get()
                            .map(|result| match result {
                                Ok(Some(item)) => {
                                    view! {
                                        <Title text=format!("{}｜知暖最新消息", item.title)/>
                                        <article>
                                            <h1 class="text-4xl font-bold text-brand-blue">{item.title.clone()}</h1>
                                            <p class="mt-3 text-base text-slate-gray">
                                                {format_date(&item.published_at)}
                                            </p>
                                            {item
                                                .cover_image_path
                                                .clone()
                                                .map(|src| {
                                                    view! {
                                                        <img
                                                            src=src
                                                            alt=item.title.clone()
                                                            class="mt-8 aspect-[16/9] w-full rounded-2xl object-cover"
                                                        />
                                                    }
                                                })}
                                            <div
                                                class="prose prose-slate mt-8 max-w-none text-base leading-[1.7] text-ink"
                                                inner_html=item.content.clone()
                                            ></div>
                                        </article>
                                        <a
                                            href="/news"
                                            class="mt-12 inline-block text-base font-medium text-brand-blue hover:underline"
                                        >
                                            "← 返回最新消息列表"
                                        </a>
                                    }
                                        .into_any()
                                }
                                Ok(None) => {
                                    view! {
                                        <p class="text-center text-slate-gray">"找不到這篇公告，可能已被下架或網址有誤。"</p>
                                        <div class="mt-6 text-center">
                                            <a href="/news" class="text-base font-medium text-brand-blue hover:underline">
                                                "← 返回最新消息列表"
                                            </a>
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <p class="text-center text-error-red">{friendly_error_message(&e)}</p>
                                    }
                                        .into_any()
                                }
                            })
                    }}
                </Suspense>
            </Reveal>
        </section>
    }
}
