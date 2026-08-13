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

/// 進場動畫的隱藏狀態 class：透明＋向下偏移 32px＋縮放至 96%（spec.md 5.8 節第 1 項，v11 加大幅度）
const REVEAL_HIDDEN_CLASSES: [&str; 3] = ["opacity-0", "translate-y-8", "scale-[0.96]"];

/// 幫單一元素套用滾動進場動畫：一開始隱藏＋向下偏移＋縮小，捲動進入可視範圍後
/// 淡入回原位＋原尺寸；離開可視範圍後重新回到隱藏狀態，再次進入時重新播放
/// （v11 起改為可重複播放，對照 spec.md 5.8 節第 1 項）。
///
/// 效能考量：持續用同一個 IntersectionObserver 訂閱該元素，不呼叫
/// unobserve／disconnect——這與 v10 版本的作法相同（v10 版本雖然邏輯上「播放一次
/// 後不再處理」，但實際上也從未主動停止觀察，只是收到後續 callback 時不做事）。
/// 所以 v11 這個改動並沒有增加 observer 或事件監聽器的數量，只是讓 callback 對
/// 「離開可視範圍」的情況也做事（重新加回隱藏 class）。每個 Reveal 元件對應一個
/// 獨立的 observer 實例（不共用／不集中管理），這點在頁面上 Reveal 元件數量很多時
/// （例如首頁一次有十幾個）會建立相對應數量的 observer，但瀏覽器原生
/// IntersectionObserver 本身設計上就是給大量元素訂閱使用的，效能開銷遠低於捲動
/// 事件監聽器，實務上不會造成明顯效能問題；真正需要留意的情境是「同一頁面有數百個
/// 以上」的規模，遠超過本站任何一頁實際會用到的數量。
pub fn reveal_on_scroll(el: web_sys::Element, delay_ms: u32) {
    if prefers_reduced_motion() {
        return;
    }

    for class in REVEAL_HIDDEN_CLASSES {
        let _ = el.class_list().add_1(class);
    }

    if delay_ms > 0 {
        if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
            let _ = html_el.style().set_property("transition-delay", &format!("{delay_ms}ms"));
        }
    }

    let closure = Closure::wrap(Box::new(move |entries: js_sys::Array, _observer: web_sys::IntersectionObserver| {
        for entry in entries.iter() {
            let entry: web_sys::IntersectionObserverEntry = entry.unchecked_into();
            let target = entry.target();
            if entry.is_intersecting() {
                for class in REVEAL_HIDDEN_CLASSES {
                    let _ = target.class_list().remove_1(class);
                }
            } else {
                for class in REVEAL_HIDDEN_CLASSES {
                    let _ = target.class_list().add_1(class);
                }
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

/// 掛載時立即淡入＋輕微向上位移，用於非捲動觸發的內容（如骨架屏載入完成後的
/// 真實內容、聯絡表單送出訊息），對照 spec.md 5.8 節第 2 項。
/// 用兩次 requestAnimationFrame 確保瀏覽器先以隱藏狀態算過一次樣式，
/// 之後再移除隱藏 class 才會觸發 CSS transition，而不是直接跳過動畫。
pub fn fade_in_on_mount(el: web_sys::Element) {
    if prefers_reduced_motion() {
        return;
    }

    let _ = el.class_list().add_2("opacity-0", "translate-y-2");

    let el_for_inner = el.clone();
    let inner_closure = Closure::once_into_js(move || {
        let _ = el_for_inner.class_list().remove_2("opacity-0", "translate-y-2");
    });

    let outer_closure = Closure::once(Box::new(move || {
        if let Some(window) = web_sys::window() {
            let _ = window.request_animation_frame(inner_closure.unchecked_ref());
        }
    }) as Box<dyn FnOnce()>);

    if let Some(window) = web_sys::window() {
        let _ = window.request_animation_frame(outer_closure.as_ref().unchecked_ref());
    }
    outer_closure.forget();
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
