use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::icons::{AiIcon, CalendarIcon, HeartIcon, PersonIcon, QuoteIcon, TrendUpIcon};
use crate::components::{ButtonVariant, Card, CtaButton, ImagePlaceholder};

const LINE_URL: &str = "https://line.me/R/ti/p/@891ivojl";

struct FeatureCard {
    title: &'static str,
    body: &'static str,
}

const FEATURE_CARDS: [FeatureCard; 4] = [
    FeatureCard { title: "溫暖陪伴", body: "我們相信陪伴的力量，用心理解每位學生，陪你一步步前進。" },
    FeatureCard { title: "客製化教學", body: "依照學生程度與目標，量身打造專屬學習計畫，學習更有效率。" },
    FeatureCard { title: "AI 輔助學習", body: "結合 AI 工具與數位資源，讓學習更有趣、更貼近未來趨勢。" },
    FeatureCard { title: "建立自主學習", body: "培養學習策略與思維，讓孩子擁有帶得走的能力，受用一生。" },
];

struct CourseCard {
    title: &'static str,
    intro: &'static str,
    anchor: &'static str,
}

const COURSE_CARDS: [CourseCard; 5] = [
    CourseCard { title: "國小英文", intro: "打好基礎，培養興趣，自然開口說英文。", anchor: "#elementary" },
    CourseCard { title: "國中英文", intro: "系統化文法與閱讀訓練，穩紮穩打打好實力。", anchor: "#middle" },
    CourseCard { title: "高中英文", intro: "大量閱讀與思辨表達，銜接大學英文能力。", anchor: "#high" },
    CourseCard { title: "成人英文", intro: "生活、職場、旅遊英文，自信開口溝通。", anchor: "#adult" },
    CourseCard { title: "線上親子共學", intro: "每次 30 分鐘，陪孩子養成持續接觸英文的習慣。", anchor: "#family" },
];

struct FlowStep {
    title: &'static str,
    body: &'static str,
}

