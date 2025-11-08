use std::{collections::HashMap, fmt};

use anyhow::{anyhow, Context};

use crate::value::Value;

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
	pub content: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Insert {
	pub name: Name,
	pub format: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SetFmt {
	pub type_name: String,
	pub format: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
	String(String),
	Insert(Insert),
	Section(Section),
	SetFmt(SetFmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
	fmt: HashMap<String, String>,
	tokens: Vec<Token>,
}

impl Template {
	pub(crate) fn new(tokens: Vec<Token>) -> Self {
		Self {
			fmt: HashMap::new(),
			tokens,
		}
	}

	#[inline]
	pub fn set_format(&mut self, type_name: String, format: String) {
		self.fmt.insert(type_name, format);
	}

	pub fn render(&mut self, value: &Value) -> anyhow::Result<String> {
		let mut buf = String::new();
		Self::render_tokens(&mut buf, &self.tokens, &[value], &mut self.fmt)?;
		Ok(buf)
	}

	fn render_tokens(
		buf: &mut String,
		tokens: &[Token],
		context: &[&Value],
		fmt: &mut HashMap<String, String>,
	) -> anyhow::Result<()> {
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
		fmt: &mut HashMap<String, String>,
	) -> anyhow::Result<()> {
		let value = Self::get_named_value(&section.name.0, context)?;

		match (section.inverted, value) {
			(false, Value::String(..) | Value::Fmt(..) | Value::Map(..)) => {
				Self::render_tokens(buf, &section.content, &[&[value], context].concat(), fmt)?
			}
			(true, Value::String(..)) => {
				return Err(anyhow!(
					"Cannot create inverted sections from string value {value}"
				))
			}
			(true, Value::Map(..)) => {
				return Err(anyhow!(
					"Cannot create inverted sections from map value {value}"
				))
			}
			(true, Value::Fmt(fmt_val)) => {
				let type_name = fmt_val.type_name();
				return Err(anyhow!(
					"Cannot create inverted sections from {type_name} value {value}"
				));
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
		fmt: &HashMap<String, String>,
	) -> anyhow::Result<()> {
		let value = Self::get_named_value(&insert.name.0, context)?;

		match value {
			Value::Vec(..) => return Err(anyhow!("Cannot directly insert array value {value}")),
			Value::Map(..) => return Err(anyhow!("Cannot directly insert map value {value}")),
			Value::Bool(bool) => buf.push_str(&bool.to_string()),
			Value::String(string) => buf.push_str(string),
			Value::Fmt(fmt_val) => buf.push_str(
				&fmt_val.format(
					insert
						.format
						.as_ref()
						.or_else(|| fmt.get(fmt_val.type_name()))
						.map(String::as_str),
				)?,
			),
			Value::Nil => (),
		}

		Ok(())
	}

	fn get_named_value<'a>(name: &'a [String], context: &[&'a Value]) -> anyhow::Result<&'a Value> {
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
				let index: usize = name[0].parse().context(format!(
					"\"{}\" is not a valid array index",
					name[0].clone()
				))?;
				if index >= vec.len() {
					return Err(anyhow!(
						"Index {index} is out of bounds for array of length {}",
						vec.len()
					));
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{fmt::test_utils::*, Format};

	#[test]
	fn render_static() {
		let mut template = Template::new(vec![
			Token::String("AAA".to_string()),
			Token::String("BBB".to_string()),
			Token::String("CCC".to_string()),
		]);

		let result = template.render(&Value::Nil).unwrap();
		assert_eq!(result, "AAABBBCCC");
	}

	#[test]
	fn render_whole_value() {
		let mut template = Template::new(vec![
			Token::String("Value: ".to_string()),
			Token::Insert(Insert {
				name: Name(vec![]),
				format: None,
			}),
		]);
		let result = template.render(&Value::String(":3c".to_string())).unwrap();
		assert_eq!(result, "Value: :3c");
	}

	#[test]
	fn render_named_value() {
		let mut template = Template::new(vec![
			Token::String("Value: ".to_string()),
			Token::Insert(Insert {
				name: Name(vec!["foo".to_string()]),
				format: None,
			}),
		]);
		let result = template
			.render(&Value::Map(
				[("foo".to_string(), Value::String(">:3".to_string()))].into(),
			))
			.unwrap();
		assert_eq!(result, "Value: >:3");
	}

	#[test]
	fn render_index_value() {
		let mut template = Template::new(vec![
			Token::String("Value: ".to_string()),
			Token::Insert(Insert {
				name: Name(vec!["1".to_string()]),
				format: None,
			}),
		]);
		let result = template
			.render(&Value::Vec(vec![
				Value::Nil,
				Value::String("^-^".to_string()),
			]))
			.unwrap();
		assert_eq!(result, "Value: ^-^");
	}

	#[test]
	fn render_deep_value() {
		let mut template = Template::new(vec![
			Token::String("Value: ".to_string()),
			Token::Insert(Insert {
				name: Name(vec!["foo".to_string(), "0".to_string(), "bar".to_string()]),
				format: None,
			}),
		]);
		let result = template
			.render(&Value::Map(
				[(
					"foo".to_string(),
					Value::Vec(vec![Value::Map(
						[("bar".to_string(), Value::String("c:".to_string()))].into(),
					)]),
				)]
				.into(),
			))
			.unwrap();
		assert_eq!(result, "Value: c:");
	}

	#[test]
	fn render_unformatted_insert() {
		let mut template = Template::new(vec![
			Token::String("Value: ".to_string()),
			Token::Insert(Insert {
				name: Name(vec![]),
				format: None,
			}),
		]);
		let result = template.render(&Value::Fmt(Box::new(TestFormat))).unwrap();
		assert_eq!(result, "Value: default: STRING VALUE :)");
	}

	#[test]
	fn render_formatted_insert() {
		let mut template = Template::new(vec![
			Token::String("Value: ".to_string()),
			Token::Insert(Insert {
				name: Name(vec![]),
				format: Some("«{string}»".to_string()),
			}),
		]);
		let result = template.render(&Value::Fmt(Box::new(TestFormat))).unwrap();
		assert_eq!(result, "Value: «STRING VALUE :)»");
	}

	#[test]
	fn render_global_formatted_insert() {
		let mut template = Template::new(vec![
			Token::String("Value: ".to_string()),
			Token::Insert(Insert {
				name: Name(vec![]),
				format: None,
			}),
		]);
		template.set_format(TestFormat.type_name().to_string(), "“{string}”".to_string());
		let result = template.render(&Value::Fmt(Box::new(TestFormat))).unwrap();
		assert_eq!(result, "Value: “STRING VALUE :)”");
	}

	#[test]
	fn override_global_formatting() {
		let mut template = Template::new(vec![
			Token::String("Value: ".to_string()),
			Token::Insert(Insert {
				name: Name(vec![]),
				format: Some("«{string}»".to_string()),
			}),
		]);
		template.set_format(TestFormat.type_name().to_string(), "“{string}”".to_string());
		let result = template.render(&Value::Fmt(Box::new(TestFormat))).unwrap();
		assert_eq!(result, "Value: «STRING VALUE :)»");
	}
}
