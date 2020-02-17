//! This example shows how to control a DOM update using element keys and empty nodes.
//! See README.md for more details.

// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value)]
use enclose::enc;
use futures::prelude::*;
use rand::prelude::*;
use scarlet::{
    color::{Color, RGBColor},
    colors::hslcolor::HSLColor,
};
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;
use web_sys::{self, DragEvent, Event, MouseEvent};

// NUM_CARDS should be in the range [0, 26] (inclusive).
// 26 because it is the number of letters in the English alphabet.
const NUM_CARDS: usize = 12;

// ------ ------
//     Model
// ------ ------

type CardId = char;
type RngSeed = u64;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
struct Card {
    id: CardId,
    fg_color: RGBColor,
    bg_color: RGBColor,
    enabled: bool,
    dragged: bool,
    dragover: bool,
    selected: bool,
}

impl Card {
    fn new(card_n: usize, total_cards: usize) -> Self {
        Self {
            id: card_id(card_n),
            fg_color: card_fg_color(card_n, total_cards),
            bg_color: card_bg_color(card_n, total_cards),
            enabled: true,
            dragged: false,
            dragover: false,
            selected: false,
        }
    }
}

#[derive(Debug)]
struct Model {
    cards: Vec<Card>,
    readme: Option<String>,
    // VDOM options
    el_key_enabled: bool,
    empty_enabled: bool,
}

impl Model {
    fn new(n_cards: usize) -> Self {
        assert!(n_cards <= 26);
        Self {
            cards: (0..n_cards).map(|n| Card::new(n, n_cards)).collect(),
            readme: None,
            el_key_enabled: true,
            empty_enabled: true,
        }
    }

    // Swap cards by drag and drop
    fn drag_start(&mut self, card_id: CardId) {
        if let Some(card) = self.cards.iter_mut().find(|card| card.id == card_id) {
            card.dragged = true;
        }
    }
    fn drag_enter(&mut self, card_id: CardId) {
        if let Some(card) = self.cards.iter_mut().find(|card| card.id == card_id) {
            card.dragover = true;
        }
    }
    fn drag_leave(&mut self, card_id: CardId) {
        if let Some(card) = self.cards.iter_mut().find(|card| card.id == card_id) {
            card.dragover = false;
        }
    }
    fn drag_end(&mut self) {
        for card in &mut self.cards {
            card.dragged = false;
            card.dragover = false;
        }
    }
    fn swap(&mut self, a_id: CardId, b_id: CardId) {
        if let (Some(a_index), Some(b_index)) =
            (card_index(&self.cards, a_id), card_index(&self.cards, b_id))
        {
            self.cards.swap(a_index, b_index);
        }
    }

    // Selecting
    fn select_none(&mut self) {
        self.cards.iter_mut().for_each(|card| card.selected = false);
    }
    fn toggle_selected(&mut self, card_id: CardId) {
        if let Some(card) = self.cards.iter_mut().find(|card| card.id == card_id) {
            card.selected = !card.selected;
        }
    }

    // Change the order and colors of cards.
    // Applies to selected cards or to all enabled if there are no selected cards.
    fn rotate_cards(&mut self) {
        let indeces = selected_or_all_enabled(&self.cards);
        rotate_by_indeces(&mut self.cards, &indeces);
    }
    fn shuffle_cards(&mut self, seed: RngSeed) {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
        let mut indeces = selected_or_all_enabled(&self.cards);
        indeces.shuffle(&mut rng);
        rotate_by_indeces(&mut self.cards, &indeces);
    }
    fn rotate_colors(&mut self) {
        let indeces = selected_or_all_enabled(&self.cards);
        rotate_colors_by_indeces(&mut self.cards, &indeces);
    }
    fn shuffle_colors(&mut self, seed: RngSeed) {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
        let mut indeces = selected_or_all_enabled(&self.cards);
        indeces.shuffle(&mut rng);
        rotate_colors_by_indeces(&mut self.cards, &indeces);
    }
    // Disable/Enable cards
    fn disable_selected(&mut self) {
        self.cards.iter_mut().for_each(|card| {
            card.enabled = card.enabled && !card.selected;
        });
    }
    fn enable_selected(&mut self) {
        self.cards.iter_mut().for_each(|card| {
            card.enabled = card.enabled || card.selected;
        });
    }

