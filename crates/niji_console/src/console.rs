use std::{
	fmt::Arguments,
	io::{self, BufRead, BufReader, IsTerminal},
};

use parking_lot::Mutex;
use termcolor::{BufferedStandardStream, Color, ColorChoice, ColorSpec, WriteColor};

pub struct Console<I = BufReader<io::Stdin>, O = BufferedStandardStream> {
	input: Mutex<I>,
	output: Mutex<O>,
	err_output: Mutex<O>,
}

impl Console {
	pub fn new_std(color_choice: ColorChoice) -> Self {
		let mut stdout_color = color_choice;
		let mut stderr_color = color_choice;

		if color_choice == ColorChoice::Auto {
			if !io::stdout().is_terminal() {
				stdout_color = ColorChoice::Never;
			}
			if !io::stderr().is_terminal() {
				stderr_color = ColorChoice::Never;
			}
		}

		Self::new(
			BufReader::new(io::stdin()),
			BufferedStandardStream::stdout(stdout_color),
			BufferedStandardStream::stderr(stderr_color),
		)
	}
}

impl<I, O> Console<I, O> {
	pub fn new(input: I, output: O, err_output: O) -> Self {
		Self {
			input: Mutex::new(input),
			output: Mutex::new(output),
			err_output: Mutex::new(err_output),
		}
	}
}

impl<I, O> Console<I, O>
where
	O: WriteColor,
{
	pub fn log_error(&self, args: &Arguments) -> anyhow::Result<()> {
		Self::log(
			&mut self.err_output.lock(),
			"ERROR",
			ColorSpec::new()
				.set_fg(Some(Color::Red))
				.set_intense(true)
				.set_bold(true),
			args,
			ColorSpec::new().set_fg(Some(Color::Red)),
		)
	}

	pub fn log_warn(&self, args: &Arguments) -> anyhow::Result<()> {
		Self::log(
			&mut self.err_output.lock(),
			" WARN",
			ColorSpec::new()
				.set_fg(Some(Color::Yellow))
				.set_intense(true)
				.set_bold(true),
			args,
			ColorSpec::new().set_fg(Some(Color::Yellow)),
		)
	}

	pub fn log_info(&self, args: &Arguments) -> anyhow::Result<()> {
		Self::log(
			&mut self.output.lock(),
			" INFO",
			ColorSpec::new()
				.set_fg(Some(Color::Blue))
				.set_intense(true)
				.set_bold(true),
			args,
			ColorSpec::new()
				.set_fg(Some(Color::White))
				.set_intense(true),
		)
	}

	pub fn log_debug(&self, args: &Arguments) -> anyhow::Result<()> {
		Self::log(
			&mut self.err_output.lock(),
			"DEBUG",
			ColorSpec::new().set_fg(Some(Color::White)),
			args,
			ColorSpec::new().set_fg(Some(Color::White)),
		)
	}

	pub fn log_trace(&self, args: &Arguments) -> anyhow::Result<()> {
		Self::log(
			&mut self.err_output.lock(),
			"TRACE",
			ColorSpec::new().set_fg(Some(Color::White)),
			args,
			ColorSpec::new().set_fg(Some(Color::White)),
		)
	}

	fn log(
		out: &mut O,
		tag: &str,
		tag_color: &ColorSpec,
		message: &Arguments,
		message_color: &ColorSpec,
	) -> anyhow::Result<()> {
		out.set_color(tag_color).unwrap();

		write!(out, "{tag}")?;

		out.set_color(
			ColorSpec::new()
				.set_fg(Some(Color::Black))
				.set_intense(true),
		)
		.unwrap();

		write!(out, " - ")?;

		out.set_color(message_color).unwrap();

		let string = format!("{message}");
		let mut lines = string.lines();
		writeln!(out, "{}", lines.next().unwrap_or_default())?;
		for line in lines {
			writeln!(out, "{}   {line}", " ".repeat(tag.len()))?;
		}

		out.reset().unwrap();
		Ok(())
	}

	pub fn heading(&self, args: &Arguments) -> anyhow::Result<()> {
		let stdout = &mut self.output.lock();

		let mut decoration_color = ColorSpec::new();
		decoration_color
			.set_fg(Some(Color::Black))
			.set_intense(true);

		stdout.set_color(&decoration_color).unwrap();

		write!(stdout, "\n ==== [ ")?;

		stdout
			.set_color(
				ColorSpec::new()
					.set_fg(Some(Color::White))
					.set_intense(true)
					.set_bold(true),
			)
			.unwrap();

		write!(stdout, "{args}")?;

		stdout.set_color(&decoration_color).unwrap();

		writeln!(stdout, " ] ====")?;

		stdout.reset().unwrap();
		Ok(())
	}

	pub fn println(&self, args: Option<&Arguments>) -> anyhow::Result<()> {
		let stdout = &mut self.output.lock();

		match args {
			Some(args) => writeln!(stdout, "{args}")?,
			None => writeln!(stdout)?,
		}

		stdout.flush()?;
		Ok(())
	}

	pub fn flush(&self) -> anyhow::Result<()> {
		let stdout = &mut self.output.lock();
		let stderr = &mut self.err_output.lock();

		stdout.flush()?;
		stderr.flush()?;
		Ok(())
	}
}

