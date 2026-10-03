use colored_text::{
    hex_to_rgb, hsl_to_rgb, resolve_ansi256, resolve_hex, resolve_hsl, resolve_named, resolve_rgb,
    AnsiColor, ColorInputError, ColorLevel, ResolvedColor,
};
use rstest::rstest;

const LEVELS: [ColorLevel; 4] = [
    ColorLevel::NoColor,
    ColorLevel::Ansi16,
    ColorLevel::Ansi256,
    ColorLevel::TrueColor,
];
const NAMED: [AnsiColor; 16] = [
    AnsiColor::Black,
    AnsiColor::Red,
    AnsiColor::Green,
    AnsiColor::Yellow,
    AnsiColor::Blue,
    AnsiColor::Magenta,
    AnsiColor::Cyan,
    AnsiColor::White,
    AnsiColor::BrightBlack,
    AnsiColor::BrightRed,
    AnsiColor::BrightGreen,
    AnsiColor::BrightYellow,
    AnsiColor::BrightBlue,
    AnsiColor::BrightMagenta,
    AnsiColor::BrightCyan,
    AnsiColor::BrightWhite,
];

#[test]
fn named_colors_preserve_palette_identity_at_every_enabled_depth() {
    for color in NAMED {
        assert_eq!(resolve_named(color, ColorLevel::NoColor), None);
        for level in &LEVELS[1..] {
            assert_eq!(
                resolve_named(color, *level),
                Some(ResolvedColor::Named(color))
            );
        }
    }
}

#[rstest]
#[case((0, 0, 0), AnsiColor::Black, 0)]
#[case((255, 255, 255), AnsiColor::BrightWhite, 15)]
#[case((255, 0, 0), AnsiColor::BrightRed, 9)]
#[case((0, 255, 0), AnsiColor::BrightGreen, 10)]
#[case((0, 0, 255), AnsiColor::BrightBlue, 12)]
#[case((95, 135, 175), AnsiColor::BrightBlack, 67)]
#[case((128, 128, 128), AnsiColor::BrightBlack, 8)]
#[case((118, 118, 118), AnsiColor::BrightBlack, 243)]
#[case((128, 0, 0), AnsiColor::Red, 1)]
#[case((215, 58, 74), AnsiColor::BrightRed, 167)]
#[case((64, 64, 64), AnsiColor::Black, 238)]
fn rgb_resolves_at_every_depth(
    #[case] rgb: (u8, u8, u8),
    #[case] named: AnsiColor,
    #[case] index: u8,
) {
    let (r, g, b) = rgb;
    assert_eq!(resolve_rgb(r, g, b, ColorLevel::NoColor), None);
    assert_eq!(
        resolve_rgb(r, g, b, ColorLevel::Ansi16),
        Some(ResolvedColor::Named(named))
    );
    assert_eq!(
        resolve_rgb(r, g, b, ColorLevel::Ansi256),
        Some(ResolvedColor::Ansi256(index))
    );
    assert_eq!(
        resolve_rgb(r, g, b, ColorLevel::TrueColor),
        Some(ResolvedColor::Rgb(r, g, b))
    );
}

#[rstest]
#[case(0, AnsiColor::Black)]
#[case(8, AnsiColor::BrightBlack)]
#[case(15, AnsiColor::BrightWhite)]
#[case(16, AnsiColor::Black)]
#[case(67, AnsiColor::BrightBlack)]
#[case(208, AnsiColor::BrightYellow)]
#[case(231, AnsiColor::BrightWhite)]
#[case(232, AnsiColor::Black)]
#[case(236, AnsiColor::Black)]
#[case(255, AnsiColor::BrightWhite)]
fn indexed_colors_stay_indexed_above_ansi16(#[case] index: u8, #[case] named: AnsiColor) {
    assert_eq!(resolve_ansi256(index, ColorLevel::NoColor), None);
    assert_eq!(
        resolve_ansi256(index, ColorLevel::Ansi16),
        Some(ResolvedColor::Named(named))
    );
    for level in [ColorLevel::Ansi256, ColorLevel::TrueColor] {
        assert_eq!(
            resolve_ansi256(index, level),
            Some(ResolvedColor::Ansi256(index))
        );
    }
}

