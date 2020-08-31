use crate::{generated::css_classes::C, Msg};
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    footer![
        C!["footer"],
        div![C!["content", "has-text-centered"], p!["footer"]]
    ]
}
