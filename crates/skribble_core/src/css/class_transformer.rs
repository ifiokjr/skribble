use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Arguments;
use crate::RunnerConfig;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum TransformerValue {
	/// A predefined value.
	Value(String),
	/// A referenced value argument.
	Reference(String),
}

impl TransformerValue {
	pub(crate) fn get_reference_value(&self) -> Option<&String> {
		match self {
			TransformerValue::Value(_) => None,
			TransformerValue::Reference(value) => Some(value),
		}
	}

	pub fn get_value(&self, name: impl AsRef<str>, config: &RunnerConfig) -> Option<String> {
		match self {
			TransformerValue::Value(value) => Some(value.clone()),
			TransformerValue::Reference(reference) => {
				let Some(transformer) = config
					.get_transformer(name.as_ref())
					.and_then(|t| t.values.as_ref())
				else {
					return None;
				};

				transformer.get(reference).cloned()
			}
		}
	}
}

impl<T: Into<String>> From<T> for TransformerValue {
	fn from(value: T) -> Self {
		let value = value.into();

		if let Some(value) = value.strip_prefix(Arguments::DELIMITER) {
			TransformerValue::Reference(value.into())
		} else {
			TransformerValue::Value(value)
		}
	}
}

impl Display for TransformerValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			TransformerValue::Value(value) => write!(f, "{value}"),
			TransformerValue::Reference(value) => write!(f, "{}{value}", Arguments::DELIMITER),
		}
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, TypedBuilder)]
pub struct ClassTransformer {
	pub name: String,
	#[builder(default, setter(strip_option))]
	pub value: Option<TransformerValue>,
}

impl ClassTransformer {
	pub fn get_reference_value(&self) -> Option<&String> {
		self.value
			.as_ref()
			.and_then(|value| value.get_reference_value())
	}
}

impl Display for ClassTransformer {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}", self.name)?;

		if let Some(value) = &self.value {
			write!(f, "{}{value}", Arguments::DELIMITER)?;
		}

		write!(f, ")")
	}
}

impl<S: AsRef<str>> From<S> for ClassTransformer {
	fn from(value: S) -> Self {
		let value = match value
			.as_ref()
			.strip_prefix('(')
			.and_then(|value| value.strip_suffix(')'))
		{
			Some(stripped_value) => stripped_value,
			None => value.as_ref(),
		};

		match value.split_once(Arguments::DELIMITER) {
			Some((name, value)) => {
				Self {
					name: name.into(),
					value: Some(value.into()),
				}
			}
			None => {
				Self {
					name: value.into(),
					value: None,
				}
			}
		}
	}
}
