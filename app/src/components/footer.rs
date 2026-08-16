use leptos::prelude::*;

use super::icons::{BookIcon, ChatIcon, FacebookIcon, InstagramIcon};

const QUICK_LINKS: [(&str, &str); 6] = [
    ("/about", "關於知暖"),
    ("/courses", "課程介紹"),
    ("/news", "最新消息"),
    ("/contact", "聯絡我們"),
    ("/faq", "常見問題"),
    ("/privacy", "隱私權政策"),
];

/// Footer 頁尾，對照 spec.md 5.4 節 Footer 規範與 4.1 ⑨
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-white">
            <div class="mx-auto grid max-w-7xl gap-10 px-6 py-16 sm:grid-cols-2 lg:grid-cols-3">
                <div class="flex flex-col gap-3">
                    <div class="flex items-center gap-2 text-brand-blue">
                        <BookIcon/>
                        <span class="font-bold">"知暖學習工作室"</span>
                    </div>
                    <p class="font-handwriting text-sky-blue">"知足上進，溫暖而堅定"</p>
                </div>

                <div class="flex flex-col gap-3">
                    <span class="text-base font-bold text-slate-gray">"快速連結"</span>
                    {QUICK_LINKS
                        .iter()
                        .map(|(href, label)| {
                            view! {
                                <a href=*href class="text-ink hover:text-brand-blue">
                                    {*label}
                                </a>
                            }
                        })
                        .collect_view()}
                </div>

                <div class="flex flex-col gap-3">
                    <span class="text-base font-bold text-slate-gray">"聯絡資訊"</span>
                    <a
                        href="https://line.me/R/ti/p/@891ivojl"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-flex w-fit items-center gap-2 rounded-full bg-line-green px-5 py-2 text-white"
                    >
                        <ChatIcon/>
                        "LINE @891ivojl"
                    </a>
                    <div class="flex items-center gap-3 pt-1">
                        <a
                            href="https://www.instagram.com/warmlearning"
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="Instagram"
                            class="text-slate-gray hover:text-brand-blue"
                        >
                            <InstagramIcon/>
                        </a>
                        <a
                            href="https://www.facebook.com/share/199rzgkvkQ/?mibextid=wwXIfr"
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="Facebook"
                            class="text-slate-gray hover:text-brand-blue"
                        >
                            <FacebookIcon/>
                        </a>
                    </div>
                    <img
                        src="/img/line-qrcode.png"
                        alt="LINE 官方帳號 QR Code"
                        class="h-20 w-20 border border-border-gray object-contain"
                    />
                </div>
            </div>

            <div class="border-t border-border-gray py-6 text-center text-sm text-slate-gray">
                "© 2026 知暖學習工作室 All Rights Reserved."
            </div>
        </footer>
    }
}
