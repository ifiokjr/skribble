use std::f32::consts::PI;
use std::fmt::Display;
use std::str::FromStr;

use lazy_static::lazy_static;
use palette::rgb::Rgba;
use palette::FromColor;
use palette::Hsla;
use palette::RgbHue;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use crate::CssVariable;
use crate::Error;
use crate::Placeholder;
use crate::RunnerConfig;

/// ColorFormat is used to determine the default format of the colors.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ColorFormat {
  #[serde(rename = "rgb")]
  Rgb,
  #[serde(rename = "hsl")]
  #[default]
  Hsl,
}

impl ColorFormat {
  /// Doesn't currently check if this is a color.
  pub fn get_color_value_with_opacity(
    &self,
    config: &RunnerConfig,
    css_variable: &CssVariable,
  ) -> crate::Result<String> {
    let prefix = &config.options().variable_prefix;
    let initial_value = css_variable
      .value
      .as_ref()
      .map(|value| Placeholder::normalize(value, config))
      .ok_or(Error::InvalidCssVariable(css_variable.name.clone()))?;
    let opacity_variable = css_variable.get_opacity_variable(prefix);

    match self {
      Self::Rgb => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_rgb()
          .to_string_with_opacity(opacity_variable);

        Ok(color)
      }
      Self::Hsl => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_hsl()
          .to_string_with_opacity(opacity_variable);

        Ok(color)
      }
    }
  }
}

impl AsRef<str> for ColorFormat {
  fn as_ref(&self) -> &str {
    match self {
      Self::Rgb => "rgb",
      Self::Hsl => "hsl",
    }
  }
}

impl<T: Into<String>> From<T> for ColorFormat {
  fn from(value: T) -> Self {
    match value.into().as_str() {
      "rgb" => Self::Rgb,
      "hsl" => Self::Hsl,
      _ => Self::Hsl,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
  Rgb(Rgba),
  Hsl(Hsla),
}

impl Color {
  /// Returns the color as an RGB value.
  pub fn into_rgb(self) -> Self {
    match self {
      Self::Rgb(_) => self,
      Self::Hsl(hsla) => Self::Rgb(Rgba::from_color(hsla)),
    }
  }

  /// Returns the color as an HSL value.
  pub fn into_hsl(self) -> Self {
    match self {
      Self::Rgb(rgb) => Self::Hsl(Hsla::from_color(rgb)),
      Self::Hsl(_) => self,
    }
  }

  pub fn to_string_with_opacity(&self, opacity_variable: impl AsRef<str>) -> String {
    let opacity_variable = opacity_variable.as_ref();

    match self {
      Self::Rgb(ref rgba) => rgb_to_string(rgba, Some(opacity_variable)),
      Self::Hsl(ref hsla) => hsl_to_string(hsla, Some(opacity_variable)),
    }
  }
}

impl Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Rgb(ref rgba) => write!(f, "{}", rgb_to_string::<String>(rgba, None)),
      Self::Hsl(ref hsla) => write!(f, "{}", hsl_to_string::<String>(hsla, None)),
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

    match from_hsl_string(value) {
      Ok(hsl) => return Ok(Self::Hsl(hsl)),
      Err(err) => {
        match err {
          ColorError::Invalid(_) => return Err(err),
          _ => {}
        }
      }
    };

    Err(ColorError::Unknown)
  }
}

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
  static ref HSL_REGEX: Regex = Regex::new(r"(?i)\s*(?:hsl\(\s*(?P<h1>\d+(?:\.\d+)?)(?P<u1>deg|grad|rad|turn)?\s*(?P<s1>\d+(?:\.\d+)?)%\s*(?P<l1>\d+(?:\.\d+)?)%\s*(?:/\s*(?:(?P<a1>\d+(?:\.\d+)?|\.\d+)|(?P<pc1>\d+(?:\.\d+)?|\.\d+)%))?\s*\)|hsla\(\s*(?P<h2>\d+(?:\.\d+)?)(?P<u2>deg|grad|rad|turn)?\s*,\s*(?P<s2>\d+(?:\.\d+)?)%\s*,\s*(?P<l2>\d+(?:\.\d+)?)%\s*,\s*(?:(?P<a2>\d+(?:\.\d+)?|\.\d+)|(?P<pc2>\d+(?:\.\d+)?|\.\d+)%)?\s*\)|hsl\(\s*(?P<h3>\d+(?:\.\d+)?)(?P<u3>deg|grad|rad|turn)?\s*,\s*(?P<s3>\d+(?:\.\d+)?)%\s*,\s*(?P<l3>\d+(?:\.\d+)?)%\s*\))\s*").unwrap();
}

