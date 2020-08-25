use seed::{prelude::*, *};

pub fn view<Ms>() -> Vec<Node<Ms>> {
    vec![
        view_hero(),
        view_content()
    ]
}

fn view_hero<Ms>() -> Node<Ms> {
    div![
        C!["hero", "is-primary", "is-bold"],
        div![
            C!["hero-body"],
            div![
                C!["container", "has-text-centered"],
                h1![C!["title"], "lol:Hub"],
            ],
        ],
    ]
}

fn view_content<Ms>() -> Node<Ms> {
    div![
        C!["container", "mt-6"],
        div![
            C!["is-desktop", "is-vcentered"],
            input![
                C!["input", "is-rounded"],
                attrs! {
                    At::from("type") => "text",
                    At::from("placheholder") => "Search..."
                }
            ]
        ]
    ]
}
