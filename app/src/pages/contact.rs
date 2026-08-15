use leptos::form::ActionForm;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::admin_auth::friendly_error_message;
use crate::components::icons::{FacebookIcon, InstagramIcon};
use crate::components::{ButtonVariant, Card, CtaButton, FadeIn, Reveal};

const LINE_URL: &str = "https://line.me/R/ti/p/@891ivojl";
const CONTACT_EMAIL: &str = "zhinuanwarmstudio@gmail.com";

const LEARNING_STAGE_OPTIONS: [&str; 5] = ["親子", "國小", "國中", "高中", "成人"];

const CONCERN_OPTIONS: [&str; 7] =
    ["英文基礎", "跟不上進度", "升學準備", "學習習慣", "學習方法", "成人英文", "其他"];

/// 聯絡表單送出，對照 spec.md 4.5 節與 9.4 節 honeypot 防機器人規範
#[server(SubmitContactForm, "/api")]
pub async fn submit_contact_form(
    name: String,
    phone: String,
    learning_stage: String,
    concern: String,
    message: String,
    honeypot: String,
) -> Result<(), ServerFnError> {
    // honeypot 有值視為機器人，靜默丟棄、不寫入也不寄信，但仍回傳成功避免暴露防護機制
    if !honeypot.trim().is_empty() {
        tracing::warn!("聯絡表單疑似機器人送出（honeypot 欄位有值），已丟棄");
        return Ok(());
    }

    let name = name.trim();
    let phone = phone.trim();
    let learning_stage = learning_stage.trim();
    let concern = concern.trim();
    let message = message.trim();

    if name.is_empty() {
        return Err(ServerFnError::new("請填寫姓名"));
    }
    if name.chars().count() > 100 {
        return Err(ServerFnError::new("姓名長度過長"));
    }

    let phone_digit_count = phone.chars().filter(|c| c.is_ascii_digit()).count();
    let phone_valid = !phone.is_empty()
        && (8..=15).contains(&phone_digit_count)
        && phone.chars().all(|c| c.is_ascii_digit() || matches!(c, '-' | ' ' | '+' | '(' | ')'));
    if !phone_valid {
        return Err(ServerFnError::new("請輸入正確格式的聯絡電話"));
    }

    if learning_stage.is_empty() {
        return Err(ServerFnError::new("請選擇目前的學習階段"));
    }
    if concern.is_empty() {
        return Err(ServerFnError::new("請選擇目前最希望改善的狀況"));
    }

    let pool = expect_context::<sqlx::SqlitePool>();

    sqlx::query(
        "INSERT INTO contact_submissions (name, phone, learning_stage, concern, message, is_read, created_at)
         VALUES (?, ?, ?, ?, ?, 0, datetime('now'))",
    )
    .bind(name)
    .bind(phone)
    .bind(learning_stage)
    .bind(concern)
    .bind((!message.is_empty()).then_some(message))
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("寫入聯絡表單資料失敗: {e}");
        ServerFnError::new("送出失敗，請稍後再試")
    })?;

    // TODO: Email 通知功能延後實作，待確認是否需要對 Gmail 帳號開啟兩步驟驗證後再處理，
    // 聯絡表單資料可透過後台查看

    Ok(())
}

