use pretty_assertions::assert_eq;
use rstest::*;

use crate::Color;
use crate::ColorError;
use crate::ExtractedParams;

#[rstest]
#[case("20  , 20, 10, 10 ", ("20,20,10,10", true, false, true))]
#[case("20 20   10 /  50", ("20,20,10,50", false, true, true))]
#[case("20,20,10,10", ("20,20,10,10", true, false, true))]
#[case("20,20,10", ("20,20,10", true, false, false))]
#[case("20 20 10", ("20,20,10", false, false, false))]
#[case("20 20 10/10", ("20,20,10,10", false, true, true))]
fn valid_extracted_params(#[case] input: &str, #[case] expected: (&str, bool, bool, bool)) {
  let extracted: ExtractedParams = input.into();
  assert_eq!(extracted.params.join(","), expected.0);
  assert_eq!(extracted.is_comma_separated(), expected.1);
  assert_eq!(extracted.is_slash_separated(), expected.2);
  assert_eq!(extracted.is_alpha(), expected.3);
  assert_eq!(extracted.is_valid(), true);
}

#[rstest]
#[case("20, 20 10 / 10")]
#[case("20, 20, 10  10")]
#[case("20,, 20, 10  10")]
#[case("20, 20, 10, ")]
#[case("20 20 10 // 10")]
#[case("20 / 20 10 10")]
#[case("20 20 10 10/")]
fn invalid_extracted_params(#[case] input: &str) {
  let extracted: ExtractedParams = input.into();
  assert_eq!(extracted.is_valid(), false);
}

#[rstest]
#[case("#fff", "#ffffff")]
#[case("#fff6", "#ffffff66")]
#[case("#b1ffb0", "#b1ffb0")]
#[case("#b1ffb0b7", "#b1ffb0b7")]
#[case("b1ffb0b7", "#b1ffb0b7")]
#[case("rgb(100, 2, 41)", "rgb(100 2 41)")]
#[case("rgb(100 2 41)", "rgb(100 2 41)")]
#[case("rgb(100 2 41 / 0.5)", "rgb(100 2 41 / 0.5)")]
#[case("rgb(100 2 41 / 50%)", "rgb(100 2 41 / 0.5)")]
#[case("rgba(100, 2, 41, 0.5)", "rgb(100 2 41 / 0.5)")]
#[case("rgba(100, 2, 41, .5)", "rgb(100 2 41 / 0.5)")]
#[case("hsl(100, 50%, 50%)", "hsl(100 50% 50%)")]
#[case("hsl(100 50% 50%)", "hsl(100 50% 50%)")]
#[case("hsl(100 50% 50% / 0.5)", "hsl(100 50% 50% / 0.5)")]
#[case("hsl(100 50% 50% / 50%)", "hsl(100 50% 50% / 0.5)")]
#[case("hsla(100, 50%, 50%, 0.5)", "hsl(100 50% 50% / 0.5)")]
#[case("hsla(100, 50%, 50%, .5)", "hsl(100 50% 50% / 0.5)")]
#[case("hwb(100 50% 50%)", "hwb(100 50% 50%)")]
#[case("hwb(100 50% 50% / 0.5)", "hwb(100 50% 50% / 0.5)")]
#[case("hwb(100 50% 50% / 50%)", "hwb(100 50% 50% / 0.5)")]
#[case("lch(61 53.12 259.4)", "lch(61% 35.413334% 259.4)")]
#[case("lch(61 53.12 259.4 / 0.5)", "lch(61% 35.413334% 259.4 / 0.5)")]
#[case("oklch(0.67 0.16 245.55)", "oklch(67% 39.999996% 245.55)")]
#[case("oklch(0.67 0.16 245.55)", "oklch(67% 39.999996% 245.55)")]
#[case("oklch(67% 0.16 245.55)", "oklch(67% 39.999996% 245.55)")]
#[case("oklch(0.67 50% 245.55 / 0.79)", "oklch(67% 50% 245.55 / 0.79)")]
#[case("lab(61 -9.8 -52.2)", "lab(61% -9.800003 -52.199997)")]
#[case("lab(61 -9.8 -52.2 / 0.79)", "lab(61% -9.800003 -52.199997 / 0.79)")]
#[case("lab(61 -9.8 -52.2 / 79%)", "lab(61% -9.800003 -52.199997 / 0.79)")]
fn valid_colors(#[case] input: &str, #[case] expected: &str) {
  let color: Color = input.parse().unwrap();
  assert_eq!(color.to_string(), expected);
}

#[rstest]
#[case("#1", ColorError::InvalidHex)]
#[case("#1b", ColorError::InvalidHex)]
#[case("#1b0b7", ColorError::InvalidHex)]
#[case("#1ffb0b7", ColorError::InvalidHex)]
#[case("#1fz", ColorError::InvalidHex)]
#[case("#1ffb0b7a3", ColorError::InvalidHex)]
#[case("b1ffb0b7a", ColorError::InvalidUnknown)]
#[case("rgb(100, 2 41 / 0.5)", ColorError::InvalidRgb)]
#[case("rgb(100px 2 41 / 0.5)", ColorError::InvalidRgb)]
#[case("hsl(100px 50% 50% / 0.5)", ColorError::InvalidHsl)]
#[case("hwb(100, 50%, 50%, 0.5)", ColorError::InvalidHwb)]
#[case("hwb(100px 50% 50% / 0.5)", ColorError::InvalidHwb)]
fn invalid_colors(#[case] input: &str, #[case] expected: ColorError) {
  let color = input.parse::<Color>();
  assert_eq!(color.unwrap_err(), expected);
}
