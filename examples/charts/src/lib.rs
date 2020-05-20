//! Charts example

mod line;

use seed::{prelude::*, *};

type Point = (f64, f64);

struct Model {
    line_data: Vec<Point>,
}

impl Default for Model {
    fn default() -> Self {
        let end = 6.0 * std::f64::consts::PI;
        let n = 300;
        Self{
            line_data: (0..n).map(|i| {
                let x = f64::from(i) * end / f64::from(n);
                let y = f1(x);
                (x, y)
            }).collect()
        }
    }
}

fn f1(x: f64) -> f64 {
    x.sin()
}

enum Msg {}

fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {}

fn view(model: &Model) -> Node<Msg> {
    div![
        style!{St::Width => "80vh", St::Height => "50vh", St::Margin => "auto"},
        line::chart(&model.line_data),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", |_, _| Default::default(), update, view);
}
