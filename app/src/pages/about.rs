use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::icons::{HeartIcon, PersonIcon, TrendUpIcon};
use crate::components::{Card, Reveal};

/// 創辦人經歷段落，沿用先前已定案的敘事文案，併入第三屏呈現（spec.md 4.2「創辦人經歷段落」）
const FOUNDER_BIO: &str = "Penny 擁有 7 年英文教學經驗，擅長在對話中快速理解真正的問題所在，用引導式的提問幫助孩子跟家長釐清想法與目標，而不是急著給答案。她重視關係經營，相信信任感是學習動力的根本；也把多年帶班、辦活動、公開表達的經驗，轉化成一套能被複製、能被系統化執行的教學方法——這也是「知暖成長之旅 Learning Journey」的由來。";

struct BeliefCard {
    title: &'static str,
    body: &'static str,
}

const BELIEF_CARDS: [BeliefCard; 3] = [
    BeliefCard {
        title: "溫暖",
        body: "我們希望先理解孩子，而不是急著替他貼上「不認真」、「沒天分」或「不夠努力」的標籤。每一個看起來不願意學、不知道怎麼開始，或總是需要被提醒的孩子，背後可能都有不同的原因。",
    },
    BeliefCard {
        title: "堅定",
        body: "理解，不代表沒有要求。真正的陪伴，是在孩子想放棄、卡住或不知道怎麼繼續的時候，陪他把問題拆小、找到方法，再一步一步往前。",
    },
    BeliefCard {
        title: "長期主義",
        body: "我們相信的不是短時間的衝刺，而是長期累積。先把方法、習慣與學習節奏建立好，再追求更穩定的成果。這就是知暖所相信的「先勝後戰」，也是我們想實踐的長期主義學習。",
    },
];

const LEARNING_JOURNEY_STEPS: [&str; 4] = ["英文能力", "學習方法", "學習習慣", "自主學習"];

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="關於知暖 Warm Learning Studio｜陪伴建立自主學習力"/>
        <Meta
            name="description"
            content="了解知暖學習工作室（Warm Learning Studio）的創立理念與教育初衷。我們深信陪伴比催促更重要，透過專業教學與溫暖引導，陪伴高雄幼兒到成人學習者建立自主學習能力。"
        />

        <HeroScreen/>
        <ProblemScreen/>
        <WhyWarmLearningScreen/>
        <WhatWeBelieveScreen/>
        <LearningJourneyScreen/>
        <VisionScreen/>
    }
}

/// 第一屏：大標區塊（spec.md 4.2①）
#[component]
fn HeroScreen() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <Reveal class="mx-auto flex max-w-3xl flex-col items-center gap-8 px-6 py-16 text-center lg:py-24"
                .to_string()>
                <img
                    src="/img/penny-photo.jpg"
                    alt="知暖學習工作室創辦人 Penny"
                    class="aspect-square w-full max-w-xs rounded-2xl object-cover"
                />
                <h1 class="text-3xl font-bold leading-[1.4] text-brand-blue lg:text-4xl">
                    "從一堂英文課，走向一段更長久的學習陪伴"
                </h1>
            </Reveal>
        </section>
    }
}

/// 第二屏：我看見的問題（spec.md 4.2②）
#[component]
fn ProblemScreen() -> impl IntoView {
    view! {
        <section class="bg-white">
            <Reveal class="mx-auto max-w-3xl px-6 py-16 lg:py-24".to_string()>
                <h2 class="text-center text-3xl font-bold text-brand-blue">"我看見的問題"</h2>
                <div class="mt-8 flex flex-col gap-5 text-base leading-[1.7] text-ink">
                    <p>
                        "一開始教英文時，我和很多老師一樣，希望把課教清楚、把學生不會的地方教會，也希望看見孩子的成績慢慢進步。"
                    </p>
                    <p>
                        "但陪伴的學生越來越多，我開始發現一件事：" <br/>
                        "很多孩子真正卡住的，並不只是單字背不起來、文法不熟，或某一次考試沒有考好。"
                    </p>
                    <p>
                        "有些孩子其實很努力，卻不知道怎麼複習；" <br/>
                        "有些孩子補了很多課，回到家還是需要爸媽每天提醒；" <br/>
                        "有些孩子成績並不差，卻總覺得自己「英文不好」；" <br/>
                        "也有些孩子在老師身邊可以完成很多事情，一離開課堂，卻不知道下一步該做什麼。"
                    </p>
                    <p>
                        "那時候我開始問自己：" <br/>
                        "如果孩子每一次遇到問題，都只能等老師告訴他答案，那我們教會他的，真的只是這一題，還是也能讓他慢慢知道下一次該怎麼面對？"
                    </p>
                </div>
            </Reveal>
        </section>
    }
}

