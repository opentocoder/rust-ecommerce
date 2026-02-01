use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    #[prop_or("Loading...".to_string())]
    pub message: String,
}

#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    html! {
        <div class="loading">
            <div class="spinner"></div>
            <p>{&props.message}</p>
        </div>
    }
}
