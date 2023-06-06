use indexmap::indexmap;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::StringList;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ColorField {
  /// Whether to disable the default colors of `inherit`, `transparent` and
  /// `current`
  #[serde(default)]
  #[builder(default, setter(into))]
  pub disable_named_defaults: bool,
  /// Whether to disable access to the palette colors.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub disable_palette: bool,
  /// Colors that should be excluded from the options.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub excluded: StringList,
  /// Additional named color fields.
  #[serde(flatten, default)]
  #[builder(default, setter(into))]
  pub named_fields: IndexMap<String, NamedColorField>,
}

lazy_static! {
  pub static ref DEFAULT_COLOR_FIELDS: IndexMap<String, NamedColorField> = indexmap! {
    "inherit".into() => NamedColorField::builder().value("inherit").build(),
    "transparent".into() => NamedColorField::builder().value("transparent").fallback_color("#0000").build(),
    "current".into() => NamedColorField::builder().value("currentColor").build(),
  };
}

impl ColorField {
  pub fn merge(&mut self, other: impl Into<Self>) {
    let other = other.into();

    self.disable_named_defaults |= other.disable_named_defaults;
    self.disable_palette |= other.disable_palette;
    self.named_fields.extend(other.named_fields);
  }

  pub fn get_fields(&self) -> IndexMap<String, NamedColorField> {
    let mut fields = self.named_fields.clone();

    if !self.disable_named_defaults {
      fields.extend(DEFAULT_COLOR_FIELDS.clone());
    }

    fields
  }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NamedColorField {
  #[builder(setter(into))]
  pub value: String,
  /// The color to fallback to. Defaults to `#000`.
  #[serde(default = "default_fallback_color")]
  #[builder(default = default_fallback_color(), setter(into))]
  pub fallback_color: String,
}

impl<T: Into<String>> From<T> for NamedColorField {
  fn from(value: T) -> Self {
    Self::builder().value(value).build()
  }
}

fn default_fallback_color() -> String {
  "#000".into()
}
