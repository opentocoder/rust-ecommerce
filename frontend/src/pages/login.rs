use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use shared::{LoginRequest, AuthResponse};
use crate::api;
use crate::state::{use_auth, AuthAction};
use crate::routes::Route;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();

    let email = use_state(String::new);
    let password = use_state(String::new);
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(|| false);

    // Redirect if already logged in
    if auth.user.is_some() {
        navigator.push(&Route::Home);
        return html! {};
    }

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let loading = loading.clone();
        let auth = auth.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let error = error.clone();
            let loading = loading.clone();
            let auth = auth.clone();
            let navigator = navigator.clone();

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let req = LoginRequest {
                    email: email_val,
                    password: password_val,
                };

                match api::post::<AuthResponse, _>("/auth/login", &req).await {
                    Ok(response) => {
                        auth.dispatch(AuthAction::Login {
                            user: response.user,
                            token: response.token,
                        });
                        navigator.push(&Route::Home);
                    }
                    Err(e) => {
                        error.set(Some(e.message));
                    }
                }
                loading.set(false);
            });
        })
    };

    html! {
        <div class="auth-page">
            <div class="auth-form-container">
                <h1>{"Login"}</h1>

                if let Some(err) = (*error).clone() {
                    <div class="error-message">{err}</div>
                }

                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="email">{"Email"}</label>
                        <input
                            type="email"
                            id="email"
                            value={(*email).clone()}
                            oninput={on_email_change}
                            required=true
                        />
                    </div>

                    <div class="form-group">
                        <label for="password">{"Password"}</label>
                        <input
                            type="password"
                            id="password"
                            value={(*password).clone()}
                            oninput={on_password_change}
                            required=true
                        />
                    </div>

                    <button type="submit" class="btn btn-primary btn-full" disabled={*loading}>
                        if *loading {
                            {"Logging in..."}
                        } else {
                            {"Login"}
                        }
                    </button>
                </form>

                <p class="auth-link">
                    {"Don't have an account? "}
                    <Link<Route> to={Route::Register}>{"Register"}</Link<Route>>
                </p>
            </div>
        </div>
    }
}
