use yew::prelude::*;
use yew_router::prelude::*;
use shared::{OrderWithItems, OrderResponse, MessageResponse, OrderStatus};
use crate::api;
use crate::components::Loading;
use crate::state::use_auth;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct OrderDetailProps {
    pub id: String,
}

#[function_component(OrderDetailPage)]
pub fn order_detail_page(props: &OrderDetailProps) -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();
    let order = use_state(|| Option::<OrderWithItems>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);
    let cancelling = use_state(|| false);

    let id = props.id.clone();

    // Redirect if not logged in
    if auth.user.is_none() {
        navigator.push(&Route::Login);
        return html! {};
    }

    {
        let order = order.clone();
        let loading = loading.clone();
        let error = error.clone();
        let id = id.clone();

        use_effect_with(id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<OrderResponse>(&format!("/orders/{}", id)).await {
                    Ok(response) => {
                        order.set(Some(response.order));
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

    let on_cancel = {
        let order = order.clone();
        let cancelling = cancelling.clone();
        let id = id.clone();

        Callback::from(move |_| {
            let order = order.clone();
            let cancelling = cancelling.clone();
            let id = id.clone();

            cancelling.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                match api::put::<MessageResponse, ()>(&format!("/orders/{}/cancel", id), &()).await {
                    Ok(_) => {
                        // Refresh order
                        if let Ok(response) = api::get::<OrderResponse>(&format!("/orders/{}", id)).await {
                            order.set(Some(response.order));
                        }
                    }
                    Err(_) => {}
                }
                cancelling.set(false);
            });
        })
    };

    if *loading {
        return html! { <Loading message="Loading order..." /> };
    }

    if let Some(err) = (*error).clone() {
        return html! {
            <div class="error-message">
                <p>{"Error: "}{err}</p>
            </div>
        };
    }

    let order_data = match (*order).clone() {
        Some(o) => o,
        None => return html! { <div>{"Order not found"}</div> },
    };

    let status_class = match order_data.order.status {
        OrderStatus::Pending => "status-pending",
        OrderStatus::Paid => "status-paid",
        OrderStatus::Shipped => "status-shipped",
        OrderStatus::Delivered => "status-delivered",
        OrderStatus::Cancelled => "status-cancelled",
    };

    html! {
        <div class="order-detail-page">
            <h1>{format!("Order #{}", &order_data.order.id.to_string()[..8])}</h1>

            <div class="order-status-section">
                <span class={classes!("order-status", "large", status_class)}>
                    {format!("{:?}", order_data.order.status)}
                </span>
                <p class="order-date">
                    {"Placed on "}{order_data.order.created_at.format("%Y-%m-%d %H:%M").to_string()}
                </p>
            </div>

            <div class="order-items">
                <h2>{"Items"}</h2>
                {for order_data.items.iter().map(|item| {
                    html! {
                        <div class="order-item">
                            <div class="item-info">
                                <h3>{&item.product_name}</h3>
                                <p>{format!("Quantity: {}", item.quantity)}</p>
                                <p>{format!("Price: ${:.2}", item.price)}</p>
                            </div>
                            <div class="item-subtotal">
                                {format!("${:.2}", item.subtotal)}
                            </div>
                        </div>
                    }
                })}
            </div>

            <div class="order-total">
                <span>{"Total:"}</span>
                <span class="total-amount">{format!("${:.2}", order_data.order.total)}</span>
            </div>

            <div class="order-actions">
                if order_data.order.can_cancel() {
                    <button
                        class="btn btn-danger"
                        onclick={on_cancel}
                        disabled={*cancelling}
                    >
                        if *cancelling {
                            {"Cancelling..."}
                        } else {
                            {"Cancel Order"}
                        }
                    </button>
                }
                <Link<Route> to={Route::Orders} classes="btn btn-secondary">
                    {"Back to Orders"}
                </Link<Route>>
            </div>
        </div>
    }
}
