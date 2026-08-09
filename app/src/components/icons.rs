use leptos::prelude::*;

/// 共用線性風格 Icon（單色線條，對照 spec.md 5.1 節「圖示走線性風格」）
#[component]
pub fn BookIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" class="h-8 w-8">
            <path d="M4 5.5C4 4.67 4.67 4 5.5 4H11v16H5.5A1.5 1.5 0 0 1 4 18.5v-13Z"/>
            <path d="M20 5.5c0-.83-.67-1.5-1.5-1.5H13v16h5.5a1.5 1.5 0 0 0 1.5-1.5v-13Z"/>
        </svg>
    }
}

#[component]
pub fn HeartIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" class="h-7 w-7">
            <path d="M12 20s-7-4.35-9.5-9C.83 7.5 2.6 4 6 4c2 0 3.5 1.2 4 2.5C10.5 5.2 12 4 14 4c3.4 0 5.17 3.5 3.5 7-2.5 4.65-9.5 9-9.5 9Z"/>
        </svg>
    }
}

#[component]
pub fn PersonIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" class="h-7 w-7">
            <circle cx="12" cy="8" r="3.5"/>
            <path d="M4.5 20c1.2-3.5 4-5.5 7.5-5.5s6.3 2 7.5 5.5"/>
        </svg>
    }
}

#[component]
pub fn AiIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" class="h-7 w-7">
            <rect x="5" y="7" width="14" height="11" rx="2.5"/>
            <path d="M12 7V3.5M9 3.5h6"/>
            <circle cx="9.5" cy="12.5" r="1"/>
            <circle cx="14.5" cy="12.5" r="1"/>
            <path d="M9 16h6"/>
        </svg>
    }
}

#[component]
pub fn TrendUpIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" class="h-7 w-7">
            <path d="M3 17l6-6 4 4 8-8"/>
            <path d="M15 7h6v6"/>
        </svg>
    }
}

#[component]
pub fn ChatIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" class="h-5 w-5">
            <path d="M4 12c0-4.4 3.8-8 8.5-8S21 7.6 21 12s-3.8 8-8.5 8c-1 0-1.9-.15-2.8-.45L6 21l1.1-3.6C5.2 16 4 14.15 4 12Z"/>
        </svg>
    }
}

#[component]
pub fn CalendarIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" class="h-7 w-7">
            <rect x="4" y="5.5" width="16" height="15" rx="2"/>
            <path d="M4 10h16M8 3.5v3M16 3.5v3"/>
        </svg>
    }
}

#[component]
pub fn QuoteIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="h-10 w-10">
            <path d="M7.5 6C4.5 6 2 8.5 2 11.5S4.5 17 7.5 17c.3 0 .5-.2.5-.5s-.2-.5-.5-.5C5.6 16 4 14.4 4 12.5S5.6 9 7.5 9 11 10.6 11 12.5c0 3.6-2.3 6.4-5.3 7.9-.3.1-.4.5-.2.7.2.3.5.4.8.2C10 19 13 15.6 13 11.5 13 8.5 10.5 6 7.5 6Zm10 0c-3 0-5.5 2.5-5.5 5.5s2.5 5.5 5.5 5.5c.3 0 .5-.2.5-.5s-.2-.5-.5-.5c-1.9 0-3.5-1.6-3.5-3.5S15.6 9 17.5 9s3.5 1.6 3.5 3.5c0 3.6-2.3 6.4-5.3 7.9-.3.1-.4.5-.2.7.2.3.5.4.8.2 3.7-1.9 6.7-5.3 6.7-9.4 0-3-2.5-5.5-5.5-5.5Z"/>
        </svg>
    }
}

#[component]
pub fn HamburgerIcon(#[prop(into)] open: Signal<bool>) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" class="h-6 w-6">
            <Show
                when=move || open.get()
                fallback=|| view! {
                    <path d="M4 7h16M4 12h16M4 17h16"/>
                }
            >
                <path d="M6 6l12 12M18 6L6 18"/>
            </Show>
        </svg>
    }
}

#[component]
pub fn InstagramIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" class="h-5 w-5">
            <rect x="3.5" y="3.5" width="17" height="17" rx="5"/>
            <circle cx="12" cy="12" r="4"/>
            <circle cx="17" cy="7" r="0.8" fill="currentColor" stroke="none"/>
        </svg>
    }
}

#[component]
pub fn FacebookIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" class="h-5 w-5">
            <path d="M14.5 21v-7h2.4l.4-3H14.5v-1.9c0-.87.24-1.46 1.5-1.46H17.5V4.9c-.26-.03-1.16-.11-2.2-.11-2.18 0-3.67 1.33-3.67 3.77V11H9.2v3h2.43v7h2.87Z"/>
        </svg>
    }
}
