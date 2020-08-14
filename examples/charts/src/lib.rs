//! Charts example

mod line;

use seed::{prelude::*, *};

type Point = (f64, f64);

struct Model {
    line_data: Vec<Point>,
    tooltip: Option<line::Tooltip>,
}

impl Default for Model {
    fn default() -> Self {
        let end = 2.0 * std::f64::consts::PI;
        let n = 300;
        Self {
            line_data: (0..=n)
                .map(|i| {
                    let x = f64::from(i) * end / f64::from(n);
                    let y = f1(x);
                    (x, y)
                })
                .collect(),
            tooltip: None,
        }
    }
}

fn f1(x: f64) -> f64 {
    x.sin() + 1.0
}

#[derive(Clone)]
enum Msg {
    ShowTooltip(line::Tooltip),
    HideTooltip,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ShowTooltip(tooltip) => {
            model.tooltip = Some(tooltip);
        }
        Msg::HideTooltip => {
            model.tooltip = None;
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        style! {St::Width => vh(80), St::Height => vh(50), St::Margin => "auto"},
        div![
            style! {
                St::Position => "relative",
                St::Border => "1px solid #ddd",
            },
            line::chart(&model.line_data, Msg::ShowTooltip, Msg::HideTooltip),
            model.tooltip.as_ref().map(|tooltip| div![
                style! {
                    St::PointerEvents => "none",
                    St::Position => "fixed",
                    St::Left => px(tooltip.position.0),
                    St::Top => px(tooltip.position.1 - 50),
                    St::Background => "#eee",
                    St::Border => "1px solid #ccc",
                    St::BoxShadow => "2px 2px 4px rgba(0, 0, 0, 0.2)",
                    St::Padding => px(10),
                    St::BorderRadius => px(3),
                },
                format!("Point ({:.2}, {:.2}) ", tooltip.data.0, tooltip.data.1),
            ])
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", |_, _| Model::default(), update, view);
}
