use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::rem, game::{Card, ColorMode, Skin, Suit}};

pub trait SkinTrait<C>: PartialEq + Clone {
    fn get_color(&self, card: &C, mode: ColorMode) -> String;
    fn render_rank(&self, card: &C) -> Element;
    fn render_suit(&self, card: &C) -> Element;
    // fn render_suit_text(&self, card: &C) -> Element;
}

pub const BASE_CARD_WIDTH: f32 = 12.;
pub const BASE_CARD_HEIGHT: f32 = 13.;
pub const CARD_HEIGHT_RATIO: f32 = BASE_CARD_HEIGHT / BASE_CARD_WIDTH;
pub const CARD_BORDER_RADIUS_RATIO: f32 = 1. / BASE_CARD_WIDTH;

#[component]
pub fn BaseCardComponent<C: PartialEq + Clone + 'static, S: SkinTrait<C> + 'static>(
    position: Vec2,
    width: f32,
    card: C,
    skin: S,

    // number_hint: Option<usize>,
    #[props(default = ColorMode::Light)]
    color_mode: ColorMode,

    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    #[props(default)]
    ondoubleclick: EventHandler<MouseEvent>,
) -> Element {
    let pt = width / BASE_CARD_WIDTH;
    let pt = |x: f32| {
        rem(x * pt)
    };

    rsx! {
        div {
            style: "place-items: center;",
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),
            background_color: color_mode.choose("#fff", "#101"),
            width: pt(BASE_CARD_WIDTH - 1.),
            height: pt(BASE_CARD_HEIGHT - 1.),
            border: "{pt(0.25)} solid",
            border_color: color_mode.choose("#000", "#868"),
            border_radius: rem(width * CARD_BORDER_RADIUS_RATIO),
            display: "grid",
            grid_template_columns: "50% 50%",
            grid_template_rows: "50% 50%",
            font_size: pt(3.4),
            text_align: "center",
            padding: pt(0.25),
            color: skin.get_color(&card, !color_mode),

            onclick, ondoubleclick,

            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_rank(&card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_suit(&card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_suit(&card)}},
            div { display: "flex", align_items: "center", pointer_events: "none", {skin.render_rank(&card)}},
        }
    }
}

pub const CARD_FRAME_DEFAULT_COLOR: &str = "#aaa";

#[component]
pub fn CardFrame(
    position: Vec2,
    width: f32,
    hint: Option<Element>,
    #[props(default = CARD_FRAME_DEFAULT_COLOR.to_string())] 
    color: String,
    onclick: EventHandler<MouseEvent>,
    // oncontextmenu: EventHandler<MouseEvent>,
    #[props(default)] dashed: bool,

    number_hint: Option<i32>,
) -> Element {
    let color = color.as_str();
    let pt = width / BASE_CARD_WIDTH;
    let pt = |x: f32| {
        rem(x * pt)
    };
    let border_style = if dashed {"dashed"} else {"solid"};
    rsx! {
        div {
            display: "flex",
            align_items: "center",
            justify_content: "center",
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),
            margin: pt(0.25), // frame must be slightly smaller than card to prevent peeking out in some platforms
            width: pt(BASE_CARD_WIDTH - 2.),
            height: pt(BASE_CARD_HEIGHT - 2.),
            border: "{pt(0.5)} {border_style} {color}",
            text_align: "center",
            color,
            border_radius: rem(width * CARD_BORDER_RADIUS_RATIO),
            font_size: pt(3.4),
            padding: pt(0.25),
            onclick, 
            //oncontextmenu,

            if let Some(hint) = hint {
                div {
                    {hint},
                }
            }
        }
    }
}

#[component]
pub fn CardComponent(
    position: Vec2,
    width: f32,
    card: Card,
    skin: Skin,

    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    #[props(default)]
    ondoubleclick: EventHandler<MouseEvent>,
    #[props(default)]
    oncontextmenu: EventHandler<MouseEvent>,
) -> Element {
    let color_mode = if matches!(card, Card::Joker) {ColorMode::Dark} else {ColorMode::Light};
    rsx! {
        BaseCardComponent { 
            position, width, 
            card, 
            skin,
            color_mode,
            onclick,
            ondoubleclick,
        }
    }
}