use leptos::form::ActionForm;
use leptos::prelude::*;

use crate::pages::admin_login::AdminLogout;

/// 後台共用導覽列，對照 spec.md 第 6 節後台管理系統
#[component]
pub fn AdminNav() -> impl IntoView {
    let logout_action = ServerAction::<AdminLogout>::new();

    view! {
        <header class="bg-brand-blue">
            <div class="mx-auto flex max-w-7xl flex-wrap items-center justify-between gap-4 px-6 py-4 text-white">
                <div class="flex flex-wrap items-center gap-6">
                    <span class="font-bold">"知暖後台管理"</span>
                    <nav class="flex flex-wrap items-center gap-4 text-sm">
                        <a href="/admin" class="hover:underline">
                            "儀表板"
                        </a>
                        <a href="/admin/announcements" class="hover:underline">
                            "公告管理"
                        </a>
                        <a href="/admin/contacts" class="hover:underline">
                            "聯絡表單"
                        </a>
                    </nav>
                </div>
                <ActionForm action=logout_action>
                    <button
                        type="submit"
                        class="rounded-full border border-white/60 px-4 py-1.5 text-sm text-white transition-colors hover:bg-white/10"
                    >
                        "登出"
                    </button>
                </ActionForm>
            </div>
        </header>
    }
}
