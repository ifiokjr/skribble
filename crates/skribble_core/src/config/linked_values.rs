use std::fmt::Write;

use indexmap::indexmap;
use indexmap::indexset;
use indexmap::IndexSet;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use skribble_color::HslaCss;

use super::Atom;
use super::ColorField;
use super::NameSet;
use super::Prioritized;
use super::Transformation;
use super::TransformationRecipient;
use crate::AnyEmptyResult;
use crate::Arguments;
use crate::CalcSymbol;
use crate::ClassTransformer;
use crate::ColorProperty;
use crate::Placeholder;
use crate::RunnerConfig;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LinkedValues {
	/// The [`ValueSet`] names that will be used to populate the names that can
	/// be used.
	Values(NameSet),
	/// The atom will be linked to colors and the settings determine how the
	/// link is made.
	Color(ColorField),
	/// The atom will be linked to all the `keyframes` that are available. This
	/// is used to generate the `animate` class name.
	Keyframes,
}

impl LinkedValues {
	pub fn get_names_from_config(&self, config: &RunnerConfig) -> IndexSet<String> {
		match self {
			Self::Values(ref value_set) => {
				let mut names = indexset! {};
				for value in value_set.iter() {
					if let Some(set) = config.value_sets.get(&value.value) {
						names.extend(set.values.keys().cloned());
					}
				}

				names
			}
			Self::Color(color_field) => {
				let mut names = indexset! {};

				names.extend(color_field.get_fields().keys().cloned());

				if !color_field.disable_palette {
					names.extend(config.palette.keys().cloned());
				}

				for (name, variable) in config.css_variables.iter() {
					if variable.is_color() {
						names.insert(name.to_owned());
					}
				}

				names
			}
			Self::Keyframes => config.keyframes.keys().cloned().collect(),
		}
	}

	pub fn merge(&mut self, other: impl Into<Self>) {
		let other = other.into();

		match self {
			Self::Values(value_set) => {
				if let Self::Values(other_value_set) = other {
					value_set.merge(other_value_set);
					value_set.sort_from_highest_priority();
				} else {
					*self = other;
				}
			}
			Self::Color(color_field) => {
				if let Self::Color(other_color_field) = other {
					color_field.merge(other_color_field);
				} else {
					*self = other;
				}
			}
			Self::Keyframes => {
				*self = other;
			}
		}
	}

