use rstest::rstest;
use similar_asserts::assert_eq;

use super::*;
use crate::AnyEmptyResult;

#[rstest]
#[case("hsl", "hsl(314 100% 47.0588%)", "314 100% 47.0588%")]
#[case("hsl", "hsl(314 100% 47.0588% / 1.0)", "314 100% 47.0588%")]
fn color_format_inner_color(
  #[case] format: &str,
  #[case] input: &str,
  #[case] expected: &str,
) -> AnyEmptyResult {
  let color_format = ColorFormat::from(format);
  assert_eq!(color_format.get_inner_color(input)?, expected);
  Ok(())
}

#[test]
fn check_config_can_serialize() {
  let config: StyleConfig = Default::default();
  let json = config.to_json().unwrap();
  StyleConfig::from_json(json).unwrap();
}

#[test]
fn default_config() {
  insta::assert_json_snapshot!(StyleConfig::default());
}
