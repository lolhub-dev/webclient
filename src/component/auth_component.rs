use crate::domain::user;
use crate::utils;
use crate::{LoginModalState, Model, Msg};
use seed::{prelude::*, *};
use user::UNameOrEmail;

pub fn view(model: &Model) -> Node<Msg> {
    let login_modal_content = div![
        C!["modal-card"],
        div![
            C!["modal-card-body", "box"],
            div![
                C!["modal-fixed-height"],
                div![
                    C!["container"],
                    div![
                        C!["tabs"],
                        ul![
                            li![
                                IF![model.auth_modal_state == LoginModalState::VisibleLogin => C!["is_active"]],
                                a!["Login"],
                                ev(Ev::Click, |event| {
                                    event.stop_propagation();
                                    Msg::LoginTabActive
                                })
                            ],
                            li![
                                IF![model.auth_modal_state == LoginModalState::VisibleRegister => C!["is_active"]],
                                a!["Register"],
                                ev(Ev::Click, |event| {
                                    event.stop_propagation();
                                    Msg::RegisterTabActive
                                })
                            ]
                        ]
                    ],
                    match model.auth_modal_state {
                        LoginModalState::VisibleLogin => view_login(model),
                        LoginModalState::VisibleRegister =>
                            view_register(model),
                        _ => a![],
                    }
                ]
            ]
        ]
    ];
    let login_modal_toggle_handler = ev(Ev::Click, |event| {
        event.stop_propagation();
        Msg::HideLoginModal
    });
    div![
        C![
            "modal",
            IF![not(matches!(model.auth_modal_state, LoginModalState::Hidden)) => "is-active"]
        ],
        div![C!["modal-background"], login_modal_toggle_handler.clone()],
        login_modal_content,
        button![
            C!["modal-close", "is-large",],
            attrs! {At::AriaLabel=>"close"},
            login_modal_toggle_handler
        ]
    ]
}

fn view_login(model: &Model) -> Node<Msg> {
    let username = model.login_username_value.clone();
    let password = model.login_password_value.clone();
    div![
        div![
            C!["image", "is-19by9", "is-paragraph"],
            img![
                attrs!(At::from("src")=>{"https://bulma.io/images/placeholders/640x320.png"})
            ],
        ],
        form![
            div![
                C!["field"],
                p![
                    C!["control", "has-icons-left", "has-icons-right"],
                    input![
                        C!["input",],
                        attrs![At::from("placeholder")=>"Username/Email", At::from("type")=>"email", At::from("autocomplete") =>"username"],
                        input_ev(Ev::Input, |event| {
                            Msg::ChangeLoginUsernameValue(event)
                        })
                    ],
                    span![
                        C!["icon", "is-small", "is-left"],
                        i![C!["fas", "fa-envelope"]]
                    ],
                    span![
                        C!["icon", "is-small", "is-left"],
                        i![C!["fas", "fa-check"]]
                    ]
                ]
            ],
            div![
                C!["field"],
                p![
                    C!["control", "has-icons-left", "has-icons-right"],
                    input![
                        C!["input",],
                        attrs![At::from("placeholder")=>"Password", At::from("type")=>"password", At::from("autocomplete") => "current-password"],
                        input_ev(Ev::Input, |event| {
                            Msg::ChangeLoginPasswordValue(event)
                        })
                    ],
                    span![
                        C!["icon", "is-small", "is-left"],
                        i![C!["fas", "fa-lock"]]
                    ],
                ]
            ],
            nav![
                C!["level"],
                div![
                    C!["level-left"],
                    div![
                        C!["level-item"],
                        div![
                            C!["field"],
                            p![
                                C!["control"],
                                a![
                                    C!["button ", "is-success",],
                                    "Login",
                                    ev(Ev::Click, |event| {
                                        event.stop_propagation();
                                        Msg::LogIn(user::Credentials {
                                            name_or_email:
                                                UNameOrEmail::Username(
                                                    username,
                                                ),
                                            password,
                                        })
                                    }),
                                ]
                            ],
                        ]
                    ]
                ],
                div![
                    C!["level-right"],
                    div![
                        C!["level-item"],
                        a!["Forgot Username/Password?"]
                    ]
                ]
            ]
        ]
    ]
}

fn view_register(model: &Model) -> Node<Msg> {
    let passwords_matching = model.register_password_value
        == model.register_password_comp_value;
    // TODO: actually check the username availability in the backend!
    let is_valid_username: Result<(), ()> = Ok(());
    let is_valid_email_address =
        utils::check_valid_email(&model.register_email_value);
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
                span![
                    C!["icon", "is-small", "is-left"],
                    i![C!["fas", "fa-user"]]
                ],
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
                        IF!(!is_valid_email_address && model.register_email_value!="" => "is-danger"),
                        IF!(is_valid_email_address && model.register_email_value!="" => "is-success"),
                    ],
                    attrs![At::from("placeholder")=>"Email", At::from("type")=>"email",At::from("autocomplete")=>"email"],
                    input_ev(Ev::Input, |event| {
                        Msg::ChangeRegisterEmailValue(event)
                    })
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
                    C!["input",],
                    attrs![At::from("placeholder")=>"Password", At::from("type")=>"password", At::from("autocomplete")=>"new-password"],
                    input_ev(Ev::Input, |event| {
                        Msg::ChangeRegisterPasswordValue(event)
                    })
                ],
                span![
                    C!["icon", "is-small", "is-left"],
                    i![C!["fas", "fa-lock"]]
                ],
            ],
            IF![model.register_password_value!=""=>p![
                C!["help", "is-danger"],
            ]]
        ],
        div![
            C!["field"],
            p![
                C!["control", "has-icons-left", "has-icons-right"],
                input![
                    C!["input", IF![!passwords_matching =>"is-error"],],
                    input_ev(Ev::Input, |event| {
                        Msg::ChangeRegisterPasswordCompValue(event)
                    }),
                    attrs![At::from("placeholder")=>"Repeat Password", At::from("type")=>"password",At::from("autocomplete")=>"new-password"],
                    input_ev(Ev::Input, |event| {
                        Msg::ChangeRegisterPasswordValue(event)
                    })
                ],
                span![
                    C!["icon", "is-small", "is-left"],
                    i![C!["fas", "fa-lock"]]
                ],
            ],
            IF![model.register_password_value!=""=>p![
                C!["help", "is-danger"],
            ]]
        ],
        div![label![
            C!["checkbox"],
            input![
                attrs![At::from("type") => "checkbox"],
                ev(Ev::Click, |event| {
                    event.stop_propagation();
                    Msg::ToggleRegisterAcceptedTou
                })
            ],
            " I agree to the ",
            a!["terms and conditions"],
        ],],
        nav![
            C!["level"],
            div![
                C!["level-left"],
                div![
                    C!["level-item"],
                    div![
                        C!["field"],
                        p![
                            C!["control"],
                            a![
                                C!["button ", "is-success",],
                                IF![!model.register_accepted_tou=>attrs!(At::from("disabled")=>"")],
                                "Register"
                            ]
                        ],
                    ]
                ]
            ],
            div![
                C!["level-right"],
                div![C!["level-item"], a!["Got questions?"]]
            ]
        ]
    ]
}
