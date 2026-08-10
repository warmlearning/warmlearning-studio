use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::{ButtonVariant, CtaButton};

/// 404 錯誤頁，對照 spec.md 11.1 節與 tech-implementation.md 第 5 節文案
#[component]
pub fn NotFoundPage() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let response = expect_context::<leptos_axum::ResponseOptions>();
        response.set_status(http::StatusCode::NOT_FOUND);
    }

    view! {
        <Title text="找不到頁面｜知暖學習工作室"/>
        <Meta name="robots" content="noindex"/>

        <section class="bg-white">
            <div class="mx-auto flex max-w-xl flex-col items-center gap-6 px-6 py-24 text-center">
                <img src="/img/illustration-404.png" alt="" class="w-48 sm:w-64"/>
                <h1 class="text-2xl font-bold text-brand-blue">"這個頁面好像不小心迷路了"</h1>
                <p class="text-slate-gray">"你要找的頁面不存在，或已經被移動了。"</p>
                <CtaButton href="/" label="返回首頁" variant=ButtonVariant::Primary/>
            </div>
        </section>
    }
}
