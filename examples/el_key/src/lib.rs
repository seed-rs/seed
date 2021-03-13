//! This example shows how to control a DOM update using element keys and empty nodes.
//! See README.md for more details.

use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use regex::Regex;
use scarlet::{
    color::{Color, RGBColor},
    colors::hslcolor::HSLColor,
};
use seed::{prelude::*, *};
use static_assertions::const_assert;
use std::mem;

type CardId = char;
type NumCards = u8;

// NUM_CARDS have to be in the range [0, 26] (inclusive).
// 26 is the number of letters in the English alphabet.
const NUM_CARDS: NumCards = 12;
const_assert!(NUM_CARDS <= 26);

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::new()
}

// ------ ------
//     Model
// ------ ------

// ------ Model ------

#[derive(Debug)]
struct Model {
    cards: Vec<Card>,
    // VDOM options
    el_key_enabled: bool,
    empty_enabled: bool,
    // Save the README.md content into the model to mitigate rendering slowdown
    // due to Regex replacement and MD conversion.
    readme: Vec<Node<Msg>>,
}

impl Model {
    fn new() -> Self {
        let readme = Regex::new(r"<!-- hidden begin -->[\s\S]*?<!-- hidden end -->")
            .unwrap()
            .replace_all(include_str!("../README.md"), "");

        Self {
            cards: Card::new_cards(),
            el_key_enabled: true,
            empty_enabled: true,
            readme: md!(&readme),
        }
    }
}

// ------ Card ------

#[derive(Debug, Copy, Clone)]
struct Card {
    id: CardId,
    fg_color: RGBColor,
    bg_color: RGBColor,
    enabled: bool,
    selected: bool,
    drag: Option<Drag>,
}

impl Card {
    fn new_cards() -> Vec<Self> {
        (0..NUM_CARDS)
            .map(|card_n| Self {
                id: CardId::from(b'A' + card_n),
                fg_color: Self::fg_color(card_n),
                bg_color: Self::bg_color(card_n),
                enabled: true,
                selected: false,
                drag: None,
            })
            .collect()
    }

    fn bg_color(card_n: NumCards) -> RGBColor {
        let card_hue = f64::from(card_n) / f64::from(NUM_CARDS) * 360.0;
        (HSLColor {
            h: card_hue % 360.0,
            s: 0.98,
            l: 0.81,
        })
        .convert()
    }

    fn fg_color(card_n: NumCards) -> RGBColor {
        let card_hue = f64::from(card_n) / f64::from(NUM_CARDS) * 360.0;
        (HSLColor {
            h: (card_hue + 240.0) % 360.0,
            s: 0.45,
            l: 0.31,
        })
        .convert()
    }
}

// ------ Drag ------

#[derive(Debug, Copy, Clone)]
enum Drag {
    Dragged,
    Over,
}

// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
enum Msg {
    // ------ Selecting ------
    SelectNone,
    ToggleSelected(CardId),
    // ------- Drag and drop ------
    DragStart(CardId),
    DragEnd,
    DragEnter(CardId),
    DragLeave(CardId),
    Drop(CardId, CardId),
    // ------ Control buttons ------
    DisableSelected,
    EnableSelected,
    Reset,
    // Change the order and colors of cards.
    // Applies to selected cards or to all enabled if there are no selected cards.
    RotateCards,
    RotateColors,
    ShuffleCards,
    ShuffleColors,
    // ------ Options ------
    ToggleElKey,
    ToggleEmpty,
}

