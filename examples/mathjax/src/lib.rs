#![allow(clippy::non_ascii_literal)]

use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    pub show_description: bool,
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone, Copy)]
enum Msg {
    ToggleDescription,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleDescription => model.show_description = !model.show_description,
    }
}

// ------ ------
//     View
// ------ ------

fn math_tex(expression: &str) -> Node<Msg> {
    custom![
        Tag::from("math-tex"),
        style! {
            St::Display => "none",
        },
        expression,
    ]
}

fn definition(description: &str, def: &str, index: usize) -> Node<Msg> {
    div![
        style! {
            St::Display => "flex",
            St::AlignItems => "baseline",
            St::FlexWrap => "wrap",
            St::JustifyContent => "space-between",
            St::BackgroundColor => {
                if index % 2 == 1 {
                    CSSValue::Some("aliceblue".into())
                } else {
                    CSSValue::Ignored
                }
            },
            St::Padding => px(0) + " " + &px(8)
        },
        h5![
            style! {
                St::MarginRight => px(20),
            },
            description
        ],
        math_tex(def),
    ]
}

fn _dirac_3(left: &str, middle: &str, right: &str) -> String {
    format!(
        r"\langle {} \lvert {} \rvert {} \rangle",
        left, middle, right
    )
}

#[allow(clippy::too_many_lines)]
fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        style!{
            St::MaxWidth => px(750),
            St::Margin => "auto",
        },
        h1!["Linear algebra cheatsheet"],
        p!["Intent: Provide a quick reference of definitions and identities that
        are useful in formal, symbolic linear algebra"],

        button![
            format!("{} description", if model.show_description { "Hide" } else { "Show"}),
            simple_ev(Ev::Click, Msg::ToggleDescription),
        ],

        if model.show_description {
            section![
                h2!["A description of terms"],
                div![
                    ul![
                        style!{
                            St::ListStyle => "none",
                        },
                        li![math_tex(r"\mathbf{A}, \mathbf{B} \text{ or } \mathbf{C} \text{  .....  } \text{Matrices}")],
                        li![math_tex(r"\mathbf{T} \text{ or } \mathbf{S} \text{  .....  } \text{Arbitrary operators}")],
                        li![math_tex(r"\mathbf{a}, \mathbf{b}, \mathbf{c} \text{ or } \mathbf{d} \text{  .....  } \text{Arbitrary vectors}")],
                        li![math_tex(r"\mathbf{α} \text{ or } \mathbf{β} \text{  .....  } \text{Arbitrary constants}")],
                        li![math_tex(r"\mathbf{i}, \mathbf{j} \text{ or } \mathbf{k} \text{  .....  } \text{Basis vectors}")],
                    ]
                ]
            ]
        } else {
            empty![]
        },

        section![
            vec![
                (
                    "When dividing by an operator on the right, move it to the right",
                    r"\mathbf{A} = \mathbf{B}\mathbf{C}\mathbf{D} \rightarrow \mathbf{A}\mathbf{D}^{-1} = \mathbf{B}\mathbf{C}"
                ),
                (
                    "When dividing by an operator on the left, move it to the left",
                    r"\mathbf{A} = \mathbf{B}\mathbf{C}\mathbf{D} \rightarrow \mathbf{B}^{-1}\mathbf{A} = \mathbf{C}\mathbf{D}"
                ),
                (
                    "Dagger associativity",
                    r"(\mathbf{S T})^\dagger = \mathbf{T}^\dagger \mathbf{S}^\dagger"
                ),
                (
                    "Dagger commuting",
                    r"(\mathbf{T})^\dagger (a b) = a \mathbf{T})^\dagger b"
                ),
                (
                    "Determinant associativity",
                    r"det(\mathbf{S T}) = det(\mathbf{T}) det(\mathbf{S})"
                ),
                (
                    "Definition of matrix multiplication",
                    r"C_{ij} = \sum_k A_{ik} B_{kj}"
                ),
                (
                    "Definition of unit matrix",
                    r"\mathbf{1A} = \mathbf{A1} = \mathbf{A}"
                ),
                (
                    "Definition of inverse matrix",
                    r"\mathbf{A}^{-1} \mathbf{A} = \mathbf{A} \mathbf{A}^{-1} = \mathbf{1}"
                ),
                (
                    "The most general operator",
                    r"\mathbf{T} = \lvert e_i \rangle (\mathbf{T})_{ij} \langle e j \rvert",
                ),
                (
                    "Swapping bras and kets conjugates",
                    r"\langle a \vert b \rangle = \langle b \vert a \rangle^*"
                ),
                (
                    "A statement of basis completeness",
                    r"\mathbb{1} = \sum_i \lvert i \rangle \langle i \rvert"
                ),
                (
                    "Delta functions in Dirac notation",
                    r"\delta_{i, j} = \langle i \vert j \rangle"
                ),
                (
                    "A property of Trace",
                    r"tr(\mathbf{AB}) = tr(\mathbf{BA})"
                ),
                (
                    "Subtraction in Dirac notation",
                    r"\langle b \lvert \mathbf{T} \rvert a \rangle - \langle b \lvert \mathbf{S} \rvert a \rangle =
                      \mathbf{T} - \mathbf{S} \langle b \vert a \rangle "
                ),
                (
                    "Dirac notation as integrals",
                    r"\int dx a^*(x) \mathbf{T} b(x) = \langle a \lvert \mathbf{T} \rvert b \rangle"
                ),
                (
                    "Dirac notation as integrals continued",
                    r"\int dx a^*(x) b(x) = \langle a \vert b \rangle"
                ),
                (
                    "Sum and delta manipulation",
                    r"O_{ji} \sum_j \delta_{kj} = O_{ki}"
                ),
                (
                    "Functions in dirac notation",
                    r"\langle x \vert a \rangle = a(x)"
                ),
                (
                    "A delta equivalence",
                    r"\langle \mathbf{A_\alpha} \vert \mathbf{A_\beta} \rangle = \delta_{\alpha \beta}"
                ),
            ].into_iter().enumerate().map(|(i, (description, def))| definition(description, def, i))
        ],


        section![
            h2!["Special properties"],
            vec![
                (
                    "Hermitian matrix: Self-adjoint",
                    r"\mathbf{A}^\dagger = \mathbf{A}"
                ),
                (
                    "Unitary matrix: Inverse is its adjoint",
                    r"\mathbf{A}^\dagger = \mathbf{A}^{-1}"
                ),
                (
                    "An orthonormal function",
                    r"\langle \mathbf{A} \vert \mathbf{A} \rangle = \mathbb{1}"
                ),
                (
                    "A property if a certain commutation rule applies",
                    r"[[\mathbf{A}, \mathbf{B}], \mathbf{A}] = 0 → e^{\mathbf{A}} \mathbf{B}
                      e^{-\mathbf{A}} = \mathbf{B} + [\mathbf{A}, \mathbf{B}]"
                ),
                (
                    "Functions in dirac notation",
                    r"f_p(x) = \langle x \vert p \rangle"
                ),
            ].into_iter().enumerate().map(|(i, (description, def))| definition(description, def, i))
        ],

        footer![
            h3!["References"],
            ul![
                li!["Modern Quantum Chemistry, by Attila Szabo and Neil S. Ostlund"]
            ]
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
