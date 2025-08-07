use std::{collections::HashMap, fmt, num::ParseIntError};

use thiserror::Error;

use crate::{fmt::FmtError, value::Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Name(pub Vec<String>);

impl fmt::Display for Name {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.0.is_empty() {
			return write!(f, ".");
		}
		write!(f, "{}", self.0.join("."))
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Section {
	pub name: Name,
	pub inverted: bool,
	pub content: Vec<Token>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Insert {
	pub name: Name,
	pub format: Option<String>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SetFmt {
	pub type_name: String,
	pub format: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
	String(String),
	Insert(Insert),
	Section(Section),
	SetFmt(SetFmt)
}

#[derive(Debug, Error)]
pub enum RenderError {
	#[error("Couldn't resolve name {0}")]
	NameNotFound(String),

	#[error("Cannot directly insert type {0}")]
	CannotInsert(&'static str),

	#[error("Cannot create inverted sections from type {0}")]
	CannotCreateInvertedSection(&'static str),

	#[error("\"{0}\" is not a valid array index: {1}")]
	InvalidVecIndex(String, ParseIntError),

	#[error("Index {0} is out of bounds for array of length {0}")]
	IndexOutOfBounds(usize, usize),

	#[error("Key \"{0}\" doesn't exist on this map")]
	UnknownKey(String),

	#[error(transparent)]
	Fmt(#[from] FmtError)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
	fmt: HashMap<String, String>,
	tokens: Vec<Token>
}

impl Template {
	pub(crate) fn new(tokens: Vec<Token>) -> Self {
		Self {
			fmt: HashMap::new(),
			tokens
		}
	}

	#[inline]
	pub fn set_format(&mut self, type_name: String, format: String) {
		self.fmt.insert(type_name, format);
	}

	pub fn render(&mut self, value: &Value) -> Result<String, RenderError> {
		let mut buf = String::new();
		Self::render_tokens(&mut buf, &self.tokens, &[value], &mut self.fmt)?;
		Ok(buf)
	}

	fn render_tokens(
		buf: &mut String,
		tokens: &[Token],
		context: &[&Value],
		fmt: &mut HashMap<String, String>
	) -> Result<(), RenderError> {
		for token in tokens {
			match token {
				Token::String(string) => buf.push_str(string),
				Token::Insert(insert) => Self::render_insert(buf, insert, context, fmt)?,
				Token::Section(section) => Self::render_section(buf, section, context, fmt)?,
				Token::SetFmt(setfmt) => {
					fmt.insert(setfmt.type_name.clone(), setfmt.format.clone());
				}
			}
		}
		Ok(())
	}

	fn render_section(
		buf: &mut String,
		section: &Section,
		context: &[&Value],
		fmt: &mut HashMap<String, String>
	) -> Result<(), RenderError> {
		let value = Self::get_named_value(&section.name.0, context)?;

		match (section.inverted, value) {
			(false, Value::String(..) | Value::Fmt(..) | Value::Map(..)) => {
				Self::render_tokens(buf, &section.content, &[&[value], context].concat(), fmt)?
			}
			(true, Value::String(..)) => {
				return Err(RenderError::CannotCreateInvertedSection("string"))
			}
			(true, Value::Map(..)) => return Err(RenderError::CannotCreateInvertedSection("map")),
			(true, Value::Fmt(fmt_val)) => {
				return Err(RenderError::CannotCreateInvertedSection(
					fmt_val.type_name()
				))
			}
			(invert, Value::Bool(bool)) => {
				if bool ^ invert {
					Self::render_tokens(buf, &section.content, &[&[value], context].concat(), fmt)?
				}
			}
			(invert, Value::Nil) => {
				if invert {
					Self::render_tokens(buf, &section.content, &[&[value], context].concat(), fmt)?
				}
			}
			(false, Value::Vec(vec)) => {
				for val in vec {
					Self::render_tokens(buf, &section.content, &[&[val], context].concat(), fmt)?;
				}
			}
			(true, Value::Vec(vec)) => {
				for val in vec.iter().rev() {
					Self::render_tokens(buf, &section.content, &[&[val], context].concat(), fmt)?;
				}
			}
		}

		Ok(())
	}

	fn render_insert(
		buf: &mut String,
		insert: &Insert,
		context: &[&Value],
		fmt: &HashMap<String, String>
	) -> Result<(), RenderError> {
		let value = Self::get_named_value(&insert.name.0, context)?;

		match value {
			Value::Vec(..) => return Err(RenderError::CannotInsert("array")),
			Value::Map(..) => return Err(RenderError::CannotInsert("map")),
			Value::Bool(bool) => buf.push_str(&bool.to_string()),
			Value::String(string) => buf.push_str(string),
			Value::Fmt(fmt_val) => buf.push_str(
				&fmt_val.format(
					insert
						.format
						.as_ref()
						.or_else(|| fmt.get(fmt_val.type_name()))
						.map(String::as_str)
				)?
			),
			Value::Nil => ()
		}

		Ok(())
	}

	fn get_named_value<'a>(
		name: &'a [String],
		context: &[&'a Value]
	) -> Result<&'a Value, RenderError> {
		let Some(&value) = context.first() else {
			return Ok(&Value::Nil);
		};

		if name.is_empty() {
			return Ok(value);
		}

		match value {
			Value::Nil | Value::Bool(..) | Value::Fmt(..) | Value::String(..) => {
				Self::get_named_value(name, &context[1..])
			}
			Value::Vec(vec) => {
				let index: usize = name[0]
					.parse()
					.map_err(|e| RenderError::InvalidVecIndex(name[0].clone(), e))?;
				if index >= vec.len() {
					return Err(RenderError::IndexOutOfBounds(index, vec.len()));
				}

				Self::get_named_value(&name[1..], &[&[&vec[index]], &context[1..]].concat())
			}
			Value::Map(map) => {
				let Some(value) = map.get(&name[0]) else {
					return Self::get_named_value(name, &context[1..]);
				};
				Self::get_named_value(&name[1..], &[&[value], &context[1..]].concat())
			}
		}
	}
}