#[allow(clippy::needless_pass_by_value)]
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    log!("update:", msg);
    match msg {
        // ------ Selecting ------
        Msg::SelectNone => {
            model
                .cards
                .iter_mut()
                .for_each(|card| card.selected = false);
        }
        Msg::ToggleSelected(card_id) => {
            if let Some(card) = model.cards.iter_mut().find(|card| card.id == card_id) {
                card.selected = !card.selected;
            }
        }
        // ------- Drag and drop ------
        Msg::DragStart(card_id) => {
            if let Some(card) = model.cards.iter_mut().find(|card| card.id == card_id) {
                card.drag = Some(Drag::Dragged);
            }
        }
        Msg::DragEnd => {
            for card in &mut model.cards {
                card.drag = None;
            }
        }
        Msg::DragEnter(card_id) => {
            if let Some(card) = model.cards.iter_mut().find(|card| card.id == card_id) {
                card.drag = Some(Drag::Over);
            }
        }
        Msg::DragLeave(card_id) => {
            if let Some(card) = model.cards.iter_mut().find(|card| card.id == card_id) {
                card.drag = None;
            }
        }
        Msg::Drop(from_id, to_id) => {
            swap(&mut model.cards, from_id, to_id);
            orders.skip().send_msg(Msg::DragEnd);
        }
        // ------ Control buttons ------
        Msg::DisableSelected => {
            model.cards.iter_mut().for_each(|card| {
                card.enabled = card.enabled && !card.selected;
            });
        }
        Msg::EnableSelected => {
            model.cards.iter_mut().for_each(|card| {
                card.enabled = card.enabled || card.selected;
            });
        }
        Msg::Reset => model.cards = Card::new_cards(),
        // Change the order and colors of cards.
        // Applies to selected cards or to all enabled if there are no selected cards.
        Msg::RotateCards => {
            let indices = selected_or_all_enabled(&model.cards);
            rotate_by_indices(&mut model.cards, &indices);
        }
        Msg::RotateColors => {
            let indices = selected_or_all_enabled(&model.cards);
            rotate_colors_by_indices(&mut model.cards, &indices);
        }
        Msg::ShuffleCards => {
            let mut indices = selected_or_all_enabled(&model.cards);
            indices.shuffle(&mut SmallRng::from_entropy());
            rotate_by_indices(&mut model.cards, &indices);
        }
        Msg::ShuffleColors => {
            let mut indices = selected_or_all_enabled(&model.cards);
            indices.shuffle(&mut SmallRng::from_entropy());
            rotate_colors_by_indices(&mut model.cards, &indices);
        }
        // ------ Options ------
        Msg::ToggleElKey => model.el_key_enabled = !model.el_key_enabled,
        Msg::ToggleEmpty => model.empty_enabled = !model.empty_enabled,
    };
}

fn selected_or_all_enabled(cards: &[Card]) -> Vec<usize> {
    let selected: Vec<usize> = cards
        .iter()
        .enumerate()
        .filter_map(|(i, card)| IF!(card.selected && card.enabled => i))
        .collect();

    if !selected.is_empty() {
        return selected;
    }

    cards
        .iter()
        .enumerate()
        .filter_map(|(i, card)| IF!(card.enabled => i))
        .collect()
}

fn rotate_by_indices<T>(arr: &mut [T], indices: &[usize]) {
    indices
        .iter()
        .rev()
        .skip(1)
        .zip(indices.iter().rev())
        .for_each(|(&a, &b)| arr.swap(a, b));
}

fn rotate_colors_by_indices(cards: &mut [Card], indices: &[usize]) {
    if let Some(&last_i) = indices.last() {
        let mut bg = cards[last_i].bg_color;
        let mut fg = cards[last_i].fg_color;
        for &i in indices {
            let card = &mut cards[i];
            mem::swap(&mut bg, &mut card.bg_color);
            mem::swap(&mut fg, &mut card.fg_color);
        }
    }
}

fn swap(cards: &mut [Card], a_id: CardId, b_id: CardId) {
    if let (Some(a_index), Some(b_index)) = (card_index(cards, a_id), card_index(cards, b_id)) {
        cards.swap(a_index, b_index);
    }
}

fn card_index(cards: &[Card], card_id: CardId) -> Option<usize> {
    cards
        .iter()
        .enumerate()
        .find_map(|(i, card)| IF!(card.id == card_id => i))
}

// ------ ------
//     View
// ------ ------

// Note: It's macro so you can use it with all events.
macro_rules! stop_and_prevent {
    { $event:expr } => {
        {
            $event.stop_propagation();
            $event.prevent_default();
        }
     };
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        id!["content"],
        h1![id!["title"], "Element key example"],
        card_table(model),
        control_buttons(),
        options(model),
        readme(&model.readme),
    ]
}

fn card_table(model: &Model) -> Node<Msg> {
    section![
        id!["card-table"],
        div![
            id!["card-table__grid--enabled"],
            model.cards.iter().filter_map(|card| {
                if card.enabled {
                    Some(enabled_card(card, model.el_key_enabled))
                } else if model.empty_enabled {
                    Some(empty![])
                } else {
                    None
                }
            })
        ],
        div![
            id!["card-table__grid--disabled"],
            model.cards.iter().filter_map(|card| {
                if !card.enabled {
                    Some(disabled_card(card, model.el_key_enabled))
                } else if model.empty_enabled {
                    Some(empty![])
                } else {
                    None
                }
            })
        ]
    ]
}

fn disabled_card(card: &Card, el_key_enabled: bool) -> Node<Msg> {
    let card_id = card.id;
    div![
        id!(format!("card-table__card-{}", card.id)),
        IF!(el_key_enabled => el_key(&card_id)),
        C![
            "card-table__card",
            "card-table__card--disabled",
            IF!(card.selected => "card-table__card--selected"),
        ],
        style! {
            St::Color => card.fg_color.to_string(),
        },
        ev(Ev::Click, move |_| Msg::ToggleSelected(card_id)),
        format!("{}", card.id),
    ]
}

