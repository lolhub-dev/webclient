use crate::utils;
use crate::Msg;
use seed::{prelude::*, *};

pub fn view(
    visible: bool,
    register_tab_active: bool,
    username: &str,
    email_address: &str,
    password: &str,
) -> Node<Msg> {
    let login_modal_content = div![
        C!["modal-card"],
        div![
            C!["modal-card-body", "modal-fixed", "box"],
            div![
                C!["tabs", "is-boxed"],
                ul![
                    li![
                        C![IF![!register_tab_active =>"is_active"]],
                        a!["Login"],
                        ev(Ev::Click, |event| {
                            event.stop_propagation();
                            Msg::RegisterTabActive
                        })
                    ],
                    li![
                        C![IF![register_tab_active =>"is_active"]],
                        a!["Register"],
                        ev(Ev::Click, |event| {
                            event.stop_propagation();
                            Msg::LoginTabActive
                        })
                    ]
                ]
            ],
            IF![register_tab_active =>view_login()],
            IF![!register_tab_active =>view_register(username, email_address, password)]
        ]
    ];
    let login_modal_toggle_handler = ev(Ev::Click, |event| {
        event.stop_propagation();
        Msg::ToggleLoginModal
    });
    div![
        C!["modal", IF![visible => "is-active"]],
        div![C!["modal-background"], login_modal_toggle_handler.clone()],
        login_modal_content,
        button![
            C!["modal-close", "is-large",],
            attrs! {At::AriaLabel=>"close"},
            login_modal_toggle_handler
        ]
    ]
}

fn view_login<Msg>() -> Node<Msg> {
    form![
        div![
            C!["field"],
            p![
                C!["control", "has-icons-left", "has-icons-right"],
                input![
                    C!["input"],
                    attrs![At::from("placeholder")=>"Email", At::from("type")=>"email"]
                ],
                span![
                    C!["icon", "is-small", "is-left"],
                    i![C!["fas", "fa-envelope"]]
                ],
                span![C!["icon", "is-small", "is-left"], i![C!["fas", "fa-check"]]]
            ]
        ],
        div![
            C!["field"],
            p![
                C!["control", "has-icons-left", "has-icons-right"],
                input![
                    C!["input"],
                    attrs![At::from("placeholder")=>"Password", At::from("type")=>"password"]
                ],
                span![C!["icon", "is-small", "is-left"], i![C!["fas", "fa-lock"]]],
            ]
        ]
    ]
}

fn view_register(username: &str, email_address: &str, password: &str) -> Node<Msg> {
    let is_valid_username: Result<(), ()> = Ok(()); // TODO: actually check the username availability in the backend!
    let is_valid_email_address = utils::check_valid_email(email_address);
    let is_valid_password = utils::check_valid_password(password);
    // TODO: also check whether the email and username are already taken !!!
    form![
        div![
            C!["field"],
            p![
                C!["control", "has-icons-left", "has-icons-right"],
                input![
                    C!["input"],
                    attrs![At::from("placeholder")=>"Username"],
                    input_ev(Ev::Input, |event| {
                        Msg::ChangeRegisterUsernameValue(event)
                    })
                ],
                span![C!["icon", "is-small", "is-left"], i![C!["fas", "fa-user"]]],
                span![
                    C!["icon", "is-small", "is-right"],
                    i![C!["fas", "fa-check"]]
                ]
            ]
        ],
        div![
            C!["field"],
            p![
                C!["control", "has-icons-left", "has-icons-right"],
                input![
                    C![
                        "input",
                        IF!(!is_valid_email_address && email_address!="" => "is-danger"),
                        IF!(is_valid_email_address && email_address!="" => "is-success"),
                    ],
                    attrs![At::from("placeholder")=>"Email", At::from("type")=>"email"],
                    input_ev(Ev::Input, |event| { Msg::ChangeRegisterEmailValue(event) })
                ],
                span![
                    C!["icon", "is-small", "is-left"],
                    i![C!["fas", "fa-envelope",]]
                ],
                span![
                    C!["icon", "is-small", "is-right",],
                    i![C!["fas", "fa-check",]]
                ]
            ]
        ],
        div![
            C!["field"],
            p![
                C!["control", "has-icons-left", "has-icons-right"],
                input![
                    C![
                        "input",
                        IF![password!="" => match is_valid_password {
                            Ok(_) => "is-success",
                            Err(_) => "is-danger",
                        }]
                    ],
                    attrs![At::from("placeholder")=>"Password", At::from("type")=>"password"],
                    input_ev(Ev::Input, |event| {
                        Msg::ChangeRegisterPasswordValue(event)
                    })
                ],
                span![C!["icon", "is-small", "is-left"], i![C!["fas", "fa-lock"]]],
            ],
            IF![password!=""=>p![
                C!["help", "is-danger"],
                match is_valid_password {
                    Ok(_) => "", //this shouldnt happen, since this is only the help for 'is-danger'
                    Err(utils::InvalidPasswordErr::TooShort) => "Minimum length is 8 characters.",
                    Err(utils::InvalidPasswordErr::TooLong) => "Maximal lenght is 32 characters.",
                    Err(utils::InvalidPasswordErr::MissingDigit) => "At least one digit!",
                    Err(utils::InvalidPasswordErr::MissingLowercase) =>
                        "At least one lowercase character!",
                    Err(utils::InvalidPasswordErr::MissingUppercase) =>
                        "At least one uppercase character!",
                    Err(utils::InvalidPasswordErr::MissingSpecialChar) =>
                        "At least one special character!",
                }
            ]]
        ]
    ]
}
