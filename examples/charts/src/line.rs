use itertools::Itertools;
use seed::{prelude::*, *};

#[derive(Copy, Clone)]
pub struct Tooltip {
    pub position: (i32, i32),
    pub data: (f64, f64),
}

#[allow(clippy::too_many_lines)]
pub fn chart<Ms: Clone + 'static>(
    data: &[(f64, f64)],
    on_enter: impl Fn(Tooltip) -> Ms + Clone + 'static,
    on_out: Ms,
) -> Node<Ms> {
    #![allow(clippy::enum_glob_use)]
    use At::*;
    // domain
    let x_bounds = data.iter().map(|(x, _)| x).minmax().into_option().unwrap();

    let padding = 30;
    let width = 600;
    let height = 300;

    let dx = (x_bounds.1 - x_bounds.0) / (f64::from(width) - f64::from(2 * padding)); // scale factor
    svg![
        C!["chart"],
        style! { St::Display => "block" },
        attrs! {
            ViewBox => format!("0 0 {} {}", width, height),
        },
        g![
            C!["x-axis"],
            line_![attrs! {
                X1 => padding,
                X2 => width - padding,
                Y1 => height - padding,
                Y2 => height - padding,
                Stroke => "#ccc",
            }],
            g![
                C!["x-ticks"],
                (1..25).map(|i| rect![attrs! {
                    X => (f64::from(i) * 0.25) / dx + f64::from(padding) - 0.5,
                    Y => height - padding,
                    Width => 1,
                    Height => if i % 2 == 0 { 3 } else { 2 },
                    Fill => if i % 2 == 0 { "#999" } else { "#bbb" },
                }]),
            ],
            g![
                C!["x-labels"],
                (0..13).map(|i| text![
                    style! {St::FontSize => px(10)},
                    attrs! {
                        X => (f64::from(i) * 0.5) / dx + f64::from(padding) - 0.5,
                        Y => height - padding + 20,
                        Fill => "#999",
                        TextAnchor => "middle",
                    },
                    format!("{:.1}", (f64::from(i) * 0.5)),
                ]),
            ],
        ],
        g![
            C!["y-axis"],
            line_![attrs! {
                X1 => padding,
                X2 => padding,
                Y1 => padding,
                Y2 => height - padding,
                Stroke => "#ccc",
            }],
            g![
                C!["y-ticks"],
                (1..11).map(|i| rect![attrs! {
                    X => padding - if i % 2 == 0 { 3 } else { 2 },
                    Y => f64::from(height - padding) - (f64::from(i) * 0.25) / dx - 0.5, // -0.5 to center rect
                    Width => if i % 2 == 0 { 3 } else { 2 },
                    Height => 1,
                    Fill => if i % 2 == 0 { "#999" } else { "#bbb" },
                }],),
            ],
            g![
                C!["y-labels"],
                (0..6).map(|i| text![
                    style! {St::FontSize => px(10)},
                    attrs! {
                        X => padding - 6,
                        Y => f64::from(height - padding)  - (f64::from(i) * 0.5) / dx - 0.5, // -0.5 to center rect
                        TextAnchor => "end",
                        Fill => "#999",
                    },
                    format!("{:.1}", f64::from(i) * 0.5),
                ]),
            ]
        ],
        g![
            C!["plot-area"],
            polyline![attrs! {
                Fill => "none",
                Stroke => "rgb(0, 86, 91)",
                Points => data
                    .iter()
                    .map(|(x, y)| format!(
                        "{},{}",
                        x / dx + f64::from(padding),
                        -y / dx + f64::from(height - padding),
                    ))
                    .collect::<Vec<_>>()
                    .join(" ")
            }],
            data.iter()
                .enumerate()
                .filter_map(|(i, (x, y))| IF!(i % 15 == 0 => {
                    let onenter = onenter.clone();
                    let onout = onout.clone();
                    let data = (*x, *y);
                    Some(g![
                        C!["data-point"],
                        mouse_ev(Ev::MouseOver, move |event| {
                            onenter(Tooltip {
                                position: (event.x(), event.y()),
                                data,
                            })
                        }),
                        mouse_ev(Ev::MouseOut, move |_| onout),
                        circle![attrs! {
                            Cx => x / dx + f64::from(padding),
                            Cy => -y / dx + f64::from(height - padding),
                            R => 2,
                            Fill => "rgba(0, 86, 91, 0.8)",
                        }],
                    ])
                }))
        ]
    ]
}