/// 第三屏：為什麼有知暖（spec.md 4.2③），末段併入創辦人經歷段落
#[component]
fn WhyWarmLearningScreen() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <Reveal class="mx-auto max-w-3xl px-6 py-16 lg:py-24".to_string()>
                <h2 class="text-center text-3xl font-bold text-brand-blue">"為什麼有知暖"</h2>
                <div class="mt-8 flex flex-col gap-5 text-base leading-[1.7] text-ink">
                    <p>
                        "我慢慢發現，一堂真正有價值的課，不應該只結束在「今天教完多少內容」。除了英文能力之外，孩子其實也需要慢慢學會：怎麼看見自己的問題、怎麼安排練習、怎麼回頭檢查、怎麼調整方法，以及遇到不會的事情時，怎麼繼續往下走。"
                    </p>
                    <p>
                        "學習不只是把答案做對。更重要的是，孩子能不能逐漸知道：「我現在在哪裡？」「我卡在哪裡？」「接下來我可以做什麼？」這也成為後來知暖很重要的起點。"
                    </p>
                    <p>
                        "「知暖學習工作室」的誕生，不只是因為我想開一間英文教室。而是因為我希望，有一個地方可以把「英文學習」和「學會學習」真正放在一起。"
                    </p>
                    <p>
                        "在知暖，我們仍然重視單字、文法、閱讀、寫作與升學成果。但我們同樣在意孩子怎麼走到那個成果。我們會先了解孩子現在的程度、習慣與卡關的位置，再決定適合他的學習方式；也透過課堂、練習、陪跑、回顧與調整，把學習慢慢從一週的一堂課，延伸到孩子真正的生活裡。"
                    </p>
                    <p>"因為我們相信：好的教學，不只是老師越來越會教，而是孩子慢慢越來越會學。"</p>
                </div>

                <Card class="mt-10 p-6".to_string()>
                    <p class="text-sm font-bold text-brand-blue">"創辦人 Penny"</p>
                    <p class="mt-2 text-sm leading-[1.7] text-slate-gray">{FOUNDER_BIO}</p>
                </Card>
            </Reveal>
        </section>
    }
}

/// 第四屏：知暖相信的學習（spec.md 4.2④），溫暖／堅定／長期主義做成 3 個視覺重點卡片
#[component]
fn WhatWeBelieveScreen() -> impl IntoView {
    view! {
        <section class="bg-white">
            <Reveal class="mx-auto max-w-5xl px-6 py-16 lg:py-24".to_string()>
                <h2 class="text-center text-3xl font-bold text-brand-blue">"知暖相信的學習"</h2>
                <p class="mx-auto mt-6 max-w-3xl text-center text-base leading-[1.7] text-ink">
                    "「知暖」對我們來說，有兩個很重要的力量。"
                </p>

                <div class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-3">
                    {BELIEF_CARDS
                        .iter()
                        .enumerate()
                        .map(|(i, belief)| {
                            view! {
                                <Reveal delay_ms=(i as u32) * 110>
                                    <div class="flex flex-col items-center gap-3 text-center">
                                        <div class="flex h-16 w-16 items-center justify-center rounded-full bg-pale-blue text-brand-blue">
                                            {match i {
                                                0 => view! { <HeartIcon/> }.into_any(),
                                                1 => view! { <PersonIcon/> }.into_any(),
                                                _ => view! { <TrendUpIcon/> }.into_any(),
                                            }}
                                        </div>
                                        <h3 class="text-xl font-bold text-ink">{belief.title}</h3>
                                        <p class="text-sm leading-[1.7] text-slate-gray">{belief.body}</p>
                                    </div>
                                </Reveal>
                            }
                        })
                        .collect_view()}
                </div>
            </Reveal>
        </section>
    }
}

