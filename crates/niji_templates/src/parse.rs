use std::{
	fmt,
	iter::Peekable,
	str::{Chars, FromStr}
};

use thiserror::Error;

use crate::template::{Insert, Name, Section, SetFmt, Template, Token};

// Tokens := (Token | SetDelimiters)*
// Token := Section | SetFmt | Insert | String
// Section := "{{", ("#" | "^"), Name, "}}", Tokens, "{{/", Name, "}}"
// Name := Ident, (".", Ident)*
// SetDelimiters := "{{=", Delimiter, Delimiter, "=}}"
// Insert := "{{", Name, [":", String], "}}"
// SetFmt := "{{%", String, ":", String, "%}}"

#[derive(Debug, Error)]
pub enum ParseErrorKind {
	#[error("Expected an identifier")]
	ExpectedIdent,

	#[error("Expected closing delimiter \"{0}\"")]
	ExpectedClosingDelim(String),

	#[error("Expected a name")]
	ExpectedName,

	#[error("Mismatched section end: expected \"/{1}\", found \"/{0}\"")]
	MismatchedSectionEnd(String, String),

	#[error("Section \"{0}\" was never closed")]
	MissingSectionEnd(String),

	#[error("Missing a definition for the start delimiter")]
	MissingStartDelimiterDef,

	#[error("Missing a definition for the end delimiter")]
	MissingEndDelimiterDef,

	#[error("Unterminated string literal")]
	UnterminatedStrLit,

	#[error("Expected string literal")]
	ExpectedStrLit,

	#[error("'{0}' cannot be used in a delimiter")]
	ForbiddenDelimiterChar(char),

	#[error("Expected '{0}'")]
	ExpectedOp(char)
}

#[derive(Debug, Error)]
#[error("{kind} ({position})")]
pub struct ParseError {
	position: Position,
	kind: ParseErrorKind
}

impl ParseError {
	fn new(kind: ParseErrorKind, position: Position) -> Self {
		Self { kind, position }
	}
}

impl FromStr for Template {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut source = Source::new(s.chars());
		let mut state = State::default();
		let tokens = parse_template(&mut source, &mut state)?;

		Ok(Template::new(tokens.unwrap()))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
	line: usize,
	column: usize
}

impl Position {
	pub fn step(&mut self, c: char) {
		if c == '\n' {
			self.column = 0;
			self.line += 1;
		} else {
			self.column += 1;
		}
	}
}

impl Default for Position {
	fn default() -> Self {
		Self { line: 1, column: 0 }
	}
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}", self.line, self.column)
	}
}

#[derive(Debug, Clone)]
struct Source<'a> {
	chars: Peekable<Chars<'a>>,
	position: Position
}

impl<'a> Source<'a> {
	fn new(chars: Chars<'a>) -> Self {
		Self {
			chars: chars.peekable(),
			position: Position::default()
		}
	}

	#[inline]
	fn peek(&mut self) -> Option<char> {
		self.chars.peek().copied()
	}
}

impl<'a> Iterator for Source<'a> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		let c = self.chars.next()?;
		self.position.step(c);
		Some(c)
	}
}

struct State {
	start_delimiter: String,
	end_delimiter: String
}

impl Default for State {
	fn default() -> Self {
		Self {
			start_delimiter: "{{".to_string(),
			end_delimiter: "}}".to_string()
		}
	}
}

type ParseResult<T> = Result<Option<T>, ParseError>;

fn skip_whitespace(source: &mut Source) {
	loop {
		let Some(char) = source.peek() else {
			return;
		};
		if char.is_whitespace() {
			source.next().unwrap();
		} else {
			return;
		}
	}
}

fn has_delimiter(source: &mut Source, delimiter: &str) -> bool {
	let mut src = source.clone();

	for ch in delimiter.chars() {
		if src.next() != Some(ch) {
			return false;
		}
	}

	*source = src;
	true
}

