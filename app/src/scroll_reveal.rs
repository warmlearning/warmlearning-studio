//! 滾動進場動畫（Scroll Reveal），對照 spec.md 5.8 節第 1 項
//!
//! 純用瀏覽器原生 IntersectionObserver 實作，不依賴任何第三方動畫套件。
//! 尊重 prefers-reduced-motion；SSR 輸出的 HTML 本身不帶任何隱藏用的 class，
//! 一律先完整可見，只有在 JS 成功執行且使用者未停用動態效果時，才會由 JS
//! 主動加上暫時隱藏 class 再透過 observer 淡入，因此未執行 JS 時內容不受影響。

use leptos::wasm_bindgen::closure::Closure;
use leptos::wasm_bindgen::JsCast;
use leptos::wasm_bindgen::JsValue;

fn prefers_reduced_motion() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
        .map(|mql| mql.matches())
        .unwrap_or(false)
}

/// 幫單一元素套用滾動進場動畫：一開始隱藏＋向下偏移，捲動進入可視範圍後淡入回原位，
/// 只播放一次（離開視窗再捲回不會重複觸發，因為淡入後就不會再被隱藏）
pub fn reveal_on_scroll(el: web_sys::Element, delay_ms: u32) {
    if prefers_reduced_motion() {
        return;
    }

    let _ = el.class_list().add_1("opacity-0");
    let _ = el.class_list().add_1("translate-y-4");

    if delay_ms > 0 {
        if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
            let _ = html_el.style().set_property("transition-delay", &format!("{delay_ms}ms"));
        }
    }

    let closure = Closure::wrap(Box::new(move |entries: js_sys::Array, _observer: web_sys::IntersectionObserver| {
        for entry in entries.iter() {
            let entry: web_sys::IntersectionObserverEntry = entry.unchecked_into();
            if entry.is_intersecting() {
                let target = entry.target();
                let _ = target.class_list().remove_1("opacity-0");
                let _ = target.class_list().remove_1("translate-y-4");
            }
        }
    }) as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

    let opts = web_sys::IntersectionObserverInit::new();
    opts.set_threshold(&JsValue::from_f64(0.1));

    if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(closure.as_ref().unchecked_ref(), &opts) {
        observer.observe(&el);
    }
    // observer 存活期間需要這個 closure 持續有效，交給瀏覽器管理，故意 leak 掉
    closure.forget();
}

const FLOATING_BUTTON_SCROLL_THRESHOLD: f64 = 400.0;

fn update_floating_button_visibility(el: &web_sys::Element) {
    let scrolled_past_threshold = web_sys::window()
        .and_then(|w| w.scroll_y().ok())
        .map(|y| y > FLOATING_BUTTON_SCROLL_THRESHOLD)
        .unwrap_or(false);

    let class_list = el.class_list();
    if scrolled_past_threshold {
        let _ = class_list.remove_2("opacity-0", "pointer-events-none");
        let _ = class_list.add_2("opacity-100", "pointer-events-auto");
    } else {
        let _ = class_list.remove_2("opacity-100", "pointer-events-auto");
        let _ = class_list.add_2("opacity-0", "pointer-events-none");
    }
}

/// 固定式浮動 LINE CTA 按鈕的顯示邏輯，對照 spec.md 5.8 節第 3 項：
/// 捲動超過約 400px 後淡入，捲回頂部附近淡出。這顆按鈕本身在 SSR 輸出時就是
/// 隱藏狀態（`opacity-0 pointer-events-none`），因為它的存在意義完全依賴捲動
/// 位置判斷，沒有 JS 就無法運作；但全站其他地方（Footer、頁面內按鈕等）都已
/// 有相同的 LINE 連結，所以未執行 JS 時不影響訪客實際聯繫管道。
pub fn setup_floating_line_button(el: web_sys::Element) {
    let reduced_motion = prefers_reduced_motion();
    if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
        if !reduced_motion {
            let _ = html_el.style().set_property("transition", "opacity 300ms ease-out");
        }
    }

    update_floating_button_visibility(&el);

    let el_for_closure = el.clone();
    let closure = Closure::wrap(Box::new(move || {
        update_floating_button_visibility(&el_for_closure);
    }) as Box<dyn FnMut()>);

    if let Some(window) = web_sys::window() {
        let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
    }
    closure.forget();
}
