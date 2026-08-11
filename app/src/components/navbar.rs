use leptos::prelude::*;

use super::cta_button::{ButtonVariant, CtaButton};
use super::icons::{BookIcon, HamburgerIcon};

const NAV_LINKS: [(&str, &str); 5] = [
    ("/", "首頁"),
    ("/about", "關於知暖"),
    ("/courses", "課程介紹"),
    ("/news", "最新消息"),
    ("/contact", "聯絡我們"),
];

/// Header 導覽列，對照 spec.md 5.4 節 Header 規範
/// 「最新消息」／「聯絡我們」頁面將於 Phase C 建立，Phase B 階段點擊會顯示路由 fallback
#[component]
pub fn Navbar() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    view! {
        <header class="sticky top-0 z-50 bg-white">
            <div class="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
                <a href="/" class="flex items-center gap-2 text-brand-blue">
                    <BookIcon/>
                    <span class="flex flex-col leading-tight">
                        <span class="text-base font-bold text-brand-blue">"知暖學習工作室"</span>
                        <span class="text-xs uppercase tracking-widest text-slate-gray">
                            "Warm Learning Studio"
                        </span>
                    </span>
                </a>

                <nav class="hidden items-center gap-8 lg:flex">
                    {NAV_LINKS
                        .iter()
                        .map(|(href, label)| {
                            view! {
                                <a href=*href class="text-ink transition-colors hover:text-brand-blue">
                                    {*label}
                                </a>
                            }
                        })
                        .collect_view()}
                </nav>

                <div class="hidden lg:block">
                    <CtaButton href="/contact" label="預約諮詢" variant=ButtonVariant::Primary/>
                </div>

                <button
                    type="button"
                    class="text-brand-blue lg:hidden"
                    aria-label="開啟選單"
                    on:click=move |_| set_menu_open.update(|open| *open = !*open)
                >
                    <HamburgerIcon open=Signal::derive(move || menu_open.get())/>
                </button>
            </div>

            <Show when=move || menu_open.get()>
                <nav class="flex flex-col gap-4 border-t border-border-gray bg-white px-6 py-6 lg:hidden">
                    {NAV_LINKS
                        .iter()
                        .map(|(href, label)| {
                            view! {
                                <a
                                    href=*href
                                    class="text-ink"
                                    on:click=move |_| set_menu_open.set(false)
                                >
                                    {*label}
                                </a>
                            }
                        })
                        .collect_view()}
                    <CtaButton
                        href="/contact"
                        label="預約諮詢"
                        variant=ButtonVariant::Primary
                        class="w-full".to_string()
                    />
                </nav>
            </Show>
        </header>
    }
}
