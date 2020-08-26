use seed::{prelude::*, *};

pub fn view<Ms>() -> Vec<Node<Ms>> {
    vec![div![C!["container", "contentainer"], view_content(),]]
}

fn view_content<Ms>() -> Vec<Node<Ms>> {
    vec![
        p![
            C!["title", "is-3"],
            "What is lol:Hub?",
        ],
        p![
            C!["is-size-5", "is-paragraph"],
            "lol:Hub is a social platform for the well known MOBA game League of Legends. The goal of this social platform is to bring people together, that want to play casual fun game modes that required everyone to follow certain rules like only picking certain factions.\n"
        ],
        p![
            C!["title", "is-3"],
            "About the authors"
        ],
        p![
            C!["is-size-5", "is-paragraph"],
            "We are two computer science students from Germany. We have been playing League of Legends since Season 3 and wanted to bring back custom game modes like 'Find The Teemo' and other fun modes. Since it is hard to play these modes in normal games and finding custom game lobbies is not really that much easier, we decided to develop a platform on which players that want to enjoy a round of 'Protect the Soraka Bot' can find teammates to play it with.",
        ],
        p![
            C!["title", "is-3"],
            "Bug Reports"
        ],
        p![
            C!["is-size-5", "is-paragraph"],
            "You can report any bugs you encounter by opening an issue on our Github page."
        ]
    ]
}

