use leptos::form::ActionForm;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::icons::{ChatIcon, FacebookIcon, InstagramIcon};

const LINE_URL: &str = "https://line.me/R/ti/p/@891ivojl";

const COURSE_OPTIONS: [&str; 6] = ["國小英文", "國中英文", "高中英文", "成人英文", "線上親子共學", "尚未確定"];

/// 聯絡表單送出，對照 spec.md 4.5 節與 9.4 節 honeypot 防機器人規範
#[server(SubmitContactForm, "/api")]
pub async fn submit_contact_form(
    name: String,
    phone: String,
    course_interest: String,
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
    let course_interest = course_interest.trim();
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

    let pool = expect_context::<sqlx::SqlitePool>();

    sqlx::query(
        "INSERT INTO contact_submissions (name, phone, course_interest, message, is_read, created_at)
         VALUES (?, ?, ?, ?, 0, datetime('now'))",
    )
    .bind(name)
    .bind(phone)
    .bind((!course_interest.is_empty()).then_some(course_interest))
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
        <Title text="聯絡知暖｜高雄英文課程免費諮詢"/>
        <Meta
            name="description"
            content="歡迎聯絡知暖學習工作室，提供高雄國小至高中英文、成人英文課程與免費學習諮詢，協助學生找到適合自己的學習方式，培養自主學習能力。"
        />

        <ContactInfoSection/>

        <section class="bg-white">
            <div class="mx-auto max-w-2xl px-6 py-16 lg:py-24">
                <h2 class="text-center text-3xl font-bold text-brand-blue">"聯絡表單"</h2>
                <p class="mt-3 text-center text-sm text-slate-gray">
                    "若不方便使用 LINE，也可以透過以下表單留下您的資訊，我們會盡快與您聯繫。"
                </p>

                <ActionForm action=submit_action attr:class="mt-10 flex flex-col gap-5">
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
                            placeholder="例如：0910-406-387"
                            class="rounded-xl border border-border-gray px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                        />
                    </div>

                    <div class="flex flex-col gap-1.5">
                        <label for="contact-course" class="text-sm font-medium text-ink">
                            "想諮詢的課程類型"
                        </label>
                        <select
                            id="contact-course"
                            name="course_interest"
                            class="rounded-xl border border-border-gray bg-white px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                        >
                            {COURSE_OPTIONS
                                .iter()
                                .map(|option| view! { <option value=*option>{*option}</option> })
                                .collect_view()}
                        </select>
                    </div>

                    <div class="flex flex-col gap-1.5">
                        <label for="contact-message" class="text-sm font-medium text-ink">
                            "留言內容（選填）"
                        </label>
                        <textarea
                            id="contact-message"
                            name="message"
                            rows="4"
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
                                    <p class="rounded-xl bg-mist-blue px-4 py-3 text-sm text-success-green">
                                        "送出成功，我們會盡快與您聯繫"
                                    </p>
                                }
                                    .into_any()
                            }
                            Some(Err(e)) => {
                                view! {
                                    <p class="rounded-xl bg-mist-blue px-4 py-3 text-sm text-error-red">
                                        {e.to_string()}
                                    </p>
                                }
                                    .into_any()
                            }
                            None => view! { <span/> }.into_any(),
                        }}
                    </Show>
                </ActionForm>
            </div>
        </section>
    }
}

#[component]
fn ContactInfoSection() -> impl IntoView {
    view! {
        <section class="bg-mist-blue">
            <div class="mx-auto flex max-w-3xl flex-col items-center gap-6 px-6 py-16 text-center lg:py-24">
                <h1 class="text-4xl font-bold text-brand-blue">"聯絡我們"</h1>
                <p class="max-w-xl text-base leading-[1.7] text-slate-gray">
                    "有任何課程或試聽相關問題，歡迎透過 LINE 官方帳號與我們聯繫，我們會盡快回覆您。"
                </p>

                <a
                    href=LINE_URL
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex items-center gap-2 rounded-full bg-line-green px-10 py-4 text-lg font-medium text-white transition-all hover:brightness-95"
                >
                    <ChatIcon/>
                    "加入 LINE 諮詢 @891ivojl"
                </a>

                <a href="tel:0910406387" class="text-sm text-slate-gray hover:text-brand-blue">
                    "或撥打電話 0910-406-387"
                </a>

                <div class="flex items-center gap-4 pt-2">
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
            </div>
        </section>
    }
}
