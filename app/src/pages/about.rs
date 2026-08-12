use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::{Card, ImagePlaceholder};

struct Keyword {
    title: &'static str,
    body: &'static str,
}

const KEYWORDS: [Keyword; 4] = [
    Keyword { title: "知足上進", body: "珍惜每一次成長的累積，在穩定中持續突破，成為更好的自己。" },
    Keyword { title: "溫暖而堅定", body: "用理解陪伴每位孩子，同時堅持培養良好的學習態度與習慣。" },
    Keyword { title: "陪伴式教育", body: "重視每位學生的節奏，陪伴探索、鼓勵嘗試，一起走過學習歷程。" },
    Keyword { title: "終身學習", body: "相信學習沒有終點，培養自主學習能力，讓成長成為一輩子的習慣。" },
];

const LONG_TERM_PARAGRAPHS: [&str; 5] = [
    "不只教英文，更教孩子「如何學習」。",
    "不只服務課堂，更重視課後學習——透過了解系統式陪跑，讓學習延續到每一天。",
    "建立專屬自己的學習系統，讓學生清楚知道「我在哪裡、為什麼要學、下一步怎麼做」。",
    "從「被老師帶著學」到「自己知道怎麼學」，培養自主學習與自我調整的能力，而非依賴老師。",
    "不追求短暫充電，而是培養學習續航力，讓學習熱度持續保溫，從每週一次的課程，變成長期穩定的學習習慣。",
];

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="關於知暖 Warm Learning Studio｜陪伴建立自主學習力"/>
        <Meta
            name="description"
            content="了解知暖學習工作室（Warm Learning Studio）的創立理念與教育初衷。我們深信陪伴比催促更重要，透過專業教學與溫暖引導，陪伴高雄幼兒到成人學習者建立自主學習能力。"
        />

        <BrandStorySection/>
        <LongTermLearningSection/>
        <AiFeatureSection/>
        <FounderSection/>
        <VisionSection/>
    }
}

#[component]
fn BrandStorySection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <div class="mx-auto max-w-4xl px-6 py-16 lg:py-24">
                <h1 class="text-center text-3xl font-bold text-brand-blue">"為什麼創立知暖？"</h1>

                <div class="mt-8 flex flex-col gap-5 text-base leading-[1.7] text-ink">
                    <p>
                        "知暖學習工作室的誕生，源自一個簡單卻深刻的信念：真正的教育，不只是讓孩子考高分，而是陪伴他找到持續學習的能力。"
                    </p>
                    <p>
                        "在多年教學的過程中，我發現許多孩子並不是沒有能力，而是缺少適合自己的學習方法與一位願意陪伴他們的人。當學習變成壓力，孩子容易失去信心，也逐漸忘記學習本來可以是一件充滿成就感的事。"
                    </p>
                    <p>
                        "因此，我創立了知暖，希望打造一個兼具溫度與專業的學習空間。我們重視每一位孩子的學習歷程，不只提升英文能力，更陪伴他建立自信、培養自主學習習慣，讓學習不只是為了考試，而是成為未來一生受用的能力。"
                    </p>
                </div>

                <div class="mt-12 grid grid-cols-1 gap-6 sm:grid-cols-2">
                    {KEYWORDS
                        .iter()
                        .map(|kw| {
                            view! {
                                <Card class="p-6".to_string()>
                                    <h3 class="text-xl font-bold text-brand-blue">{kw.title}</h3>
                                    <p class="mt-2 text-sm leading-[1.7] text-slate-gray">{kw.body}</p>
                                </Card>
                            }
                        })
                        .collect_view()}
                </div>

                <p class="mt-12 text-center text-xl font-medium text-brand-blue">
                    "我們相信，不是教出高分，而是教出會學習的人。"
                </p>
            </div>
        </section>
    }
}

