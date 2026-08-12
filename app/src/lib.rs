use leptos::hydration::{AutoReload, HydrationScripts};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::hooks::use_location;
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment,
};

pub mod admin_auth;
pub mod pages;
pub mod components;

use components::{Footer, Navbar};
use pages::about::AboutPage;
use pages::admin_announcement_form::AdminAnnouncementFormPage;
use pages::admin_announcements::AdminAnnouncementsListPage;
use pages::admin_contacts::AdminContactsPage;
use pages::admin_dashboard::AdminDashboardPage;
use pages::admin_login::AdminLoginPage;
use pages::contact::ContactPage;
use pages::courses::CoursesPage;
use pages::faq::FaqPage;
use pages::home::HomePage;
use pages::news::{NewsDetailPage, NewsListPage};
use pages::not_found::NotFoundPage;
use pages::privacy::PrivacyPage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="zh-Hant">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/warmlearning-studio.css"/>
        <Title text="知暖學習工作室"/>

        <Router>
            <AppShell/>
        </Router>
    }
}

/// 對外頁面顯示公用 Navbar／Footer，後台頁面（/admin/*）自己有一套導覽列，不顯示
#[component]
fn AppShell() -> impl IntoView {
    let location = use_location();
    let is_admin = move || location.pathname.get().starts_with("/admin");

    view! {
        <Show when=move || !is_admin()>
            <Navbar/>
        </Show>
        <main>
            <Routes fallback=NotFoundPage>
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=StaticSegment("about") view=AboutPage/>
                <Route path=StaticSegment("courses") view=CoursesPage/>
                <Route path=StaticSegment("contact") view=ContactPage/>
                <Route path=StaticSegment("news") view=NewsListPage/>
                <Route path=(StaticSegment("news"), ParamSegment("slug")) view=NewsDetailPage/>
                <Route path=StaticSegment("faq") view=FaqPage/>
                <Route path=StaticSegment("privacy") view=PrivacyPage/>

                <Route path=(StaticSegment("admin"), StaticSegment("login")) view=AdminLoginPage/>
                <Route path=StaticSegment("admin") view=AdminDashboardPage/>
                <Route path=(StaticSegment("admin"), StaticSegment("announcements")) view=AdminAnnouncementsListPage/>
                <Route
                    path=(StaticSegment("admin"), StaticSegment("announcements"), StaticSegment("new"))
                    view=AdminAnnouncementFormPage
                />
                <Route
                    path=(StaticSegment("admin"), StaticSegment("announcements"), ParamSegment("id"), StaticSegment("edit"))
                    view=AdminAnnouncementFormPage
                />
                <Route path=(StaticSegment("admin"), StaticSegment("contacts")) view=AdminContactsPage/>
            </Routes>
        </main>
        <Show when=move || !is_admin()>
            <Footer/>
        </Show>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
