use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::ColorFormat;
use super::Formatter;
use super::MergeRules;

/// Options to use in the configuration.
#[derive(Clone, Debug, Deserialize, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Options {
	/// The path to the output css file. If not specified then the output is set
	/// to `skribble.css` in the current working directory.
	#[serde(default = "default_css_output")]
	#[builder(default = default_css_output(), setter(into))]
	pub output: PathBuf,
	/// Root directory to use when resolving paths. If relative then it is
	/// relative to the CWD.
	#[serde(default = "default_root")]
	#[builder(default = default_root(), setter(into))]
	pub root: PathBuf,
	/// The globs to match the files, relative to the CWD. Under the hood this
	/// uses `globset`.To exclude a pattern prefix it with `!`.
	#[serde(default = "default_globs")]
	#[builder(default = default_globs(), setter(into))]
	pub files: Vec<String>,
	/// The character encoding used in the style sheet
	#[serde(default = "default_charset")]
	#[builder(default = default_charset(), setter(into))]
	pub charset: String,
	/// The default layer to use when no layer is specified.
	#[serde(default = "default_first_layer")]
	#[builder(default = default_first_layer(), setter(into))]
	pub default_layer: String,
	/// This is the default format of colors rendered in css.
	#[serde(default)]
	#[builder(default, setter(into))]
	pub color_format: ColorFormat,
	/// The rules to control how the user configuration is merged with the
	/// configuration extracted from plugins.
	#[serde(default)]
	#[builder(default, setter(into))]
	pub merge_rules: MergeRules,
	/// This determines whether the new [`@property`](https://developer.mozilla.org/en-US/docs/Web/CSS/@property) syntax
	///  is used for variables. Defaults to `false`.
	#[serde(default)]
	#[builder(default, setter(into))]
	pub use_registered_properties: bool,
	/// Set the prefix that all css variables should use.
	#[serde(default = "default_variable_prefix")]
	#[builder(default = default_variable_prefix(), setter(into))]
	pub variable_prefix: String,
	/// The default color value to use when no color is specified.
	#[serde(default = "default_hex_color")]
	#[builder(default = default_hex_color(), setter(into))]
	pub default_color: String,
	/// Whether to minify the generated stylesheet.
	#[serde(default)]
	#[builder(default)]
	pub minify: bool,
	/// Whether to disable formatting of the generated files and stylesheets.
	/// Formatters can be configured in the `formatters` field.
	#[serde(default)]
	#[builder(default)]
	pub disable_formatting: bool,
	/// Formatters which are used to format files.
	#[serde(default)]
	#[builder(default)]
	pub formatters: Vec<Formatter>,
}

impl Default for Options {
	fn default() -> Self {
		Self::builder().build()
	}
}

fn default_css_output() -> PathBuf {
	PathBuf::from("skribble.css")
}

fn default_root() -> PathBuf {
	PathBuf::from("./")
}

fn default_variable_prefix() -> String {
	"sk".into()
}

fn default_charset() -> String {
	"utf-8".into()
}

fn default_first_layer() -> String {
	"default".into()
}

fn default_globs() -> Vec<String> {
	vec!["**".into()]
}

fn default_hex_color() -> String {
	"#000000".into()
}
