use yew::prelude::*;
use yew_router::prelude::*;
use shared::{Order, OrderListResponse};
use crate::api;
use crate::components::Loading;
use crate::state::use_auth;
use crate::routes::Route;

#[function_component(OrderListPage)]
pub fn order_list_page() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();
    let orders = use_state(|| Vec::<Order>::new());
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    // Redirect if not logged in
    if auth.user.is_none() {
        navigator.push(&Route::Login);
        return html! {};
    }

    {
        let orders = orders.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::get::<OrderListResponse>("/orders").await {
                    Ok(response) => {
                        orders.set(response.orders);
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
        return html! { <Loading message="Loading orders..." /> };
    }

    if let Some(err) = (*error).clone() {
        return html! {
            <div class="error-message">
                <p>{"Error: "}{err}</p>
            </div>
        };
    }

    html! {
        <div class="order-list-page">
            <h1>{"My Orders"}</h1>

            if orders.is_empty() {
                <div class="no-orders">
                    <p>{"You haven't placed any orders yet"}</p>
                    <Link<Route> to={Route::Products} classes="btn btn-primary">
                        {"Start Shopping"}
                    </Link<Route>>
                </div>
            } else {
                <div class="orders-list">
                    {for orders.iter().map(|order| {
                        let status_class = match order.status {
                            shared::OrderStatus::Pending => "status-pending",
                            shared::OrderStatus::Paid => "status-paid",
                            shared::OrderStatus::Shipped => "status-shipped",
                            shared::OrderStatus::Delivered => "status-delivered",
                            shared::OrderStatus::Cancelled => "status-cancelled",
                        };

                        html! {
                            <div class="order-card">
                                <div class="order-header">
                                    <span class="order-id">{format!("Order #{}", &order.id.to_string()[..8])}</span>
                                    <span class={classes!("order-status", status_class)}>
                                        {format!("{:?}", order.status)}
                                    </span>
                                </div>
                                <div class="order-info">
                                    <p class="order-date">{order.created_at.format("%Y-%m-%d %H:%M").to_string()}</p>
                                    <p class="order-total">{format!("${:.2}", order.total)}</p>
                                </div>
                                <Link<Route> to={Route::OrderDetail { id: order.id.to_string() }} classes="btn btn-secondary">
                                    {"View Details"}
                                </Link<Route>>
                            </div>
                        }
                    })}
                </div>
            }
        </div>
    }
}
