use dioxus::prelude::*;

use crate::{components::{Emoji, SkinTrait}, game::{Card, ColorMode, KATEX_SUITS_FONT_STR, SYMBOLS_2_FONT_STR, Skin, SuitSkin}};

impl Skin {
    fn render_suit_internal(&self, card: &Card, _text_mode: bool) -> Element {
        match *card {
            Card::Suited { rank: _, suit } => {
                match self.suits {
                    SuitSkin::Animals => rsx! {
                        Emoji { 
                            text: self.suits.suit_symbol(suit)
                        }
                    },
                    SuitSkin::Traditional => rsx! {
                        span {
                            font_family: KATEX_SUITS_FONT_STR,
                            {self.suits.suit_symbol(suit)}
                        }
                    },
                }
            }
            Card::Joker => rsx!{},
        }
    }
}

impl SkinTrait<Card> for Skin {
    fn get_color(&self, card: &Card, mode: ColorMode) -> String {
        match *card {
            Card::Suited { rank: _, suit } => self.colors.color(suit, mode).to_string(),
            Card::Joker => self.colors.color_joker(mode).to_string(),
        }
    }

    fn render_rank(&self, card: &Card) -> Element {
        match *card {
            Card::Suited { rank, suit: _ } => rsx! {
                span {
                    font_family: KATEX_SUITS_FONT_STR,
                    {self.ranks.rank_text(rank)}
                }
            },
            Card::Joker => rsx! {
                span {
                    font_family: SYMBOLS_2_FONT_STR,
                    position: "relative",
                    top: "0.13em",
                    "✪"
                }
            }
        }
    }

    fn render_suit(&self, card: &Card) -> Element {
        self.render_suit_internal(card, false)
    }

    // fn render_suit_text(&self, card: &Card) -> Element {
    //     self.render_suit_internal(card, true)
    // }
}