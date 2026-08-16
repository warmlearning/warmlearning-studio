use leptos::prelude::*;

/// 卡片統一風格：大圓角＋極輕陰影，hover 時陰影加深並輕微上浮（spec.md 5.1／5.5 節）
#[component]
pub fn Card(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            id=id
            class=format!(
                "rounded-2xl bg-white shadow-md transition-all duration-200 hover:shadow-xl hover:-translate-y-1 {class}",
            )
        >
            {children()}
        </div>
    }
}

/// 圖片準備中的通用佔位區塊，統一標示外觀，方便日後搜尋替換
/// TODO: 待對應素材到位後，改用真實圖片並移除此元件呼叫
#[component]
pub fn ImagePlaceholder(
    #[prop(into)] label: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div class=format!(
            "flex items-center justify-center bg-pale-blue text-center text-base text-slate-gray {class}",
        )>
            <span>{label}</span>
        </div>
    }
}
