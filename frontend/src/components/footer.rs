use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="footer-container">
                <p>{"RustShop - Built with Rust, Axum & Yew"}</p>
                <p>{"Copyright 2024. All rights reserved."}</p>
            </div>
        </footer>
    }
}
