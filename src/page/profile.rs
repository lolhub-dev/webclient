use seed::{prelude::*, *};

use crate::User;

// ----------------
//     Init
// ----------------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {}

// ----------------
//    Model
// ----------------



// ----------------
//    Update
// ----------------

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {}
}

// ----------------
//      View
// ----------------

pub fn view<Ms>(model: &Model) -> Vec<Node<Ms>> {
    vec![div![C!["container", "contentainer"], "Profile view"]]
}
