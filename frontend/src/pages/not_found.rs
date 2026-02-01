use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="not-found-page">
            <h1>{"404"}</h1>
            <p>{"Page not found"}</p>
            <Link<Route> to={Route::Home} classes="btn btn-primary">
                {"Go Home"}
            </Link<Route>>
        </div>
    }
}
