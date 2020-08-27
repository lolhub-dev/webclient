use crate::utils;
use crate::Msg;
use seed::{prelude::*, *};

pub fn view(
    visible: bool,
    register_tab_active: bool,
    username: &str,
    email_address: &str,
    password: &str,
    password_comp: &str,
    tou_accepted: bool,
) -> Node<Msg> {
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
                                IF![!register_tab_active => C!["is_active"]],
                                a!["Login"],
                                ev(Ev::Click, |event| {
                                    event.stop_propagation();
                                    Msg::RegisterTabActive
                                })
                            ],
                            li![
                                IF![register_tab_active => C!["is_active"]],
                                a!["Register"],
                                ev(Ev::Click, |event| {
                                    event.stop_propagation();
                                    Msg::LoginTabActive
                                })
                            ]
                        ]
                    ],
                    IF![register_tab_active =>view_login()],
                    IF![!register_tab_active =>view_register(username, email_address, password, password_comp, tou_accepted)]
                ]
            ]
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

fn view_login() -> Node<Msg> {
    div![
        div![
            C!["image", "is-19by9", "is-paragraph"],
            img![attrs!(At::from("src")=>{"https://bulma.io/images/placeholders/640x320.png"})],
        ],
        form![
            div![
                C!["field"],
                p![
                    C!["control", "has-icons-left", "has-icons-right"],
                    input![
                        C!["input",],
                        attrs![At::from("placeholder")=>"Username/Email", At::from("type")=>"email"]
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
                        C!["input",],
                        attrs![At::from("placeholder")=>"Password", At::from("type")=>"password"]
                    ],
                    span![C!["icon", "is-small", "is-left"], i![C!["fas", "fa-lock"]]],
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
                                button![C!["button ", "is-success", "is-fullwidth"], "Login"]
                            ],
                        ]
                    ]
                ],
                div![
                    C!["level-right"],
                    div![C!["level-item"], a!["Forgot Username/Password?"]]
                ]
            ]
        ]
    ]
}

fn view_register(
    username: &str,
    email_address: &str,
    password: &str,
    password_comp: &str,
    tou_accepted: bool,
) -> Node<Msg> {
    let passwords_matching = password == password_comp;
    let is_valid_username: Result<(), ()> = Ok(()); // TODO: actually check the username availability in the backend!
    let is_valid_email_address = utils::check_valid_email(email_address);
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
                    C!["input",],
                    attrs![At::from("placeholder")=>"Password", At::from("type")=>"password"],
                    input_ev(Ev::Input, |event| {
                        Msg::ChangeRegisterPasswordValue(event)
                    })
                ],
                span![C!["icon", "is-small", "is-left"], i![C!["fas", "fa-lock"]]],
            ],
            IF![password!=""=>p![
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
                    attrs![At::from("placeholder")=>"Repeat Password", At::from("type")=>"password"],
                    input_ev(Ev::Input, |event| {
                        Msg::ChangeRegisterPasswordValue(event)
                    })
                ],
                span![C!["icon", "is-small", "is-left"], i![C!["fas", "fa-lock"]]],
            ],
            IF![password!=""=>p![
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
                            button![
                                C!["button ", "is-success", "is-fullwidth",],
                                IF![!tou_accepted=>attrs!(At::from("disabled")=>"")],
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
