use console::Console;
pub use log::LevelFilter;
use logger::Logger;
pub use termcolor::ColorChoice;

pub mod api;
mod console;
mod logger;

#[macro_use]
mod macros;

pub fn init(level: LevelFilter, color_choice: ColorChoice) {
	let console = Console::new_std(color_choice);
	if level != LevelFilter::Off {
		api::set_console(console);
	}

	let logger = Box::new(Logger::new(level));
	log::set_logger(Box::leak(logger)).unwrap();
	log::set_max_level(level);
}
