mod api;
mod components;
mod pages;
mod state;
mod routes;

use yew::prelude::*;
use yew_router::prelude::*;

use routes::Route;
use pages::*;
use state::AuthProvider;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <AuthProvider>
                <div class="app">
                    <components::Header />
                    <main class="main-content">
                        <Switch<Route> render={switch} />
                    </main>
                    <components::Footer />
                </div>
            </AuthProvider>
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Products => html! { <ProductListPage /> },
        Route::ProductDetail { id } => html! { <ProductDetailPage {id} /> },
        Route::Cart => html! { <CartPage /> },
        Route::Orders => html! { <OrderListPage /> },
        Route::OrderDetail { id } => html! { <OrderDetailPage {id} /> },
        Route::Login => html! { <LoginPage /> },
        Route::Register => html! { <RegisterPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

// WASM entry point
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    // Mount the app
    yew::Renderer::<App>::new().render();
}
