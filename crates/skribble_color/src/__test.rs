use crate::Color;

#[test]
fn from_hex_3() {
  let r = "#fff";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgb(255, 255, 255)");
}

#[test]
fn from_hex_4() {
  let r = "#fff6";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgba(255, 255, 255, 0.4)");
}

#[test]
fn from_hex_6() {
  let r = "#b1ffb0";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgb(177, 255, 176)");
}

#[test]
fn from_hex_8() {
  let r = "#b1ffb0b7";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgba(177, 255, 176, 0.7176471)");
}

#[test]
fn invalid_hex() {
  let r = "#1ffb0b7";
  let error = r.parse::<Color>().unwrap_err();
  insta::assert_debug_snapshot!(error, @r###"
    Invalid(
        "hex",
    )
    "###);
}

#[test]
fn from_rgb() {
  let r = "rgb(100, 2, 41)";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgb(100, 2, 41)");
}

#[test]
fn from_rgba() {
  let r = "rgba(100, 2, 41, 0.5)";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgba(100, 2, 41, 0.5)");
}

#[test]
fn from_rgba_no_zero_in_alpha() {
  let r = "rgba(100, 2, 41, .5)";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgba(100, 2, 41, 0.5)");
}

#[test]
fn from_rgba_no_zero_in_percentage() {
  let r = "rgba(100, 2, 41, .5%)";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgba(100, 2, 41, 0.005)");
}

#[test]
fn from_rgba_percentage() {
  let r = "rgba(100, 2, 41, 50%)";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgba(100, 2, 41, 0.5)");
}

#[test]
fn from_rgb_css() {
  let r = "rgb(100 2 41 / 0.5)";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgba(100, 2, 41, 0.5)");
}

#[test]
fn from_rgb_css_alpha1() {
  let r = "rgb(100 2 41 / 1)";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgb(100, 2, 41)");
}

#[test]
fn from_rgb_css_simple() {
  let r = "rgb(100 2 41)";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgb(100, 2, 41)");
}

#[test]
fn from_rgb_css_percentage() {
  let r = "rgb(100 2 41 / 50%)";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgba(100, 2, 41, 0.5)");
}

#[test]
fn from_rgb_css_no_leading_zero_in_alpha() {
  let r = "rgb(100 2 41 / .50)";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgba(100, 2, 41, 0.5)");
}

#[test]
fn from_rgb_css_no_leading_zero_in_percentage() {
  let r = "rgb(100 2 41 / .50%)";
  let rgb: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(rgb, @"rgba(100, 2, 41, 0.005)");
}

#[test]
fn invalid_rgb() {
  let r = "rgba(100, 2 41 / 0.5)";
  let error = r.parse::<Color>().unwrap_err();
  insta::assert_debug_snapshot!(error, @r###"
    Invalid(
        "rgb",
    )
    "###);
}
#[test]
fn from_hsl() {
  let r = "hsl(100, 50%, 50%)";
  let hsl: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(hsl, @"hsl(100, 50%, 50%)");
}

#[test]
fn from_hsla() {
  let r = "hsla(100, 50%, 50%, 0.5)";
  let hsl: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(hsl, @"hsla(100, 50%, 50%, 0.5)");
}

#[test]
fn from_hsla_no_zero_in_alpha() {
  let r = "hsla(100, 50%, 50%, .5)";
  let hsl: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(hsl, @"hsla(100, 50%, 50%, 0.5)");
}

#[test]
fn from_hsla_no_zero_in_percentage() {
  let r = "hsla(100, 50%, 50%, .5%)";
  let hsl: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(hsl, @"hsla(100, 50%, 50%, 0.005)");
}

#[test]
fn from_hsla_percentage() {
  let r = "hsla(100, 50%, 50%, 50%)";
  let hsl: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(hsl, @"hsla(100, 50%, 50%, 0.5)");
}

#[test]
fn from_hsl_css() {
  let r = "hsl(100 50% 50% / 0.5)";
  let hsl: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(hsl, @"hsla(100, 50%, 50%, 0.5)");
}

#[test]
fn from_hsl_css_alpha1() {
  let r = "hsl(100 50% 50% / 1)";
  let hsl: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(hsl, @"hsl(100, 50%, 50%)");
}

#[test]
fn from_hsl_css_simple() {
  let r = "hsl(100 50% 50%)";
  let hsl: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(hsl, @"hsl(100, 50%, 50%)");
}

#[test]
fn from_hsl_css_percentage() {
  let r = "hsl(100 50% 50% / 50%)";
  let hsl: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(hsl, @"hsla(100, 50%, 50%, 0.5)");
}

#[test]
fn from_hsl_css_no_leading_zero_in_alpha() {
  let r = "hsl(100 50% 50% / .50)";
  let hsl: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(hsl, @"hsla(100, 50%, 50%, 0.5)");
}

#[test]
fn from_hsl_css_no_leading_zero_in_percentage() {
  let r = "hsl(100 50% 50% / .50%)";
  let hsl: Color = r.parse().unwrap();
  insta::assert_display_snapshot!(hsl, @"hsla(100, 50%, 50%, 0.005)");
}

#[test]
fn invalid_hsl() {
  let r = "hsl(100, 50% 50% / 0.5)";
  let error = r.parse::<Color>().unwrap_err();
  insta::assert_debug_snapshot!(error, @r###"
    Invalid(
        "hsl",
    )
    "###);
}
