#![allow(clippy::wildcard_imports)]

use crate::domain::user;
use crate::gateway::mock::mock_user_gateway::MockUserGateway;
use crate::port::user_port::AuthResult;
use seed::{prelude::*, *};
use serde::Deserialize;

// Re-export components module so we can use it in page module
pub mod components;
pub mod domain;
pub mod driver;
pub mod gateway;
pub mod port;
pub mod usecase;

mod page;
mod utils;
use crate::components::auth_component;

const PROFILE: &str = "profile";
const ABOUT: &str = "about";

const mock_user_gateway: MockUserGateway = MockUserGateway {};
// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .stream(streams::window_event(Ev::Click, |_| Msg::HideMenu));
    Model {
        user: None,
        base_url: url.to_base_url(),
        menu_visible: true,
        login_modal_visible: false,
        login_modal_register_tab_active: false,
        register_email_value: String::from(""),
        register_username_value: String::from(""),
        register_password_value: String::from(""),
        register_password_comp_value: String::from(""),
        register_accepted_tou: false,
        page: Page::Home,
    }
}

// ------ ------
//   Messages
// ------ ------

pub enum Msg {
    UrlChanged(subs::UrlChanged),

    // Navbar
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

    // Routing messages
    ProfileMsg(page::profile::Msg),
}

// ------ ------
//     Model
// ------ ------

struct Model {
    user: Option<AuthResult<user::User>>,
    base_url: Url,
    page: Page,
    menu_visible: bool,
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
    login_modal_visible: bool,
    login_modal_register_tab_active: bool,
    register_email_value: String,
    register_username_value: String,
    register_password_value: String,
    register_password_comp_value: String,
    register_accepted_tou: bool,
}

struct RegisterContext {
    email: String,
    username: String,
    password: String,
    password_comp: String,
    accepted_tou: bool,
}

enum LoginModalState {
    Hidden,
    VisibleLogin,
    VisibleRegister,
}

// ------------
//    Pages
// ------------

// @TODO: Add the rest of the pages
enum Page {
    Home,
    Profile(page::profile::Model),
    About,
    NotFound,
}

impl Page {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => Self::Home,
            [PROFILE] => {
                Self::Profile(page::profile::init(url, &mut orders.proxy(Msg::ProfileMsg)))
            }
            [ABOUT] => Self::About,
            _ => Self::NotFound,
        }
    }
}

