use serde::Deserialize;
use serde::Serialize;
use skribble_color::Color;

use crate::wrap_css_variable;
use crate::CssVariable;
use crate::Error;
use crate::Placeholder;
use crate::RunnerConfig;

/// ColorFormat is used to determine the default format of the colors. The
/// integration with lightning css may cause this to be overridden in the
/// generated css.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ColorFormat {
  /// Produce the color in hex format.
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
  /// Doesn't currently check if this is a color.
  pub fn get_color_value(
    &self,
    config: &RunnerConfig,
    css_variable: &CssVariable,
    initial_value: Option<&String>,
  ) -> crate::Result<String> {
    let initial_value = if let Some(initial_value) = initial_value {
      initial_value.clone()
    } else {
      css_variable
        .value
        .as_ref()
        .map(|value| Placeholder::normalize(value, config))
        .ok_or(Error::InvalidCssVariable(css_variable.name.clone()))?
    };

    match self {
      Self::Hex => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_hex()
          .to_string();

        Ok(color)
      }
      Self::Rgb => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_rgb()
          .to_string();

        Ok(color)
      }
      Self::Hsl => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_hsl()
          .to_string();

        Ok(color)
      }
      Self::Hwb => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_hwb()
          .to_string();

        Ok(color)
      }
      Self::Lch => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_lch()
          .to_string();

        Ok(color)
      }
      Self::Oklch => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_oklch()
          .to_string();

        Ok(color)
      }
      Self::Lab => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_lab()
          .to_string();

        Ok(color)
      }
      Self::Oklab => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_oklab()
          .to_string();

        Ok(color)
      }
    }
  }

  pub fn get_color_value_with_opacity(
    &self,
    config: &RunnerConfig,
    css_variable: &CssVariable,
    initial_value: Option<&String>,
  ) -> crate::Result<String> {
    let initial_value = if let Some(initial_value) = initial_value {
      initial_value.clone()
    } else {
      css_variable
        .value
        .as_ref()
        .map(|value| Placeholder::normalize(value, config))
        .ok_or(Error::InvalidCssVariable(css_variable.name.clone()))?
    };
    let opacity_variable = css_variable.get_opacity_variable(config.options());

    match self {
      Self::Hex => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_hex()
          .to_string_with_opacity(wrap_css_variable(opacity_variable, None));

        Ok(color)
      }
      Self::Rgb => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_rgb()
          .to_string_with_opacity(wrap_css_variable(opacity_variable, None));

        Ok(color)
      }
      Self::Hsl => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_hsl()
          .to_string_with_opacity(wrap_css_variable(opacity_variable, None));

        Ok(color)
      }
      Self::Hwb => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_hwb()
          .to_string_with_opacity(wrap_css_variable(opacity_variable, None));

        Ok(color)
      }
      Self::Lch => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_lch()
          .to_string_with_opacity(wrap_css_variable(opacity_variable, None));

        Ok(color)
      }
      Self::Oklch => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_oklch()
          .to_string_with_opacity(wrap_css_variable(opacity_variable, None));

        Ok(color)
      }
      Self::Lab => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_lab()
          .to_string_with_opacity(wrap_css_variable(opacity_variable, None));

        Ok(color)
      }
      Self::Oklab => {
        let color = initial_value
          .parse::<Color>()
          .map_err(Error::from)?
          .into_oklab()
          .to_string_with_opacity(wrap_css_variable(opacity_variable, None));

        Ok(color)
      }
    }
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
      "hex" => Self::Hex,
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
