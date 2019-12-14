//! [MDN web docs](https://developer.mozilla.org/en-US/docs/Learn/CSS/Introduction_to_CSS/Values_and_units)

macro_rules! create_unit_items {
    { $( $variant:tt => $function:tt => $literal_unit:tt ),* $(,)?} => {
        // ---- Create macro `unit!` ----
        #[macro_export]
        macro_rules! unit {
            { $value:expr } => {
                {
                    $value.to_string()
                }
             };
             $(
                { $value:expr, $literal_unit } => {
                    {
                        $value.to_string() + stringify!($literal_unit)
                    }
                 };
             )*
             { $value:expr, $unit:expr } => {
                {
                    let unit: Unit = $unit;
                    format!("{}{}", $value, unit)
                }
             };
        }

        // ---- Create enum `Unit` ----
        #[allow(dead_code)]
        #[derive(Clone, Copy)]
        pub enum Unit {
            $($variant,)*
        }
        impl std::fmt::Display for Unit {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let text_unit = match self {
                    $(Unit::$variant => stringify!($literal_unit),)*
                };
                write!(f, "{}", text_unit)
            }
        }

        // ---- Create unit functions (e.g. `px(..)`) ----
        $(
            #[allow(dead_code)]
            pub fn $function(value: impl std::fmt::Display) -> String {
                format!("{}{}", value, stringify!($literal_unit))
            }
        )*
    }
}

// https://flaviocopes.com/css-units/
// Unit::Variant => function name => literal unit
create_unit_items! {
    // Is like ex, but measures the width of 0 (zero).
    Ch => ch => ch,
    // Centimeter (maps to 37.8 pixels).
    Cm => cm => cm,
    // Value assigned to that element’s font-size, measures the width of the m letter.
    Em => em => em,
    // Fraction units, and they are used in CSS Grid to divide space into fractions.
    Fr => fr => fr,
    // Is like em, but measures the height of the x letter.
    Ex => ex => ex,
    // Inch (maps to 96 pixels).
    In => inch => in,
    // Millimeter.
    Mm => mm => mm,
    // Pica (1 pica = 12 points).
    Pc => pc => pc,
    // Percent.
    Percent => percent => %,
    // Point (1 inch = 72 points).
    Pt => pt => pt,
    // Pixel.
    Px => px => px,
    // Quarter of a millimeter.
    Q => q => q,
    // Is similar to em, but uses the root element (html) font-size.
    Rem => rem => rem,
    // Viewport height unit represents a percentage of the viewport height.
    Vh => vh => vh,
    // Viewport minimum unit represents the minimum between the height or width in terms of percentage.
    Vmin => vmin => vmin,
    // Viewport maximum unit represents the maximum between the height or width in terms of percentage.
    Vmax => vmax => vmax,
    // Viewport width unit represents a percentage of the viewport width.
    Vw => vw => vw,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    // ----------------- Macro Unit -----------------
    #[wasm_bindgen_test]
    fn variable() {
        let width = -100;
        assert_eq!(unit!(width, px), "-100px");
    }

    #[wasm_bindgen_test]
    fn expression() {
        assert_eq!(unit!(100 + 350, px), "450px");
    }

    #[wasm_bindgen_test]
    fn without_unit() {
        assert_eq!(unit!(2.5), "2.5");
    }

    #[wasm_bindgen_test]
    fn str_with_variant() {
        assert_eq!(unit!("68", Unit::Mm), "68mm");
    }

    #[wasm_bindgen_test]
    fn percent_unit() {
        assert_eq!(unit!(15_236.56f64, %), "15236.56%");
    }

    #[wasm_bindgen_test]
    fn in_unit_with_negative_zero() {
        assert_eq!(unit!(-0, in), "0in");
    }

    // ----------------- Functions -----------------
    #[wasm_bindgen_test]
    fn px_function() {
        assert_eq!(px(15), "15px");
    }

    #[wasm_bindgen_test]
    fn inch_function() {
        assert_eq!(inch(-15.63), "-15.63in");
    }

    #[wasm_bindgen_test]
    fn percent_function() {
        assert_eq!(percent("35"), "35%");
    }
}
