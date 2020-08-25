use seed::{prelude::*, *};

use crate::User;

// ----------------
//     Init
// ----------------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        errors: Vec::new(),
        user: RemoteData::NotAsked,
    }
}

// ----------------
// ----------------
pub struct Model {
    errors: Vec<FetchError>,
    user: RemoteData<User>,
}

enum RemoteData<T> {
    NotAsked,
    Loading,
    Loaded(T),
}

// ----------------
//    Update 
// ----------------

pub enum Msg {
    UserFetched(fetch::Result<User>),
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UserFetched(Ok(user)) => log!("user fetched"),
        Msg::UserFetched(Err(err)) => log!("error user fetched"),
    }
}

// ----------------
//      View 
// ----------------

pub fn view<Ms>(model: &Model) -> Vec<Node<Ms>> {
    vec![div!["Profile view"]]
}
