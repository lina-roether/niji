mod app;
mod cli;
mod config;
mod file_manager;
mod files;
mod lua;
mod module;
mod module_manager;
mod template;
mod theme_manager;
mod types;
mod utils;

fn main() {
	cli::run();
}