impl<I, O> Console<I, O>
where
	I: BufRead,
	O: WriteColor,
{
	pub fn prompt(&self, args: &Arguments, default: Option<bool>) -> anyhow::Result<bool> {
		let stdout = &mut self.output.lock();

		loop {
			stdout
				.set_color(
					ColorSpec::new()
						.set_fg(Some(Color::White))
						.set_intense(true),
				)
				.unwrap();

			write!(stdout, "{args} ")?;

			stdout
				.set_color(
					ColorSpec::new()
						.set_fg(Some(Color::Blue))
						.set_intense(true)
						.set_bold(true),
				)
				.unwrap();

			match default {
				Some(true) => write!(stdout, "[Y/n]")?,
				Some(false) => write!(stdout, "[y/N]")?,
				None => write!(stdout, "[y/n]")?,
			}

			stdout
				.set_color(
					ColorSpec::new()
						.set_fg(Some(Color::White))
						.set_intense(true),
				)
				.unwrap();

			write!(stdout, ": ")?;
			stdout.flush()?;

			let mut response = String::new();
			self.input.lock().read_line(&mut response)?;

			response = response.trim().to_string().to_lowercase();

			match response.as_str() {
				"y" => return Ok(true),
				"n" => return Ok(false),
				"" => {
					if let Some(default) = default {
						return Ok(default);
					}
				}
				_ => (),
			}

			stdout.reset().unwrap();
		}
	}
}

#[cfg(test)]
mod tests {
	use std::io::Cursor;

	use super::*;

	#[test]
	fn log_message() {
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new((), &mut out, &mut err);

		console.log_info(&format_args!("Test")).unwrap();

		assert_eq!(String::from_utf8_lossy(&out.into_inner()), " INFO - Test\n");
		assert_eq!(&err.into_inner(), b"");
	}

	#[test]
	fn log_diagnostics() {
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new((), &mut out, &mut err);

		console.log_trace(&format_args!("Test #1")).unwrap();
		console.log_debug(&format_args!("Test #2")).unwrap();
		console.log_warn(&format_args!("Test #3")).unwrap();
		console.log_error(&format_args!("Test #4")).unwrap();

		assert_eq!(&out.into_inner(), b"");
		assert_eq!(
			String::from_utf8_lossy(&err.into_inner()),
			"TRACE - Test #1\nDEBUG - Test #2\n WARN - Test #3\nERROR - Test #4\n"
		);
	}