fn enabled_card(card: &Card, el_key_enabled: bool) -> Node<Msg> {
    div![
        id!(format!("card-table__card-{}", card.id)),
        IF!(el_key_enabled => el_key(&card.id)),
        C![
            "card-table__card",
            "card-table__card--enabled",
            card.drag.map(|drag| match drag {
                Drag::Dragged => "card-table__card--dragged",
                Drag::Over => "card-table__card--dragover",
            }),
            IF!(card.selected => "card-table__card--selected"),
        ],
        attrs! {
            At::Draggable => true
        },
        enabled_card_graphics(card),
        enabled_card_event_handlers(card),
    ]
}

fn enabled_card_graphics(card: &Card) -> Node<Msg> {
    svg![
        rect![
            C!("card-table__card-background"),
            attrs! {
                At::Fill => card.bg_color.to_string(),
            },
        ],
        circle![
            C!("card-table__card-spinner"),
            attrs! {
                At::Stroke => card.fg_color.to_string(),
            }
        ],
        text![
            C!("card-table__card-text"),
            attrs! {
                // SVG Text's `x` and `y` is not CSS properties.
                At::X => percent(50),
                At::Y => percent(if card.id == 'J' { 50 } else { 55 }),
                At::Fill => card.fg_color.to_string(),
            },
            card.id.to_string(),
        ],
    ]
}

fn enabled_card_event_handlers(card: &Card) -> Vec<EventHandler<Msg>> {
    let card_id = card.id;
    vec![
        ev(Ev::Click, move |_| Msg::ToggleSelected(card_id)),
        drag_ev(Ev::DragStart, move |event| {
            event
                .data_transfer()
                .map(|dt| dt.set_data("card_id", &card_id.to_string()));
            Msg::DragStart(card_id)
        }),
        drag_ev(Ev::DragEnter, move |event| {
            stop_and_prevent!(event);
            event.data_transfer().unwrap().set_drop_effect("move");
            Msg::DragEnter(card_id)
        }),
        drag_ev(Ev::DragLeave, move |event| {
            stop_and_prevent!(event);
            event.data_transfer().unwrap().set_drop_effect("move");
            Msg::DragLeave(card_id)
        }),
        drag_ev(Ev::DragOver, |event| -> Option<Msg> {
            stop_and_prevent!(event);
            event.data_transfer().unwrap().set_drop_effect("move");
            None
        }),
        drag_ev(Ev::Drop, move |event| {
            stop_and_prevent!(event);
            event
                .data_transfer()
                .unwrap()
                .get_data("card_id")
                .ok()
                .and_then(|d| d.parse().ok())
                .map(|src_id| Msg::Drop(src_id, card_id))
        }),
        drag_ev(Ev::DragEnd, |_| Msg::DragEnd),
    ]
}

fn control_buttons() -> Node<Msg> {
    section![
        id!["control-buttons"],
        div![
            id!["control-buttons__order-buttons"],
            div!["Cards:"],
            button!["Rotate", ev(Ev::Click, |_| Msg::RotateCards)],
            button!["Shuffle", ev(Ev::Click, |_| Msg::ShuffleCards)],
            div!["Colors:"],
            button!["Rotate", ev(Ev::Click, |_| Msg::RotateColors)],
            button!["Shuffle", ev(Ev::Click, |_| Msg::ShuffleColors)],
        ],
        div![
            id!("control-buttons__buttons"),
            button!["Disable", ev(Ev::Click, |_| Msg::DisableSelected)],
            button!["Enable", ev(Ev::Click, |_| Msg::EnableSelected)],
            button!["Select none", ev(Ev::Click, |_| Msg::SelectNone)],
            button!["Reset", ev(Ev::Click, |_| Msg::Reset)],
        ],
    ]
}

fn options(model: &Model) -> Node<Msg> {
    section![
        id!["options"],
        div![
            input![attrs! {
                At::Type => "checkbox",
                At::Name => "use-el-key",
                At::Checked => model.el_key_enabled.as_at_value(),
            },],
            label![
                attrs! {
                    At::For => "use-el-key"
                },
                "Use element keys",
            ],
            ev(Ev::Click, |_| Msg::ToggleElKey),
        ],
        div![
            input![attrs! {
                At::Type => "checkbox",
                At::Name => "use-empty",
                At::Checked => model.empty_enabled.as_at_value(),
            },],
            label![
                attrs! {
                    At::For => "use-empty"
                },
                "Use empty nodes",
            ],
            ev(Ev::Click, |_| Msg::ToggleEmpty),
        ],
    ]
}

fn readme(readme: &[Node<Msg>]) -> Node<Msg> {
    section![
        id!("readme"),
        details![attrs! {At::Open => true}, summary!["Readme"], readme]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
