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

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="關於知暖｜陪伴孩子建立自主學習力"/>
        <Meta
            name="description"
            content="了解知暖學習工作室的創立理念與教育初衷。我們深信陪伴比催促更重要，透過專業教學與溫暖引導，陪伴高雄國小至高中學生建立自主學習能力。"
        />

        <BrandStorySection/>
        <FounderSection/>
        <VisionSection/>
    }
}

#[component]
fn BrandStorySection() -> impl IntoView {
    view! {
        <section class="bg-white">
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

#[component]
fn FounderSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <div class="mx-auto flex max-w-7xl flex-col items-center gap-10 px-6 py-16 lg:flex-row lg:py-24">
                // TODO: Penny 個人照片尚未提供（見 docs/asset-list.md「人物照片」）
                <ImagePlaceholder
                    label="創辦人照片準備中"
                    class="aspect-square w-full max-w-xs rounded-2xl flex-shrink-0"
                />
                <div class="flex-1 text-center lg:text-left">
                    <h2 class="text-2xl font-bold text-brand-blue">"創辦人 Penny"</h2>
                    <blockquote class="mt-4 text-lg leading-[1.7] text-ink">
                        "Hi，我是 Penny。我相信，每個孩子都能找到適合自己的學習方式。我們教的不只是英文，而是一輩子的學習能力。"
                    </blockquote>
                    // TODO: 完整經歷（教學年資、證照、專長）尚未提供，見 spec.md 4.2 ②
                    <p class="mt-4 text-sm text-slate-gray">"完整經歷（教學年資、證照、專長）｜資料整理中"</p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn VisionSection() -> impl IntoView {
    view! {
        <section class="bg-white">
            <div class="mx-auto max-w-3xl px-6 py-16 text-center lg:py-24">
                <h2 class="text-3xl font-bold text-brand-blue">"品牌願景"</h2>
                <p class="mt-6 text-lg leading-[1.7] text-ink">
                    "陪伴每位學習者建立自主學習能力，讓英文成為探索世界與實現夢想的力量。"
                </p>
            </div>
        </section>
    }
}
