use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;
use crate::state::{use_auth, AuthAction};

#[function_component(Header)]
pub fn header() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();

    let on_logout = {
        let auth = auth.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            auth.dispatch(AuthAction::Logout);
            navigator.push(&Route::Home);
        })
    };

    html! {
        <header class="header">
            <div class="header-container">
                <Link<Route> to={Route::Home} classes="logo">
                    {"RustShop"}
                </Link<Route>>

                <nav class="nav">
                    <Link<Route> to={Route::Products} classes="nav-link">
                        {"Products"}
                    </Link<Route>>

                    if auth.user.is_some() {
                        <>
                            <Link<Route> to={Route::Cart} classes="nav-link">
                                {"Cart"}
                            </Link<Route>>
                            <Link<Route> to={Route::Orders} classes="nav-link">
                                {"Orders"}
                            </Link<Route>>
                            <button class="btn btn-secondary" onclick={on_logout}>
                                {"Logout"}
                            </button>
                        </>
                    } else {
                        <>
                            <Link<Route> to={Route::Login} classes="nav-link">
                                {"Login"}
                            </Link<Route>>
                            <Link<Route> to={Route::Register} classes="btn btn-primary">
                                {"Register"}
                            </Link<Route>>
                        </>
                    }
                </nav>
            </div>
        </header>
    }
}
