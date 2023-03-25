use std::fmt::Display;
use std::str::FromStr;

use lazy_static::lazy_static;
use palette::rgb::Rgba;
use palette::Hsla;
// use palette::Hwba;
// use palette::Laba;
// use palette::Lcha;
// use palette::Oklaba;
// use palette::Oklcha;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use crate::CssVariable;

/// ColorFormat is used to determine the default format of the colors.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ColorFormat {
  #[serde(rename = "rgb")]
  Rgb,
  #[serde(rename = "hex")]
  Hex,
  #[serde(rename = "hsl")]
  #[default]
  Hsl,
  // #[serde(rename = "lch")]
  // Lch,
}

impl ColorFormat {
  pub fn get_format(color: impl AsRef<str>) -> Option<Self> {
    // let color = color.as_ref();

    // if let Ok(_a) = color.parse::<Hsl>() {
    //   Some(Self::Hsl)
    // } else if let Ok(_a) = color.parse::<Rgb>() {
    //   Some(Self::Rgb)
    // } else {
    //   None
    // }

    None
  }

  /// Doesn't currently check if this is a color.
  pub fn get_color_value_with_opacity(&self, css_variable: CssVariable) {}
}

impl AsRef<str> for ColorFormat {
  fn as_ref(&self) -> &str {
    match self {
      Self::Rgb => "rgb",
      Self::Hex => "hex",
      Self::Hsl => "hsl",
      // Self::Lch => "lch",
    }
  }
}

impl<T: Into<String>> From<T> for ColorFormat {
  fn from(value: T) -> Self {
    match value.into().as_str() {
      "rgb" => Self::Rgb,
      "hex" => Self::Hex,
      "hsl" => Self::Hsl,
      // "lch" => Self::Lch,
      _ => Self::Hsl,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
  Rgb(Rgba),
  Hsl(Hsla),
  // Hwb(Hwba),
  // Lch(Lcha),
  // Oklch(Oklcha),
  // Lab(Laba),
  // Oklab(Oklaba),
}

impl Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Rgb(ref rgba) => write!(f, "{}", rgb_to_string(rgba)),
      Self::Hsl(ref hsla) => write!(f, "{}", hsl_to_string(hsla)),
      // Self::Hwb(hwba) => write!(f, "{}", "hwba"),
      // Self::Lch(lcha) => write!(f, "{}", "lcha"),
      // Self::Oklch(oklcha) => write!(f, "{}", "oklcha"),
      // Self::Lab(laba) => write!(f, "{}", "laba"),
      // Self::Oklab(oklaba) => write!(f, "{}", "oklaba"),
    }
  }
}

impl FromStr for Color {
  type Err = ColorError;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match from_hex_string(value) {
      Ok(rgba) => return Ok(Self::Rgb(rgba)),
      Err(err) => {
        match err {
          ColorError::Invalid(_) => return Err(err),
          _ => {}
        }
      }
    };

    match from_rgb_string(value) {
      Ok(rgba) => return Ok(Self::Rgb(rgba)),
      Err(err) => {
        match err {
          ColorError::Invalid(_) => return Err(err),
          _ => {}
        }
      }
    };

    return Err(ColorError::Unknown);
  }
}

impl Color {}

#[derive(thiserror::Error, Debug)]
pub enum ColorError {
  #[error("could not match the color format")]
  Unknown,

