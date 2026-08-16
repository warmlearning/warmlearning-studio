use leptos::prelude::*;

use super::icons::ChatIcon;

/// 按鈕樣式對照 spec.md 5.6 節
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Line,
}

#[component]
pub fn CtaButton(
    #[prop(into)] href: String,
    #[prop(into)] label: String,
    variant: ButtonVariant,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Primary => "bg-brand-blue text-white hover:bg-[#14294F]",
        ButtonVariant::Secondary => {
            "border border-brand-blue bg-transparent text-brand-blue hover:bg-mist-blue"
        }
        ButtonVariant::Line => "bg-line-green text-white hover:brightness-95",
    };
    let is_line = variant == ButtonVariant::Line;

    view! {
        <a
            href=href
            class=format!(
                "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-full px-8 py-3 text-lg font-medium transition-all duration-200 {variant_class} {class}",
            )
        >
            <Show when=move || is_line>
                <ChatIcon/>
            </Show>
            {label}
        </a>
    }
}
