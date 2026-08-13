use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// 滾動進場動畫包裝元件，對照 spec.md 5.8 節第 1 項。
///
/// SSR 輸出時只是一個普通 `<div>`，內容一律完整可見；掛載到瀏覽器後才由
/// `scroll_reveal::reveal_on_scroll` 視情況（尊重 prefers-reduced-motion）
/// 加上暫時隱藏的 class 並透過 IntersectionObserver 監看，捲動進入可視範圍
/// 後淡入回原位；離開可視範圍後重新隱藏，再次進入時重新播放（v11 起可重複播放）。
#[component]
pub fn Reveal(
    /// 群組內依序錯開進場的延遲時間（毫秒），例如同排卡片可依索引乘上 110ms
    #[prop(optional, into)]
    delay_ms: u32,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    node_ref.on_load(move |el: leptos::web_sys::HtmlDivElement| {
        crate::scroll_reveal::reveal_on_scroll(el.unchecked_into(), delay_ms);
    });

    view! {
        <div node_ref=node_ref class=format!("transition-all duration-[550ms] ease-out {class}")>
            {children()}
        </div>
    }
}