fn parse_ident(source: &mut Source) -> ParseResult<String> {
	let mut src = source.clone();
	let mut ident = String::new();

	while matches!(
		src.peek(),
		Some('A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '-')
	) {
		ident.push(src.next().unwrap());
	}

	if ident.is_empty() {
		return Ok(None);
	}

	*source = src;
	Ok(Some(ident))
}

fn parse_name(source: &mut Source) -> ParseResult<Name> {
	let mut src = source.clone();

	if src.peek() == Some('.') {
		src.next().unwrap();
		*source = src;
		return Ok(Some(Name(vec![])));
	}

	let mut segments = Vec::<String>::with_capacity(1);

	loop {
		let Some(segment) = parse_ident(&mut src)? else {
			return Err(ParseError::new(ParseErrorKind::ExpectedIdent, src.position));
		};
		segments.push(segment);
		if src.peek() != Some('.') {
			break;
		}
		src.next().unwrap();
	}

	*source = src;
	Ok(Some(Name(segments)))
}

fn parse_tag(source: &mut Source, state: &State, operator: Option<char>) -> ParseResult<Name> {
	let mut src = source.clone();

	if !has_delimiter(&mut src, &state.start_delimiter) {
		return Ok(None);
	}

	skip_whitespace(&mut src);

	if let Some(operator) = operator {
		if src.peek() != Some(operator) {
			return Ok(None);
		}
		src.next().unwrap();
	}
	skip_whitespace(&mut src);
	let Some(name) = parse_name(&mut src)? else {
		return Err(ParseError::new(ParseErrorKind::ExpectedName, src.position));
	};
	skip_whitespace(&mut src);
	if !has_delimiter(&mut src, &state.end_delimiter) {
		return Err(ParseError::new(
			ParseErrorKind::ExpectedClosingDelim(state.end_delimiter.to_string()),
			src.position
		));
	}

	*source = src;
	Ok(Some(name))
}

fn parse_str_lit(source: &mut Source) -> ParseResult<String> {
	let mut src = source.clone();
	let mut str = String::new();
	let mut escape = false;
	let mut complete = false;

	if src.next() != Some('"') {
		return Ok(None);
	}

	for ch in &mut src {
		if escape {
			str.push(ch);
			escape = false;
			continue;
		}
		if ch == '\\' {
			escape = true;
			continue;
		}
		if ch == '"' {
			complete = true;
			break;
		}
		str.push(ch)
	}

	if !complete {
		return Err(ParseError::new(
			ParseErrorKind::UnterminatedStrLit,
			src.position
		));
	}

	*source = src;
	Ok(Some(str))
}

fn parse_insert(source: &mut Source, state: &State) -> ParseResult<Insert> {
	let mut src = source.clone();

	if !has_delimiter(&mut src, &state.start_delimiter) {
		return Ok(None);
	}

	skip_whitespace(&mut src);

	let Some(name) = parse_name(&mut src)? else {
		return Err(ParseError::new(ParseErrorKind::ExpectedName, src.position));
	};

	skip_whitespace(&mut src);

	let mut format = None;
	if src.peek() == Some(':') {
		src.next().unwrap();
		skip_whitespace(&mut src);

		let Some(format_str) = parse_str_lit(&mut src)? else {
			return Err(ParseError::new(
				ParseErrorKind::ExpectedStrLit,
				src.position
			));
		};

		skip_whitespace(&mut src);

		format = Some(format_str)
	}

	if !has_delimiter(&mut src, &state.end_delimiter) {
		return Err(ParseError::new(
			ParseErrorKind::ExpectedClosingDelim(state.end_delimiter.clone()),
			src.position
		));
	}

	*source = src;
	Ok(Some(Insert { name, format }))
}

fn parse_section(source: &mut Source, state: &mut State) -> ParseResult<Section> {
	let mut src = source.clone();
	let mut content = Vec::<Token>::new();

	let name: Name;
	let inverted: bool;

	if let Some(n) = parse_tag(&mut src, state, Some('#'))? {
		name = n;
		inverted = false;
	} else if let Some(n) = parse_tag(&mut src, state, Some('^'))? {
		name = n;
		inverted = true;
	} else {
		return Ok(None);
	}

	loop {
		if let Some(end_name) = parse_tag(&mut src, state, Some('/'))? {
			if end_name != name {
				return Err(ParseError::new(
					ParseErrorKind::MismatchedSectionEnd(end_name.to_string(), name.to_string()),
					src.position
				));
			}
			break;
		}

		if let Some(maybe_token) = parse_token_or_instruction(&mut src, state)? {
			if let Some(token) = maybe_token {
				content.push(token);
			}
		} else {
			return Err(ParseError::new(
				ParseErrorKind::MissingSectionEnd(name.to_string()),
				src.position
			));
		}
	}

	*source = src;
	Ok(Some(Section {
		name,
		inverted,
		content
	}))
}

fn parse_token_or_instruction(
	source: &mut Source,
	state: &mut State
) -> ParseResult<Option<Token>> {
	if parse_instruction(source, state)?.is_some() {
		return Ok(Some(None));
	}
	let Some(token) = parse_token(source, state)? else {
		return Ok(None);
	};
	Ok(Some(Some(token)))
}

fn parse_delimiter_definition(source: &mut Source) -> ParseResult<String> {
	let mut src = source.clone();
	let mut delimiter = String::new();

	while let Some(ch) = src.peek() {
		if ch.is_whitespace() || ch == '=' {
			break;
		}
		delimiter.push(src.next().unwrap());
	}

	if delimiter.is_empty() {
		return Ok(None);
	}

	if delimiter.contains(':') {
		return Err(ParseError::new(
			ParseErrorKind::ForbiddenDelimiterChar(':'),
			source.position
		));
	}

	*source = src;
	Ok(Some(delimiter))
}

fn parse_set_fmt(source: &mut Source, state: &State) -> ParseResult<SetFmt> {
	let mut src = source.clone();
	let setfmt_start = format!("{}%", state.start_delimiter);
	let setfmt_end = format!("%{}", state.end_delimiter);

	if !has_delimiter(&mut src, &setfmt_start) {
		return Ok(None);
	}

	skip_whitespace(&mut src);

	let Some(type_name) = parse_str_lit(&mut src)? else {
		return Err(ParseError::new(
			ParseErrorKind::ExpectedStrLit,
			src.position
		));
	};

	skip_whitespace(&mut src);

	if src.peek() == Some(':') {
		src.next().unwrap();
	} else {
		return Err(ParseError::new(
			ParseErrorKind::ExpectedOp(':'),
			src.position
		));
	}

	skip_whitespace(&mut src);

	let Some(format) = parse_str_lit(&mut src)? else {
		return Err(ParseError::new(
			ParseErrorKind::ExpectedStrLit,
			src.position
		));
	};

	skip_whitespace(&mut src);

	if !has_delimiter(&mut src, &setfmt_end) {
		return Err(ParseError::new(
			ParseErrorKind::ExpectedClosingDelim(setfmt_end.clone()),
			src.position
		));
	}

	*source = src;
	Ok(Some(SetFmt { type_name, format }))
}

fn parse_instruction(source: &mut Source, state: &mut State) -> ParseResult<()> {
	let mut src = source.clone();
	let instr_start = format!("{}=", state.start_delimiter);
	let instr_end = format!("={}", state.end_delimiter);

	if !has_delimiter(&mut src, &instr_start) {
		return Ok(None);
	}

	skip_whitespace(&mut src);

	let Some(start) = parse_delimiter_definition(&mut src)? else {
		return Err(ParseError::new(
			ParseErrorKind::MissingStartDelimiterDef,
			src.position
		));
	};

	skip_whitespace(&mut src);

	let Some(end) = parse_delimiter_definition(&mut src)? else {
		return Err(ParseError::new(
			ParseErrorKind::MissingEndDelimiterDef,
			src.position
		));
	};

	skip_whitespace(&mut src);

	if !has_delimiter(&mut src, &instr_end) {
		return Err(ParseError::new(
			ParseErrorKind::ExpectedClosingDelim(instr_end),
			src.position
		));
	}

	*source = src;
	state.start_delimiter = start;
	state.end_delimiter = end;
	Ok(Some(()))
}

fn parse_string(source: &mut Source, state: &State) -> ParseResult<String> {
	let mut checkpoint = source.clone();
	let mut checkpoint_dist = 0;
	let mut buffer = String::new();

	for ch in &mut *source {
		buffer.push(ch);
		if checkpoint_dist == state.start_delimiter.len() {
			checkpoint.next().unwrap();
		} else {
			checkpoint_dist += 1;
		}

		if buffer.ends_with(&state.start_delimiter) {
			for _ in 0..state.start_delimiter.len() {
				buffer.pop();
			}
			*source = checkpoint;
			break;
		}
	}

	if buffer.is_empty() {
		return Ok(None);
	}

	Ok(Some(buffer))
}

fn parse_token(source: &mut Source, state: &mut State) -> ParseResult<Token> {
	if let Some(section) = parse_section(source, state)? {
		Ok(Some(Token::Section(section)))
	} else if let Some(value) = parse_set_fmt(source, state)? {
		Ok(Some(Token::SetFmt(value)))
	} else if let Some(value) = parse_insert(source, state)? {
		Ok(Some(Token::Insert(value)))
	} else if let Some(string) = parse_string(source, state)? {
		Ok(Some(Token::String(string)))
	} else {
		Ok(None)
	}
}

fn parse_template(source: &mut Source, state: &mut State) -> ParseResult<Vec<Token>> {
	let mut tokens = Vec::new();
	while let Some(maybe_token) = parse_token_or_instruction(source, state)? {
		if let Some(token) = maybe_token {
			tokens.push(token);
		}
	}
	Ok(Some(tokens))
}

#[cfg(test)]
mod tests {
	use crate::template::Insert;

	use super::*;

	#[test]
	fn basic_template() {
		let template: Template = "Abc {{value1}} def {{value2}}".parse().unwrap();

		assert_eq!(
			template,
			Template::new(vec![
				Token::String("Abc ".to_string()),
				Token::Insert(Insert {
					name: Name(vec!["value1".to_string()]),
					format: None
				}),
				Token::String(" def ".to_string()),
				Token::Insert(Insert {
					name: Name(vec!["value2".to_string()]),
					format: None
				})
			])
		);
	}

	#[test]
	fn inherent_values() {
		let template: Template = "{{.}}".parse().unwrap();

		assert_eq!(
			template,
			Template::new(vec![Token::Insert(Insert {
				name: Name(vec![]),
				format: None
			})])
		);
	}

	#[test]
	fn dotted_names() {
		let template: Template = "{{abc.def}}".parse().unwrap();

		assert_eq!(
			template,
			Template::new(vec![Token::Insert(Insert {
				name: Name(vec!["abc".to_string(), "def".to_string()]),
				format: None
			})])
		);
	}

	#[test]
	fn sections() {
		let template: Template = "{{#abc}}{{^def}}{{value}}{{/def}}{{/abc}}".parse().unwrap();

		assert_eq!(
			template,
			Template::new(vec![Token::Section(Section {
				name: Name(vec!["abc".to_string()]),
				inverted: false,
				content: vec![Token::Section(Section {
					name: Name(vec!["def".to_string()]),
					inverted: true,
					content: vec![Token::Insert(Insert {
						name: Name(vec!["value".to_string()]),
						format: None
					})]
				})]
			})])
		);
	}

	#[test]
	fn delimiter_reassignment() {
		let template: Template = "{{=.% %.=}}.%value%..%= || || =%.||value2||"
			.parse()
			.unwrap();

		assert_eq!(
			template,
			Template::new(vec![
				Token::Insert(Insert {
					name: Name(vec!["value".to_string()]),
					format: None
				}),
				Token::Insert(Insert {
					name: Name(vec!["value2".to_string()]),
					format: None
				})
			])
		);
	}

	#[test]
	fn format_definition() {
		let template: Template = r#"{{value : "{a}"}}"#.parse().unwrap();

		assert_eq!(
			template,
			Template::new(vec![Token::Insert(Insert {
				name: Name(vec!["value".to_string()]),
				format: Some("{a}".to_string())
			})])
		)
	}

	#[test]
	fn setfmt() {
		let template: Template = r#"{{% "color" : "{r}" %}}"#.parse().unwrap();

		assert_eq!(
			template,
			Template::new(vec![Token::SetFmt(SetFmt {
				type_name: "color".to_string(),
				format: "{r}".to_string()
			})])
		)
	}
}
