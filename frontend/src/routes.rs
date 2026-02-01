use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/products")]
    Products,
    #[at("/products/:id")]
    ProductDetail { id: String },
    #[at("/cart")]
    Cart,
    #[at("/orders")]
    Orders,
    #[at("/orders/:id")]
    OrderDetail { id: String },
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}