#[component]
pub fn ContactPage() -> impl IntoView {
    let submit_action = ServerAction::<SubmitContactForm>::new();
    let response = submit_action.value();

    view! {
        <Title text="聯絡知暖 Warm Learning Studio｜高雄英文課程免費諮詢"/>
        <Meta
            name="description"
            content="歡迎聯絡知暖學習工作室（Warm Learning Studio），提供高雄幼兒到成人英文課程與免費學習諮詢，協助找到適合的學習方式，建立自主學習系統。"
        />

        <section class="bg-mist-blue">
            <Reveal class="mx-auto grid max-w-6xl grid-cols-1 gap-8 px-6 py-16 lg:grid-cols-2 lg:py-24"
                .to_string()>
                <ContactInfoCard/>

                <Card class="p-6 lg:p-10".to_string()>
                    <h2 class="text-2xl font-bold text-brand-blue">"預約學習諮詢"</h2>

                    <ActionForm action=submit_action attr:class="mt-6 flex flex-col gap-5">
                        <div class="flex flex-col gap-1.5">
                            <label for="contact-name" class="text-sm font-medium text-ink">
                                "姓名"
                            </label>
                            <input
                                id="contact-name"
                                type="text"
                                name="name"
                                required
                                class="rounded-xl border border-border-gray px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                            />
                        </div>

                        <div class="flex flex-col gap-1.5">
                            <label for="contact-phone" class="text-sm font-medium text-ink">
                                "聯絡電話"
                            </label>
                            <input
                                id="contact-phone"
                                type="tel"
                                name="phone"
                                required
                                placeholder="例如：0912-345-678"
                                class="rounded-xl border border-border-gray px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                            />
                        </div>

                        <div class="flex flex-col gap-1.5">
                            <label for="contact-learning-stage" class="text-sm font-medium text-ink">
                                "目前的學習階段"
                            </label>
                            <select
                                id="contact-learning-stage"
                                name="learning_stage"
                                required
                                class="rounded-xl border border-border-gray bg-white px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                            >
                                {LEARNING_STAGE_OPTIONS
                                    .iter()
                                    .map(|option| view! { <option value=*option>{*option}</option> })
                                    .collect_view()}
                            </select>
                        </div>

                        <div class="flex flex-col gap-1.5">
                            <label for="contact-concern" class="text-sm font-medium text-ink">
                                "目前最希望改善的狀況"
                            </label>
                            <select
                                id="contact-concern"
                                name="concern"
                                required
                                class="rounded-xl border border-border-gray bg-white px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                            >
                                {CONCERN_OPTIONS
                                    .iter()
                                    .map(|option| view! { <option value=*option>{*option}</option> })
                                    .collect_view()}
                            </select>
                        </div>

                        <div class="flex flex-col gap-1.5">
                            <label for="contact-message" class="text-sm font-medium text-ink">
                                "想告訴我們的事情（選填）"
                            </label>
                            <textarea
                                id="contact-message"
                                name="message"
                                rows="4"
                                placeholder="當上一項選「其他」時，可在這裡補充說明"
                                class="rounded-xl border border-border-gray px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                            ></textarea>
                        </div>

                        // honeypot：一般訪客看不到、不會填寫，機器人常會自動填入所有欄位
                        <div class="absolute left-[-9999px] top-auto h-0 w-0 overflow-hidden" aria-hidden="true">
                            <label for="contact-honeypot">"網站"</label>
                            <input id="contact-honeypot" type="text" name="honeypot" tabindex="-1" autocomplete="off"/>
                        </div>

                        <button
                            type="submit"
                            class="mt-2 rounded-full bg-brand-blue px-8 py-3 font-medium text-white transition-all hover:bg-[#14294F]"
                        >
                            "送出"
                        </button>

                        <Show when=move || response.get().is_some()>
                            {move || match response.get() {
                                Some(Ok(())) => {
                                    view! {
                                        <FadeIn class="rounded-xl bg-mist-blue px-4 py-3 text-sm text-success-green"
                                            .to_string()>
                                            "送出成功，我們會盡快與您聯繫"
                                        </FadeIn>
                                    }
                                        .into_any()
                                }
                                Some(Err(e)) => {
                                    view! {
                                        <FadeIn class="rounded-xl bg-mist-blue px-4 py-3 text-sm text-error-red"
                                            .to_string()>
                                            {friendly_error_message(&e)}
                                        </FadeIn>
                                    }
                                        .into_any()
                                }
                                None => view! { <span/> }.into_any(),
                            }}
                        </Show>
                    </ActionForm>
                </Card>
            </Reveal>
        </section>
    }
}

/// 左欄卡片「先聊聊目前的學習狀況」，對照 spec.md 4.5 節
#[component]
fn ContactInfoCard() -> impl IntoView {
    view! {
        <Card class="flex flex-col gap-5 p-6 lg:p-10".to_string()>
            <h1 class="text-2xl font-bold text-brand-blue">"先聊聊目前的學習狀況"</h1>
            <p class="text-base leading-[1.7] text-ink">
                "不確定該選哪一堂課沒關係。告訴我們目前遇到的問題，我們會先一起釐清真正需要的是什麼。"
            </p>

            <CtaButton href=LINE_URL label="LINE 詢問" variant=ButtonVariant::Line/>

            <p class="text-sm text-slate-gray">"我們會盡快透過 LINE 或電話與您聯繫"</p>

            <div class="flex items-center gap-4">
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

            <p class="mt-auto text-xs text-slate-gray">{format!("{CONTACT_EMAIL}（後台通知信箱）")}</p>
        </Card>
    }
}
