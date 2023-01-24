use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Error;
use crate::Options;
use crate::Result;

/// The style configuration which can also use the builder pattern.
#[derive(Serialize, Deserialize, TypedBuilder, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StyleConfig {
  /// The general options.
  pub options: Options,
}

impl StyleConfig {
  pub fn from_json(json: &str) -> Result<Self> {
    let config: Self =
      serde_json::from_str(json).map_err(|source| Error::InvalidConfig { source })?;
    Ok(config)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_config_can_serialize() {
    let config: StyleConfig = serde_json::from_str(include_str!("default.json")).unwrap();
    let json = serde_json::to_string(&config).unwrap();
    assert_eq!(config, serde_json::from_str(&json).unwrap());
  }

  #[test]
  fn default_config() {
    insta::assert_json_snapshot!(StyleConfig::default());
  }
}
