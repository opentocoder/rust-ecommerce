use yew::prelude::*;
use shared::{Product, ProductListResponse};
use crate::api;
use crate::components::{ProductCard, Loading};

#[function_component(ProductListPage)]
pub fn product_list_page() -> Html {
    let products = use_state(|| Vec::<Product>::new());
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    {
        let products = products.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<ProductListResponse>("/products").await {
                    Ok(response) => {
                        products.set(response.products);
                    }
                    Err(e) => {
                        error.set(Some(e.message));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    if *loading {
        return html! { <Loading message="Loading products..." /> };
    }

    if let Some(err) = (*error).clone() {
        return html! {
            <div class="error-message">
                <p>{"Error: "}{err}</p>
            </div>
        };
    }

    html! {
        <div class="product-list-page">
            <h1>{"Products"}</h1>

            <div class="product-grid">
                {for products.iter().map(|product| {
                    html! { <ProductCard product={product.clone()} /> }
                })}
            </div>

            if products.is_empty() {
                <p class="no-products">{"No products available"}</p>
            }
        </div>
    }
}
