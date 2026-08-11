use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::{ButtonVariant, Card, CtaButton};

const LINE_URL: &str = "https://line.me/R/ti/p/@891ivojl";

struct Stage {
    emoji: &'static str,
    title: &'static str,
    anchor: &'static str,
    image: &'static str,
    audience: &'static str,
    basic: &'static str,
    advanced: &'static str,
    ability: &'static str,
    outcome: &'static str,
}

const STAGES: [Stage; 3] = [
    Stage {
        emoji: "🌱",
        title: "國小英文（打底）",
        anchor: "elementary",
        image: "/img/illustration-elementary.png",
        audience: "國小一年級～六年級",
        basic: "Phonics 自然發音、基礎字彙、簡單會話、繪本閱讀入門",
        advanced: "閱讀理解、口說表達、繪本延伸閱讀、AI 互動練習",
        ability: "聽、說、讀、寫、自主完成作業",
        outcome: "建立英文自信、喜歡閱讀、敢開口說英文",
    },
    Stage {
        emoji: "🚀",
        title: "國中英文（建立能力）",
        anchor: "middle",
        image: "/img/illustration-middle.png",
        audience: "國一～國三",
        basic: "文法基礎、單字累積、閱讀入門、聽力入門",
        advanced: "長篇閱讀、寫作訓練、聽力強化、自主讀書規劃",
        ability: "能閱讀長篇文章、建立筆記能力、自主規劃讀書、學會錯題整理",
        outcome: "英文能力穩定提升、建立自主學習習慣",
    },
    Stage {
        emoji: "🎓",
        title: "高中英文（突破）",
        anchor: "high",
        image: "/img/illustration-high.png",
        audience: "高一～高三",
        basic: "文法統整、閱讀測驗基礎、寫作入門",
        advanced: "英文思辨、進階寫作、AI 輔助閱讀與寫作應用",
        ability: "大量閱讀、英文表達、邏輯思考、自主學習、簡報能力",
        outcome: "具備英文自主閱讀能力、銜接大學英文程度",
    },
];

#[component]
pub fn CoursesPage() -> impl IntoView {
    view! {
        <Title text="知暖英文課程｜幼兒、國小、國中、高中與成人英文"/>
        <Meta
            name="description"
            content="知暖學習工作室（Warm Learning Studio）提供高雄幼兒親子共學、國小、國中、高中英文及成人英文課程，依程度客製化規劃，結合陪伴式教育與自主學習訓練，協助建立完整英文學習系統。"
        />

        <PageHeader/>
        <TimelineSection/>
        <AddonSection/>
        <FooterCtaSection/>
    }
}

#[component]
fn PageHeader() -> impl IntoView {
    view! {
        <section class="bg-gradient-to-b from-mist-blue to-white">
            <div class="mx-auto max-w-4xl px-6 py-16 text-center lg:py-24">
                <h1 class="text-4xl font-bold text-brand-blue">"知暖 Learning Journey"</h1>
                <p class="mt-4 text-base leading-[1.7] text-slate-gray">
                    "陪伴孩子持續學習、不中斷的成長地圖，而非單堂課程列表。知暖所有課程設計的共同目標，都是讓孩子能長期、穩定地接觸英文，而不是學一下就中斷。"
                </p>
            </div>
        </section>
    }
}