fn from_hsl_string(value: &str) -> Result<Hsla, ColorError> {
  match HSL_REGEX.captures(value) {
    Some(capture) => {
      if let Some((r, g, b)) = capture
        .name("h1")
        .zip(capture.name("s1"))
        .zip(capture.name("l1"))
        .map(|((h, s), l)| (h, s, l))
      {
        let hue = f32::from_str(r.as_str()).map_err(ColorError::from)?;
        let saturation = f32::from_str(g.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 100.0)
          / 100.0;
        let lightness = f32::from_str(b.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 100.0)
          / 100.0;

        let unit = if let Some(unit) = capture.name("u1") {
          unit.as_str()
        } else {
          "deg"
        };

        let hue = match unit {
          "deg" => RgbHue::from_degrees(hue),
          "grad" => RgbHue::from_radians(hue * PI / 200.0),
          "rad" => RgbHue::from_radians(hue),
          "turn" => RgbHue::from_degrees(hue * 360.0),
          _ => return Err(ColorError::Invalid(format!("hue units: `{unit}`"))),
        };

        let alpha = if let Some(alpha) = capture.name("a1") {
          f32::from_str(alpha.as_str())?.clamp(0.0, 1.0)
        } else if let Some(percentage) = capture.name("pc1") {
          f32::from_str(percentage.as_str())?.clamp(0.0, 100.0) / 100.0
        } else {
          1.0
        };

        return Ok(Hsla::new(hue, saturation, lightness, alpha));
      }

      if let Some((r, g, b)) = capture
        .name("h2")
        .zip(capture.name("s2"))
        .zip(capture.name("l2"))
        .map(|((h, s), l)| (h, s, l))
      {
        let hue = f32::from_str(r.as_str()).map_err(ColorError::from)?;
        let saturation = f32::from_str(g.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 100.0)
          / 100.0;
        let lightness = f32::from_str(b.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 100.0)
          / 100.0;

        let unit = if let Some(unit) = capture.name("u2") {
          unit.as_str()
        } else {
          "deg"
        };

        let hue = match unit {
          "deg" => RgbHue::from_degrees(hue),
          "grad" => RgbHue::from_radians(hue * PI / 200.0),
          "rad" => RgbHue::from_radians(hue),
          "turn" => RgbHue::from_degrees(hue * 360.0),
          _ => return Err(ColorError::Invalid(format!("hue units: `{unit}`"))),
        };

        let alpha = if let Some(alpha) = capture.name("a2") {
          f32::from_str(alpha.as_str())?.clamp(0.0, 1.0)
        } else if let Some(percentage) = capture.name("pc2") {
          f32::from_str(percentage.as_str())?.clamp(0.0, 100.0) / 100.0
        } else {
          1.0
        };

        return Ok(Hsla::new(hue, saturation, lightness, alpha));
      }

      if let Some((r, g, b)) = capture
        .name("h3")
        .zip(capture.name("s3"))
        .zip(capture.name("l3"))
        .map(|((h, s), l)| (h, s, l))
      {
        let hue = f32::from_str(r.as_str()).map_err(ColorError::from)?;
        let saturation = f32::from_str(g.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 100.0)
          / 100.0;
        let lightness = f32::from_str(b.as_str())
          .map_err(ColorError::from)?
          .clamp(0.0, 100.0)
          / 100.0;

        let unit = if let Some(unit) = capture.name("u3") {
          unit.as_str()
        } else {
          "deg"
        };

        let hue = match unit {
          "deg" => RgbHue::from_degrees(hue),
          "grad" => RgbHue::from_radians(hue * PI / 200.0),
          "rad" => RgbHue::from_radians(hue),
          "turn" => RgbHue::from_degrees(hue * 360.0),
          _ => return Err(ColorError::Invalid(format!("hue units: `{unit}`"))),
        };

        let alpha = if let Some(alpha) = capture.name("a3") {
          f32::from_str(alpha.as_str())?.clamp(0.0, 1.0)
        } else if let Some(percentage) = capture.name("pc3") {
          f32::from_str(percentage.as_str())?.clamp(0.0, 100.0) / 100.0
        } else {
          1.0
        };

        return Ok(Hsla::new(hue, saturation, lightness, alpha));
      }

      Err(ColorError::Invalid("hsl".into()))
    }

    None => {
      if value.trim().starts_with("hsl") {
        Err(ColorError::Invalid("hsl".into()))
      } else {
        Err(ColorError::Unknown)
      }
    }
  }
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

        let red = red / 255.0;
        let green = green / 255.0;
        let blue = blue / 255.0;

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

        let red = red / 255.0;
        let green = green / 255.0;
        let blue = blue / 255.0;

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

        let red = red / 255.0;
        let green = green / 255.0;
        let blue = blue / 255.0;

        return Ok(Rgba::new(red, green, blue, 1.0));
      }

      Err(ColorError::Invalid("rgb".into()))
    }

    None => {
      if value.trim().starts_with("rgb") {
        Err(ColorError::Invalid("rgb".into()))
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
      if value.trim().starts_with('#') {
        Err(ColorError::Invalid("hex".into()))
      } else {
        Err(ColorError::Unknown)
      }
    }
  }
}

fn rgb_to_string<T: AsRef<str>>(rgba: &Rgba, opacity: Option<T>) -> String {
  let is_alpha = opacity.is_some() || rgba.alpha != 1.0;
  let red = (rgba.red * 255.0) as u8;
  let green = (rgba.green * 255.0) as u8;
  let blue = (rgba.blue * 255.0) as u8;
  let alpha = opacity
    .map(|v| v.as_ref().to_string())
    .unwrap_or(rgba.alpha.to_string());

  if !is_alpha {
    return format!("rgb({red}, {green}, {blue})");
  }

  format!("rgba({red}, {green}, {blue}, {alpha})",)
}

fn hsl_to_string<T: AsRef<str>>(hsla: &Hsla, opacity: Option<T>) -> String {
  let is_alpha = opacity.is_some() || hsla.alpha != 1.0;
  let hue = hsla.hue.to_positive_degrees();
  let saturation = hsla.saturation * 100.0;
  let lightness = hsla.lightness * 100.0;
  let alpha = opacity
    .map(|v| v.as_ref().to_string())
    .unwrap_or(hsla.alpha.to_string());

  if !is_alpha {
    return format!("hsl({hue}, {saturation}%, {lightness}%)");
  }

  format!("hsla({hue}, {saturation}%, {lightness}%, {alpha})",)
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
}
