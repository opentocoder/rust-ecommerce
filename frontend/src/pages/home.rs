use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="home-page">
            <section class="hero">
                <h1>{"Welcome to RustShop"}</h1>
                <p>{"Your one-stop shop for quality products"}</p>
                <Link<Route> to={Route::Products} classes="btn btn-primary btn-large">
                    {"Shop Now"}
                </Link<Route>>
            </section>

            <section class="features">
                <div class="feature">
                    <h3>{"Fast & Secure"}</h3>
                    <p>{"Built with Rust for maximum performance and safety"}</p>
                </div>
                <div class="feature">
                    <h3>{"Wide Selection"}</h3>
                    <p>{"Browse through our extensive catalog of products"}</p>
                </div>
                <div class="feature">
                    <h3>{"Easy Checkout"}</h3>
                    <p>{"Simple and streamlined checkout process"}</p>
                </div>
            </section>
        </div>
    }
}
