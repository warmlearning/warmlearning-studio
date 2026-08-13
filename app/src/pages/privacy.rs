use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::Reveal;

#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        <Title text="隱私權政策與服務條款｜知暖學習工作室"/>
        <Meta name="robots" content="noindex"/>

        <section class="bg-white">
            <Reveal class="mx-auto max-w-3xl px-6 py-16 lg:py-24".to_string()>
                <h1 class="text-center text-3xl font-bold text-brand-blue">"隱私權政策與服務條款"</h1>

                <nav class="mt-8 flex justify-center gap-6 text-sm">
                    <a href="#privacy-policy" class="text-brand-blue hover:underline">
                        "隱私權政策"
                    </a>
                    <a href="#terms-of-service" class="text-brand-blue hover:underline">
                        "服務條款"
                    </a>
                </nav>

                <article id="privacy-policy" class="mt-12 scroll-mt-24">
                    <h2 class="text-2xl font-bold text-brand-blue">"隱私權政策"</h2>
                    <div class="mt-4 flex flex-col gap-4 text-sm leading-[1.7] text-ink">
                        <p>
                            "知暖學習工作室（以下稱「本工作室」）非常重視您的隱私權，以下說明我們如何蒐集、使用及保護您透過本網站提供的個人資料。"
                        </p>

                        <h3 class="mt-2 font-bold text-ink">"一、蒐集的資料項目"</h3>
                        <p>"當您透過本網站的聯絡表單或訂閱表單與我們互動時，我們會蒐集以下資訊："</p>
                        <ul class="list-disc pl-6">
                            <li>"姓名"</li>
                            <li>"聯絡電話"</li>
                            <li>"欲諮詢的課程類型"</li>
                            <li>"您主動留下的留言內容"</li>
                            <li>"訂閱電子報時提供的 Email 信箱"</li>
                        </ul>

                        <h3 class="mt-2 font-bold text-ink">"二、蒐集目的"</h3>
                        <p>"我們蒐集上述資料，僅用於："</p>
                        <ul class="list-disc pl-6">
                            <li>"回覆您的課程諮詢與試聽預約"</li>
                            <li>"透過電話或 LINE 與您聯繫，說明課程內容與相關資訊"</li>
                            <li>"（若您訂閱電子報）不定期寄送知暖學習工作室的最新消息與學習資源"</li>
                        </ul>

                        <h3 class="mt-2 font-bold text-ink">"三、資料的保存與保護"</h3>
                        <p>
                            "您的資料將儲存於本工作室的內部系統中，僅供本工作室內部人員因前項目的使用，我們不會將您的個人資料出售、出租或提供給第三方，法律另有規定者除外。"
                        </p>

                        <h3 class="mt-2 font-bold text-ink">"四、您的權利"</h3>
                        <p>"您可以隨時透過本網站的聯絡方式，要求查詢、更正、或刪除我們所保有的您的個人資料。"</p>

                        <h3 class="mt-2 font-bold text-ink">"五、未成年學生的資料"</h3>
                        <p>"若您代表未成年子女填寫本網站表單，即表示您已取得該未成年人之同意，並同意本政策之內容。"</p>

                        <h3 class="mt-2 font-bold text-ink">"六、Cookie 使用說明"</h3>
                        <p>
                            "本網站可能使用基本的技術性 Cookie 以維持網站正常運作，不會用於追蹤您在其他網站上的行為，也不會用於廣告投放。"
                        </p>

                        <h3 class="mt-2 font-bold text-ink">"七、政策修訂"</h3>
                        <p>"本隱私權政策如有修訂，將於本頁面公告，恕不另行個別通知。"</p>

                        <h3 class="mt-2 font-bold text-ink">"八、聯絡我們"</h3>
                        <p>"若您對本隱私權政策有任何疑問，歡迎透過 LINE 官方帳號（@891ivojl）與我們聯繫。"</p>

                        // 最後更新日期依 spec.md 4.6 節原文標示為待正式上線日確認後補上
                        <p class="mt-4 text-slate-gray">"最後更新日期：待填寫（正式上線日期）"</p>
                    </div>
                </article>

                <article id="terms-of-service" class="mt-16 scroll-mt-24">
                    <h2 class="text-2xl font-bold text-brand-blue">"服務條款"</h2>
                    <div class="mt-4 flex flex-col gap-4 text-sm leading-[1.7] text-ink">
                        <p>
                            "歡迎使用知暖學習工作室（以下稱「本工作室」）官方網站。當您使用本網站，即表示您同意以下條款："
                        </p>

                        <h3 class="mt-2 font-bold text-ink">"一、服務內容"</h3>
                        <p>
                            "本網站提供本工作室之課程資訊介紹、試聽預約、諮詢聯絡等功能，實際課程安排、上課方式與收費內容，將由本工作室另行與您以口頭或書面方式確認，網站內容僅供參考，不構成正式報價或合約。"
                        </p>

                        <h3 class="mt-2 font-bold text-ink">"二、資訊正確性"</h3>
                        <p>
                            "本工作室致力於確保網站內容之正確性，但不保證所有資訊完全無誤或即時更新，如有課程異動，以本工作室最新公告或與您確認之內容為準。"
                        </p>

                        <h3 class="mt-2 font-bold text-ink">"三、智慧財產權"</h3>
                        <p>
                            "本網站之文字、圖片、課程設計等內容，除另有標示外，均為本工作室所有，未經同意不得任意重製、轉載或使用於商業用途。"
                        </p>

                        <h3 class="mt-2 font-bold text-ink">"四、使用者行為"</h3>
                        <p>"使用者於本網站留言、送出表單時，應提供真實正確之資訊，不得利用本網站進行任何違法或侵害他人權益之行為。"</p>

                        <h3 class="mt-2 font-bold text-ink">"五、條款修訂"</h3>
                        <p>"本工作室保留隨時修訂本服務條款之權利，修訂後將於本頁面公告。"</p>

                        <h3 class="mt-2 font-bold text-ink">"六、聯絡方式"</h3>
                        <p>"如對本服務條款有任何疑問，歡迎透過 LINE 官方帳號（@891ivojl）與我們聯繫。"</p>

                        <p class="mt-4 text-slate-gray">"最後更新日期：待填寫（正式上線日期）"</p>
                    </div>
                </article>
            </Reveal>
        </section>
    }
}
