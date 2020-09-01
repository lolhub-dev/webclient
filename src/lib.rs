// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]

#![allow(
    clippy::used_underscore_binding,
    clippy::non_ascii_literal,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

pub mod component;
pub mod domain;
pub mod driver;
pub mod gateway;
pub mod port;
pub mod usecase;

mod generated;
mod page;
mod utils;

use crate::domain::user::{Credentials, UNameOrEmail, User};
// use crate::gateway::mock::mock_user_gateway::MockUserGateway;
use crate::port::user_port::{AuthError, AuthResult};
use gateway::mock::mock_user_gateway::MockUserGateway;
use generated::css_classes::C;
use seed::{prelude::*, *};
use serde_json::error::Error as JsonError;
use std::fmt;
use MenuVisibility::*;

const TITLE_SUFFIX: &str = "Custom League of Legends Gamemodes";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const ABOUT: &str = "about";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged)
    // .stream(streams::window_event(Ev::Scroll, |_| Msg::Scrolled))
    .stream(streams::window_event(Ev::Click, |_| Msg::HideMenu));

    Model {
        base_url: url.to_base_url(),
        page: Page::init(url),
        menu_visibility: Hidden,
        in_prerendering: is_in_prerendering(),
        session: None,
        auth_modal_state: LoginModalState::Hidden,
        // auth_modal_visible: false,
        // auth_modal_register_tab_active: false,
        register_email_value: String::from(""),
        register_username_value: String::from(""),
        register_password_value: String::from(""),
        register_password_comp_value: String::from(""),
        register_accepted_tou: false,
    }
}

fn is_in_prerendering() -> bool {
    let user_agent =
        window().navigator().user_agent().expect("cannot get user agent");

    user_agent == USER_AGENT_FOR_PRERENDERING
}

// ------ ------
//     Model
// ------ ------

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum MenuVisibility {
    Visible,
    Hidden,
}

impl MenuVisibility {
    pub fn toggle(&mut self) {
        *self = match self {
            Visible => Hidden,
            Hidden => Visible,
        }
    }
}

//This is needed for the use of MenuVisibility in IF! macros...dont ask me why
//deriving Display doesnt work either...maybe theres a better solution
impl fmt::Display for MenuVisibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "test")
    }
}

pub struct Model {
    pub base_url: Url,
    pub page: Page,
    pub menu_visibility: MenuVisibility,
    pub in_prerendering: bool,
    pub session: Session,
    // auth_modal_visible: bool,
    // auth_modal_register_tab_active: bool,
    auth_modal_state: LoginModalState,
    register_email_value: String,
    register_username_value: String,
    register_password_value: String,
    register_password_comp_value: String,
    register_accepted_tou: bool,
    //TODO: add input value handlers for login form
    // login_username_value: String,
    // login_password_value: String,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum LoginModalState {
    Hidden,
    VisibleLogin,
    VisibleRegister,
}

pub type Session = Option<AuthResult<User>>;

// ------ Page ------

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Page {
    Home,
    About,
    NotFound,
}

impl Page {
    pub fn init(mut url: Url) -> Self {
        let (page, title) = match url.remaining_path_parts().as_slice() {
            [] => (Self::Home, TITLE_SUFFIX.to_owned()),
            [ABOUT] => (Self::About, format!("About - {}", TITLE_SUFFIX)),
            _ => (Self::NotFound, format!("404 - {}", TITLE_SUFFIX)),
        };
        document().set_title(&title);
        page
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }

