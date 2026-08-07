use leptos::prelude::*;

/// Phase A 骨架佔位首頁，實際區塊內容於 Phase B 依 spec.md 4.1 節建立
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col items-center justify-center gap-4 bg-mist-blue px-6 text-center">
            <p class="font-handwriting text-sky-blue text-lg">"知足上進，溫暖而堅定"</p>
            <h1 class="font-sans text-4xl font-bold text-brand-blue">"知暖學習工作室"</h1>
            <p class="text-slate-gray text-base">"專案骨架建置中，Phase B 將補上完整首頁內容。"</p>
        </div>
    }
}