// ----------
//    Urls
// ----------

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
    fn profile(self) -> Url {
        self.base_url().add_path_part(PROFILE)
    }
    fn about(self) -> Url {
        self.base_url().add_path_part(ABOUT)
    }
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url, orders),

        // Navbar visibility
        Msg::ToggleMenu => model.menu_visible = !model.menu_visible,
        Msg::HideMenu => model.menu_visible = false,

        // Login buttons
        Msg::LogIn => {
            log!("logOut message");
            // TODO: (mock) use_cases asynchronous
            let login_res = usecase::user::login_user(
                &mock_user_gateway,
                &user::Credentials {
                    name_or_email: user::UNameOrEmail::Email("test".to_string()),
                    password: "test".to_string(),
                },
            );
            orders.send_msg(Msg::LogInResult(login_res));
        }
        Msg::LogOut => log!("logOut message"),
        Msg::SignUp => log!("signUp message"),
        Msg::LogInResult(auth_result) => model.user = Some(auth_result),
        Msg::LogOutResult(Ok(_)) => model.user = None,
        Msg::LogOutResult(Err(auth_err)) => {
            orders.skip();
            ()
        }
        Msg::SignUpResult(auth_result) => model.user = Some(auth_result),

        // Login modal visibility
        Msg::ToggleLoginModal => model.login_modal_visible = !model.login_modal_visible,
        Msg::RegisterTabActive => model.login_modal_register_tab_active = true,
        Msg::LoginTabActive => model.login_modal_register_tab_active = false,

        // Login/Register form
        Msg::ChangeRegisterEmailValue(email_address) => model.register_email_value = email_address,
        Msg::ChangeRegisterUsernameValue(username) => model.register_username_value = username,
        Msg::ChangeRegisterPasswordValue(password) => model.register_password_value = password,
        Msg::ChangeRegisterPasswordCompValue(password) => {
            model.register_password_comp_value = password
        }
        Msg::ToggleRegisterAcceptedTou => {
            model.register_accepted_tou = !model.register_accepted_tou
        }

        // Routing related
        Msg::ProfileMsg(msg) => {
            if let Page::Profile(model) = &mut model.page {
                page::profile::update(msg, model, &mut orders.proxy(Msg::ProfileMsg))
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    let maybe_user: Option<&user::User> = match &model.user {
        None => None,
        Some(Err(_)) => None,
        Some(Ok(user)) => Some(user),
    };
    vec![div![
        C!["page-wrapper"], //enable sticky footer
        view_navbar(model.menu_visible, &model.base_url, maybe_user, &model.page,),
        div![C!["content-wrapper"], view_content(&model.page)], //enable sticky footer
        view_footer(),
        // MODALS:
        // @TODO: Refactor would greatly reduce the verbosity here
        auth_component::view(
            model.login_modal_visible,
            model.login_modal_register_tab_active,
            &model.register_username_value,
            &model.register_email_value,
            &model.register_password_value,
            &model.register_password_comp_value,
            model.register_accepted_tou,
        )
    ]]
}

// ----- view_content ------

fn view_content(page: &Page) -> Node<Msg> {
    div![match page {
        Page::Home => page::home::view(),
        Page::Profile(model) => page::profile::view(model).map_msg(Msg::ProfileMsg),
        Page::NotFound => page::not_found::view(),
        Page::About => page::about::view(),
    }]
}

// ----- view_navbar ------
fn view_navbar(
    menu_visible: bool,
    base_url: &Url,
    user: Option<&user::User>,
    page: &Page,
) -> Node<Msg> {
    nav![
        C!["navbar", "is-dark"],
        attrs! {
            At::from("role") => "navigation",
            At::AriaLabel => "main navigation",
        },
        view_brand_and_hamburger(menu_visible, base_url),
        view_navbar_menu(menu_visible, base_url, user, page),
    ]
}

fn view_brand_and_hamburger(menu_visible: bool, base_url: &Url) -> Node<Msg> {
    div![
        C!["navbar-brand"],
        // ------ Logo ------
        a![
            C!["navbar-item", "has-text-weight-bold", "is-size-3"],
            attrs! {At::Href => Urls::new(base_url).home()},
            "lol:Hub"
        ],
        // ------ Hamburger ------
        a![
            C!["navbar-burger", "burger", IF!(menu_visible => "is-active")],
            attrs! {
                At::from("role") => "button",
                At::AriaLabel => "menu",
                At::AriaExpanded => menu_visible,
            },
            ev(Ev::Click, |event| {
                event.stop_propagation();
                Msg::ToggleMenu
            }),
            span![attrs! {At::AriaHidden => "true"}],
            span![attrs! {At::AriaHidden => "true"}],
            span![attrs! {At::AriaHidden => "true"}],
        ]
    ]
}

fn view_navbar_menu(
    menu_visible: bool,
    base_url: &Url,
    user: Option<&user::User>,
    page: &Page,
) -> Node<Msg> {
    div![
        C!["navbar-menu", IF!(menu_visible => "is-active")],
        view_navbar_menu_start(base_url, page),
        view_navbar_menu_end(base_url, user),
    ]
}

fn view_navbar_menu_start(base_url: &Url, page: &Page) -> Node<Msg> {
    div![
        C!["navbar-start"],
        a![
            C![
                "navbar-item",
                "is-tab",
                IF!(matches!(page, Page::Home) => "is-active"),
            ],
            attrs! {At::Href => Urls::new(base_url).home()},
            "Home",
        ],
        a![
            C![
                "navbar-item",
                "is-tab",
                IF!(matches!(page, Page::Profile(_)) => "is-active"),
            ],
            attrs! {At::Href => Urls::new(base_url).profile()},
            "Profile",
        ],
        a![
            C![
                "navbar-item",
                "is-tab",
                IF!(matches!(page, Page::About) => "is-active"),
            ],
            attrs! {At::Href => Urls::new(base_url).about()},
            "About",
        ],
    ]
}

fn view_navbar_menu_end(base_url: &Url, user: Option<&user::User>) -> Node<Msg> {
    div![
        C!["navbar-end"],
        div![
            C!["navbar-item"],
            div![
                C!["buttons"],
                if let Some(user) = user {
                    view_buttons_for_logged_in_user(base_url, user)
                } else {
                    view_buttons_for_anonymous_user()
                }
            ]
        ]
    ]
}

fn view_buttons_for_logged_in_user(base_url: &Url, user: &user::User) -> Vec<Node<Msg>> {
    vec![
        a![
            C!["button", "is-primary"],
            attrs![
                At::Href => Urls::new(base_url).profile(),
            ],
            strong![&user.username],
        ],
        a![
            C!["button", "is-light"],
            "Log out",
            ev(Ev::Click, |_| Msg::LogOut),
        ],
    ]
}

fn view_buttons_for_anonymous_user() -> Vec<Node<Msg>> {
    vec![
        a![
            C!["button", "is-primary"],
            strong!["Sign up"],
            ev(Ev::Click, |_| Msg::ToggleLoginModal),
        ],
        a![
            C!["button", "is-light"],
            "Log in",
            ev(Ev::Click, |_| Msg::ToggleLoginModal),
        ],
    ]
}

fn view_footer() -> Node<Msg> {
    footer![
        C!["footer"],
        div![C!["content", "has-text-centered"], p!["footer"]]
    ]
}

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    App::start("app", init, update, view);
}
