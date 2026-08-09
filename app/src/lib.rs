use leptos::hydration::{AutoReload, HydrationScripts};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub mod pages;
pub mod components;

use components::{Footer, Navbar};
use pages::about::AboutPage;
use pages::courses::CoursesPage;
use pages::home::HomePage;
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
            <Navbar/>
            <main>
                <Routes fallback=|| view! { <p class="px-6 py-24 text-center">"這個頁面好像不小心迷路了"</p> }>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("about") view=AboutPage/>
                    <Route path=StaticSegment("courses") view=CoursesPage/>
                    <Route path=StaticSegment("privacy") view=PrivacyPage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
