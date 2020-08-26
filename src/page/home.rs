use seed::{prelude::*, *};

use crate::components;

// ----------------
//     view 
// ----------------

pub fn view<Ms>() -> Vec<Node<Ms>> {
    vec![components::view_hero(), view_search()]
}


fn view_search<Ms>() -> Node<Ms> {
    div![
        C!["container", "mt-6"],
        div![
            C!["field", "has-addons",],
            div![
                C!["control", "is-expanded"],
                input![
                    C!["input"],
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
