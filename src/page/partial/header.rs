use crate::domain::user;
use crate::{
    generated::css_classes::C,
    port::user_port::AuthError,
    MenuVisibility::{self, *},
    Model, Msg, Page, Session, Urls,
};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Node<Msg> {
    view_navbar(model)
}
//
// ----- view_navbar ------
fn view_navbar(model: &Model) -> Node<Msg> {
    nav![
        C!["navbar", "is-dark"],
        attrs! {
            At::from("role") => "navigation",
            At::AriaLabel => "main navigation",
        },
        view_brand_and_hamburger(model.menu_visibility, &model.base_url),
        view_navbar_menu(
            model.menu_visibility,
            &model.base_url,
            &model.session,
            &model.page
        ),
    ]
}

fn view_brand_and_hamburger(
    menu_visible: MenuVisibility,
    base_url: &Url,
) -> Node<Msg> {
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
            C![
                "navbar-burger",
                "burger",
                IF![menu_visible == MenuVisibility::Visible => "is-active"
                ],
            ],
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
    menu_visible: MenuVisibility,
    base_url: &Url,
    session: &Session,
    page: &Page,
) -> Node<Msg> {
    div![
        C![
            "navbar-menu",
            IF!(menu_visible == MenuVisibility::Visible => "is-active")
        ],
        view_navbar_menu_start(base_url, page),
        view_navbar_menu_end(base_url, session),
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
                IF!(matches!(page, Page::About) => "is-active"),
            ],
            attrs! {At::Href => Urls::new(base_url).about()},
            "About",
        ],
    ]
}

fn view_navbar_menu_end(base_url: &Url, session: &Session) -> Node<Msg> {
    div![
        C!["navbar-end"],
        div![
            C!["navbar-item"],
            div![
                C!["buttons"],
                if let Some(Ok(user)) = session {
                    view_buttons_for_logged_in_user(base_url, &user)
                } else {
                    view_buttons_for_anonymous_user()
                }
            ]
        ]
    ]
}

fn view_buttons_for_logged_in_user(
    base_url: &Url,
    user: &user::User,
) -> Vec<Node<Msg>> {
    vec![
        a![C!["button", "is-primary"], strong![&user.username],],
        a![C!["button", "is-light"], "Log out", ev(Ev::Click, |_| Msg::LogOut),],
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
