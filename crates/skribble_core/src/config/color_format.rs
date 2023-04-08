use serde::Deserialize;
use serde::Serialize;
use skribble_color::Color;

use crate::CssVariable;
use crate::Error;
use crate::Options;
use crate::Placeholder;
use crate::Result;
use crate::RunnerConfig;

/// ColorFormat is used to determine the default format of the colors. The
/// integration with lightning css may cause this to be overridden in the
/// generated css.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ColorFormat {
  #[serde(rename = "hex")]
  Hex,
  #[serde(rename = "rgb")]
  Rgb,
  #[serde(rename = "hsl")]
  #[default]
  Hsl,
  #[serde(rename = "hwb")]
  Hwb,
  #[serde(rename = "lch")]
  Lch,
  #[serde(rename = "oklch")]
  Oklch,
  #[serde(rename = "lab")]
  Lab,
  #[serde(rename = "oklab")]
  Oklab,
}

impl ColorFormat {
  pub fn get_color(&self, value: impl AsRef<str>) -> Result<Color> {
    let value = value.as_ref();

    match self {
      Self::Hex => {
        let color = value.parse::<Color>().map_err(Error::from)?.into_hex();
        Ok(color)
      }
      Self::Rgb => {
        let color = value.parse::<Color>().map_err(Error::from)?.into_rgb();
        Ok(color)
      }
      Self::Hsl => {
        let color = value.parse::<Color>().map_err(Error::from)?.into_hsl();
        Ok(color)
      }
      Self::Hwb => {
        let color = value.parse::<Color>().map_err(Error::from)?.into_hwb();
        Ok(color)
      }
      Self::Lch => {
        let color = value.parse::<Color>().map_err(Error::from)?.into_lch();
        Ok(color)
      }
      Self::Oklch => {
        let color = value.parse::<Color>().map_err(Error::from)?.into_oklch();
        Ok(color)
      }
      Self::Lab => {
        let color = value.parse::<Color>().map_err(Error::from)?.into_lab();
        Ok(color)
      }
      Self::Oklab => {
        let color = value.parse::<Color>().map_err(Error::from)?.into_oklab();
        Ok(color)
      }
    }
  }

  pub fn get_normalized_color(
    &self,
    config: &RunnerConfig,
    css_variable: &CssVariable,
    initial_value: Option<&String>,
  ) -> Result<Color> {
    let initial_value = if let Some(initial_value) = initial_value {
      initial_value.clone()
    } else {
      css_variable
        .value
        .as_ref()
        .map(|value| Placeholder::normalize(value, config))
        .ok_or(Error::InvalidCssVariable(css_variable.name.clone()))?
    };

    self.get_color(initial_value)
  }

  /// Get the inner color of the color format.
  ///
  /// Go from `hsl(100 10% 40% / 0.9)` to `100 10% 40%`
  pub fn get_inner_color(&self, value: impl AsRef<str>) -> Result<String> {
    let value = value.as_ref();
    let color = if self == &Self::Hex {
      Self::Rgb.get_color(value)?.to_string()
    } else {
      self.get_color(value)?.to_string()
    };

    color
      .split('/')
      .next()
      .and_then(|value| value.split('(').nth(1))
      .and_then(|value| value.get(0..value.len() - 1))
      .map(|value| value.trim().to_string())
      .ok_or(Error::InnerColor)
  }

  /// Get the color value with the parts and opacity.
  pub fn get_color_with_parts_and_opacity(
    &self,
    variable: &CssVariable,
    options: &Options,
  ) -> String {
    let prefix = match self {
      Self::Hex => "rgb",
      Self::Rgb => "rgb",
      Self::Hsl => "hsl",
      Self::Hwb => "hwb",
      Self::Lch => "lch",
      Self::Oklch => "oklch",
      Self::Lab => "lab",
      Self::Oklab => "oklab",
    };
    let color = variable.get_wrapped_color_variable(options, None);
    let opacity = variable.get_wrapped_opacity_variable(options, None);

    format!("{}({} / {})", prefix, color, opacity)
  }
}

impl AsRef<str> for ColorFormat {
  fn as_ref(&self) -> &str {
    match self {
      Self::Hex => "hex",
      Self::Rgb => "rgb",
      Self::Hsl => "hsl",
      Self::Hwb => "hwb",
      Self::Lch => "lch",
      Self::Oklch => "oklch",
      Self::Lab => "lab",
      Self::Oklab => "oklab",
    }
  }
}

impl<T: Into<String>> From<T> for ColorFormat {
  fn from(value: T) -> Self {
    match value.into().as_str() {
      "rgb" => Self::Rgb,
      "hsl" => Self::Hsl,
      "hwb" => Self::Hwb,
      "lch" => Self::Lch,
      "oklch" => Self::Oklch,
      "lab" => Self::Lab,
      "oklab" => Self::Oklab,
      _ => Self::Hsl,
    }
  }
}
