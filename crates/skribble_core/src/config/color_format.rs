use serde::Deserialize;
use serde::Serialize;
use skribble_color::Color;

use crate::wrap_css_variable;
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
