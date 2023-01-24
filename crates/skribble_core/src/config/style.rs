use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Options;

/// The style configuration which can also use the builder pattern.
#[derive(Serialize, Deserialize, TypedBuilder, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StyleConfig {
  /// The general options.
  pub options: Options,
}