const FLOW_STEPS: [FlowStep; 5] = [
    FlowStep { title: "① 免費諮詢", body: "了解學習需求與目標，提供最適合的課程建議。" },
    FlowStep { title: "② 能力評估", body: "透過專業評估，掌握學生目前程度與學習關鍵。" },
    FlowStep { title: "③ 安排試聽", body: "實際體驗課程，找到最適合的學習方式與節奏。" },
    FlowStep { title: "④ 正式課程", body: "依照個人程度規劃內容，循序建立英文能力與學習習慣。" },
    FlowStep { title: "⑤ 成果追蹤", body: "持續檢視學習成果，調整教學策略，陪伴孩子穩定成長。" },
];

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="知暖英文｜高雄陪伴式英文學習品牌"/>
        <Meta
            name="description"
            content="知暖學習工作室位於高雄，提供國小至高中英文與成人英文課程，以陪伴式教育培養自主學習能力，協助學生建立穩固英文基礎、自信與持續成長的學習習慣。"
        />

        <HeroSection/>
        <WhyChooseSection/>
        <CoursesSection/>
        <LearningFlowSection/>
        <ResultsSection/>
        <FounderSection/>
        <SubscribeSection/>
        <CtaSection/>
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <section class="bg-gradient-to-b from-mist-blue to-white">
            <div class="mx-auto flex max-w-7xl flex-col items-center gap-10 px-6 py-16 lg:flex-row lg:py-24">
                <div class="flex flex-1 flex-col items-start gap-5 text-left">
                    <p class="font-handwriting text-lg text-sky-blue">"♡ 知足上進，溫暖而堅定"</p>
                    <h1 class="text-4xl font-bold leading-[1.3] text-brand-blue lg:text-5xl">
                        "讓孩子不只是學英文，而是學會學習"
                    </h1>
                    <p class="text-base leading-[1.7] text-slate-gray">
                        "知暖學習陪伴每位孩子建立英文能力、自主學習習慣與未來競爭力。"
                    </p>
                    <div class="flex flex-wrap gap-4 pt-2">
                        <CtaButton href="/contact" label="免費預約試聽" variant=ButtonVariant::Primary/>
                        <CtaButton href="/courses" label="了解課程" variant=ButtonVariant::Secondary/>
                    </div>
                </div>

                <div class="relative flex-1">
                    // TODO: Hero 主視覺照片尚未提供（見 docs/asset-list.md「人物照片」），待 Penny 自然情境照確定後替換
                    <ImagePlaceholder
                        label="Hero 主視覺圖片準備中"
                        class="aspect-[4/5] w-full max-w-md rounded-2xl mx-auto"
                    />
                    <div class="absolute bottom-4 right-4 rounded-xl bg-brand-blue/90 px-4 py-3 text-white shadow-lg sm:right-8">
                        <p class="text-sm font-medium">"創辦人 Penny"</p>
                        // TODO: 教學年資待 Penny 提供確切數字（spec.md 4.1 ⑥）
                        <p class="text-xs text-white/80">"英文教學經驗｜資料整理中"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn WhyChooseSection() -> impl IntoView {
    view! {
        <section class="bg-white">
            <div class="mx-auto max-w-7xl px-6 py-16 lg:py-24">
                <h2 class="flex items-center justify-center gap-3 text-center text-3xl font-bold text-brand-blue">
                    <span class="text-pale-blue">"—"</span>
                    "為什麼選擇知暖？"
                    <span class="text-pale-blue">"—"</span>
                </h2>

                <div class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-4">
                    {FEATURE_CARDS
                        .iter()
                        .enumerate()
                        .map(|(i, card)| {
                            view! {
                                <div class="flex flex-col items-center gap-3 text-center">
                                    <div class="flex h-16 w-16 items-center justify-center rounded-full bg-pale-blue text-brand-blue">
                                        {match i {
                                            0 => view! { <HeartIcon/> }.into_any(),
                                            1 => view! { <PersonIcon/> }.into_any(),
                                            2 => view! { <AiIcon/> }.into_any(),
                                            _ => view! { <TrendUpIcon/> }.into_any(),
                                        }}
                                    </div>
                                    <h3 class="text-xl font-bold text-ink">{card.title}</h3>
                                    <p class="text-sm leading-[1.7] text-slate-gray">{card.body}</p>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>

                <details class="mx-auto mt-12 max-w-3xl rounded-2xl bg-mist-blue p-6">
                    <summary class="cursor-pointer font-medium text-brand-blue">
                        "看更多：AI 輔助學習怎麼做？"
                    </summary>
                    <div class="mt-4 flex flex-col gap-4 sm:flex-row sm:items-center">
                        <p class="text-sm leading-[1.7] text-slate-gray sm:order-2">
                            "AI 正在改變學習方式，而知暖希望帶領學生學會善用 AI，而不是依賴 AI。我們將 AI 工具融入英文學習，引導學生練習口說、寫作、閱讀理解、情境對話與自主複習，提升學習效率與思考能力。課程更重視如何正確提問、整理資訊及培養自主學習策略，讓科技成為學習的助力，而不是答案的替代品，幫助學生建立未來不可或缺的英語與 AI 素養。"
                        </p>
                        <img
                            src="/img/illustration-ai-feature.png"
                            alt="AI 輔助學習插圖：機器人與 ABC 字母"
                            class="w-32 flex-shrink-0 sm:order-1 sm:w-40"
                        />
                    </div>
                </details>
            </div>
        </section>
    }
}

#[component]
fn CoursesSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <div class="mx-auto max-w-7xl px-6 py-16 lg:py-24">
                <h2 class="text-center text-3xl font-bold text-brand-blue">"我們提供多元課程"</h2>

                <div class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-5">
                    {COURSE_CARDS
                        .iter()
                        .map(|course| {
                            view! {
                                <Card class="overflow-hidden flex flex-col".to_string()>
                                    // TODO: 課程情境照片待整理歸類（見 docs/asset-list.md「課程情境照片」）
                                    <ImagePlaceholder
                                        label="課程情境照片準備中"
                                        class="aspect-[4/3] w-full rounded-t-2xl"
                                    />
                                    <div class="flex flex-1 flex-col gap-2 p-5">
                                        <h3 class="text-xl font-bold text-ink">{course.title}</h3>
                                        <p class="flex-1 text-sm leading-[1.7] text-slate-gray">{course.intro}</p>
                                        <a
                                            href=format!("/courses{}", course.anchor)
                                            class="group inline-flex items-center gap-1 text-sm font-medium text-brand-blue"
                                        >
                                            "了解更多"
                                            <span class="transition-transform group-hover:translate-x-1">"→"</span>
                                        </a>
                                    </div>
                                </Card>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}

#[component]
fn LearningFlowSection() -> impl IntoView {
    view! {
        <section class="bg-white">
            <div class="mx-auto max-w-7xl px-6 py-16 lg:py-24">
                <h2 class="text-center text-3xl font-bold text-brand-blue">"教學流程"</h2>

                <div class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-5">
                    {FLOW_STEPS
                        .iter()
                        .map(|step| {
                            view! {
                                <div class="flex flex-col items-center gap-2 text-center">
                                    <h3 class="text-lg font-bold text-brand-blue">{step.title}</h3>
                                    <p class="text-sm leading-[1.7] text-slate-gray">{step.body}</p>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}

#[component]
fn ResultsSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <div class="mx-auto grid max-w-7xl gap-12 px-6 py-16 lg:grid-cols-2 lg:py-24">
                <div>
                    <h2 class="text-3xl font-bold text-brand-blue">"學生成果與家長好評"</h2>
                    // TODO: 以下三項為示意數據，待 Henry／Penny 提供真實數字後替換（spec.md 4.1 ⑤、14 待補內容清單）
                    <div class="mt-8 grid grid-cols-3 divide-x divide-border-gray">
                        <div class="px-2 text-center">
                            <p class="text-3xl font-bold text-brand-blue">"待更新"</p>
                            <p class="mt-1 text-xs text-slate-gray">"學生成績提升幅度"</p>
                        </div>
                        <div class="px-2 text-center">
                            <p class="text-3xl font-bold text-brand-blue">"待更新"</p>
                            <p class="mt-1 text-xs text-slate-gray">"學生找到自信與方向"</p>
                        </div>
                        <div class="px-2 text-center">
                            <p class="text-3xl font-bold text-brand-blue">"待更新"</p>
                            <p class="mt-1 text-xs text-slate-gray">"家長滿意度"</p>
                        </div>
                    </div>
                </div>

                // TODO: 真實家長回饋文字尚未提供（spec.md 4.1 ⑤、14 待補內容清單），以下為佔位卡片
                <div class="grid grid-cols-1 gap-6 sm:grid-cols-3 lg:grid-cols-1">
                    {(0..3)
                        .map(|_| {
                            view! {
                                <Card class="p-6".to_string()>
                                    <div class="text-pale-blue">
                                        <QuoteIcon/>
                                    </div>
                                    <p class="mt-2 text-sm leading-[1.7] text-slate-gray">"家長真實回饋準備中"</p>
                                    <p class="mt-4 text-xs text-slate-gray">"— 待補充身分標註"</p>
                                </Card>
                            }
                        })
                        .collect_view()}
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
                    <blockquote class="text-xl font-medium leading-[1.7] text-ink">
                        "Hi，我是 Penny。我相信，每個孩子都能找到適合自己的學習方式。我們教的不只是英文，而是一輩子的學習能力。"
                    </blockquote>
                    // TODO: 簡短資歷標註（教學年資等）尚未提供，待補充後移除下方提示文字
                    <p class="mt-4 text-sm text-slate-gray">"教學資歷｜資料整理中"</p>
                </div>
                // TODO: Penny 個人照片尚未提供（見 docs/asset-list.md「人物照片」）
                <ImagePlaceholder
                    label="創辦人照片準備中"
                    class="aspect-square w-full max-w-xs rounded-2xl flex-shrink-0 lg:order-1"
                />
            </div>
        </section>
    }
}

#[component]
fn SubscribeSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <div class="mx-auto flex max-w-3xl flex-col items-center gap-4 px-6 py-16 text-center">
                <h2 class="text-2xl font-bold text-brand-blue">"訂閱最新消息"</h2>
                <p class="text-sm text-slate-gray">"掌握知暖最新課程資訊與學習分享"</p>
                // NOTE: 表單畫面先行建立，實際寫入 subscribers 資料表的 server function 於 Phase C 建立
                <form class="flex w-full max-w-md flex-col gap-3 pt-2 sm:flex-row">
                    <input
                        type="email"
                        required
                        placeholder="輸入您的 Email"
                        class="flex-1 rounded-full border border-border-gray bg-white px-5 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                    />
                    <button
                        type="submit"
                        disabled
                        title="訂閱功能將於 Phase C 開放"
                        class="rounded-full bg-brand-blue px-8 py-3 font-medium text-white opacity-60"
                    >
                        "訂閱"
                    </button>
                </form>
            </div>
        </section>
    }
}

#[component]
fn CtaSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <div class="mx-auto flex max-w-7xl flex-col items-center gap-8 px-6 py-16 lg:flex-row lg:justify-between lg:py-24">
                <div class="flex items-center gap-4 text-center lg:text-left">
                    <div class="hidden h-14 w-14 flex-shrink-0 items-center justify-center rounded-full bg-brand-blue text-white sm:flex">
                        <CalendarIcon/>
                    </div>
                    <p class="max-w-xl text-lg leading-[1.7] text-ink">
                        "知暖陪伴孩子從國小到高中，不只是學好英文，更一步步建立自主學習能力，成為能持續成長的終身學習者。"
                    </p>
                </div>
                <div class="flex flex-shrink-0 flex-wrap justify-center gap-4">
                    <CtaButton href="/contact" label="立即預約免費試聽" variant=ButtonVariant::Primary/>
                    <CtaButton href=LINE_URL label="加入 LINE 諮詢" variant=ButtonVariant::Line/>
                </div>
            </div>
        </section>
    }
}
