use std::fmt::Display;

use heck::ToLowerCamelCase;
use serde::Deserialize;
use serde::Serialize;

pub(crate) const ERIC_MEYER_CSS: &str = include_str!("./eric_meyer.css");
pub(crate) const NORMALIZE_CSS: &str = include_str!("./normalize.css");
pub(crate) const SANITIZE_CSS: &str = include_str!("./sanitize.css");
pub(crate) const TAILWIND_CSS: &str = include_str!("./tailwind.css");
pub(crate) const TAILWIND_COMPAT_CSS: &str = include_str!("./tailwind_compat.css");

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CssReset {
  EricMeyer,
  Normalize,
  Sanitize,
  Tailwind,
  #[default]
  TailwindCompat,
}

impl CssReset {
  pub fn get_css(&self) -> &'static str {
    match self {
      Self::EricMeyer => ERIC_MEYER_CSS,
      Self::Normalize => NORMALIZE_CSS,
      Self::Sanitize => SANITIZE_CSS,
      Self::Tailwind => TAILWIND_CSS,
      Self::TailwindCompat => TAILWIND_COMPAT_CSS,
    }
  }
}

impl Display for CssReset {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::EricMeyer => write!(f, "ericMeyer"),
      Self::Normalize => write!(f, "normalize"),
      Self::Sanitize => write!(f, "sanitize"),
      Self::Tailwind => write!(f, "tailwind"),
      Self::TailwindCompat => write!(f, "tailwindCompat"),
    }
  }
}

impl<S: AsRef<str>> From<S> for CssReset {
  fn from(s: S) -> Self {
    match s.as_ref().to_lower_camel_case().as_str() {
      "ericMeyer" => Self::EricMeyer,
      "normalize" => Self::Normalize,
      "sanitize" => Self::Sanitize,
      "tailwind" => Self::Tailwind,
      "tailwindCompat" => Self::TailwindCompat,
      _ => Self::TailwindCompat,
    }
  }
}