/// 第五屏：從被帶著學到知道怎麼學（spec.md 4.2⑤），含「英文能力→學習方法→學習習慣→自主學習」視覺流程
#[component]
fn LearningJourneyScreen() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <Reveal class="mx-auto max-w-3xl px-6 py-16 lg:py-24".to_string()>
                <h2 class="text-center text-3xl font-bold text-brand-blue">"從被帶著學到知道怎麼學"</h2>

                <div class="mt-10 flex flex-col items-center gap-3 lg:flex-row lg:justify-center lg:gap-4">
                    {LEARNING_JOURNEY_STEPS
                        .iter()
                        .enumerate()
                        .map(|(i, step)| {
                            view! {
                                <div class="rounded-full bg-white px-6 py-3 text-center text-sm font-bold text-brand-blue shadow-md lg:text-base">
                                    {*step}
                                </div>
                                {(i < LEARNING_JOURNEY_STEPS.len() - 1)
                                    .then(|| {
                                        view! {
                                            <span aria-hidden="true" class="text-xl font-bold text-sky-blue lg:hidden">
                                                "↓"
                                            </span>
                                            <span aria-hidden="true" class="hidden text-xl font-bold text-sky-blue lg:inline">
                                                "→"
                                            </span>
                                        }
                                    })}
                            }
                        })
                        .collect_view()}
                </div>

                <div class="mt-10 flex flex-col gap-5 text-base leading-[1.7] text-ink">
                    <p>
                        "現在的知暖，希望做的不只是英文教學。我們正在建立一套從英文能力、學習方法、學習習慣，到自主學習，逐漸成長的學習系統。"
                    </p>
                    <p>
                        "老師一開始可以陪得多一點、提醒得多一點、帶著孩子走得多一點。但我們真正期待的，不是孩子永遠依賴老師。而是有一天，他開始可以自己發現：「這裡我還不熟。」「這一次的方法好像不適合。」「我可以重新安排一下。」「我知道下一步該怎麼做。」"
                    </p>
                    <p>"那一刻，學習才真正開始變成他自己的能力。"</p>
                </div>
            </Reveal>
        </section>
    }
}

/// 第六屏：我們想陪孩子去的地方（spec.md 4.2⑥），品牌願景收尾屏
#[component]
fn VisionScreen() -> impl IntoView {
    view! {
        <section class="bg-white">
            <Reveal class="mx-auto max-w-3xl px-6 py-16 lg:py-24".to_string()>
                <h2 class="text-center text-3xl font-bold text-brand-blue">"我們想陪孩子去的地方"</h2>
                <div class="mt-8 flex flex-col gap-5 text-base leading-[1.7] text-ink">
                    <p>
                        "成績很重要，升學也很重要。但如果可以，我們希望孩子在一次次學習的過程裡，帶走更多東西：對自己的理解、面對困難的方法、持續練習的能力，以及相信自己可以慢慢進步的信心。"
                    </p>
                    <p>
                        "我們希望有一天，即使老師不在旁邊，即使他遇到的是一個從來沒有學過的新問題，他仍然知道：" <strong>"我可以先從哪裡開始。"</strong>
                    </p>
                    <p>"如果這份能力能陪著孩子走進國中、高中、大學，甚至走進未來的人生，那就是知暖最希望留下的事情。"</p>
                    <p class="text-lg font-medium text-brand-blue">
                        "知足上進，溫暖而堅定。陪伴每一個孩子，把學習慢慢變成自己的力量。"
                    </p>
                </div>
            </Reveal>
        </section>
    }
}
