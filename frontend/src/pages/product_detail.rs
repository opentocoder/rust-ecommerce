use yew::prelude::*;
use shared::{Product, ProductResponse, AddToCartRequest, CartResponse};
use crate::api;
use crate::components::Loading;
use crate::state::use_auth;

#[derive(Properties, PartialEq)]
pub struct ProductDetailProps {
    pub id: String,
}

#[function_component(ProductDetailPage)]
pub fn product_detail_page(props: &ProductDetailProps) -> Html {
    let auth = use_auth();
    let product = use_state(|| Option::<Product>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);
    let quantity = use_state(|| 1i32);
    let adding = use_state(|| false);
    let message = use_state(|| Option::<String>::None);

    let id = props.id.clone();

    {
        let product = product.clone();
        let loading = loading.clone();
        let error = error.clone();
        let id = id.clone();

        use_effect_with(id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<ProductResponse>(&format!("/products/{}", id)).await {
                    Ok(response) => {
                        product.set(Some(response.product));
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

    let on_quantity_change = {
        let quantity = quantity.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<i32>() {
                quantity.set(val.max(1));
            }
        })
    };

    let on_add_to_cart = {
        let product = product.clone();
        let quantity = quantity.clone();
        let adding = adding.clone();
        let message = message.clone();

        Callback::from(move |_| {
            if let Some(p) = (*product).clone() {
                let product_id = p.id;
                let qty = *quantity;
                let adding = adding.clone();
                let message = message.clone();

                adding.set(true);
                wasm_bindgen_futures::spawn_local(async move {
                    let req = AddToCartRequest {
                        product_id,
                        quantity: qty,
                    };
                    match api::post::<CartResponse, _>("/cart", &req).await {
                        Ok(_) => {
                            message.set(Some("Added to cart!".to_string()));
                        }
                        Err(e) => {
                            message.set(Some(format!("Error: {}", e.message)));
                        }
                    }
                    adding.set(false);
                });
            }
        })
    };

    if *loading {
        return html! { <Loading message="Loading product..." /> };
    }

    if let Some(err) = (*error).clone() {
        return html! {
            <div class="error-message">
                <p>{"Error: "}{err}</p>
            </div>
        };
    }

    let product = match (*product).clone() {
        Some(p) => p,
        None => return html! { <div>{"Product not found"}</div> },
    };

    html! {
        <div class="product-detail-page">
            <div class="product-detail">
                <div class="product-image-large">
                    if let Some(url) = &product.image_url {
                        <img src={url.clone()} alt={product.name.clone()} />
                    } else {
                        <div class="placeholder-image">{"No Image"}</div>
                    }
                </div>

                <div class="product-info-detail">
                    <h1>{&product.name}</h1>
                    <p class="category">{"Category: "}{&product.category}</p>
                    <p class="price">{format!("${:.2}", product.price)}</p>
                    <p class="description">{&product.description}</p>

                    if product.stock > 0 {
                        <p class="stock in-stock">{format!("{} in stock", product.stock)}</p>

                        if auth.user.is_some() {
                            <div class="add-to-cart">
                                <input
                                    type="number"
                                    min="1"
                                    max={product.stock.to_string()}
                                    value={quantity.to_string()}
                                    onchange={on_quantity_change}
                                />
                                <button
                                    class="btn btn-primary"
                                    onclick={on_add_to_cart}
                                    disabled={*adding}
                                >
                                    if *adding {
                                        {"Adding..."}
                                    } else {
                                        {"Add to Cart"}
                                    }
                                </button>
                            </div>
                        } else {
                            <p class="login-prompt">{"Please login to add items to cart"}</p>
                        }
                    } else {
                        <p class="stock out-of-stock">{"Out of Stock"}</p>
                    }

                    if let Some(msg) = (*message).clone() {
                        <p class="message">{msg}</p>
                    }
                </div>
            </div>
        </div>
    }
}
