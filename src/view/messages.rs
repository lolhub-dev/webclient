use crate::page;
use seed::{prelude::*, *};

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    ToggleMenu,
    ToggleLoginModal,
    RegisterTabActive,
    LoginTabActive,
    ChangeRegisterEmailValue(String),
    ChangeRegisterUsernameValue(String),
    ChangeRegisterPasswordValue(String),
    HideMenu,
    LogIn,
    LogOut,
    SignUp,
    ProfileMsg(page::profile::Msg),
}
