use itertools::Itertools;
use seed::{prelude::*, *};

pub fn chart<T: Clone + 'static>(
    data: &[(f64, f64)],
    onenter: impl Fn(i32, i32) -> T + Clone + 'static,
    onout: T,
) -> Node<T> {
    use At::*;
    // domain
    let x_bounds = data.iter().map(|(x, _)| x).minmax().into_option().unwrap();

    // let y_bounds = data.iter().map(|(_, y)| y).minmax().into_option().unwrap();

    let padding = 30;
    let width = 800;
    let height = 500;

    // let n = data.len() as f64;
    let n = width as f64;
    let dx = (x_bounds.1 - x_bounds.0) / (n - (2 * padding) as f64); // scale factor
    let ticks_num = 15;
    let ticks_step = (
        (width - 2 * padding) / ticks_num,
        (height - 2 * padding) / ticks_num,
    );
    svg![
        class!["chart"],
        style!{ St::Display => "block" },
        attrs! {
            ViewBox => format!("0 0 {} {}", width, height),
        },
        g![
            class!["x-axis"],
            line_![attrs! {
                X1 => padding,
                X2 => width - padding,
                Y1 => height/2,
                Y2 => height/2,
                Stroke => "#ccc",
            }],
            (ticks_step.0..(width - 2 * padding))
                .step_by(ticks_step.0)
                .map(|i| rect![attrs! {
                    X => i + padding,
                    Y => "50%",
                    Width => 1,
                    Height => 3,
                    Fill => "#999",
                }]),
            (ticks_step.0..(width - 2 * padding))
                .step_by(ticks_step.0)
                .map(|i| text![
                    style! {St::FontSize => px(10)},
                    attrs! {
                        Fill => "#999",
                        TextAnchor => "middle",
                        X => i + padding,
                        Y => height/2 + 20,
                    },
                    format!("{:.2}", (i as f64)*dx),
                ]),
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
            (ticks_step.1..(height - 2 * padding))
                .step_by(ticks_step.1)
                .map(|i| rect![attrs! {
                    X => padding - 3,
                    Y => i + padding,
                    Width => 3,
                    Height => 1,
                    Fill => "#999",
                }],),
            (ticks_step.1..(height - 2 * padding))
                .step_by(ticks_step.1)
                .map(|i| text![
                    style! {St::FontSize => px(10)},
                    attrs! {
                        TextAnchor => "end",
                        Fill => "#999",
                        X => padding - 6,
                        Y => i + padding + 4,
                    },
                    i,
                ]),
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
                        -y / dx + (height/2) as f64,
                    ))
                    .collect::<Vec<_>>()
                    .join(" ")
            }],
            data.iter()
                .enumerate()
                .filter_map(|(i, (x, y))| if i % 30 == 0 {
                    let onenter = onenter.clone();
                    let onout = onout.clone();
                    Some(g![
                        class!["data-point"],
                        mouse_ev(Ev::MouseOver, move |event| {
                            onenter(event.x(), event.y())
                        }),
                        mouse_ev(Ev::MouseOut, move |_| onout),
                        circle![attrs! {
                            Cx => x / dx + padding as f64,
                            Cy => -y / dx + (height/2) as f64,
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
