use colored_text::{hex_to_rgb, hsl_to_rgb, ColorInputError, HslComponent};
use rstest::rstest;

#[rstest]
#[case((0., 100., 50.), (255, 0, 0))]
#[case((360., 100., 50.), (255, 0, 0))]
#[case((60., 100., 50.), (255, 255, 0))]
#[case((120., 100., 50.), (0, 255, 0))]
#[case((180., 100., 50.), (0, 255, 255))]
#[case((240., 100., 50.), (0, 0, 255))]
#[case((300., 100., 50.), (255, 0, 255))]
#[case((0., 0., 50.), (127, 127, 127))]
#[case((360., 100., 0.), (0, 0, 0))]
#[case((0., 100., 100.), (255, 255, 255))]
#[case((-0., -0., -0.), (0, 0, 0))]
fn hsl_conversion_matches_expected_channels(
    #[case] hsl: (f32, f32, f32),
    #[case] rgb: (u8, u8, u8),
) {
    assert_eq!(hsl_to_rgb(hsl.0, hsl.1, hsl.2), Ok(rgb));
}

#[rstest]
#[case("f80", (255, 136, 0))]
#[case("#f80", (255, 136, 0))]
#[case("F80", (255, 136, 0))]
#[case("#aBc", (170, 187, 204))]
#[case("d73a4a", (215, 58, 74))]
#[case("#d73a4a", (215, 58, 74))]
#[case("#D73A4A", (215, 58, 74))]
#[case("d73A4a", (215, 58, 74))]
#[case("000", (0, 0, 0))]
#[case("#FFFFFF", (255, 255, 255))]
fn hex_parser_accepts_documented_forms(#[case] input: &str, #[case] rgb: (u8, u8, u8)) {
    assert_eq!(hex_to_rgb(input), Ok(rgb));
}

#[rstest]
#[case("")]
#[case("#")]
#[case("12")]
#[case("1234")]
#[case("12345")]
#[case("1234567")]
#[case("ggg")]
#[case("+ab")]
#[case("+abcde")]
#[case("#+ab")]
#[case("#12x456")]
#[case("##f80")]
#[case("0xff8000")]
#[case(" #f80")]
#[case("#f80 ")]
#[case("f80\n")]
#[case("éa")]
#[case("aé")]
#[case("éabcd")]
fn hex_parser_rejects_malformed_input(#[case] input: &str) {
    assert_eq!(
        hex_to_rgb(input),
        Err(ColorInputError::InvalidHex {
            input: input.to_owned()
        })
    );
}

#[rstest]
#[case(HslComponent::Hue, -1.)]
#[case(HslComponent::Hue, 361.)]
#[case(HslComponent::Saturation, -1.)]
#[case(HslComponent::Saturation, 101.)]
#[case(HslComponent::Lightness, -1.)]
#[case(HslComponent::Lightness, 101.)]
fn hsl_conversion_rejects_out_of_range_components(
    #[case] component: HslComponent,
    #[case] value: f32,
) {
    assert_invalid_hsl(component, value);
}

fn assert_invalid_hsl(component: HslComponent, value: f32) {
    let (h, s, l) = match component {
        HslComponent::Hue => (value, 100., 50.),
        HslComponent::Saturation => (0., value, 50.),
        HslComponent::Lightness => (0., 100., value),
    };
    let error = hsl_to_rgb(h, s, l).unwrap_err();
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

#[rstest]
fn hsl_conversion_rejects_non_finite_components(
    #[values(HslComponent::Hue, HslComponent::Saturation, HslComponent::Lightness)]
    component: HslComponent,
    #[values(f32::NAN, f32::INFINITY, f32::NEG_INFINITY)] value: f32,
) {
    assert_invalid_hsl(component, value);
}

#[rstest]
#[case(HslComponent::Hue, "hue")]
#[case(HslComponent::Saturation, "saturation")]
#[case(HslComponent::Lightness, "lightness")]
fn hsl_component_has_public_value_traits_and_diagnostics(
    #[case] component: HslComponent,
    #[case] name: &str,
) {
    fn assert_traits<T: Clone + Copy + std::fmt::Debug + Eq + PartialEq + std::hash::Hash>() {}
    assert_traits::<HslComponent>();
    let error = ColorInputError::InvalidHsl {
        component,
        value: -1.,
    };
    assert_eq!(component.to_string(), name);
    assert_eq!(
        error.to_string(),
        format!("invalid HSL {name}: -1 is non-finite or out of range")
    );
}
