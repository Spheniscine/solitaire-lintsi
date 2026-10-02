use dioxus::prelude::*;

use crate::{components::{VIDEO_GAMEPLAY, rem}, game::{DECK_SIZE, GameState, NUM_JOKERS, NUM_RANKS, NUM_SUITS, ScreenState,}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

#[component]
pub fn Help(mut game_state: Signal<GameState>) -> Element {
    // let st = game_state.read();
    // let skin = st.skin;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 4.4rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The deck is a standard {DECK_SIZE}-card deck, with {NUM_RANKS} ranks and {NUM_SUITS} suits. 
                    There are also {NUM_JOKERS} jokers, which start in the " Emph {"joker pile"} "."
                }

                p {
                    "Cards stack in the " Emph {"tableau"} " by " Emph {"decrementing ranks"} ", but may only be moved as
                    a group if they are also all the " Emph {"same suit"} "."
                }

                p {
                    Emph {"Jokers"} " may be placed on any card, and any card stack may be placed on a joker. An exposed joker may be
                    moved back to the joker pile."
                }

                p {
                    "To ",Emph{"win the game"},", stack all suited cards to the " Emph{"foundations"} " in incrementing order by suit."
                }

                p {
                    Emph{"Shortcut notes:"},

                    ul {
                        li {
                            Emph {"Double-clicking"}," a suited card will send it to the foundations if possible.
                            Double-clicking a joker will send it to the joker pile."
                        }

                        li {
                            "A selected stack of cards may be sent to the foundation in one move, if the cards would fit in when moved
                            one by one."
                        }
                    }
                }

                div {
                    position: "absolute",
                    bottom: rem(2.),
                    width: "92rem",
                    display: "flex",
                    justify_content: "center",

                    a {
                        href: VIDEO_GAMEPLAY,
                        target: "_blank",
                        text_decoration: "none",
                        margin_right: rem(4.),
                        div {
                            width: rem(30.),
                            position: "relative",
                            class: "game-button",
                            "Example video"
                        }
                    }

                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                        "Back to game"
                    }
                }
            }
        }
    }
}