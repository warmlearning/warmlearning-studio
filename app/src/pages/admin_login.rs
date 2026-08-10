use leptos::form::ActionForm;
use leptos::prelude::*;
use leptos_meta::Title;

use crate::admin_auth::friendly_error_message;

#[cfg(feature = "ssr")]
const LOCK_MINUTES: i64 = 15;
#[cfg(feature = "ssr")]
const MAX_FAILED_ATTEMPTS: i64 = 5;

/// 管理者登入，對照 spec.md 6.1 節：統一錯誤訊息、連續失敗 5 次鎖定 15 分鐘
#[server(AdminLogin, "/api")]
pub async fn admin_login(username: String, password: String) -> Result<(), ServerFnError> {
    use argon2::password_hash::PasswordHash;
    use argon2::{Argon2, PasswordVerifier};

    const GENERIC_ERROR: &str = "帳號或密碼錯誤";

    let username = username.trim();
    if username.is_empty() || password.is_empty() {
        return Err(ServerFnError::new(GENERIC_ERROR));
    }

    let pool = expect_context::<sqlx::SqlitePool>();

    let row: Option<(i64, String, i64, i64)> = sqlx::query_as(
        "SELECT id, password_hash, failed_login_attempts,
                (locked_until IS NOT NULL AND locked_until > datetime('now')) AS is_locked
         FROM admin_users WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let Some((admin_id, password_hash, failed_attempts, is_locked)) = row else {
        return Err(ServerFnError::new(GENERIC_ERROR));
    };

    if is_locked != 0 {
        return Err(ServerFnError::new(format!(
            "帳號已被鎖定，請於 {LOCK_MINUTES} 分鐘後再試"
        )));
    }

    let parsed_hash = PasswordHash::new(&password_hash)
        .map_err(|e| ServerFnError::new(format!("密碼雜湊格式錯誤: {e}")))?;
    let valid = Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok();

    if !valid {
        let new_failed = failed_attempts + 1;
        if new_failed >= MAX_FAILED_ATTEMPTS {
            sqlx::query(
                "UPDATE admin_users SET failed_login_attempts = 0,
                 locked_until = datetime('now', '+15 minutes') WHERE id = ?",
            )
            .bind(admin_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        } else {
            sqlx::query("UPDATE admin_users SET failed_login_attempts = ? WHERE id = ?")
                .bind(new_failed)
                .bind(admin_id)
                .execute(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }
        return Err(ServerFnError::new(GENERIC_ERROR));
    }

    sqlx::query("UPDATE admin_users SET failed_login_attempts = 0, locked_until = NULL WHERE id = ?")
        .bind(admin_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let session: tower_sessions::Session = leptos_axum::extract().await?;
    // 登入成功後重新產生 session id，避免 session fixation
    session.cycle_id().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    session
        .insert(crate::admin_auth::ADMIN_SESSION_KEY, admin_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect("/admin");
    Ok(())
}

/// 管理者登出
#[server(AdminLogout, "/api")]
pub async fn admin_logout() -> Result<(), ServerFnError> {
    let session: tower_sessions::Session = leptos_axum::extract().await?;
    session.delete().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    leptos_axum::redirect("/admin/login");
    Ok(())
}

#[component]
pub fn AdminLoginPage() -> impl IntoView {
    let login_action = ServerAction::<AdminLogin>::new();
    let response = login_action.value();

    view! {
        <Title text="後台登入｜知暖學習工作室"/>

        <section class="bg-mist-blue">
            <div class="mx-auto flex min-h-[70vh] max-w-md flex-col justify-center px-6 py-16">
                <div class="rounded-2xl bg-white p-8 shadow-md">
                    <h1 class="text-center text-2xl font-bold text-brand-blue">"後台登入"</h1>

                    <ActionForm action=login_action attr:class="mt-8 flex flex-col gap-5">
                        <div class="flex flex-col gap-1.5">
                            <label for="login-username" class="text-sm font-medium text-ink">
                                "帳號"
                            </label>
                            <input
                                id="login-username"
                                type="text"
                                name="username"
                                required
                                autocomplete="username"
                                class="rounded-xl border border-border-gray px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                            />
                        </div>

                        <div class="flex flex-col gap-1.5">
                            <label for="login-password" class="text-sm font-medium text-ink">
                                "密碼"
                            </label>
                            <input
                                id="login-password"
                                type="password"
                                name="password"
                                required
                                autocomplete="current-password"
                                class="rounded-xl border border-border-gray px-4 py-3 text-sm text-ink outline-none focus:border-brand-blue"
                            />
                        </div>

                        <button
                            type="submit"
                            class="mt-2 rounded-full bg-brand-blue px-8 py-3 font-medium text-white transition-all hover:bg-[#14294F]"
                        >
                            "登入"
                        </button>

                        <Show when=move || matches!(response.get(), Some(Err(_)))>
                            <p class="rounded-xl bg-mist-blue px-4 py-3 text-center text-sm text-error-red">
                                {move || {
                                    response
                                        .get()
                                        .and_then(|r| r.err())
                                        .map(|e| friendly_error_message(&e))
                                        .unwrap_or_default()
                                }}
                            </p>
                        </Show>
                    </ActionForm>
                </div>
            </div>
        </section>
    }
}
