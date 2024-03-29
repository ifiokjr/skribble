use indexmap::IndexSet;
use lazy_static::lazy_static;
use regex::Captures;
use regex::Regex;

use crate::wrap_css_variable;
use crate::RunnerConfig;
use crate::StringMap;

pub const INDENTATION: &str = "  ";
pub const ROOT_SELECTOR: &str = ":root";

lazy_static! {
	static ref CSS_VARIABLE_REGEX: Regex = Regex::new(
		format!(
			"(?m){}",
			Placeholder::variable("(?P<name>\\w[a-zA-Z0-9-]+)")
		)
		.as_str()
	)
	.unwrap();
	static ref MEDIA_QUERY_REGEX: Regex = Regex::new(
		format!(
			"(?m){}",
			Placeholder::media_query("(?P<name>\\w[a-zA-Z0-9-]+)")
		)
		.as_str()
	)
	.unwrap();
	static ref MODIFIER_REGEX: Regex = Regex::new(
		format!(
			"(?m){}",
			Placeholder::modifier("(?P<name>\\w[a-zA-Z0-9-]+)")
		)
		.as_str()
	)
	.unwrap();
	static ref PALETTE_REGEX: Regex =
		Regex::new(format!("(?m){}", Placeholder::palette("(?P<name>\\w[a-zA-Z0-9-]+)")).as_str())
			.unwrap();
}

pub struct Placeholder;

impl Placeholder {
	pub const CSS_VARIABLE: &'static str = "CSS_VARIABLE";
	pub const MEDIA_QUERY: &'static str = "MEDIA_QUERY";
	pub const MODIFIER: &'static str = "MODIFIER";
	pub const PALETTE: &'static str = "PALETTE";
	pub const TRANSFORMER_VALUE: &'static str = "TRANSFORMER_VALUE";
	pub const VALUE: &'static str = "VALUE";

	pub fn create(namespace: &str, name: impl AsRef<str>) -> String {
		let name = name.as_ref();
		format!("__:{namespace}::{name}:__")
	}

	pub fn normalize(content: impl AsRef<str>, config: &RunnerConfig) -> String {
		let content = Self::normalize_css_variables(content, config);
		let content = Self::normalize_palette(content, config);
		let content = Self::normalize_media_query(content, config);

		Self::normalize_modifiers(content, config)
	}

	pub fn normalize_css_variables(content: impl AsRef<str>, config: &RunnerConfig) -> String {
		CSS_VARIABLE_REGEX
			.replace_all(content.as_ref(), |caps: &Captures| {
				// value for an invalid match
				let invalid_regex = format!(
					"--{}-invalid-css-variable",
					config.options().variable_prefix
				);

				// get the name from the capture group
				let Some(name) = caps.name("name") else {
					return invalid_regex;
				};

				let name = name.as_str();

				let Some(name) = config.css_variables.get(name) else {
					return invalid_regex;
				};

				name.get_variable(config.options())
			})
			.to_string()
	}

	pub fn normalize_palette(content: impl AsRef<str>, config: &RunnerConfig) -> String {
		PALETTE_REGEX
			.replace_all(content.as_ref(), |caps: &Captures| {
				// value for an invalid match
				let default_value = "#000000".into();

				// get the name from the capture group
				let Some(name) = caps.name("name") else {
					return default_value;
				};

				let name = name.as_str();

				let Some(value) = config.palette.get(name) else {
					return default_value;
				};

				value.to_owned()
			})
			.to_string()
	}

	pub fn normalize_modifiers(content: impl AsRef<str>, config: &RunnerConfig) -> String {
		MODIFIER_REGEX
			.replace_all(content.as_ref(), |caps: &Captures| {
				let Some(name) = caps.name("name") else {
					return String::new();
				};

				let name = name.as_str();

				let Some(modifier) = config.get_modifier(name) else {
					return String::new();
				};

				modifier.values.join(", ")
			})
			.to_string()
	}

	pub fn normalize_media_query(content: impl AsRef<str>, config: &RunnerConfig) -> String {
		MEDIA_QUERY_REGEX
			.replace_all(content.as_ref(), |caps: &Captures| {
				let Some(name) = caps.name("name") else {
					return String::new();
				};

				let name = name.as_str();
				let Some(media_query) = config.get_media_query(name) else {
					return String::new();
				};

				media_query.query.clone()
			})
			.to_string()
	}

	/// Replaces all the value placeholders with the given value.
	pub fn normalize_value(
		content: impl AsRef<str>,
		values: &StringMap,
		config: &RunnerConfig,
	) -> String {
		let mut content = content.as_ref().to_string();

		for (name, value) in values.iter() {
			let regex: Regex =
				Regex::new(format!("(?m){}", Placeholder::value(name)).as_str()).unwrap();
			content = regex.replace_all(content.as_str(), value).to_string();
		}

		Self::normalize(content, config)
	}

	/// Generate a placeholder for the variable by using the name. This inserts
	/// some text which will be replaced by the actual variable name when the
	/// code is generated.
	pub fn variable(name: impl AsRef<str>) -> String {
		Self::create(Self::CSS_VARIABLE, name)
	}

	/// Generate a placeholder for a variable wrapped with `var()`.
	pub fn wrapped_variable(name: impl AsRef<str>, default: Option<String>) -> String {
		wrap_css_variable(Self::variable(name), default)
	}

	/// Extract all the variables from the given content.
	pub fn collect_css_variables(content: impl AsRef<str>, css_variable: &mut IndexSet<String>) {
		for caps in CSS_VARIABLE_REGEX.captures_iter(content.as_ref()) {
			let Some(name) = caps.name("name") else {
				continue;
			};

			css_variable.insert(name.as_str().to_owned());
		}
	}

	/// Generate a placeholder for the palette color by using the name. This
	/// inserts some text which will be replaced by the actual palette color
	/// when the code is generated.
	pub fn palette(name: impl AsRef<str>) -> String {
		Self::create(Self::PALETTE, name)
	}

	pub fn media_query(name: impl AsRef<str>) -> String {
		Self::create(Self::MEDIA_QUERY, name)
	}

	pub fn modifier(name: impl AsRef<str>) -> String {
		Self::create(Self::MODIFIER, name)
	}

	pub fn value(name: impl AsRef<str>) -> String {
		Self::create(Self::VALUE, name)
	}
}
