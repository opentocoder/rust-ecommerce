use yew::prelude::*;
use yew_router::prelude::*;
use shared::{Cart, CartResponse, UpdateCartItemRequest, CreateOrderRequest, OrderResponse, MessageResponse};
use crate::api;
use crate::components::Loading;
use crate::state::use_auth;
use crate::routes::Route;

#[function_component(CartPage)]
pub fn cart_page() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();
    let cart = use_state(|| Option::<Cart>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);
    let processing = use_state(|| false);

    // Redirect if not logged in
    if auth.user.is_none() {
        navigator.push(&Route::Login);
        return html! {};
    }

    {
        let cart = cart.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<CartResponse>("/cart").await {
                    Ok(response) => {
                        cart.set(Some(response.cart));
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

    let on_update_quantity = {
        let cart = cart.clone();
        Callback::from(move |(product_id, quantity): (String, i32)| {
            let cart = cart.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let req = UpdateCartItemRequest { quantity };
                if let Ok(response) = api::put::<CartResponse, _>(&format!("/cart/{}", product_id), &req).await {
                    cart.set(Some(response.cart));
                }
            });
        })
    };

    let on_remove_item = {
        let cart = cart.clone();
        Callback::from(move |product_id: String| {
            let cart = cart.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(_) = api::delete::<MessageResponse>(&format!("/cart/{}", product_id)).await {
                    // Refresh cart
                    if let Ok(response) = api::get::<CartResponse>("/cart").await {
                        cart.set(Some(response.cart));
                    }
                }
            });
        })
    };

    let on_checkout = {
        let processing = processing.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let processing = processing.clone();
            let navigator = navigator.clone();

            processing.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let req = CreateOrderRequest { shipping_address: None };
                match api::post::<OrderResponse, _>("/orders", &req).await {
                    Ok(response) => {
                        navigator.push(&Route::OrderDetail { id: response.order.order.id.to_string() });
                    }
                    Err(_) => {
                        processing.set(false);
                    }
                }
            });
        })
    };

    if *loading {
        return html! { <Loading message="Loading cart..." /> };
    }

    if let Some(err) = (*error).clone() {
        return html! {
            <div class="error-message">
                <p>{"Error: "}{err}</p>
            </div>
        };
    }

    let cart_data = match (*cart).clone() {
        Some(c) => c,
        None => return html! { <div>{"Cart not found"}</div> },
    };

    html! {
        <div class="cart-page">
            <h1>{"Shopping Cart"}</h1>

            if cart_data.is_empty() {
                <div class="empty-cart">
                    <p>{"Your cart is empty"}</p>
                    <Link<Route> to={Route::Products} classes="btn btn-primary">
                        {"Continue Shopping"}
                    </Link<Route>>
                </div>
            } else {
                <div class="cart-items">
                    {for cart_data.items.clone().into_iter().map(|item| {
                        let product_id = item.product_id.to_string();
                        let on_update = on_update_quantity.clone();
                        let on_remove = on_remove_item.clone();
                        let pid_update = product_id.clone();
                        let pid_remove = product_id.clone();
                        let quantity = item.quantity;

                        html! {
                            <div class="cart-item">
                                <div class="item-info">
                                    <h3>{item.product_name.clone()}</h3>
                                    <p class="price">{format!("${:.2}", item.product_price)}</p>
                                </div>
                                <div class="item-quantity">
                                    <button
                                        onclick={let on_update = on_update.clone(); let pid = pid_update.clone();
                                            Callback::from(move |_| on_update.emit((pid.clone(), quantity - 1)))}
                                        disabled={quantity <= 1}
                                    >
                                        {"-"}
                                    </button>
                                    <span>{quantity}</span>
                                    <button
                                        onclick={let on_update = on_update.clone(); let pid = pid_update.clone();
                                            Callback::from(move |_| on_update.emit((pid.clone(), quantity + 1)))}
                                    >
                                        {"+"}
                                    </button>
                                </div>
                                <div class="item-subtotal">
                                    {format!("${:.2}", item.subtotal)}
                                </div>
                                <button
                                    class="btn btn-danger"
                                    onclick={Callback::from(move |_| on_remove.emit(pid_remove.clone()))}
                                >
                                    {"Remove"}
                                </button>
                            </div>
                        }
                    })}
                </div>

                <div class="cart-summary">
                    <div class="total">
                        <span>{"Total:"}</span>
                        <span class="total-amount">{format!("${:.2}", cart_data.total)}</span>
                    </div>
                    <button
                        class="btn btn-primary btn-large"
                        onclick={on_checkout}
                        disabled={*processing}
                    >
                        if *processing {
                            {"Processing..."}
                        } else {
                            {"Checkout"}
                        }
                    </button>
                </div>
            }
        </div>
    }
}
