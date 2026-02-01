use yew::prelude::*;
use yew_router::prelude::*;
use shared::Product;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct ProductCardProps {
    pub product: Product,
}

#[function_component(ProductCard)]
pub fn product_card(props: &ProductCardProps) -> Html {
    let product = &props.product;

    html! {
        <div class="product-card">
            <Link<Route> to={Route::ProductDetail { id: product.id.to_string() }}>
                <div class="product-image">
                    if let Some(url) = &product.image_url {
                        <img src={url.clone()} alt={product.name.clone()} />
                    } else {
                        <div class="placeholder-image">{"No Image"}</div>
                    }
                </div>
                <div class="product-info">
                    <h3 class="product-name">{&product.name}</h3>
                    <p class="product-category">{&product.category}</p>
                    <p class="product-price">{format!("${:.2}", product.price)}</p>
                    if product.stock > 0 {
                        <span class="in-stock">{"In Stock"}</span>
                    } else {
                        <span class="out-of-stock">{"Out of Stock"}</span>
                    }
                </div>
            </Link<Route>>
        </div>
    }
}
