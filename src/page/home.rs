use seed::{prelude::*, *};

pub fn view<Ms>() -> Vec<Node<Ms>> {
    vec![view_hero(), view_search()]
}

fn view_hero<Ms>() -> Node<Ms> {
    div![
        C!["hero", "is-medium", "is-primary", "is-bold", "has-bg-img"],
        div![
            C!["hero-body"],
            div![
                C!["container", "has-text-centered"],
                h1![C!["title"], "lol:Hub"],
            ],
        ],
    ]
}

fn view_search<Ms>() -> Node<Ms> {
    div![
        C!["container", "mt-6"],
        div![
            C!["field", "has-addons",],
            div![
                C!["control", "is-expanded"],
                input![
                    C!["input", "is-rounded"],
                    attrs! {
                        At::from("type") => "text",
                        At::from("placeholder") => "Search..."
                    }
                ]
            ],
            div![C!["control"], a![C!["button is-info"], "Search",]]
        ]
    ]
}
