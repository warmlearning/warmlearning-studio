use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::icons::{AiIcon, CalendarIcon, HeartIcon, PersonIcon, TrendUpIcon};
use crate::components::{ButtonVariant, Card, CtaButton, ImagePlaceholder, Reveal};

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
    CourseCard { title: "幼兒線上親子共學", intro: "每次 30 分鐘，陪孩子養成持續接觸英文的習慣。", anchor: "#family" },
    CourseCard { title: "國小英文", intro: "打好基礎，培養興趣，自然開口說英文。", anchor: "#elementary" },
    CourseCard { title: "國中英文", intro: "系統化文法與閱讀訓練，穩紮穩打打好實力。", anchor: "#middle" },
    CourseCard { title: "高中英文", intro: "大量閱讀與思辨表達，銜接大學英文能力。", anchor: "#high" },
    CourseCard { title: "成人英文", intro: "生活、職場、旅遊英文，自信開口溝通。", anchor: "#adult" },
];

struct FlowStep {
    title: &'static str,
    body: &'static str,
}

const FLOW_STEPS: [FlowStep; 5] = [
    FlowStep { title: "① 預約諮詢", body: "了解學習需求與目標，提供最適合的課程建議。" },
    FlowStep { title: "② 能力評估", body: "透過專業評估，掌握學生目前程度與學習關鍵。" },
    FlowStep { title: "③ 安排試聽", body: "實際體驗課程，找到最適合的學習方式與節奏。" },
    FlowStep { title: "④ 正式課程", body: "依照個人程度規劃內容，循序建立英文能力與學習習慣。" },
    FlowStep { title: "⑤ 成果追蹤", body: "持續檢視學習成果，調整教學策略，陪伴孩子穩定成長。" },
];

struct StudentResult {
    image: &'static str,
    title: &'static str,
    narrative: &'static str,
    highlight: &'static str,
}

