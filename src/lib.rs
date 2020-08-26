#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use serde::Deserialize;
use ulid::Ulid;

mod page;

const PROFILE: &str = "profile";

// ------ ------
//     Init
// ------ ------
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .stream(streams::window_event(Ev::Click, |_| Msg::HideMenu));
    Model {
        ctx: Context {
            user: None,
            token: None,
        },
        base_url: url.to_base_url(),
        menu_visible: true,
        login_modal_visible: false,
        login_modal_register_tab_active: false,
        page: Page::Home,
    }
}

// ------ ------
//     Model
// ------ ------
struct Model {
    ctx: Context,
    base_url: Url,
    page: Page,
    menu_visible: bool,
    login_modal_visible: bool,
    login_modal_register_tab_active: bool,
}

struct Context {
    user: Option<User>,
    // @TODO: Do we need the token ? -> How is authentication done?
    token: Option<String>,
}

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    email: String,
    summoner_name: Option<String>,
    verified: bool,
    reputation: i32,
}

// ------------
//    Pages
// ------------

// @TODO: Add the rest of the pages
enum Page {
    Home,
    Profile(page::profile::Model),
    NotFound,
}

impl Page {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => Self::Home,
            [PROFILE] => {
                Self::Profile(page::profile::init(url, &mut orders.proxy(Msg::ProfileMsg)))
            }
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
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
    ToggleMenu,
    ToggleLoginModal,
    RegisterTabActive,
    LoginTabActive,
    HideMenu,
    LogIn,
    LogOut,
    SignUp,
    ProfileMsg(page::profile::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url, orders),
        Msg::ToggleMenu => model.menu_visible = !model.menu_visible,
        Msg::ToggleLoginModal => model.login_modal_visible = !model.login_modal_visible,
        Msg::RegisterTabActive => model.login_modal_register_tab_active = true,
        Msg::LoginTabActive => model.login_modal_register_tab_active = false,
        Msg::HideMenu => model.menu_visible = false,
        Msg::LogIn => log!("logIn message"),
        Msg::LogOut => log!("logOut message"),
        Msg::SignUp => log!("signUp message"),

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
    vec![div![
        C!["page-wrapper"], //enable sticky footer
        view_navbar(
            model.menu_visible,
            &model.base_url,
            model.ctx.user.as_ref(),
            &model.page,
        ),
        div![C!["content-wrapper"], view_content(&model.page)], //enable sticky footer
        view_footer(),
        // MODALS:
        view_login_or_register_modal(
            model.login_modal_visible,
            model.login_modal_register_tab_active
        )
    ]]
}

// ----- view_content ------

fn view_content(page: &Page) -> Node<Msg> {
    div![match page {
        Page::Home => page::home::view(),
        Page::Profile(model) => page::profile::view(model).map_msg(Msg::ProfileMsg),
        Page::NotFound => page::not_found::view(),
    }]
}

// ----- view_navbar ------
fn view_navbar(menu_visible: bool, base_url: &Url, user: Option<&User>, page: &Page) -> Node<Msg> {
    nav![
        C!["navbar"],
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
    user: Option<&User>,
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
    ]
}

fn view_navbar_menu_end(base_url: &Url, user: Option<&User>) -> Node<Msg> {
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

fn view_buttons_for_logged_in_user(base_url: &Url, user: &User) -> Vec<Node<Msg>> {
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

fn view_login_or_register_modal(visible: bool, register_tab_active: bool) -> Node<Msg> {
    let login_modal_content = div![
        C!["modal-content"],
        div![
            C!["tabs"],
            ul![
                li![
                    C![IF![!register_tab_active =>"is_active"]],
                    a!["Login"],
                    ev(Ev::Click, |event| {
                        event.stop_propagation();
                        Msg::RegisterTabActive
                    })
                ],
                li![
                    C![IF![register_tab_active =>"is_active"]],
                    a!["Register"],
                    ev(Ev::Click, |event| {
                        event.stop_propagation();
                        Msg::LoginTabActive
                    })
                ]
            ]
        ],
        IF![register_tab_active =>view_login_component()],
        IF![!register_tab_active =>view_register_component()]
    ];
    let login_modal_toggle_handler = ev(Ev::Click, |event| {
        event.stop_propagation();
        Msg::ToggleLoginModal
    });
    div![
        C!["modal", IF![visible => "is-active"]],
        div![C!["modal-background"], login_modal_toggle_handler.clone()],
        login_modal_content,
        button![
            C!["modal-close", "is-large",],
            attrs! {At::AriaLabel=>"close"},
            login_modal_toggle_handler
        ]
    ]
}

fn view_login_component() -> Node<Msg> {
    div![
        C!["field"],
        p![
            C!["control", "has-icons-left", "has-icons-right"],
            input![
                C!["input"],
                attrs![At::from("placeholder")=>"Email", At::from("type")=>"email"]
            ],
            span![
                C!["icon", "is-small", "is-left"],
                i![C!["fas", "fa-envelope"]]
            ],
            span![C!["icon", "is-small", "is-left"], i![C!["fas", "fa-check"]]]
        ]
    ]
}

fn view_register_component() -> Node<Msg> {
    a!["Register Component"]
}
// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    App::start("app", init, update, view);
}
