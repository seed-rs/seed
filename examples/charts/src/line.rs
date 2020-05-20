use seed::{prelude::*, *};
use itertools::Itertools;

pub fn chart<T: 'static>(data: &[(f64, f64)]) -> Node<T> {
    // domain
    let x_bounds = data
        .iter()
        .map(|(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();

    let y_bounds = data
        .iter()
        .map(|(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    let n = data.len() as f64;
    let dx = (x_bounds.1 - x_bounds.0) / n; // step between points

    svg![
        style!{
            St::Border => "1px solid #ccc",
        },

        attrs! {
            At::Width => "100%",
            At::Height => "100%",
            At::ViewBox => format!("{} {} {} {}",
                                   x_bounds.0 / dx,
                                   y_bounds.0 / dx,

                                   (x_bounds.1 - x_bounds.0) / dx,
                                   (y_bounds.1 - y_bounds.0) / dx,
            ),
        },
        // rect![attrs! {At::Width => "100%", At::Height => "100%", At::Fill => "rgba(0, 0, 0, 0.1)"},],

        polyline![attrs! {
            At::Fill => "none",
            At::Stroke => "rgb(0, 86, 91)",
            At::Points => data
                .iter()
                .map(|(x, y)| format!("{},{}", x  / dx, y / dx))
                .collect::<Vec<_>>()
                .join(" ")
        }],
    ]
}
