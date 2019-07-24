#![allow(clippy::non_ascii_literal)]

#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self { val: 0 }
    }
}

// Msg

#[derive(Clone)]
struct Msg;

// View

fn definition(description: &str, def: &str) -> Node<Msg> {
    // todo: Could add $$ here.
    div![h5![description], span![format!("$${}$$", def)],]
}

fn _dirac_3(left: &str, middle: &str, right: &str) -> String {
    format!(
        r"\langle {} \lvert {} \rvert {} \rangle",
        left, middle, right
    )
}

fn view(_: &Model) -> impl View<Msg> {
    vec![
        h1!["Linear algebra cheatsheet"],
        p!["Intent: Provide a quick reference of definitions and identities that 
        are useful in formal, symbolic linear algebra"],
        section![
            h2!["A description of terms"],
            div![
                style!{
                    // "display" => "grid";
                    // "grid"
                    },
                    ul![
                        li![r"$\mathbf{A}$, or $\mathbf{B}$, or $\mathbf{C}$ : Matrices"],
                        li![r"$\mathbf{T}$, or $\mathbf{S}$ : Arbitrary operators"],
                        li![r"$a$ or $b$ or $c$ or $d$: Arbitrary vectors"],
                        li![r"$α$ or $β$ : Arbitrary constants"], // todo maybe change these
                        li![r"$i$, $j$, or $k$ : Basis vectors"],
                    ]
            ]
        ],

        section![
            definition(
                "When dividing by an operator on the right, move it to the right",
                r"\mathbf{A} = \mathbf{B}\mathbf{C}\mathbf{D} \rightarrow \mathbf{A}\mathbf{D}^{-1} = \mathbf{B}\mathbf{C}"
            ),
            definition(
                "When dividing by an operator on the left, move it to the left",
                r"\mathbf{A} = \mathbf{B}\mathbf{C}\mathbf{D} \rightarrow \mathbf{B}^{-1}\mathbf{A} = \mathbf{C}\mathbf{D}"
            ),
            definition(
                "Dagger associativity",
                r"(\mathbf{S T})^\dagger = \mathbf{T}^\dagger \mathbf{S}^\dagger"
            ),
            definition(
                "Dagger commuting",
                r"(\mathbf{T})^\dagger (a b) = a \mathbf{T})^\dagger b"
            ),
            definition(
                "Determinant associativity",
                r"det(\mathbf{S T}) = det(\mathbf{T}) det(\mathbf{S})"
            ),
            definition(
                "Definition of matrix multiplication", 
                r"C_{ij} = \sum_k A_{ik} B_{kj}"
            ),
            definition(
                "Definition of unit matrix", 
                r"\mathbf{1A} = \mathbf{A1} = \mathbf{A}"
            ),
            definition(
                "Definition of inverse matrix", 
                r"\mathbf{A}^{-1} \mathbf{A} = \mathbf{A} \mathbf{A}^{-1} = \mathbf{1}"
            ),
            definition(
                "The most general operator",
                r"\mathbf{T} = \lvert e_i \rangle (\mathbf{T})_{ij} \langle e j \rvert",
            ),
            definition(
                "",
                // todo fix this one
                r"\langle a \lvert T \rvert b \rangle = \langle T^\dagger \lvert a \rvert b \rangle"
            ),
            definition(
                "",
                // todo fix this one
                r"\langle a \lvert T^\dagger \rvert c \rangle = \langle c \lvert T \rvert a \rangle^*"
            ),
            definition(
                "Swapping bras and kets conjugates",
                r"\langle a \vert b \rangle = \langle b \vert a \rangle^*"
            ),
            definition(
                "A statement of basis completeness",
                r"\mathbb{1} = \sum_i \lvert i \rangle \langle i \rvert"
            ),
            definition(
                "Delta functions in Dirac notation",
                r"\delta_{i, j} = \langle i \vert j \rangle"
            ),
            definition(
                "A property of Trace",
                r"tr(\mathbf{AB}) = tr(\mathbf{BA})"
            ),
            definition(
                "Subtraction in Dirac notation",
                r"\langle b \lvert \mathbf{T} \rvert a \rangle - \langle b \lvert \mathbf{S} \rvert a \rangle =
\mathbf{T} - \mathbf{S} \langle b \vert a \rangle "
            ),
            definition(
                "Dirac notation as integrals",
                r"\int dx a^*(x) \mathbf{T} b(x) = \langle a \lvert \mathbf{T} \rvert b \rangle"
            ),
            definition(
                "Dirac notation as integrals continued",
                r"\int dx a^*(x) b(x) = \langle a \vert b \rangle"
            ),
            definition(
                "Sum and delta manipulation",
                r"O_{ji} \sum_j \delta_{kj} = O_{ki}"
            ),
            definition(
                "Functions in dirac notation",
                r"\langle x \vert a \rangle = a(x)"
            ),
            definition(
                "A delta equivalence",
                r"\langle \mathbf{A_\alpha} \vert \mathbf{A_\beta} \rangle = \delta_{\alpha \beta}"
            ),
        ],

        section![
            h2!["Special properties"],
            definition(
                "Hermitian matrix: Self-adjoint",
                r"\mathbf{A}^\dagger = \mathbf{A}"
            ),
            definition(
                "Unitary matrix: Inverse is its adjoint",
                r"\mathbf{A}^\dagger = \mathbf{A}^{-1}"
            ),
            definition(
                "An orthonormal function",
                r"\langle \mathbf{A} \vert \mathbf{A} \rangle = \mathbb{1}"
            ),
            definition(
                "A property if a certain commutation rule applies",
                r"[[\mathbf{A}, \mathbf{B}], \mathbf{A}] = 0 → e^{\mathbf{A}} \mathbf{B}
e^{-\mathbf{A}} = \mathbf{B} + [\mathbf{A}, \mathbf{B}]"
            ),
            definition(
                "Functions in dirac notation",
                r"f_p(x) = \langle x \vert p \rangle"
            ),

        ],

        footer![
            h3!["References"],
            ul![
                li!["Modern Quantum Chemistry, by Attila Szabo and Neil S. Ostlund"]
            ]
        ]
    ]
}

// Init

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, |_, _, _| (), view).finish().run();
}