    pub fn about(self) -> Url {
        self.base_url().add_path_part(ABOUT)
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    ScrollToTop,
    ToggleMenu,
    HideMenu,

    // Login buttons
    LogIn(Credentials),
    LogInResult(AuthResult<User>),
    LogOut,
    LogOutResult(AuthResult<()>),
    SignUp,
    SignUpResult(AuthResult<User>),

    // Login modal visibility
    HideLoginModal,
    RegisterTabActive,
    LoginTabActive,

    // Login/Register form
    ChangeRegisterEmailValue(String),
    ChangeRegisterUsernameValue(String),
    ChangeRegisterPasswordValue(String),
    ChangeRegisterPasswordCompValue(String),
    ToggleRegisterAcceptedTou,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("url changed to: {}", url);
            model.page = Page::init(url);
        }
        Msg::ScrollToTop => window().scroll_to_with_scroll_to_options(
            web_sys::ScrollToOptions::new().top(0.),
        ),
        Msg::ToggleMenu => model.menu_visibility.toggle(),
        Msg::HideMenu => {
            model.menu_visibility = Hidden;
        }
        // Login buttons
        Msg::LogIn(credentials) => {
            orders.perform_cmd(async {
                Msg::LogInResult(
                    async move {
                        log!("Trying to find stuff");
                        let file =
                            fetch(utils::mock_path("mock_user.json"))
                                .await
                                .map_err(|_| AuthError::NetworkError)?
                                .text()
                                .await
                                .map_err(|_| AuthError::NetworkError)?;

                        let users: Result<
                            Vec<User>,
                            serde_json::error::Error,
                        > = serde_json::from_str(&file[..]);

                        log!(users);

                        let ret_user = users.map(|mut users| {
                            users.retain(|user| {
                                match &credentials.name_or_email {
                                    UNameOrEmail::Username(uname) => {
                                        user.username == *uname
                                    }
                                    UNameOrEmail::Email(email) => {
                                        user.email == *email
                                    }
                                }
                            });
                            users.pop()
                        });

                        log!(ret_user);

                        let res = match ret_user {
                            Ok(Some(user)) => Ok(user),
                            _ => Err(AuthError::InvalidCredentials),
                        };

                        log!("Done!");
                        res
                    }.await
                )
            });
        }
        Msg::LogOut => {
            orders.send_msg(Msg::LogOutResult(Ok(())));
            ()
        }
        Msg::SignUp => log!("signUp message"),
        Msg::LogInResult(auth_result) => model.session = Some(auth_result),
        Msg::LogOutResult(Ok(_)) => model.session = None,
        Msg::LogOutResult(Err(_)) => {
            orders.skip();
            ()
        }
        Msg::SignUpResult(auth_result) => {
            model.session = Some(auth_result)
        }

        // Login modal visibility
        Msg::HideLoginModal => {
            model.auth_modal_state = LoginModalState::Hidden
        }
        Msg::RegisterTabActive => {
            model.auth_modal_state = LoginModalState::VisibleRegister
        }
        Msg::LoginTabActive => {
            model.auth_modal_state = LoginModalState::VisibleLogin
        }

        // Login/Register form
        Msg::ChangeRegisterEmailValue(email_address) => {
            model.register_email_value = email_address
        }
        Msg::ChangeRegisterUsernameValue(username) => {
            model.register_username_value = username
        }
        Msg::ChangeRegisterPasswordValue(password) => {
            model.register_password_value = password
        }
        Msg::ChangeRegisterPasswordCompValue(password) => {
            model.register_password_comp_value = password
        }
        Msg::ToggleRegisterAcceptedTou => {
            model.register_accepted_tou = !model.register_accepted_tou
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        C![
            IF!(not(model.in_prerendering) => C.fade_in),
            C.min_h_screen,
            C.flex,
            C.flex_col,
        ],
        vec![div![
            C!["page-wrapper"], //enable sticky footer
            page::partial::header::view(model),
            div![C!["content-wrapper"], view_content(model)], //enable sticky footer
            page::partial::footer::view(),
            // MODALS:
            component::auth_component::view(model)
        ]]
    ]
}

fn view_content(model: &Model) -> Node<Msg> {
    div![match model.page {
        Page::Home => page::home::view(&model),
        Page::About => page::about::view(),
        Page::NotFound => page::not_found::view(),
    }]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    App::start("app", init, update, view);

    log!("App started.");
}