    // Reset card order and colors
    fn reset(&mut self) {
        let total_cards = self.cards.len();
        self.cards.sort_by_key(|card| card.id);
        self.cards.iter_mut().enumerate().for_each(|(n, card)| {
            card.fg_color = card_fg_color(n, total_cards);
            card.bg_color = card_bg_color(n, total_cards);
            card.enabled = true;
            card.selected = false;
        });
    }

    // Toggle VDOM options
    fn toggle_el_key(&mut self) {
        self.el_key_enabled = !self.el_key_enabled;
    }
    fn toggle_empty(&mut self) {
        self.empty_enabled = !self.empty_enabled;
    }
}

fn selected_or_all_enabled(cards: &[Card]) -> Vec<usize> {
    let selected: Vec<usize> = cards
        .iter()
        .enumerate()
        .filter_map(|(i, card)| {
            if card.selected && card.enabled {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    if selected.is_empty() {
        cards
            .iter()
            .enumerate()
            .filter_map(|(i, card)| if card.enabled { Some(i) } else { None })
            .collect()
    } else {
        selected
    }
}

fn rotate_by_indeces<T>(arr: &mut [T], indeces: &[usize]) {
    indeces
        .iter()
        .rev()
        .skip(1)
        .zip(indeces.iter().rev())
        .for_each(|(&a, &b)| arr.swap(a, b));
}

fn rotate_colors_by_indeces(cards: &mut [Card], indeces: &[usize]) {
    if let Some(&last_i) = indeces.last() {
        let mut bg = cards[last_i].bg_color;
        let mut fg = cards[last_i].fg_color;
        for &i in indeces {
            let card = &mut cards[i];
            std::mem::swap(&mut bg, &mut card.bg_color);
            std::mem::swap(&mut fg, &mut card.fg_color);
        }
    }
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn card_bg_color(card_n: usize, total_cards: usize) -> RGBColor {
    let card_hue = card_n as f64 / total_cards as f64 * 360.0;
    (HSLColor {
        h: card_hue % 360.0,
        s: 0.98,
        l: 0.81,
    })
    .convert()
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn card_fg_color(card_n: usize, total_cards: usize) -> RGBColor {
    let card_hue = card_n as f64 / total_cards as f64 * 360.0;
    (HSLColor {
        h: (card_hue + 240.0) % 360.0,
        s: 0.45,
        l: 0.31,
    })
    .convert()
}

#[allow(clippy::cast_possible_truncation)]
fn card_id(card_n: usize) -> char {
    assert!(card_n < 26);
    (b'A' + card_n as u8) as char
}

fn card_index(cards: &[Card], card_id: CardId) -> Option<usize> {
    cards
        .iter()
        .enumerate()
        .find_map(|(i, card)| if card.id == card_id { Some(i) } else { None })
}

// ------ ------
//     Init
// ------ ------

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.send_msg(Msg::FetchReadme);
    Model::new(NUM_CARDS)
}

// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
enum Msg {
    // Readme
    FetchReadme,
    ReadmeReceived(String),
    // Selecting
    SelectNone,
    ToggleSelected(CardId),
    // Drag and drop
    DragStart(CardId),
    DragEnd,
    DragEnter(CardId),
    DragLeave(CardId),
    Drop(CardId, CardId),
    // Control buttons
    DisableSelected,
    EnableSelected,
    Reset,
    RotateCards,
    RotateColors,
    ShuffleCards(RngSeed),
    ShuffleColors(RngSeed),
    // Options
    ToggleElKey,
    ToggleEmpty,
}

fn fetch_readme() -> impl Future<Output = Msg> {
    async {
        fetch("README.md")
            .await?
            .check_status()?
            .text()
            .await
            .map(|readme| {
                // Remove sections mrked as hidden.
                readme
                    .split("<!-- hidden begin -->")
                    .map(|txt| txt.split("<!-- hidden end -->").nth(1).unwrap_or(txt))
                    .fold(String::new(), |mut clean, part| {
                        clean.push_str(part);
                        clean
                    })
            })
    }
    .map(|result| Msg::ReadmeReceived(result.unwrap_or_else(|err| format!("{:?}", err))))
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    log!("update:", msg);
    match msg {
        // Readme
        Msg::FetchReadme => {
            orders.skip();
            orders.perform_cmd(fetch_readme());
        }
        Msg::ReadmeReceived(readme) => {
            model.readme.replace(readme);
        }
        // Selecting
        Msg::SelectNone => model.select_none(),
        Msg::ToggleSelected(card_id) => model.toggle_selected(card_id),
        // Drag and drop
        Msg::DragStart(card_id) => model.drag_start(card_id),
        Msg::DragEnd => model.drag_end(),
        Msg::DragEnter(card_id) => model.drag_enter(card_id),
        Msg::DragLeave(card_id) => model.drag_leave(card_id),
        Msg::Drop(from_id, to_id) => {
            model.drag_end();
            model.swap(from_id, to_id);
        }
        // Control buttons
        Msg::DisableSelected => model.disable_selected(),
        Msg::EnableSelected => model.enable_selected(),
        Msg::Reset => model.reset(),
        Msg::RotateCards => model.rotate_cards(),
        Msg::RotateColors => model.rotate_colors(),
        Msg::ShuffleCards(seed) => model.shuffle_cards(seed),
        Msg::ShuffleColors(seed) => model.shuffle_colors(seed),
        // Options
        Msg::ToggleElKey => model.toggle_el_key(),
        Msg::ToggleEmpty => model.toggle_empty(),
    };
}

// ------ ------
//     View
// ------ ------

trait IntoDragEvent {
    fn into_drag_event(self) -> DragEvent;
}

impl IntoDragEvent for Event {
    fn into_drag_event(self) -> DragEvent {
        self.dyn_into::<web_sys::DragEvent>()
            .expect("cannot cast given event into DragEvent")
    }
}

trait IntoMouseEvent {
    fn into_mouse_event(self) -> MouseEvent;
}

impl IntoMouseEvent for Event {
    fn into_mouse_event(self) -> MouseEvent {
        self.dyn_into::<web_sys::MouseEvent>()
            .expect("cannot cast given event into MouseEvent")
    }
}

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
    // log!(model);
    div![
        id!["content"],
        h1![id!["title"], "Element key example"],
        card_table(model),
        control_buttons(),
        options(model),
        readme(model),
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
                    Some(empty())
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
                    Some(empty())
                } else {
                    None
                }
            })
        ]
    ]
}