#[rstest]
#[case((0., 100., 50.))]
#[case((360., 100., 50.))]
#[case((0., 0., 50.))]
#[case((360., 100., 0.))]
#[case((0., 100., 100.))]
fn hsl_uses_rgb_resolution_at_every_depth(#[case] hsl: (f32, f32, f32)) {
    let (r, g, b) = hsl_to_rgb(hsl.0, hsl.1, hsl.2).unwrap();
    for level in LEVELS {
        assert_eq!(
            resolve_hsl(hsl.0, hsl.1, hsl.2, level),
            Ok(resolve_rgb(r, g, b, level))
        );
    }
}

#[rstest]
#[case("hue", -1.)]
#[case("hue", 361.)]
#[case("saturation", -1.)]
#[case("saturation", 101.)]
#[case("lightness", -1.)]
#[case("lightness", 101.)]
fn out_of_range_hsl_is_an_error_even_when_color_is_disabled(
    #[case] component: &'static str,
    #[case] value: f32,
) {
    assert_invalid_hsl(component, value);
}

fn assert_invalid_hsl(component: &'static str, value: f32) {
    let (h, s, l) = match component {
        "hue" => (value, 100., 50.),
        "saturation" => (0., value, 50.),
        _ => (0., 100., value),
    };
    for level in LEVELS {
        let error = resolve_hsl(h, s, l, level).unwrap_err();
        assert_eq!(
            format!("{error:?}"),
            format!("{:?}", hsl_to_rgb(h, s, l).unwrap_err())
        );
        match error {
            ColorInputError::InvalidHsl {
                component: actual,
                value: actual_value,
            } => {
                assert_eq!(actual, component);
                assert_eq!(actual_value.to_bits(), value.to_bits());
            }
            _ => panic!("unexpected error: {error:?}"),
        }
    }
}

#[rstest]
fn non_finite_hsl_is_rejected(
    #[values("hue", "saturation", "lightness")] component: &'static str,
    #[values(f32::NAN, f32::INFINITY, f32::NEG_INFINITY)] value: f32,
) {
    assert_invalid_hsl(component, value);
}

#[rstest]
#[case("f80")]
#[case("#aBc")]
#[case("d73a4a")]
#[case("#D73A4A")]
fn hex_uses_rgb_resolution_at_every_depth(#[case] input: &str) {
    let (r, g, b) = hex_to_rgb(input).unwrap();
    for level in LEVELS {
        assert_eq!(resolve_hex(input, level), Ok(resolve_rgb(r, g, b, level)));
    }
}

#[rstest]
#[case("")]
#[case("1234")]
#[case("#12x456")]
#[case("##f80")]
#[case("#f80 ")]
#[case("éa")]
fn malformed_hex_is_an_error_even_when_color_is_disabled(#[case] input: &str) {
    let error = hex_to_rgb(input).unwrap_err();
    assert_eq!(
        error,
        ColorInputError::InvalidHex {
            input: input.to_owned()
        }
    );
    for level in LEVELS {
        assert_eq!(resolve_hex(input, level), Err(error.clone()));
    }
}

#[test]
fn input_errors_have_useful_diagnostics_and_implement_std_error() {
    let hex = resolve_hex("invalid", ColorLevel::NoColor).unwrap_err();
    let hsl = resolve_hsl(0., 101., 50., ColorLevel::NoColor).unwrap_err();
    assert!(hex.to_string().contains("invalid"));
    assert!(hsl.to_string().contains("saturation"));
    assert!(hsl.to_string().contains("101"));
    for error in [&hex, &hsl] {
        let error: &dyn std::error::Error = error;
        assert!(error.source().is_none());
    }
}