	pub fn write_css_properties(
		&self,
		writer: &mut dyn Write,
		config: &RunnerConfig,
		atom: &Atom,
		name: impl AsRef<str>,
		transformers: &IndexSet<ClassTransformer>,
	) -> AnyEmptyResult {
		let name = name.as_ref();

		match self {
			Self::Values(ref value_set) => {
				for Prioritized { value: key, .. } in value_set.iter() {
					if let Some(css_value) = config
						.value_sets
						.get(key)
						.and_then(|value_set| value_set.values.get(name))
					{
						css_value.write_css(writer, config, atom, transformers)?;
						break;
					}
				}
			}
			Self::Color(color_field) => {
				let options = config.options();

				if let Some(variable) = config.css_variables.get(name) {
					if !variable.is_color() {
						return Ok(());
					}

					let default_value = if transformers.is_empty() {
						let transformed_color = variable.get_wrapped_variable(options);
						apply_transformers(
							transformed_color,
							transformers,
							config,
							TransformationRecipient::Value,
						)
					} else {
						let parts = variable.hsla_color_variable(options);
						let mut hsla_css = parts.hsla_css();

						let transformed_color =
							apply_color_transformers(transformers, config, &mut hsla_css);
						apply_transformers(
							transformed_color,
							transformers,
							config,
							TransformationRecipient::Value,
						)
					};
					let transparent_value = {
						let parts = variable.hsla_color_variable(options);
						parts.wrapped_transparent()
					};

					write_color_values(
						&default_value,
						&transparent_value,
						atom,
						config,
						transformers,
						writer,
					)?;

					return Ok(());
				}

				if let Some(palette_value) = config.palette.get(name) {
					if color_field.disable_palette {
						return Ok(());
					}

					let default_value = if transformers.is_empty() {
						let transformed_color =
							options.color_format.get_color(palette_value)?.to_string();
						apply_transformers(
							transformed_color,
							transformers,
							config,
							TransformationRecipient::Value,
						)
					} else {
						let hsla = options.color_format.get_hsla(palette_value)?;
						let mut hsla_css = HslaCss::new(&hsla);
						let transformed_color =
							apply_color_transformers(transformers, config, &mut hsla_css);
						apply_transformers(
							transformed_color,
							transformers,
							config,
							TransformationRecipient::Value,
						)
					};
					let transparent_value = {
						let hsla = options.color_format.get_hsla(palette_value)?;
						let mut hsla_css = HslaCss::new(&hsla);
						hsla_css.a = Some("0".into());
						hsla_css.to_string()
					};
					write_color_values(
						&default_value,
						&transparent_value,
						atom,
						config,
						transformers,
						writer,
					)?;

					return Ok(());
				}

				let fields = color_field.get_fields();
				let Some(field_value) = fields.get(name) else {
					return Ok(());
				};

				let normalized_value = Placeholder::normalize(&field_value.value, config);
				let default_value = apply_transformers(
					normalized_value,
					transformers,
					config,
					TransformationRecipient::Value,
				);
				let transparent_value = {
					let hsla = options.color_format.get_hsla(&field_value.fallback_color)?;
					let mut hsla_css = HslaCss::new(&hsla);
					hsla_css.a = Some("0".into());
					hsla_css.to_string()
				};
				write_color_values(
					&default_value,
					&transparent_value,
					atom,
					config,
					transformers,
					writer,
				)?;
			}
			Self::Keyframes => {
				for (keyframe_name, _keyframe) in config.keyframes.iter() {
					if name != keyframe_name {
						continue;
					}

					for (property, css_value) in atom.styles.iter() {
						let property = Placeholder::normalize(property, config);
						let values = indexmap! { "" => keyframe_name.as_str() }.into();
						let css_value = {
							let property_value = css_value
								.as_ref()
								.map(|value| Placeholder::normalize_value(value, &values, config))
								.unwrap_or_else(|| keyframe_name.clone());
							apply_transformers(
								property_value,
								transformers,
								config,
								TransformationRecipient::Property,
							)
						};

						writeln!(writer, "{}: {};", property, css_value)?;
					}

					break;
				}
			}
		}

		Ok(())
	}

	pub fn write_css_argument(
		&self,
		writer: &mut dyn Write,
		config: &RunnerConfig,
		atom: &Atom,
		argument: &Arguments,
		transformers: &IndexSet<ClassTransformer>,
	) -> AnyEmptyResult {
		argument.write_css_atom(writer, config, atom, transformers)?;
		Ok(())
	}

	pub fn collect_css_variables(
		&self,
		config: &RunnerConfig,
		name: impl AsRef<str>,
		css_variables: &mut IndexSet<String>,
	) {
		match self {
			Self::Values(ref value_set) => {
				for Prioritized { value: key, .. } in value_set.iter() {
					if let Some(css_value) = config
						.value_sets
						.get(key)
						.and_then(|value_set| value_set.values.get(name.as_ref()))
					{
						css_value.collect_css_variables(css_variables);
						break;
					}
				}
			}
			Self::Color(_) => {
				// TODO look at this again (not sure if this actually works)
				css_variables.insert(name.as_ref().to_owned());
			}
			Self::Keyframes => {
				for (keyframe_name, keyframe) in config.keyframes.iter() {
					if name.as_ref() != keyframe_name {
						continue;
					}

					keyframe.collect_css_variables(css_variables);
					break;
				}
			}
		}
	}
}

fn write_color_values(
	default_value: &str,
	transparent_value: &str,
	atom: &Atom,
	config: &RunnerConfig,
	transformers: &IndexSet<ClassTransformer>,
	writer: &mut dyn Write,
) -> AnyEmptyResult {
	let values = indexmap! {
	  "" => default_value,
	  "transparent" => transparent_value
	}
	.into();

	for (property, css_value) in atom.styles.iter() {
		let property = Placeholder::normalize(property, config);
		let css_value = {
			let property_value = css_value
				.as_ref()
				.map(|value| Placeholder::normalize_value(value, &values, config))
				.unwrap_or_else(|| default_value.into());
			apply_transformers(
				property_value,
				transformers,
				config,
				TransformationRecipient::Property,
			)
		};

		writeln!(writer, "{}: {};", property, css_value)?;
	}

	Ok(())
}

