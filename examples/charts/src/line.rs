use itertools::Itertools;
use seed::{prelude::*, *};

#[derive(Clone)]
pub struct Tooltip {
    pub position: (i32, i32),
    pub data: (f64, f64),
}

pub fn chart<T: Clone + 'static>(
    data: &[(f64, f64)],
    onenter: impl Fn(Tooltip) -> T + Clone + 'static,
    onout: T,
) -> Node<T> {
    use At::*;
    // domain
    let x_bounds = data.iter().map(|(x, _)| x).minmax().into_option().unwrap();

    let padding = 30;
    let width = 600;
    let height = 300;

    let dx = (x_bounds.1 - x_bounds.0) / (width as f64 - (2 * padding) as f64); // scale factor
    svg![
        class!["chart"],
        style! { St::Display => "block" },
        attrs! {
            ViewBox => format!("0 0 {} {}", width, height),
        },
        g![
            class!["x-axis"],
            line_![attrs! {
                X1 => padding,
                X2 => width - padding,
                Y1 => height - padding,
                Y2 => height - padding,
                Stroke => "#ccc",
            }],
            g![
                class!["x-ticks"],
                (1..25).map(|i| rect![attrs! {
                    X => (i as f64 * 0.25) / dx + padding as f64 - 0.5,
                    Y => height - padding,
                    Width => 1,
                    Height => if i % 2 == 0 { 3 } else { 2 },
                    Fill => if i % 2 == 0 { "#999" } else { "#bbb" },
                }]),
            ],
            g![
                class!["x-labels"],
                (0..13).map(|i| text![
                    style! {St::FontSize => px(10)},
                    attrs! {
                        X => (i as f64 * 0.5) / dx + padding as f64 - 0.5,
                        Y => height - padding + 20,
                        Fill => "#999",
                        TextAnchor => "middle",
                    },
                    format!("{:.1}", (i as f64 * 0.5)),
                ]),
            ],
        ],
        g![
            class!["y-axis"],
            line_![attrs! {
                X1 => padding,
                X2 => padding,
                Y1 => padding,
                Y2 => height - padding,
                Stroke => "#ccc",
            }],
            g![
                class!["y-ticks"],
                (1..11).map(|i| rect![attrs! {
                    X => padding - if i % 2 == 0 { 3 } else { 2 },
                    Y => (height - padding) as f64 - (i as f64 * 0.25) / dx - 0.5, // -0.5 to center rect
                    Width => if i % 2 == 0 { 3 } else { 2 },
                    Height => 1,
                    Fill => if i % 2 == 0 { "#999" } else { "#bbb" },
                }],),
            ],
            g![
                class!["y-labels"],
                (0..6).map(|i| text![
                    style! {St::FontSize => px(10)},
                    attrs! {
                        X => padding - 6,
                        Y => (height - padding) as f64 - (i as f64 * 0.5) / dx - 0.5, // -0.5 to center rect
                        TextAnchor => "end",
                        Fill => "#999",
                    },
                    format!("{:.1}", i as f64 * 0.5),
                ]),
            ]
        ],
        g![
            class!["plot-area"],
            polyline![attrs! {
                Fill => "none",
                Stroke => "rgb(0, 86, 91)",
                Points => data
                    .iter()
                    .map(|(x, y)| format!(
                        "{},{}",
                        x / dx + padding as f64,
                        -y / dx + (height - padding) as f64,
                    ))
                    .collect::<Vec<_>>()
                    .join(" ")
            }],
            data.iter()
                .enumerate()
                .filter_map(|(i, (x, y))| if i % 15 == 0 {
                    let onenter = onenter.clone();
                    let onout = onout.clone();
                    let data = (*x, *y);
                    Some(g![
                        class!["data-point"],
                        mouse_ev(Ev::MouseOver, move |event| {
                            onenter(Tooltip {
                                position: (event.x(), event.y()),
                                data,
                            })
                        }),
                        mouse_ev(Ev::MouseOut, move |_| onout),
                        circle![attrs! {
                            Cx => x / dx + padding as f64,
                            Cy => -y / dx + (height - padding) as f64,
                            R => 2,
                            Fill => "rgba(0, 86, 91, 0.8)",
                        }],
                    ])
                } else {
                    None
                })
        ]
    ]
}
