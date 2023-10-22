use super::*;

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
