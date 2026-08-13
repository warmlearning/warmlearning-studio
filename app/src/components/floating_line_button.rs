use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

use super::icons::ChatIcon;

const LINE_URL: &str = "https://line.me/R/ti/p/@891ivojl";

/// 固定式浮動 LINE CTA 按鈕，對照 spec.md 5.8 節第 3 項。只在對外頁面顯示，
/// 由呼叫端（AppShell）決定是否要渲染（/admin/* 不渲染）。
#[component]
pub fn FloatingLineButton() -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::A>::new();

    node_ref.on_load(move |el: leptos::web_sys::HtmlAnchorElement| {
        crate::scroll_reveal::setup_floating_line_button(el.unchecked_into());
    });

    view! {
        <a
            node_ref=node_ref
            href=LINE_URL
            target="_blank"
            rel="noopener noreferrer"
            aria-label="加入 LINE 諮詢"
            class="fixed bottom-6 right-6 z-40 flex h-14 w-14 items-center justify-center rounded-full bg-line-green text-white opacity-0 shadow-lg pointer-events-none sm:bottom-8 sm:right-8"
        >
            <ChatIcon/>
        </a>
    }
}
