use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PropertySyntax {
	Value(PropertySyntaxValue),
	List(Vec<PropertySyntaxValue>),
}

impl PropertySyntax {
	#[inline]
	pub fn from_string<T: Into<String>>(value: T) -> Self {
		PropertySyntax::Value(PropertySyntaxValue::from(value))
	}

	#[inline]
	pub fn from_iterator<V: Into<String>, I: IntoIterator<Item = V>>(iter: I) -> Self {
		let property = iter
			.into_iter()
			.map(|v| PropertySyntaxValue::from(v))
			.collect();

		PropertySyntax::List(property)
	}

	#[inline]
	pub fn is_color(&self) -> bool {
		match self {
			PropertySyntax::Value(value) => *value == PropertySyntaxValue::Color,
			PropertySyntax::List(_) => false,
		}
	}
}

impl Default for PropertySyntax {
	fn default() -> Self {
		PropertySyntax::Value(PropertySyntaxValue::Any)
	}
}

impl From<PropertySyntaxValue> for PropertySyntax {
	fn from(value: PropertySyntaxValue) -> Self {
		PropertySyntax::Value(value)
	}
}

impl From<Vec<PropertySyntaxValue>> for PropertySyntax {
	fn from(value: Vec<PropertySyntaxValue>) -> Self {
		PropertySyntax::List(value)
	}
}

impl Display for PropertySyntax {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			PropertySyntax::Value(value) => write!(f, "{value}"),
			PropertySyntax::List(values) => {
				let values = values
					.iter()
					.map(|v| v.to_string())
					.collect::<Vec<String>>()
					.join(" | ");

				write!(f, "{values}")
			}
		}
	}
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PropertySyntaxValue {
	/// Any valid <length> values.
	#[serde(rename = "<length>")]
	Length,
	#[serde(rename = "<number>")]
	Number,
	/// Any valid <percentage> values.
	#[serde(rename = "<percentage>")]
	Percentage,
	/// Any valid <length-percentage> values.
	#[serde(rename = "<length-percentage>")]
	LengthPercentage,
	/// Any valid <color> values.
	#[serde(rename = "<color>")]
	Color,
	/// Any valid <image> values.
	#[serde(rename = "<image>")]
	Image,
	/// Any valid url() values.
	#[serde(rename = "<url>")]
	Url,
	/// Any valid <integer> values.
	#[serde(rename = "<integer>")]
	Integer,
	/// Any valid <angle> values.
	#[serde(rename = "<angle>")]
	Angle,
	/// Any valid <time> values.
	#[serde(rename = "<time>")]
	Time,
	/// Any valid <resolution> values.
	#[serde(rename = "<resolution>")]
	Resolution,
	/// Any valid <transform-function> values.
	#[serde(rename = "<transform-function>")]
	TransformFunction,
	/// Any valid <custom-ident> values.
	#[serde(rename = "<custom-ident>")]
	CustomIdent,
	/// A list of valid <transform-function> values.
	#[serde(rename = "<transform-list>")]
	TransformList,
	/// Any valid token
	#[serde(rename = "*")]
	#[default]
	Any,
	/// Accepts this value as custom idents
	String(String),
}

impl<T: Into<String>> From<T> for PropertySyntaxValue {
	fn from(value: T) -> Self {
		let value = value.into();

		match value.as_str() {
			"<length>" => PropertySyntaxValue::Length,
			"<number>" => PropertySyntaxValue::Number,
			"<percentage>" => PropertySyntaxValue::Percentage,
			"<length-percentage>" => PropertySyntaxValue::LengthPercentage,
			"<color>" => PropertySyntaxValue::Color,
			"<image>" => PropertySyntaxValue::Image,
			"<url>" => PropertySyntaxValue::Url,
			"<integer>" => PropertySyntaxValue::Integer,
			"<angle>" => PropertySyntaxValue::Angle,
			"<time>" => PropertySyntaxValue::Time,
			"<resolution>" => PropertySyntaxValue::Resolution,
			"<transform-function>" => PropertySyntaxValue::TransformFunction,
			"<custom-ident>" => PropertySyntaxValue::CustomIdent,
			"<transform-list>" => PropertySyntaxValue::TransformList,
			"*" => PropertySyntaxValue::Any,
			_ => PropertySyntaxValue::String(value),
		}
	}
}

impl AsRef<str> for PropertySyntaxValue {
	fn as_ref(&self) -> &str {
		match self {
			PropertySyntaxValue::Length => "<length>",
			PropertySyntaxValue::Number => "<number>",
			PropertySyntaxValue::Percentage => "<percentage>",
			PropertySyntaxValue::LengthPercentage => "<length-percentage>",
			PropertySyntaxValue::Color => "<color>",
			PropertySyntaxValue::Image => "<image>",
			PropertySyntaxValue::Url => "<url>",
			PropertySyntaxValue::Integer => "<integer>",
			PropertySyntaxValue::Angle => "<angle>",
			PropertySyntaxValue::Time => "<time>",
			PropertySyntaxValue::Resolution => "<resolution>",
			PropertySyntaxValue::TransformFunction => "<transform-function>",
			PropertySyntaxValue::CustomIdent => "<custom-ident>",
			PropertySyntaxValue::TransformList => "<transform-list>",
			PropertySyntaxValue::Any => "*",
			PropertySyntaxValue::String(value) => value,
		}
	}
}

impl Display for PropertySyntaxValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}
