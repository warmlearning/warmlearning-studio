use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::{ButtonVariant, CtaButton, Reveal};

const LINE_URL: &str = "https://line.me/R/ti/p/@891ivojl";

struct AnsweredFaq {
    question: &'static str,
    answer: &'static str,
}

const ANSWERED_FAQS: [AnsweredFaq; 3] = [
    AnsweredFaq {
        question: "家教不就是一對一，應該很貴吧？",
        answer: "知暖不是單純賣一堂英文課，而是依照孩子的學習階段跟需求，設計適合的學習方式。我們不會讓您第一時間只比較「一小時多少錢」——歡迎直接加入 LINE 諮詢，我們會先了解孩子的狀況，再說明合適的方案。",
    },
    AnsweredFaq {
        question: "孩子英文成績不錯，是不是就不需要補習／家教了？",
        answer: "知暖也很適合已經有一定程度、希望孩子能更自主、更有系統學習的家庭，幫助孩子建立長期主義學習的習慣，直到畢業出社會都受用。",
    },
    AnsweredFaq {
        question: "英文不好，是不是找一個很會教、英文很好的老師就好？",
        answer: "每個孩子的問題不一樣，有些是基礎、有些是方法、有些其實是學習習慣。知暖會先了解孩子目前的狀況，再找適合他的學習方式。",
    },
];

const PENDING_QUESTIONS: [&str; 5] =
    ["需要程度很好嗎？", "可以試聽嗎？", "多久看到成果？", "有線上課嗎？", "教材是什麼？"];

/// 常見問題頁，對照 spec.md 4.7 節（v7 新增）
#[component]
pub fn FaqPage() -> impl IntoView {
    view! {
        <Title text="常見問題 FAQ｜知暖 Warm Learning Studio"/>
        <Meta
            name="description"
            content="家長常見疑問一次看：學費怎麼算、需要程度多好、多久看到成果——了解知暖學習工作室（Warm Learning Studio）如何陪伴孩子建立長期學習能力。"
        />

        <section class="bg-mist-blue">
            <Reveal class="mx-auto max-w-3xl px-6 py-16 lg:py-24".to_string()>
                <h1 class="text-center text-3xl font-bold text-brand-blue">"常見問題"</h1>

                <div class="mt-12 flex flex-col gap-6">
                    {ANSWERED_FAQS
                        .iter()
                        .enumerate()
                        .map(|(i, faq)| {
                            view! {
                                <Reveal delay_ms=(i as u32) * 110>
                                    <div class="rounded-2xl bg-white p-6 shadow-md">
                                        <h2 class="text-lg font-bold text-brand-blue">"Q：" {faq.question}</h2>
                                        <p class="mt-3 text-sm leading-[1.7] text-slate-gray">{faq.answer}</p>
                                    </div>
                                </Reveal>
                            }
                        })
                        .collect_view()}

                    {PENDING_QUESTIONS
                        .iter()
                        .enumerate()
                        .map(|(i, question)| {
                            let delay_ms = ((ANSWERED_FAQS.len() + i) as u32) * 110;
                            view! {
                                <Reveal delay_ms=delay_ms>
                                    <div class="rounded-2xl bg-white p-6 shadow-md">
                                        <h2 class="text-lg font-bold text-brand-blue">"Q：" {*question}</h2>
                                        <p class="mt-3 text-sm leading-[1.7] text-slate-gray">"內容準備中"</p>
                                    </div>
                                </Reveal>
                            }
                        })
                        .collect_view()}
                </div>

                <div class="mt-12 flex justify-center">
                    <CtaButton href=LINE_URL label="還有其他問題？加入 LINE 諮詢" variant=ButtonVariant::Line/>
                </div>
            </Reveal>
        </section>
    }
}
