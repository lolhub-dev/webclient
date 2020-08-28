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

use crate::domain::user;
use crate::gateway::mock::mock_user_gateway::MockUserGateway;
use crate::port::user_port::{AuthError, AuthResult};
use generated::css_classes::C;
use seed::{prelude::*, *};
use std::fmt;
use MenuVisibility::*;

const TITLE_SUFFIX: &str = "Title Suffix? What is that?";
// https://mailtolink.me/
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const STATIC_PATH: &str = "static";
const MOCK_PATH: &str = "static/mocks";
const IMAGES_PATH: &str = "static/images";

const ABOUT: &str = "about";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        // .stream(streams::window_event(Ev::Scroll, |_| Msg::Scrolled))
        .stream(streams::window_event(Ev::Click, |_| Msg::HideMenu));

    Model {
        base_url: url.to_base_url(),
        page: Page::init(url),
        menu_visibility: Hidden,
        in_prerendering: is_in_prerendering(),
        session: None,

        auth_modal_visible: false,
        auth_modal_register_tab_active: false,
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

    // @TODO: Refactor (?) Should probably encapsulate all the register and login related
    // stuff in a separate struct.  Something like
    //
    // struct Model {
    //    ...
    //    register_context: RegisterContext
    //    ...
    // }
    //
    // and then
    //
    // struct RegisterContext {
    //     email: String,
    //     username: String,
    //     password: String,
    //     password_comp: String,
    //     accepted_tou: bool
    // }
    //
    // Maybe also LoginModalState like
    //
    // enum LoginModalState {
    //     Hidden,
    //     VisibleRegister,
    //     VisibleLogin
    // }
    //
    // to reduce the number of booleans in our model
    auth_modal_visible: bool,
    auth_modal_register_tab_active: bool,
    register_email_value: String,
    register_username_value: String,
    register_password_value: String,
    register_password_comp_value: String,
    register_accepted_tou: bool,
}

pub type Session = Option<AuthResult<user::User>>;

// ------ Page ------

#[derive(Clone, Copy, Eq, PartialEq)]
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
    LogIn,
    LogInResult(AuthResult<user::User>),
    LogOut,
    LogOutResult(AuthResult<()>),
    SignUp,
    SignUpResult(AuthResult<user::User>),

    // Login modal visibility
    // @TODO: Could change this to HideLoginModal and then send this from auth_component
    ToggleLoginModal,
    // @TODO Send these messages from the respective button in the navbar
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
        Msg::LogIn => {
            log!("logIn message");
            // let login_res = Err(AuthError::InvalidCredentials);
            // TODO: (mock) use_cases asynchronous
            // let login_res = usecase::user::login_user(
            // &mock_user_gateway,
            // &user::Credentials {
            // name_or_email: user::UNameOrEmail::Email("test".to_string()),
            // password: "test".to_string(),
            // },
            // );
            // orders.send_msg(Msg::LogInResult(login_res));
        }
        Msg::LogOut => log!("logOut message"),
        Msg::SignUp => log!("signUp message"),
        Msg::LogInResult(auth_result) => model.session = Some(auth_result),
        Msg::LogOutResult(Ok(_)) => model.session = None,
        Msg::LogOutResult(Err(auth_err)) => {
            orders.skip();
            ()
        }
        Msg::SignUpResult(auth_result) => model.session = Some(auth_result),

        // Login modal visibility
        Msg::ToggleLoginModal => {
            model.auth_modal_visible = !model.auth_modal_visible
        }
        Msg::RegisterTabActive => model.auth_modal_register_tab_active = true,
        Msg::LoginTabActive => model.auth_modal_register_tab_active = false,

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

// Notes:
// - \u{00A0} is the non-breaking space
//   - https://codepoints.net/U+00A0
//
// - "▶\u{fe0e}" - \u{fe0e} is the variation selector, it prevents ▶ to change to emoji in some browsers
//   - https://codepoints.net/U+FE0E

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

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}

pub fn mock_path(mock: &str) -> String {
    format!("{}/{}", MOCK_PATH, mock)
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
