use serde::Deserialize;
use serde::Serialize;
use skribble_color::Color;
use skribble_color::ColorError;
use skribble_color::Hsla;

use crate::Error;
use crate::Result;

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

	pub fn get_hsla(&self, value: impl AsRef<str>) -> Result<Hsla> {
		let color = self.get_color(value)?;
		color
			.into_hsl()
			.get_hsl()
			.copied()
			.ok_or(Error::Color(ColorError::InvalidHsl))
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
