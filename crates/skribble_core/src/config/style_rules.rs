use indexmap::IndexMap;
use serde::ser::SerializeMap;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;

use crate::CssValue;

/// `StyleRules` connect all the atomic name to their atomic styles.
///
/// ```json
/// {
///   "p": ["padding"],
///   "py": ["padding-top", "padding-bottom"],
///   "px": ["padding-right", "padding-left"],
///   "pt": ["padding-top"],
///   "pr": ["padding-right"],
///   "pb": ["padding-bottom"],
///   "pl": ["padding-left"],
///   "pbl": ["padding-block"],
///   "pbls": ["padding-block-start"],
///   "pble": ["padding-block-end"],
///   "pin": ["padding-inline"],
///   "pins": ["padding-inline-start"],
///   "pine": ["padding-inline-end"]
/// }
/// ```
///
/// Each of the style rules above maps an atomic style name to a list of CSS
/// properties that it controls. The styles rules are later connected with
/// `AtomicValues` which are passed to each individual style rule.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StyleRules(IndexMap<String, Vec<StyleRule>>);

impl Serialize for StyleRules {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut map = serializer.serialize_map(Some(self.0.len()))?;

    for (key, value) in self.0.iter() {
      map.serialize_entry(key, value)?;
    }

    map.end()
  }
}

impl<'de> Deserialize<'de> for StyleRules {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let map = IndexMap::<String, Vec<StyleRule>>::deserialize(deserializer)?;
    Ok(StyleRules(map))
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, PartialOrd)]
#[serde(untagged)]
pub enum StyleRule {
  /// The rule has a value.
  WithValue(String, CssValue),
  Name(String),
}

impl StyleRule {
  pub fn get_style_declaration(&self, css_value: Option<CssValue>) -> String {
    match self {
      StyleRule::WithValue(name, value) => format!("{}: {}", name, value.get_string()),
      StyleRule::Name(name) => {
        let value = if let Some(v) = css_value {
          v.get_string()
        } else {
          "".to_string()
        };

        format!("{name}: {value}")
      }
    }
  }
}