/// Apply the color transformers to the given HSLA CSS value.
fn apply_color_transformers(
	transformers: &IndexSet<ClassTransformer>,
	config: &RunnerConfig,
	hsla_css: &mut HslaCss,
) -> String {
	for ClassTransformer { name, value } in transformers.iter() {
		let Some(transformer) = config.get_transformer(name) else {
			continue;
		};

		let Some(value) = value.as_ref().and_then(|v| v.get_value(name, config)) else {
			continue;
		};

		// Handle color transformations only
		let Transformation::Color(transformation) = transformer.transformation else {
			continue;
		};

		match transformation {
			ColorProperty::Hue(symbol) => {
				if symbol == CalcSymbol::Set {
					hsla_css.h = Some(value.to_string());
				} else {
					hsla_css.h = Some(format!("calc({} {symbol} {value})", hsla_css.hue()));
				}
			}
			ColorProperty::Saturation(symbol) => {
				if symbol == CalcSymbol::Set {
					hsla_css.s = Some(format!("clamp({value}, 0%, 100%)"));
				} else {
					hsla_css.s = Some(format!(
						"clamp(calc({} {symbol} {value}), 0%, 100%)",
						hsla_css.saturation(),
					));
				}
			}
			ColorProperty::Lightness(symbol) => {
				if symbol == CalcSymbol::Set {
					hsla_css.l = Some(format!("clamp({value}, 0%, 100%)"));
				} else {
					hsla_css.l = Some(format!(
						"clamp(calc({} {symbol} {value}), 0%, 100%)",
						hsla_css.lightness(),
					));
				}
			}
			ColorProperty::Alpha(symbol) => {
				if symbol == CalcSymbol::Set {
					hsla_css.a = Some(value.clone());
				} else {
					hsla_css.a = Some(format!(
						"clamp(calc({} {symbol} {value}), 0.0, 1.0)",
						hsla_css.alpha(),
					));
				}
			}
		}
	}

	hsla_css.to_string()
}

pub(crate) fn apply_transformers(
	current_value: impl AsRef<str>,
	transformers: &IndexSet<ClassTransformer>,
	config: &RunnerConfig,
	recipient: TransformationRecipient,
) -> String {
	let mut current_value = current_value.as_ref().to_string();

	for ClassTransformer { name, value } in transformers.iter() {
		let Some(transformer) = config.get_transformer(name) else {
			continue;
		};

		if transformer.recipient != recipient {
			continue;
		}

		match &transformer.transformation {
			Transformation::Color(_) => {
				continue;
			}
			Transformation::Replacement(replacement) => {
				current_value = match value.as_ref().and_then(|v| v.get_value(name, config)) {
					Some(value) => {
						replacement
							.replace(
								format!("[{}]", Placeholder::TRANSFORMER_VALUE).as_str(),
								&value,
							)
							.replace('&', &current_value)
					}
					None => replacement.replace('&', &current_value),
				};
			}
			Transformation::RegexReplacement { regex, replacement } => {
				let Some(regex) = Regex::new(regex).ok() else {
					continue;
				};

				current_value = match value.as_ref().and_then(|v| v.get_value(name, config)) {
					Some(value) => {
						regex
							.replace(
								&current_value,
								replacement.replace(
									format!("[{}]", Placeholder::TRANSFORMER_VALUE).as_str(),
									&value,
								),
							)
							.to_string()
					}
					None => regex.replace(&current_value, replacement).to_string(),
				};
			}
		}
	}

	current_value
}

impl Default for LinkedValues {
	fn default() -> Self {
		Self::Values(NameSet::default())
	}
}

impl<V: Into<NameSet>> From<V> for LinkedValues {
	fn from(value: V) -> Self {
		Self::Values(value.into())
	}
}

impl From<ColorField> for LinkedValues {
	fn from(value: ColorField) -> Self {
		Self::Color(value)
	}
}
