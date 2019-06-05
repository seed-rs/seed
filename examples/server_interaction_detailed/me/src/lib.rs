#[macro_use]
extern crate seed;
use seed::prelude::*;

use futures::Future;
use serde::Deserialize;

use shared::interfaces;
use shared::interfaces::{Gradesheet, Line, Mission, Person, Syllabus, Upgrade, UpgradeEvent};
use shared::util;

#[wasm_bindgen]
pub fn render() {
    let state = seed::App::build(Model::default(), update, view)
        .finish()
        .run();

    state.update(Msg::GetData)
}