  #[error("unable to parse the integer")]
  ParseIntError(#[from] std::num::ParseIntError),

  #[error("unable to parse the floating point number")]
  ParseFloatError(#[from] std::num::ParseFloatError),

  #[error("invalid `{0}` format")]
  Invalid(String),
}

lazy_static! {
  static ref HEX_REGEX: Regex = Regex::new(r"^(?i)\s*#(?:(?P<r1>[a-f0-9])(?P<g1>[a-f0-9])(?P<b1>[a-f0-9])(?P<a1>[a-f0-9])?|(?P<r2>[a-f0-9]{2})(?P<g2>[a-f0-9]{2})(?P<b2>[a-f0-9]{2})(?P<a2>[a-f0-9]{2})?)\s*$").unwrap();
  static ref RGB_REGEX: Regex = Regex::new(r"(?i)\s*(?:rgb\(\s*(?P<r1>\d+(?:\.\d+)?)\s*(?P<g1>\d+(?:\.\d+)?)\s*(?P<b1>\d+(?:\.\d+)?)\s*(?:/\s*(?:(?P<a1>\d+(?:\.\d+)?|\.\d+)|(?P<pc1>\d+(?:\.\d+)?|\.\d+)%))?\s*\)|rgba\(\s*(?P<r2>\d+(?:\.\d+)?)\s*,\s*(?P<g2>\d+(?:\.\d+)?)\s*,\s*(?P<b2>\d+(?:\.\d+)?)\s*,\s*(?:(?P<a2>\d+(?:\.\d+)?|\.\d+)|(?P<pc2>\d+(?:\.\d+)?|\.\d+)%)?\s*\)|rgb\(\s*(?P<r3>\d+(?:\.\d+)?)\s*,\s*(?P<g3>\d+(?:\.\d+)?)\s*,\s*(?P<b3>\d+(?:\.\d+)?)\s*\))\s*").unwrap();
}

fn from_rgb_string(value: &str) -> Result<Rgba, ColorError> {
  match RGB_REGEX.captures(value) {
    Some(capture) => {
      if let Some((r, g, b)) = capture
        .name("r1")
        .zip(capture.name("g1"))
        .zip(capture.name("b1"))
        .map(|((r, g), b)| (r, g, b))
      {
        let red = f32::from_str(r.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 255.0);
        let green = f32::from_str(g.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 255.0);
        let blue = f32::from_str(b.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 255.0);

        let alpha = if let Some(alpha) = capture.name("a1") {
          f32::from_str(alpha.as_str())?.clamp(0.0, 1.0)
        } else if let Some(percentage) = capture.name("pc1") {
          f32::from_str(percentage.as_str())?.clamp(0.0, 100.0) / 100.0
        } else {
          1.0
        };

        let red = (red as f32) / 255.0;
        let green = (green as f32) / 255.0;
        let blue = (blue as f32) / 255.0;

        return Ok(Rgba::new(red, green, blue, alpha));
      }

      if let Some((r, g, b)) = capture
        .name("r2")
        .zip(capture.name("g2"))
        .zip(capture.name("b2"))
        .map(|((r, g), b)| (r, g, b))
      {
        let red = f32::from_str(r.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 255.0);
        let green = f32::from_str(g.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 255.0);
        let blue = f32::from_str(b.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 255.0);

        let alpha = if let Some(alpha) = capture.name("a2") {
          f32::from_str(alpha.as_str())?.clamp(0.0, 1.0)
        } else if let Some(percentage) = capture.name("pc2") {
          f32::from_str(percentage.as_str())?.clamp(0.0, 100.0) / 100.0
        } else {
          1.0
        };

        let red = (red as f32) / 255.0;
        let green = (green as f32) / 255.0;
        let blue = (blue as f32) / 255.0;

        return Ok(Rgba::new(red, green, blue, alpha));
      }

      if let Some((r, g, b)) = capture
        .name("r3")
        .zip(capture.name("g3"))
        .zip(capture.name("b3"))
        .map(|((r, g), b)| (r, g, b))
      {
        let red = f32::from_str(r.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 255.0);
        let green = f32::from_str(g.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 255.0);
        let blue = f32::from_str(b.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 255.0);

        let red = (red as f32) / 255.0;
        let green = (green as f32) / 255.0;
        let blue = (blue as f32) / 255.0;

        return Ok(Rgba::new(red, green, blue, 1.0));
      }

      Err(ColorError::Invalid("rgb".into()))
    }

    None => {
      if value.trim().starts_with("rgb") {
        return Err(ColorError::Invalid("rgb".into()));
      } else {
        Err(ColorError::Unknown)
      }
    }
  }
}

fn from_hex_string(value: &str) -> Result<Rgba, ColorError> {
  match HEX_REGEX.captures(value) {
    Some(capture) => {
      if let Some((r, g, b)) = capture
        .name("r1")
        .zip(capture.name("g1"))
        .zip(capture.name("b1"))
        .map(|((r, g), b)| (r, g, b))
      {
        let red = u8::from_str_radix(r.as_str(), 16).map_err(ColorError::from)? * 17;
        let green = u8::from_str_radix(g.as_str(), 16).map_err(ColorError::from)? * 17;
        let blue = u8::from_str_radix(b.as_str(), 16).map_err(ColorError::from)? * 17;

        let alpha = if let Some(value) = capture.name("a1") {
          u8::from_str_radix(value.as_str(), 16).map_err(ColorError::from)? * 17
        } else {
          255
        };

        let red = (red as f32) / 255.0;
        let green = (green as f32) / 255.0;
        let blue = (blue as f32) / 255.0;
        let alpha = (alpha as f32) / 255.0;

        return Ok(Rgba::new(red, green, blue, alpha));
      }

      if let Some(matches) = capture
        .name("r2")
        .zip(capture.name("g2"))
        .zip(capture.name("b2"))
        .map(|((r, g), b)| (r, g, b))
      {
        let red = u8::from_str_radix(matches.0.as_str(), 16).map_err(ColorError::from)?;
        let green = u8::from_str_radix(matches.1.as_str(), 16).map_err(ColorError::from)?;
        let blue = u8::from_str_radix(matches.2.as_str(), 16).map_err(ColorError::from)?;

        let alpha = if let Some(value) = capture.name("a2") {
          u8::from_str_radix(value.as_str(), 16).map_err(ColorError::from)?
        } else {
          255
        };

        println!("ALPHA!! {alpha}");

        let red = (red as f32) / 255.0;
        let green = (green as f32) / 255.0;
        let blue = (blue as f32) / 255.0;
        let alpha = (alpha as f32) / 255.0;

        return Ok(Rgba::new(red, green, blue, alpha));
      }

      Err(ColorError::Invalid("hex".into()))
    }
    None => {
      if value.trim().starts_with("#") {
        return Err(ColorError::Invalid("hex".into()));
      } else {
        Err(ColorError::Unknown)
      }
    }
  }
}

fn rgb_to_string(rgba: &Rgba) -> String {
  let red = (rgba.red * 255.0) as u8;
  let green = (rgba.green * 255.0) as u8;
  let blue = (rgba.blue * 255.0) as u8;
  let alpha = rgba.alpha;

  if alpha == 1.0 {
    return format!("rgb({red}, {green}, {blue})");
  }

  format!("rgba({red}, {green}, {blue}, {alpha})",)
}

fn hsl_to_string(hsla: &Hsla) -> String {
  let hue = hsla.hue.to_positive_degrees();
  let saturation = hsla.saturation * 100.0;
  let lightness = hsla.lightness * 100.0;
  let alpha = hsla.alpha;

  if alpha == 1.0 {
    return format!("hsl({hue}deg, {saturation}%, {lightness}%)");
  }

  format!("hsla({hue}deg, {saturation}%, {lightness}%, {alpha})",)
}

#[cfg(test)]
mod test {

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
}
