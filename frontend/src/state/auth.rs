use yew::prelude::*;
use shared::UserProfile;
use crate::api;

#[derive(Clone, PartialEq)]
pub struct AuthState {
    pub user: Option<UserProfile>,
    pub token: Option<String>,
    pub loading: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: None,
            token: api::get_token(),
            loading: false,
        }
    }
}

pub enum AuthAction {
    Login { user: UserProfile, token: String },
    Logout,
    SetLoading(bool),
}

impl Reducible for AuthState {
    type Action = AuthAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            AuthAction::Login { user, token } => {
                api::set_token(&token);
                Self {
                    user: Some(user),
                    token: Some(token),
                    loading: false,
                }
            }
            AuthAction::Logout => {
                api::remove_token();
                Self {
                    user: None,
                    token: None,
                    loading: false,
                }
            }
            AuthAction::SetLoading(loading) => Self {
                loading,
                ..(*self).clone()
            },
        }
        .into()
    }
}

pub type AuthContext = UseReducerHandle<AuthState>;

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let auth = use_reducer(AuthState::default);

    html! {
        <ContextProvider<AuthContext> context={auth}>
            {props.children.clone()}
        </ContextProvider<AuthContext>>
    }
}

#[hook]
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>().expect("AuthContext not found")
}
