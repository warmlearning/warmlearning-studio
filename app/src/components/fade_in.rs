use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// 掛載時淡入＋輕微向上位移的包裝元件，對照 spec.md 5.8 節第 2 項
/// （骨架屏載入完成後的真實內容、聯絡表單送出訊息淡入效果）。
///
/// 與 `Reveal` 的差異在於：`Reveal` 是捲動進入可視範圍才觸發，這個元件則是
/// 一掛載到瀏覽器就立刻觸發，用於非捲動情境的內容出現（例如非同步資料載入
/// 完成、表單送出後顯示的訊息）。SSR 輸出時同樣是完整可見的普通 `<div>`。
#[component]
pub fn FadeIn(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    node_ref.on_load(move |el: leptos::web_sys::HtmlDivElement| {
        crate::scroll_reveal::fade_in_on_mount(el.unchecked_into());
    });

    view! {
        <div node_ref=node_ref class=format!("transition-all duration-500 ease-out {class}")>
            {children()}
        </div>
    }
}
