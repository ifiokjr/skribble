use std::fmt::Display;

use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;

use crate::data::OPEN_COLOR_PALETTE;
use crate::data::TAILWIND_PALETTE;

/// Choose how `light` and `dark` mode are handled. via the media query
/// `prefers-color-scheme` or `class` based.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DarkMode {
	Class,
	#[default]
	Media,
}

impl<S: Into<String>> From<S> for DarkMode {
	fn from(value: S) -> Self {
		match value.into().as_str() {
			"class" => DarkMode::Class,
			"media" => DarkMode::Media,
			_ => DarkMode::Media,
		}
	}
}

impl Display for DarkMode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DarkMode::Class => write!(f, "class"),
			DarkMode::Media => write!(f, "media"),
		}
	}
}

/// The color palette to use.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum PaletteType {
	OpenColor,
	#[default]
	Tailwind,
}

impl PaletteType {
	pub fn palette(&self) -> IndexMap<String, String> {
		match self {
			PaletteType::OpenColor => {
				OPEN_COLOR_PALETTE
					.clone()
					.into_iter()
					.map(|(k, v)| (k.into(), v.into()))
					.collect()
			}
			PaletteType::Tailwind => {
				TAILWIND_PALETTE
					.clone()
					.into_iter()
					.map(|(k, v)| (k.into(), v.into()))
					.collect()
			}
		}
	}
}

impl<S: Into<String>> From<S> for PaletteType {
	fn from(s: S) -> Self {
		match s.into().as_str() {
			"openColor" => PaletteType::OpenColor,
			"tailwind" => PaletteType::Tailwind,
			_ => PaletteType::Tailwind,
		}
	}
}

impl Display for PaletteType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PaletteType::OpenColor => write!(f, "openColor"),
			PaletteType::Tailwind => write!(f, "tailwind"),
		}
	}
}