#[allow(clippy::cognitive_complexity)]
fn enabled_card(card: &Card, el_key_enabled: bool) -> Node<Msg> {
    div![
        id!(format!("card-table__card-{}", card.id)),
        IF!(el_key_enabled => el_key(&format!("{}", card.id))),
        C![
            "card-table__card",
            "card-table__card--enabled",
            IF!(card.dragged => "card-table__card--dragged"),
            IF!(card.dragover => "card-table__card--dragover"),
            IF!(card.selected => "card-table__card--selected"),
        ],
        attrs! {
            At::Draggable => true
        },
        // Card graphics
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
                format!("{}", card.id),
            ],
        ],
        // Events
        ev(
            Ev::Click,
            enc!((card.id => card_id) move |_| Msg::ToggleSelected(card_id))
        ),
        ev(
            Ev::DragStart,
            enc!(
                (card.id => card_id) move | event | {
                    event
                        .into_drag_event()
                        .data_transfer()
                        .map(|dt| dt.set_data("card_id", format!("{}", card_id).as_str()));
                    Msg::DragStart(card_id)
                }
            )
        ),
        ev(
            Ev::DragEnter,
            enc!(
                (card.id => card_id) move | event | {
                    let drag_event = event.into_drag_event();
                    stop_and_prevent!(drag_event);
                    drag_event.data_transfer().unwrap().set_drop_effect("move");
                    Msg::DragEnter(card_id)
                }
            ),
        ),
        ev(
            Ev::DragLeave,
            enc!(
                (card.id => card_id) move | event | {
                    let drag_event = event.into_drag_event();
                    stop_and_prevent!(drag_event);
                    drag_event.data_transfer().unwrap().set_drop_effect("move");
                    Msg::DragLeave(card_id)
                }
            ),
        ),
        ev(Ev::DragOver, |event| -> Option<Msg> {
            let drag_event = event.into_drag_event();
            stop_and_prevent!(drag_event);
            drag_event.data_transfer().unwrap().set_drop_effect("move");
            None
        }),
        ev(
            Ev::Drop,
            enc!(
                (card.id => card_id) move | event | {
                    let drag_event = event.into_drag_event();
                    stop_and_prevent!(drag_event);
                    drag_event
                        .data_transfer()
                        .unwrap()
                        .get_data("card_id")
                        .ok()
                        .map(|d| d.parse().ok())
                        .flatten()
                        .map(|src_id| Msg::Drop(src_id, card_id))
                }
            )
        ),
        ev(Ev::DragEnd, |_| Msg::DragEnd),
    ]
}

