use seed::{prelude::*, *};

pub fn view_hero<Ms>() -> Node<Ms> {
    div![
        C!["hero", "is-medium", "is-light", "is-bold", "has-bg-img"],
        div![
            C!["hero-body"],
            div![
                C!["container", "has-text-centered"],
                h1![C!["title", "is-size-1"], "lol:Hub"],
            ],
        ],
    ]
}
