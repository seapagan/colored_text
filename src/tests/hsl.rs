use super::TestStateGuard;
use crate::*;
use rstest::rstest;

fn assert_rgb_approx_eq(actual: &str, expected: &str) {
    let extract_rgb = |s: &str| {
        let start = s.find("38;2;").or_else(|| s.find("48;2;"));
        if let Some(start) = start {
            let sequence = &s[start..];
            let parts: Vec<&str> = sequence.split(';').collect();
            let r = parts.get(2).and_then(|part| part.parse::<i32>().ok());
            let g = parts.get(3).and_then(|part| part.parse::<i32>().ok());
            let b = parts
                .get(4)
                .and_then(|part| part.split('m').next())
                .and_then(|part| part.parse::<i32>().ok());

            if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                return (r, g, b);
            }
        }

        panic!("Invalid ANSI color sequence");
    };

    let (r1, g1, b1) = extract_rgb(actual);
    let (r2, g2, b2) = extract_rgb(expected);

    assert!(
        (r1 - r2).abs() <= 1 && (g1 - g2).abs() <= 1 && (b1 - b2).abs() <= 1,
        "RGB values differ by more than 1: ({}, {}, {}) vs ({}, {}, {})",
        r1,
        g1,
        b1,
        r2,
        g2,
        b2
    );
}

#[rstest(h, s, l, r, g, b)]
#[case(0.0, 100.0, 50.0, 255, 0, 0)]
#[case(60.0, 100.0, 50.0, 255, 255, 0)]
#[case(90.0, 100.0, 50.0, 128, 255, 0)]
#[case(120.0, 100.0, 50.0, 0, 255, 0)]
#[case(150.0, 100.0, 50.0, 0, 255, 128)]
#[case(180.0, 100.0, 50.0, 0, 255, 255)]
#[case(210.0, 100.0, 50.0, 0, 128, 255)]
#[case(240.0, 100.0, 50.0, 0, 0, 255)]
#[case(300.0, 100.0, 50.0, 255, 0, 255)]
#[case(330.0, 100.0, 50.0, 255, 0, 128)]
#[case(360.0, 100.0, 50.0, 255, 0, 0)]
fn test_hsl_colors_comprehensive(h: f32, s: f32, l: f32, r: u8, g: u8, b: u8) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let actual = "test".hsl(h, s, l).to_string();
    let expected = "test".rgb(r, g, b).to_string();
    assert_rgb_approx_eq(&actual, &expected);
}

#[test]
fn test_hsl_edge_cases() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);

    let assert_hsl_rgb = |h, s, l, r, g, b| {
        let actual = "test".hsl(h, s, l).to_string();
        let expected = "test".rgb(r, g, b).to_string();
        assert_rgb_approx_eq(&actual, &expected);
    };

    assert_hsl_rgb(0.0, 0.0, 0.0, 0, 0, 0);
    assert_hsl_rgb(0.0, 0.0, 25.0, 64, 64, 64);
    assert_hsl_rgb(0.0, 0.0, 50.0, 128, 128, 128);
    assert_hsl_rgb(0.0, 0.0, 75.0, 191, 191, 191);
    assert_hsl_rgb(0.0, 0.0, 100.0, 255, 255, 255);

    assert_hsl_rgb(0.0, 25.0, 50.0, 159, 96, 96);
    assert_hsl_rgb(0.0, 50.0, 50.0, 191, 64, 64);
    assert_hsl_rgb(0.0, 75.0, 50.0, 223, 32, 32);

    assert_hsl_rgb(120.0, 100.0, 25.0, 0, 128, 0);
    assert_hsl_rgb(120.0, 100.0, 75.0, 128, 255, 128);
}

#[test]
fn test_hsl_background_colors() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let actual = "test".on_hsl(0.0, 100.0, 50.0).to_string();
    let expected = "test".on_rgb(255, 0, 0).to_string();
    assert_rgb_approx_eq(&actual, &expected);

    let actual = "test".on_hsl(120.0, 100.0, 50.0).to_string();
    let expected = "test".on_rgb(0, 255, 0).to_string();
    assert_rgb_approx_eq(&actual, &expected);

    let actual = "test".on_hsl(240.0, 100.0, 50.0).to_string();
    let expected = "test".on_rgb(0, 0, 255).to_string();
    assert_rgb_approx_eq(&actual, &expected);
}

#[test]
#[should_panic(expected = "Invalid ANSI color sequence")]
fn test_assert_rgb_approx_eq_invalid_sequence() {
    assert_rgb_approx_eq("invalid", "also invalid");
}

#[test]
#[should_panic(expected = "RGB values differ by more than 1: (255, 0, 0) vs (252, 0, 0)")]
fn test_assert_rgb_approx_eq_large_diff() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let color1 = "test".rgb(255, 0, 0).to_string();
    let color2 = "test".rgb(252, 0, 0).to_string();
    assert_rgb_approx_eq(&color1, &color2);
}

#[rstest(hsl)]
#[case((-1., 100., 50.))]
#[case((361., 100., 50.))]
#[case((0., -1., 50.))]
#[case((0., 101., 50.))]
#[case((0., 100., -1.))]
#[case((0., 100., 101.))]
fn out_of_range_hsl_clears_styling(hsl: (f32, f32, f32)) {
    assert_invalid_hsl_clears_styling(hsl);
}

#[rstest]
fn non_finite_hsl_clears_styling(
    #[values(0, 1, 2)] component: usize,
    #[values(f32::NAN, f32::INFINITY, f32::NEG_INFINITY)] value: f32,
) {
    let mut hsl = [0., 100., 50.];
    hsl[component] = value;
    assert_invalid_hsl_clears_styling((hsl[0], hsl[1], hsl[2]));
}

fn assert_invalid_hsl_clears_styling(hsl: (f32, f32, f32)) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let (h, s, l) = hsl;
    assert_eq!("test".hsl(h, s, l).to_string(), "test");
    assert_eq!("test".on_hsl(h, s, l).to_string(), "test");
    let styled = "test".colorize("4").bold().red().on_blue();
    assert_eq!(styled.clone().hsl(h, s, l).to_string(), "test");
    assert_eq!(styled.on_hsl(h, s, l).to_string(), "test");
}
