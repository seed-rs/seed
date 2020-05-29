use fluent::fluent_args;
use seed::{prelude::*, *};
use strum::IntoEnumIterator;

mod i18n;
use i18n::{I18n, Lang};

const DEFAULT_LANG: Lang = Lang::EnUS;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        i18n: I18n::new(DEFAULT_LANG),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    i18n: I18n,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    LangChanged(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::LangChanged(lang) => {
            model
                .i18n
                .set_lang(lang.parse().expect("supported language"));
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    let args_male_sg = fluent_args![
      "userName" => "Stephan",
      "photoCount" => 1,
      "userGender" => "male",
      "tabCount" => 1,
      "formal" => "true"
    ];

    let args_female_pl = fluent_args![
      "userName" => "Anna",
      "photoCount" => 5,
      "userGender" => "female",
      "tabCount" => 7,
      "formal" => "false"
    ];

    // Macro from the module `i18n`. It allows us to call `t!(..)` - see the code below.
    create_t!(model.i18n);

    div![
        div![select![
            attrs! {At::Name => "lang"},
            Lang::iter().map(|lang| option![attrs! {At::Value => lang}, lang.label()]),
            input_ev(Ev::Change, Msg::LangChanged),
        ],],
        div![p!["Language in Model: ", model.i18n.lang().label()]],
        div![],
        div![
            p![t!("hello-world")],
            p![t!("hello-user", args_male_sg)],
            p![t!("shared-photos", args_male_sg)],
            p![t!("tabs-close-button")],
            p![t!("tabs-close-tooltip", args_male_sg)],
            p![t!("tabs-close-warning", args_male_sg)],
            p![t!("hello-user", args_female_pl)],
            p![t!("shared-photos", args_female_pl)],
            p![t!("tabs-close-button")],
            p![t!("tabs-close-tooltip", args_female_pl)],
            p![t!("tabs-close-warning", args_female_pl)],
            p![t!("sync-dialog-title")],
            p![t!("sync-headline-title")],
            p![t!("sync-signedout-title")],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
