use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::{Card, Reveal};

#[derive(Clone, Copy)]
struct Stage {
    emoji: Option<&'static str>,
    title: &'static str,
    anchor: &'static str,
    image: &'static str,
    audience: &'static str,
    basic: &'static str,
    advanced: &'static str,
    ability: &'static str,
    outcome: &'static str,
}

const CYCLE_STEPS: [&str; 5] = ["了解狀況", "課堂學習", "課後練習與陪跑", "回顧與調整", "找到下一步"];

struct LearningFeatureCard {
    title: &'static str,
    body: &'static str,
}

const LEARNING_FEATURE_CARDS: [LearningFeatureCard; 5] = [
    LearningFeatureCard { title: "學習導航", body: "知道「我現在在哪裡、下一步做什麼」。" },
    LearningFeatureCard {
        title: "學習續航",
        body: "讓學習的溫度不因下課而降溫，持續保溫到下一次上課，學習力也跟著穩定升溫。",
    },
    LearningFeatureCard { title: "學習方法", body: "學會複習、整理、安排與自我檢查。" },
    LearningFeatureCard {
        title: "親子共學",
        body: "讓家長也能了解如何陪伴孩子一起成長，成為孩子成長路上最踏實的助力。",
    },
    LearningFeatureCard { title: "AI 學習力", body: "學會利用工具幫助思考，而不是依賴答案。" },
];

const STAGES: [Stage; 3] = [
    Stage {
        emoji: Some("🌱"),
        title: "國小英文（打底）",
        anchor: "elementary",
        image: "/img/illustration-elementary.png",
        audience: "國小一年級～六年級（6～12 歲）",
        basic: "Phonics 自然發音、基礎字彙、簡單會話、繪本閱讀入門",
        advanced: "閱讀理解、口說表達、繪本延伸閱讀、AI 互動練習",
        ability: "聽、說、讀、寫、自主完成作業",
        outcome: "建立英文自信、喜歡閱讀、敢開口說英文",
    },
    Stage {
        emoji: Some("🚀"),
        title: "國中英文（建立能力）",
        anchor: "middle",
        image: "/img/illustration-middle.png",
        audience: "國一～國三（12～15 歲）",
        basic: "文法基礎、單字累積、閱讀入門、聽力入門",
        advanced: "長篇閱讀、寫作訓練、聽力強化、自主讀書規劃",
        ability: "能閱讀長篇文章、建立筆記能力、自主規劃讀書、學會錯題整理",
        outcome: "英文能力穩定提升、建立自主學習習慣",
    },
    Stage {
        emoji: Some("🎓"),
        title: "高中英文（突破）",
        anchor: "high",
        image: "/img/illustration-high.png",
        audience: "高一～高三（15～18 歲）",
        basic: "文法統整、閱讀測驗基礎、寫作入門",
        advanced: "英文思辨、進階寫作、AI 輔助閱讀與寫作應用",
        ability: "大量閱讀、英文表達、邏輯思考、自主學習、簡報能力",
        outcome: "具備英文自主閱讀能力、銜接大學英文程度",
    },
];

// 幼兒線上親子共學與成人英文，v13 改用跟國小～高中三階段相同的卡片版面，
// 內容依 spec.md 4.3③ 表格拆解；不存在的欄位標註「待填寫詳細內文」，不代填臆測內容
const FAMILY_STAGE: Stage = Stage {
    emoji: None,
    title: "幼兒線上親子共學",
    anchor: "family",
    image: "/img/illustration-kids-parents.png",
    audience: "學齡前～國小低年級親子（6 歲以下）",
    basic: "待填寫詳細內文",
    advanced: "待填寫詳細內文",
    ability: "待填寫詳細內文",
    outcome: "陪伴孩子從小養成規律接觸英文的習慣，是知暖「持續學習」理念的入門形式",
};

const ADULT_STAGE: Stage = Stage {
    emoji: None,
    title: "成人英文",
    anchor: "adult",
    image: "/img/illustration-adult.png",
    audience: "想重新建立英文基礎、提升口說表達、準備職場需求，或希望找回學習自信的學習者（18 歲以上）",
    basic: "小班互動與陪伴式教學，結合生活情境、實用會話及聽說讀寫整合訓練",
    advanced: "待填寫詳細內文",
    ability: "待填寫詳細內文",
    outcome: "英文逐漸融入日常，建立持續學習的習慣與自信",
};

#[component]
pub fn CoursesPage() -> impl IntoView {
    view! {
        <Title text="知暖英文課程｜幼兒、國小、國中、高中與成人英文"/>
        <Meta
            name="description"
            content="知暖學習工作室（Warm Learning Studio）提供高雄幼兒親子共學、國小、國中、高中英文及成人英文課程，依程度客製化規劃，結合陪伴式教育與自主學習訓練，協助建立完整英文學習系統。"
        />

        <OpeningSection/>
        <TimelineSection/>
        <FooterCtaSection/>
    }
}