// 內文已去識別化處理，不含真實姓名；健康相關描述一律用中性、非診斷性說法呈現，
// 不點名任何醫療診斷或治療狀態（spec.md 4.1⑤ 隱私處理備註，v9 更新）
const STUDENT_RESULTS: [StudentResult; 3] = [
    StudentResult {
        image: "/img/result-1.jpg",
        title: "①「英文程度落後，但重新找回學習信心」的孩子",
        narrative: "原本專注力較不集中、對英文沒有信心的孩子，透過重新建立基礎與學習方法，逐漸找到與英文學習的連結點，也開始相信自己做得到。",
        highlight: "👉 知暖不是只教英文，而是找回孩子的學習信心。",
    },
    StudentResult {
        image: "/img/result-2.jpg",
        title: "②「成績不一定最差，但缺乏學習習慣」的孩子",
        narrative: "原本需要家長不斷提醒、缺乏自主學習習慣的孩子，透過陪跑與學習方法建立，慢慢從「被要求學」轉變成「知道自己該怎麼學」。每日練習總結、逐步修正，誠實了解自己的狀況與清晰目標，調整出最適合自己的讀書節奏，運用天賦、擅長處來幫上學習大忙——不再帶著罪惡感休息，能更有效善用時間，玩樂休息取得平衡，相信自己是優秀的。",
        highlight: "👉 從英文學習延伸到自主學習。",
    },
    StudentResult {
        image: "/img/result-3.jpg",
        title: "③「願意和老師一起成長的家庭」",
        narrative: "家長願意與老師合作，不只關注孩子的成績，也願意一起調整陪伴方式，最後讓親子關係與孩子的學習狀態都逐漸變得更穩定。",
        highlight: "👉 知暖服務的不只是孩子，而是一個家庭。",
    },
];

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="知暖英文 Warm Learning Studio｜高雄陪伴式英文學習品牌"/>
        <Meta
            name="description"
            content="知暖學習工作室（Warm Learning Studio）位於高雄，提供幼兒到成人的英文課程，以陪伴式教育培養自主學習能力，協助學習者建立穩固英文基礎與持續成長的學習系統。"
        />

        <HeroSection/>
        <WhyChooseSection/>
        <CoursesSection/>
        <LearningFlowSection/>
        <ResultsSection/>
        <FounderSection/>
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
                        <CtaButton href="/contact" label="預約諮詢" variant=ButtonVariant::Primary/>
                        <CtaButton href="/courses" label="了解課程" variant=ButtonVariant::Secondary/>
                    </div>
                </div>

                <div class="relative flex-1">
                    <img
                        src="/img/hero-photo.jpg"
                        alt="知暖學習工作室教學情境"
                        class="aspect-[4/5] w-full max-w-md rounded-2xl mx-auto object-cover"
                    />
                    <div class="absolute bottom-4 right-4 rounded-xl bg-brand-blue/90 px-4 py-3 text-white shadow-lg sm:right-8">
                        <p class="text-sm font-medium">"創辦人 Penny"</p>
                        // TODO: 此處教學年資標註 spec.md 4.1⑥ 仍標示待填寫（4.2④ 的 7 年是另一段敘事文案的數字，
                        // 不代表本欄位已定案，未經明確確認前不代填）
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
                                <Reveal delay_ms=(i as u32) * 110>
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
                                        <a
                                            href="/about#long-term-learning"
                                            class="text-sm font-medium text-brand-blue hover:underline"
                                        >
                                            "了解更多"
                                        </a>
                                    </div>
                                </Reveal>
                            }
                        })
                        .collect_view()}
                </div>
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
                        .enumerate()
                        .map(|(i, course)| {
                            view! {
                                <Reveal delay_ms=(i as u32) * 110>
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
                                </Reveal>
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
                <h2 class="text-center text-3xl font-bold text-brand-blue">"如何加入知暖？"</h2>

                <div class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-5">
                    {FLOW_STEPS
                        .iter()
                        .enumerate()
                        .map(|(i, step)| {
                            view! {
                                <Reveal delay_ms=(i as u32) * 110>
                                    <div class="flex flex-col items-center gap-2 text-center">
                                        <h3 class="text-lg font-bold text-brand-blue">{step.title}</h3>
                                        <p class="text-sm leading-[1.7] text-slate-gray">{step.body}</p>
                                    </div>
                                </Reveal>
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
            <div class="mx-auto max-w-7xl px-6 py-16 lg:py-24">
                <h2 class="text-center text-3xl font-bold text-brand-blue">"學生成果與家長好評"</h2>

                <div class="mt-10 grid grid-cols-1 items-start gap-10 lg:grid-cols-3 lg:gap-8">
                    {STUDENT_RESULTS
                        .iter()
                        .enumerate()
                        .map(|(i, result)| {
                            view! {
                                <Reveal delay_ms=(i as u32) * 110>
                                    <div class="flex flex-col gap-4">
                                        // 圖片維持各自原始比例，不裁切、不強制統一比例，三欄頂部對齊
                                        // （spec.md 4.1⑤ v9 圖片顯示規則）
                                        <img
                                            src=result.image
                                            alt=result.title
                                            class="h-auto w-full max-w-xs mx-auto rounded-2xl lg:max-w-none"
                                        />
                                        <div class="flex flex-col gap-2">
                                            <h3 class="text-base font-bold text-brand-blue">{result.title}</h3>
                                            <p class="text-sm leading-[1.7] text-slate-gray">{result.narrative}</p>
                                            <p class="text-sm font-medium text-brand-blue">{result.highlight}</p>
                                        </div>
                                    </div>
                                </Reveal>
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
            <Reveal class="mx-auto flex max-w-7xl flex-col items-center gap-10 px-6 py-16 lg:flex-row lg:py-24".to_string()>
                <div class="flex-1 text-center lg:order-2 lg:text-left">
                    <blockquote class="text-xl font-medium leading-[1.7] text-ink">
                        "Hi，我是 Penny。我相信，每個孩子都能找到適合自己的學習方式。我們教的不只是英文，而是一輩子的學習能力。"
                    </blockquote>
                    // TODO: 簡短資歷標註（教學年資等）尚未提供，待補充後移除下方提示文字
                    <p class="mt-4 text-sm text-slate-gray">"教學資歷｜資料整理中"</p>
                </div>
                <img
                    src="/img/penny-photo.jpg"
                    alt="創辦人 Penny"
                    class="aspect-square w-full max-w-xs rounded-2xl object-cover flex-shrink-0 lg:order-1"
                />
            </Reveal>
        </section>
    }
}

#[component]
fn CtaSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <Reveal class="mx-auto flex max-w-7xl flex-col items-center gap-8 px-6 py-16 lg:flex-row lg:justify-between lg:py-24".to_string()>
                <div class="flex items-center gap-4 text-center lg:text-left">
                    <div class="hidden h-14 w-14 flex-shrink-0 items-center justify-center rounded-full bg-brand-blue text-white sm:flex">
                        <CalendarIcon/>
                    </div>
                    <p class="max-w-xl text-lg leading-[1.7] text-ink">
                        "知暖陪伴學習者從幼兒到成人，不只是學好英文，更一步步建立自主學習能力，成為能持續成長的終身學習者。"
                    </p>
                </div>
                <div class="flex flex-shrink-0 flex-wrap justify-center gap-4">
                    <CtaButton href="/contact" label="預約諮詢" variant=ButtonVariant::Primary/>
                    <CtaButton href=LINE_URL label="加入 LINE 諮詢" variant=ButtonVariant::Line/>
                </div>
            </Reveal>
        </section>
    }
}
