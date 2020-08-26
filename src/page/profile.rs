use seed::{prelude::*, *};

pub struct Model;
pub enum Msg {

}

// ----------------
//     Init
// ----------------

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model
}

// ----------------
//    Model
// ----------------



// ----------------
//    Update
// ----------------

pub fn update(msg: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {}
}

// ----------------
//      View
// ----------------

pub fn view<Ms>(_: &Model) -> Vec<Node<Ms>> {
    vec![div![C!["container", "contentainer"], "Profile view"]]
}