fn disabled_card(card: &Card, el_key_enabled: bool) -> Node<Msg> {
    let card_id = card.id;
    div![
        id!(format!("card-table__card-{}", card.id)),
        IF!(el_key_enabled => el_key(&format!("{}", card_id))),
        C![
            "card-table__card",
            "card-table__card--disabled",
            IF!(card.selected => "card-table__card--selected"),
        ],
        style! {
            St::Color => card.fg_color.to_string(),
        },
        ev(
            Ev::Click,
            enc!((card_id) move |_| Msg::ToggleSelected(card_id))
        ),
        format!("{}", card.id),
    ]
}

fn control_buttons() -> Node<Msg> {
    section![
        id!["control-buttons"],
        div![
            id!["control-buttons__order-buttons"],
            div!["Cards:"],
            button!["Rotate", ev(Ev::Click, |_| Msg::RotateCards)],
            button![
                "Shuffle",
                input_ev(Ev::Click, |_| Msg::ShuffleCards(random()))
            ],
            div!["Colors:"],
            button!["Rotate", ev(Ev::Click, |_| Msg::RotateColors)],
            button![
                "Shuffle",
                input_ev(Ev::Click, |_| Msg::ShuffleColors(random()))
            ],
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
            input![
                attrs! {
                    At::Type => "checkbox",
                    At::Name => "use-el-key",
                    At::Checked => model.el_key_enabled.as_at_value(),
                },
                ev(Ev::Input, |_| Msg::ToggleElKey),
            ],
            label![
                attrs! {
                    At::For => "use-el-key"
                },
                "Use element keys",
            ]
        ],
        div![
            input![
                attrs! {
                    At::Type => "checkbox",
                    At::Name => "use-empty",
                    At::Checked => model.empty_enabled.as_at_value(),
                },
                ev(Ev::Input, |_| Msg::ToggleEmpty),
            ],
            label![
                attrs! {
                    At::For => "use-empty"
                },
                "Use empty nodes",
            ]
        ],
    ]
}

fn readme(model: &Model) -> Node<Msg> {
    section![
        id!("readme"),
        details![
            attrs! {At::Open => true},
            summary!["Readme"],
            md!(model.readme.as_deref().unwrap_or_else(|| "Loading..."))
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