#[component]
fn TimelineSection() -> impl IntoView {
    view! {
        <section class="bg-white">
            <Reveal class="mx-auto max-w-5xl px-6 py-16 lg:py-24".to_string()>
                <StageCard stage=FAMILY_STAGE/>

                <div class="mt-16 flex flex-col gap-16">
                    {STAGES
                        .iter()
                        .map(|stage| view! { <StageCard stage=*stage/> })
                        .collect_view()}
                </div>

                <div class="mt-16">
                    <StageCard stage=ADULT_STAGE/>
                </div>
            </Reveal>
        </section>
    }
}

/// 課程地圖卡片版面（spec.md 4.3③），幼兒線上親子共學／國小／國中／高中／成人英文
/// 五張卡片共用同一版面：圖片＋固定欄位結構（適合對象／基礎班內容／進階班內容／能力養成／成果）
#[component]
fn StageCard(stage: Stage) -> impl IntoView {
    view! {
        <div id=stage.anchor class="scroll-mt-24">
            <div class="flex flex-col items-center gap-8 lg:flex-row">
                <div class="flex-1 lg:order-2">
                    <h2 class="text-3xl font-bold text-brand-blue">
                        {stage.emoji.map(|emoji| format!("{emoji} "))} {stage.title}
                    </h2>
                    <dl class="mt-4 grid grid-cols-1 gap-4 text-base leading-[1.7] sm:grid-cols-2">
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
}

/// 開場區塊：頁首＋「知暖的課程，不只是在上課的那一段時間」合併為單一區塊
/// （spec.md 4.3①，v14 全面改版，取代原本分開的 PageHeader／LearningCycleSection 兩段式結構）
#[component]
fn OpeningSection() -> impl IntoView {
    view! {
        <section class="bg-gradient-to-b from-mist-blue to-white">
            <Reveal class="mx-auto max-w-5xl px-6 py-16 text-center lg:py-24".to_string()>
                <h1 class="text-5xl font-bold text-brand-blue">"知暖 Learning Journey"</h1>
                <p class="mt-4 text-4xl font-bold text-slate-gray">
                    "知暖的課程，不只是在上課的那一段時間"
                </p>

                // 循環式流程靜態版面，動畫留待之後跟 Learning Journey 時間軸一起處理（spec.md 4.3① 動畫備註）
                <div class="mt-12 flex flex-col items-center gap-3 lg:flex-row lg:justify-center lg:gap-4">
                    {CYCLE_STEPS
                        .iter()
                        .enumerate()
                        .map(|(i, step)| {
                            view! {
                                <div class="rounded-full bg-white px-5 py-3 text-center text-base font-bold text-brand-blue shadow-md lg:text-lg">
                                    {*step}
                                </div>
                                {(i < CYCLE_STEPS.len() - 1)
                                    .then(|| {
                                        view! {
                                            <span aria-hidden="true" class="text-2xl font-bold text-sky-blue lg:hidden">
                                                "↓"
                                            </span>
                                            <span aria-hidden="true" class="hidden text-2xl font-bold text-sky-blue lg:inline">
                                                "→"
                                            </span>
                                        }
                                    })}
                            }
                        })
                        .collect_view()}
                </div>

                <p class="mx-auto mt-8 max-w-3xl text-lg leading-[1.7] text-slate-gray">
                    "知暖的課程不是一份單堂課程列表，而是一張陪伴孩子持續學習、不中斷成長的地圖。我們所有課程設計的共同目標，都是讓孩子能長期、穩定地接觸英文，而不是學一下就中斷。為了做到這件事，我們把學習拆解成一套孩子可以逐漸接手的完整流程——從了解目前的狀況開始，到課堂上的學習、下課後的練習與陪跑、定期的回顧與調整，一路陪著孩子知道自己現在在哪裡、下一步該怎麼走，也知道怎麼回頭看見自己一路走來的進步。"
                </p>

                <div class="mt-16 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-5">
                    {LEARNING_FEATURE_CARDS
                        .iter()
                        .enumerate()
                        .map(|(i, card)| {
                            view! {
                                <Reveal delay_ms=(i as u32) * 110>
                                    <Card class="p-6".to_string()>
                                        <h3 class="text-xl font-bold text-ink">{card.title}</h3>
                                        <p class="mt-2 text-base leading-[1.7] text-slate-gray">{card.body}</p>
                                    </Card>
                                </Reveal>
                            }
                        })
                        .collect_view()}
                </div>
            </Reveal>
        </section>
    }
}

/// 頁尾收尾標語（spec.md 4.3③，v14：取代原「想更了解適合孩子的方案？」文字＋ LINE 按鈕）
#[component]
fn FooterCtaSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <Reveal class="mx-auto max-w-3xl px-6 py-16 text-center lg:py-24".to_string()>
                <p class="text-3xl font-bold text-brand-blue">"讓你從英文廢柴變成英文小天才🔥"</p>
            </Reveal>
        </section>
    }
}
