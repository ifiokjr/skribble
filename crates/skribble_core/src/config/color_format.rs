use serde::Deserialize;
use serde::Serialize;
use skribble_color::Color;

use crate::CssVariable;
use crate::Error;
use crate::Placeholder;
use crate::Result;
use crate::RunnerConfig;

/// ColorFormat is used to determine the default format of the colors. The
/// integration with lightning css may cause this to be overridden in the
/// generated css.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ColorFormat {
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
}

impl AsRef<str> for ColorFormat {
  fn as_ref(&self) -> &str {
    match self {
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
