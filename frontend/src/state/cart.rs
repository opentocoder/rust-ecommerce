use yew::prelude::*;
use shared::Cart;

#[derive(Clone, PartialEq, Default)]
pub struct CartState {
    pub cart: Option<Cart>,
    pub loading: bool,
}

pub enum CartAction {
    SetCart(Cart),
    ClearCart,
    SetLoading(bool),
}

impl Reducible for CartState {
    type Action = CartAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            CartAction::SetCart(cart) => Self {
                cart: Some(cart),
                loading: false,
            },
            CartAction::ClearCart => Self {
                cart: None,
                loading: false,
            },
            CartAction::SetLoading(loading) => Self {
                loading,
                ..(*self).clone()
            },
        }
        .into()
    }
}

pub type CartContext = UseReducerHandle<CartState>;

#[derive(Properties, PartialEq)]
pub struct CartProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CartProvider)]
pub fn cart_provider(props: &CartProviderProps) -> Html {
    let cart = use_reducer(CartState::default);

    html! {
        <ContextProvider<CartContext> context={cart}>
            {props.children.clone()}
        </ContextProvider<CartContext>>
    }
}

#[hook]
pub fn use_cart() -> CartContext {
    use_context::<CartContext>().expect("CartContext not found")
}