/// 先「勝」後「戰」・長期主義學習品牌方法論，對照 spec.md 4.2 ② 節（v7 新增）
#[component]
fn LongTermLearningSection() -> impl IntoView {
    view! {
        <section id="long-term-learning" class="scroll-mt-24 bg-white">
            <div class="mx-auto max-w-3xl px-6 py-16 lg:py-24">
                <h2 class="text-center text-3xl font-bold text-brand-blue">
                    "先「勝」後「戰」・長期主義學習"
                </h2>
                <div class="mt-8 flex flex-col gap-5 text-base leading-[1.7] text-ink">
                    {LONG_TERM_PARAGRAPHS.iter().map(|p| view! { <p>{*p}</p> }).collect_view()}
                </div>
            </div>
        </section>
    }
}

/// AI 融入教學，對照 spec.md 4.2 ③ 節（v7 補上實際內文與貼紙標語）
#[component]
fn AiFeatureSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <div class="mx-auto flex max-w-7xl flex-col items-center gap-10 px-6 py-16 lg:flex-row lg:py-24">
                <div class="flex-1">
                    <h2 class="text-3xl font-bold text-brand-blue">"AI 融入教學"</h2>
                    <p class="mt-6 text-base leading-[1.7] text-ink">
                        "AI 正在改變學習方式，而知暖希望帶領學生學會善用 AI，而不是依賴 AI。我們將 AI 工具融入英文學習，引導學生練習口說、寫作、閱讀理解、情境對話與自主複習，提升學習效率與思考能力。課程更重視如何正確提問、整理資訊及培養自主學習策略，讓科技成為學習的助力，而不是答案的替代品，幫助學生建立未來不可或缺的英語與 AI 素養。"
                    </p>
                </div>

                <div class="relative mx-auto w-full max-w-xs flex-shrink-0 sm:max-w-sm">
                    <img
                        src="/img/illustration-ai-feature.png"
                        alt="AI 輔助學習插圖：機器人與 ABC 字母"
                        class="w-full"
                    />
                    // 全站唯一活潑俏皮語氣的貼紙標語，刻意獨立於周圍文案語氣之外，範圍僅限這個小標籤
                    <span class="absolute -right-2 -top-2 inline-block max-w-[9rem] -rotate-6 rounded-2xl bg-warm-amber px-3 py-1.5 text-center text-xs font-bold leading-tight text-ink shadow-md sm:-right-4 sm:-top-4">
                        "讓你從英文廢柴變成英文小天才🔥"
                    </span>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FounderSection() -> impl IntoView {
    view! {
        <section class="bg-white">
            <div class="mx-auto flex max-w-7xl flex-col items-center gap-10 px-6 py-16 lg:flex-row lg:py-24">
                <div class="flex-1 text-center lg:order-2 lg:text-left">
                    <h2 class="text-2xl font-bold text-brand-blue">"創辦人 Penny"</h2>
                    <blockquote class="mt-4 text-lg leading-[1.7] text-ink">
                        "Hi，我是 Penny。我相信，每個孩子都能找到適合自己的學習方式。我們教的不只是英文，而是一輩子的學習能力。"
                    </blockquote>
                    <p class="mt-4 text-sm leading-[1.7] text-slate-gray">
                        "Penny 擁有 7 年英文教學經驗，擅長在對話中快速理解真正的問題所在，用引導式的提問幫助孩子跟家長釐清想法與目標，而不是急著給答案。她重視關係經營，相信信任感是學習動力的根本；也把多年帶班、辦活動、公開表達的經驗，轉化成一套能被複製、能被系統化執行的教學方法——這也是「知暖成長之旅 Learning Journey」的由來。"
                    </p>
                </div>
                // TODO: Penny 個人照片尚未正式整合（見 docs/asset-list.md「人物照片」）
                <ImagePlaceholder
                    label="創辦人照片準備中"
                    class="aspect-square w-full max-w-xs rounded-2xl flex-shrink-0 lg:order-1"
                />
            </div>
        </section>
    }
}

#[component]
fn VisionSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <div class="mx-auto max-w-3xl px-6 py-16 text-center lg:py-24">
                <h2 class="text-3xl font-bold text-brand-blue">"品牌願景"</h2>
                <p class="mt-6 text-lg leading-[1.7] text-ink">
                    "陪伴每位學習者建立自主學習能力，讓英文成為探索世界與實現夢想的力量。"
                </p>
            </div>
        </section>
    }
}
