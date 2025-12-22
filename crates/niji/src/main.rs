use std::process::ExitCode;

mod app;
mod cli;
mod config;
mod files;
mod lua;
mod managed_fs;
mod module;
mod module_manager;
mod template;
mod theme;
mod theme_manager;
mod types;
mod utils;

fn main() -> ExitCode {
	cli::run()
}
