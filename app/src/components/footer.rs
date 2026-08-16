use leptos::prelude::*;

use super::icons::{BookIcon, ChatIcon, FacebookIcon, InstagramIcon};
use super::reveal::Reveal;

// v16 精簡：拿掉跟主導覽列重複的「關於知暖／課程介紹／最新消息」，只留三項（spec.md 4.1⑧）
const QUICK_LINKS: [(&str, &str); 3] = [("/contact", "聯絡我們"), ("/faq", "常見問題"), ("/privacy", "隱私權政策")];

/// Footer 頁尾，對照 spec.md 5.4 節 Footer 規範與 4.1⑧
/// v16：底色改白色，視覺背景改由固定 90px 高的動態波浪色帶負責（見 5.8 節第 4 項）
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="relative overflow-hidden bg-white">
            // 波浪色帶：固定 90px 高，貼齊 Footer 最頂端，波峰觸頂（y=0）／波谷落於色帶底部
            <div class="pointer-events-none absolute inset-x-0 top-0 h-[90px] overflow-hidden" aria-hidden="true">
                <svg class="footer-wave h-full w-[200%]" viewBox="0 0 200 100" preserveAspectRatio="none">
                    <path
                        d="M0,0 C16.67,0 33.33,100 50,100 C66.67,100 83.33,0 100,0 C116.67,0 133.33,100 150,100 C166.67,100 183.33,0 200,0 L200,100 L0,100 Z"
                        fill="#4A7FC9"
                        fill-opacity="0.35"
                    ></path>
                </svg>
            </div>
            // 色帶下方的實色延伸區塊，跟波浪同色同透明度，無縫銜接到 Footer 最底部
            <div
                class="pointer-events-none absolute inset-x-0 bottom-0 top-[90px]"
                style="background-color: rgba(74, 127, 201, 0.35);"
                aria-hidden="true"
            ></div>

            <div class="relative z-10 mx-auto grid max-w-7xl gap-10 px-6 pb-16 pt-[130px] sm:grid-cols-2 lg:grid-cols-3">
                <Reveal class="flex flex-col gap-3".to_string()>
                    <div class="flex items-center gap-2 text-brand-blue">
                        <BookIcon/>
                        <span class="font-bold">"知暖學習工作室"</span>
                    </div>
                    <p class="font-handwriting text-sky-blue">"知足上進，溫暖而堅定"</p>
                </Reveal>

                <Reveal delay_ms=110u32 class="flex flex-col gap-3".to_string()>
                    <span class="text-base font-bold text-slate-gray">"快速連結"</span>
                    {QUICK_LINKS
                        .iter()
                        .map(|(href, label)| {
                            view! {
                                <a
                                    href=*href
                                    class="group relative w-fit text-slate-gray transition-colors hover:text-brand-blue"
                                >
                                    {*label}
                                    <span class="pointer-events-none absolute inset-x-0 -bottom-0.5 h-px origin-left scale-x-0 bg-brand-blue transition-transform duration-[280ms] ease-out group-hover:scale-x-100"></span>
                                </a>
                            }
                        })
                        .collect_view()}
                </Reveal>

                <Reveal delay_ms=220u32 class="flex flex-col gap-3".to_string()>
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
                </Reveal>
            </div>

            <div class="relative z-10 border-t border-border-gray py-6 text-center text-sm text-slate-gray">
                "© 2026 知暖學習工作室 All Rights Reserved."
            </div>
        </footer>
    }
}
