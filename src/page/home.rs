use crate::component::hero_component;
use crate::{generated::css_classes::C, Model, Msg};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Node<Msg> {
    div![hero_component::view(), view_search()]
}

fn view_search() -> Node<Msg> {
    div![
        C!["container", "contentainer"],
        div![
            C!["field", "has-addons",],
            div![
                C!["control", "is-expanded"],
                input![
                    C!["input"],
                    attrs! {
                        At::from("type") => "text",
                        At::from("placeholder") => "Search for gamemodes..."
                    }
                ]
            ],
            div![C!["control"], a![C!["button is-info"], "Search",]]
        ]
    ]
}
