#![deny(clippy::all)]
#![forbid(clippy::indexing_slicing)]

doc_comment::doctest!("../readme.md");

use data::*;
use enums::*;
use serde::Deserialize;
use serde::Serialize;
use skribble_core::*;
use typed_builder::TypedBuilder;

pub(crate) mod data;
mod enums;

#[derive(Debug, Clone, Default, Deserialize, TypedBuilder, Serialize)]
pub struct PresetPlugin {
	/// Choose the palette colors from either Tailwind or OpenColors.
	#[builder(default, setter(into))]
	pub palette: PaletteType,
	/// Choose how `light` and `dark` mode are handled. via
	/// `prefers-color-scheme` or `class` based.
	#[builder(default, setter(into))]
	pub dark_mode: DarkMode,
	/// Choose to ignore the color variables so that you can define your own
	/// custom theme.
	#[builder(default, setter(into))]
	pub ignore_colors: bool,
	/// The reset to use for the CSS.
	#[builder(default, setter(into, strip_option))]
	reset: Option<CssReset>,
}

impl Plugin for PresetPlugin {
	fn get_data(&self) -> PluginData {
		PluginData::builder()
			.id("skribble_preset")
			.name("Preset Plugin")
			.description(
				"This plugin provides a default preset for `skribble` which is similar to \
				 `tailwindcss` and `unocss`.",
			)
			.version(crate_version!())
			.build()
	}

	fn mutate_config(&mut self, config: &mut PluginConfig, _options: &Options) -> AnyEmptyResult {
		self.update_aliases(&mut config.aliases);
		self.update_atoms(&mut config.atoms);
		self.update_css_chunks(&mut config.css_chunks);
		self.update_keyframes(&mut config.keyframes);
		self.update_media_queries(&mut config.media_queries);
		self.update_modifiers(&mut config.modifiers);
		self.update_transformers(&mut config.transformers);
		self.update_named_classes(&mut config.classes);
		self.update_palette(&mut config.palette);
		self.update_value_sets(&mut config.value_sets);
		self.update_variables(&mut config.variables);

		Ok(())
	}
}

impl PresetPlugin {
	fn update_aliases(&self, aliases: &mut Aliases) {
		aliases.extend(ALIASES.clone());
	}

	fn update_atoms(&self, atoms: &mut Atoms) {
		atoms.extend(ATOMS.clone());
	}

	fn update_css_chunks(&self, css_chunks: &mut CssChunks) {
		let Some(reset) = &self.reset else {
			return;
		};

		css_chunks.push(
			CssChunk::builder()
				.name("reset")
				.layer("base")
				.css(reset.get_css())
				.auto_include(true)
				.priority(Priority::LOW)
				.build(),
		);
	}

	fn update_keyframes(&self, keyframes: &mut Keyframes) {
		keyframes.extend(KEYFRAMES.clone());
	}

	fn update_media_queries(&self, media_queries: &mut MediaQueries) {
		media_queries.extend(MEDIA_QUERIES.clone());

		if self.dark_mode == DarkMode::Media {
			media_queries.extend(DARK_MEDIA_QUERIES.clone());
		}
	}

	fn update_modifiers(&self, modifiers: &mut Modifiers) {
		modifiers.extend(MODIFIERS.clone());

		if self.dark_mode == DarkMode::Class {
			modifiers.extend_group(DARK_PARENT_MODIFIERS.clone());
		}
	}

	fn update_transformers(&self, transformers: &mut Transformers) {
		transformers.extend(TRANSFORMERS.clone());
	}

	fn update_named_classes(&self, named_classes: &mut NamedClasses) {
		named_classes.extend(NAMED_CLASSES.clone());
	}

	fn update_palette(&self, palette: &mut Palette) {
		palette.extend(self.palette.palette());
	}

	fn update_value_sets(&self, value_sets: &mut ValueSets) {
		value_sets.extend(ATOM_VALUE_SETS.clone());
	}

	fn update_variables(&self, css_variables: &mut CssVariables) {
		if !self.ignore_colors {
			css_variables.extend(COLOR_CSS_VARIABLES.clone());
		}

		css_variables.extend(CSS_VARIABLES.clone());
	}
}

#[cfg(test)]
mod __tests;