#[component]
fn TimelineSection() -> impl IntoView {
    view! {
        <section class="bg-white">
            <div class="mx-auto max-w-5xl px-6 py-16 lg:py-24">
                <div id="family" class="scroll-mt-24">
                    <Card class="p-6".to_string()>
                        <h2 class="text-xl font-bold text-ink">"幼兒線上親子共學"</h2>
                        <p class="mt-3 text-sm leading-[1.7] text-slate-gray">
                            "每次 30 分鐘的線上課程，陪伴孩子從小養成規律接觸英文的習慣，是知暖「持續學習」理念的入門形式，適合學齡前～國小低年級親子一起參與。"
                        </p>
                        // TODO: 詳細課程說明文案待 Penny 提供，見 spec.md 4.3 節與 14 待補內容清單
                        <p class="mt-2 text-sm text-slate-gray">"詳細課程內容｜資料整理中"</p>
                    </Card>
                </div>

                <p class="mt-16 text-center text-lg font-medium text-brand-blue">
                    "🌱 國小英文（打底）→ 🚀 國中英文（建立能力）→ 🎓 高中英文（突破）"
                </p>
                <p class="mt-2 text-center text-sm text-slate-gray">
                    "每個階段不以考試為導向，內容分為「基礎」與「進階」兩個層級，家長可依孩子程度選擇切入點。"
                </p>

                <div class="mt-12 flex flex-col gap-16">
                    {STAGES
                        .iter()
                        .map(|stage| {
                            view! {
                                <div id=stage.anchor class="scroll-mt-24">
                                    <div class="flex flex-col items-center gap-8 lg:flex-row">
                                        <div class="flex-1 lg:order-2">
                                            <h2 class="text-2xl font-bold text-brand-blue">
                                                {stage.emoji} " " {stage.title}
                                            </h2>
                                            <dl class="mt-4 grid grid-cols-1 gap-4 text-sm leading-[1.7] sm:grid-cols-2">
                                                <div>
                                                    <dt class="font-bold text-ink">"適合對象"</dt>
                                                    <dd class="text-slate-gray">{stage.audience}</dd>
                                                </div>
                                                <div>
                                                    <dt class="font-bold text-ink">"能力養成"</dt>
                                                    <dd class="text-slate-gray">{stage.ability}</dd>
                                                </div>
                                                <div>
                                                    <dt class="font-bold text-ink">"基礎班內容"</dt>
                                                    <dd class="text-slate-gray">{stage.basic}</dd>
                                                </div>
                                                <div>
                                                    <dt class="font-bold text-ink">"進階班內容"</dt>
                                                    <dd class="text-slate-gray">{stage.advanced}</dd>
                                                </div>
                                                <div class="sm:col-span-2">
                                                    <dt class="font-bold text-ink">"成果"</dt>
                                                    <dd class="text-slate-gray">{stage.outcome}</dd>
                                                </div>
                                            </dl>
                                        </div>
                                        <img
                                            src=stage.image
                                            alt=format!("{} 插圖", stage.title)
                                            class="w-full max-w-xs flex-shrink-0 lg:order-1 lg:max-w-sm"
                                        />
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>

                <div id="adult" class="mt-16 scroll-mt-24">
                    <Card class="p-6".to_string()>
                        <h2 class="text-xl font-bold text-ink">"成人英文"</h2>
                        <p class="mt-3 text-sm leading-[1.7] text-slate-gray">
                            "學習英文，不該只是背單字、記文法，更重要的是能真正運用在生活與工作中。知暖成人英文課程適合想重新建立英文基礎、提升口說表達、準備職場需求，或希望找回學習自信的學習者。我們採小班互動與陪伴式教學，結合生活情境、實用會話及聽說讀寫整合訓練，依照每位學員的程度調整學習內容，讓英文逐漸融入日常，建立持續學習的習慣與自信。"
                        </p>
                    </Card>
                </div>
            </div>
        </section>
    }
}

#[component]
fn AddonSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <div class="mx-auto max-w-5xl px-6 py-16 lg:py-24">
                <h2 class="text-center text-3xl font-bold text-brand-blue">"學習加值方案"</h2>
                <div class="mt-10 grid grid-cols-1 gap-8 sm:grid-cols-2">
                    <Card class="relative p-6".to_string()>
                        <span class="absolute right-6 top-6 rounded-md bg-warm-amber px-3 py-1 text-xs font-medium text-ink">
                            "推薦"
                        </span>
                        <h3 class="text-xl font-bold text-ink">"🌿 陪跑續航"</h3>
                        <p class="mt-2 text-sm text-slate-gray">"適合對象：需要持續陪伴、養成學習習慣的學生"</p>
                        <p class="mt-2 text-sm leading-[1.7] text-slate-gray">
                            "每週學習任務指派、課後訊息追蹤與答疑、自學力與讀書習慣技法養成"
                        </p>
                    </Card>

                    <Card class="p-6".to_string()>
                        <h3 class="text-xl font-bold text-ink">"🏆 檢定／升學加購"</h3>
                        <p class="mt-2 text-sm text-slate-gray">
                            "適合對象：有明確檢定或升學目標的學生（GEPT、全民英檢、會考、學測等）"
                        </p>
                        <p class="mt-2 text-sm leading-[1.7] text-slate-gray">
                            "客製學習計畫、模考分析、考前衝刺、弱點分析"
                        </p>
                    </Card>
                </div>

                <div class="mt-10 flex flex-col items-center gap-4 text-center">
                    <p class="text-sm text-slate-gray">"詳細方案請加入 LINE 諮詢"</p>
                    <CtaButton href=LINE_URL label="加入 LINE 諮詢" variant=ButtonVariant::Line/>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FooterCtaSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <div class="mx-auto flex max-w-3xl flex-col items-center gap-4 px-6 py-16 text-center lg:py-24">
                <p class="text-lg text-ink">"想更了解適合孩子的方案？"</p>
                <CtaButton href=LINE_URL label="加入 LINE 諮詢，了解適合方案" variant=ButtonVariant::Line/>
            </div>
        </section>
    }
}
