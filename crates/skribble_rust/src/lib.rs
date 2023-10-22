#![deny(clippy::all)]
#![deny(clippy::indexing_slicing)]

doc_comment::doctest!("../readme.md");

use generate::generate_file_contents;
use heck::ToPascalCase;
use heck::ToSnakeCase;
use indexmap::IndexMap;
use indoc::indoc;
use scan::scan;
use serde::Deserialize;
use serde::Serialize;
use skribble_core::crate_version;
use skribble_core::AnyResult;
use skribble_core::Classes;
use skribble_core::GeneratedFile;
use skribble_core::GeneratedFiles;
use skribble_core::Plugin;
use skribble_core::PluginData;
use skribble_core::RunnerConfig;
use typed_builder::TypedBuilder;

mod generate;
mod scan;

/// This plugin generates `rust` code from the configuration.
#[derive(Debug, Clone, Default, Deserialize, TypedBuilder, Serialize)]
pub struct RustPlugin {
	/// The method names used in the generated code. This is also used to remap
	/// method names to the stored names.
	#[builder(default, setter(skip))]
	#[serde(skip)]
	method_names: IndexMap<String, String>,
}

impl Plugin for RustPlugin {
	fn get_data(&self) -> PluginData {
		PluginData::builder()
			.id("skribble_rust")
			.name("Rust Plugin")
			.globs(vec!["**/*.rs"])
			.description(
				"This plugin provides support for generating rust code from your `skribble` \
				 configuration.",
			)
			.version(crate_version!())
			.build()
	}

	fn generate_code(&mut self, config: &RunnerConfig) -> AnyResult<GeneratedFiles> {
		let mut files = GeneratedFiles::default();
		let (contents, method_names) = generate_file_contents(config)?;
		let method_names_json = serde_json::to_string_pretty(&method_names)?;

		self.method_names = method_names;

		files.insert(
			GeneratedFile::builder()
				.path("./src/skribble.rs")
				.content(contents)
				.build(),
		);
		files.insert(
			GeneratedFile::builder()
				// TODO where should this be placed
				.path("./cache/skribble_rust.json")
				.content(method_names_json)
				.build(),
		);

		Ok(files)
	}

	fn scan_code(
		&mut self,
		config: &RunnerConfig,
		file_path: &str,
		content: &str,
	) -> AnyResult<Classes> {
		scan(config, file_path, content, &self.method_names)
	}
}

impl RustPlugin {
	pub fn get_method_names(&self) -> &IndexMap<String, String> {
		&self.method_names
	}
}

#[cfg(test)]
pub use rstest_reuse;
#[cfg(test)]
mod __tests;