	#[test]
	fn print_heading() {
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new((), &mut out, &mut err);

		console.heading(&format_args!("TEST :)")).unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			" ==== [ TEST :) ] ====\n"
		);
		assert_eq!(&err.into_inner(), b"");
	}

	#[test]
	fn print_raw_line() {
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new((), &mut out, &mut err);

		console.println(Some(&format_args!("Test"))).unwrap();

		assert_eq!(String::from_utf8_lossy(&out.into_inner()), "Test\n");
		assert_eq!(&err.into_inner(), b"");
	}

	#[test]
	fn prompt_yes() {
		let input = BufReader::new(Cursor::new("y\n".to_string().into_bytes()));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console.prompt(&format_args!("Test Prompt"), None).unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [y/n]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(result);
	}

	#[test]
	fn prompt_no() {
		let input = BufReader::new(Cursor::new("n\n".to_string().into_bytes()));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console.prompt(&format_args!("Test Prompt"), None).unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [y/n]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(!result);
	}

	#[test]
	fn prompt_garbage() {
		let input = BufReader::new(Cursor::new(
			"\nnsdfksdf\nysydfds\nn\n".to_string().into_bytes(),
		));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console.prompt(&format_args!("Test Prompt"), None).unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [y/n]: Test Prompt [y/n]: Test Prompt [y/n]: Test Prompt [y/n]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(!result);
	}

	#[test]
	fn prompt_lenient_yes() {
		let input = BufReader::new(Cursor::new("  Y    \n".to_string().into_bytes()));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console.prompt(&format_args!("Test Prompt"), None).unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [y/n]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(result);
	}

	#[test]
	fn prompt_lenient_no() {
		let input = BufReader::new(Cursor::new("  N    \n".to_string().into_bytes()));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console.prompt(&format_args!("Test Prompt"), None).unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [y/n]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(!result);
	}

	#[test]
	fn prompt_default_yes() {
		let input = BufReader::new(Cursor::new("\n".to_string().into_bytes()));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console
			.prompt(&format_args!("Test Prompt"), Some(true))
			.unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [Y/n]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(result);
	}

	#[test]
	fn prompt_default_no() {
		let input = BufReader::new(Cursor::new("\n".to_string().into_bytes()));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console
			.prompt(&format_args!("Test Prompt"), Some(false))
			.unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [y/N]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(!result);
	}

	#[test]
	fn prompt_default_yes_redundant() {
		let input = BufReader::new(Cursor::new("y\n".to_string().into_bytes()));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console
			.prompt(&format_args!("Test Prompt"), Some(true))
			.unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [Y/n]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(result);
	}

	#[test]
	fn prompt_default_no_redundant() {
		let input = BufReader::new(Cursor::new("n\n".to_string().into_bytes()));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console
			.prompt(&format_args!("Test Prompt"), Some(false))
			.unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [y/N]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(!result);
	}

	#[test]
	fn prompt_default_yes_override() {
		let input = BufReader::new(Cursor::new("n\n".to_string().into_bytes()));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console
			.prompt(&format_args!("Test Prompt"), Some(true))
			.unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [Y/n]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(!result);
	}

	#[test]
	fn prompt_default_no_override() {
		let input = BufReader::new(Cursor::new("y\n".to_string().into_bytes()));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console
			.prompt(&format_args!("Test Prompt"), Some(false))
			.unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [y/N]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(result);
	}

	#[test]
	fn prompt_default_garbage() {
		let input = BufReader::new(Cursor::new(
			"nsdfnfsdf\nyfsddfgfgsdfgdf\n\n".to_string().into_bytes(),
		));
		let mut out = termcolor::Buffer::no_color();
		let mut err = termcolor::Buffer::no_color();
		let console = Console::new(input, &mut out, &mut err);

		let result = console
			.prompt(&format_args!("Test Prompt"), Some(false))
			.unwrap();

		assert_eq!(
			String::from_utf8_lossy(&out.into_inner()),
			"Test Prompt [y/N]: Test Prompt [y/N]: Test Prompt [y/N]: "
		);
		assert_eq!(&err.into_inner(), b"");
		assert!(!result);
	}
}
