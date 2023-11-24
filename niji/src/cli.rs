use clap::{Arg, ArgAction, ArgMatches, Command};
use log::{error, LevelFilter};
use niji_console::ColorChoice;

use crate::app::NijiApp;

const AUTHOR: &str = "Nicholas Roether <nicholas.roether@t-online.de>";

macro_rules! handle {
	($expr:expr, $cleanup:expr) => {
		match $expr {
			Ok(val) => val,
			Err(err) => {
				log::error!("{err}");

				#[allow(clippy::redundant_closure_call)]
				$cleanup();

				return;
			}
		}
	};
	($expr:expr) => {
		handle!($expr, || ())
	};
}

pub fn run() {
	let matches = Command::new("niji")
		.author(AUTHOR)
		.about("An extensible desktop theming utility")
		.version(env!("CARGO_PKG_VERSION"))
		.subcommand_required(true)
		.arg_required_else_help(true)
		.arg(
			Arg::new("quiet")
				.long("quiet")
				.short('q')
				.action(ArgAction::SetTrue)
				.conflicts_with("verbose")
				.help("Disables all log messages")
		)
		.arg(
			Arg::new("verbose")
				.long("verbose")
				.short('v')
				.action(ArgAction::SetTrue)
				.conflicts_with("quiet")
				.help("Prints additional debug output")
		)
		.arg(
			Arg::new("no_color")
				.long("no-color")
				.short('b')
				.action(ArgAction::SetTrue)
				.help("Disable color output")
		)
		.subcommand(
			Command::new("apply")
				.about("Apply (or re-apply) the current theme and and configuration")
		)
		.subcommand(
			Command::new("theme")
				.about(
					"Perform actions related to themes, such as changing the theme or listing \
					 available themes"
				)
				.subcommand_required(true)
				.subcommand(Command::new("get").about("Get the name of the current theme"))
				.subcommand(
					Command::new("show")
						.about("Display a preview of a theme in the console")
						.arg(Arg::new("name").help(
							"The theme to preview. Defaults to the current theme if not set."
						))
				)
				.subcommand(
					Command::new("set")
						.about("Change the current theme")
						.arg_required_else_help(true)
						.arg(Arg::new("name").help("The name of the theme to change to"))
						.arg(
							Arg::new("apply")
								.long("no-apply")
								.short('N')
								.action(ArgAction::SetFalse)
								.help("Don't apply the theme after setting it")
						)
				)
				.subcommand(Command::new("list").about("List the names of available themes"))
				.subcommand(Command::new("reset").about(
					"Reset the current theme. Note that this will not make any changes to the \
					 emitted files!"
				))
		)
		.get_matches();

	cmd(&matches)
}

fn cmd(args: &ArgMatches) {
	let quiet = *args.get_one::<bool>("quiet").unwrap();
	let verbose = *args.get_one::<bool>("verbose").unwrap();
	let no_color = *args.get_one::<bool>("no_color").unwrap();

	let level = if quiet {
		LevelFilter::Off
	} else if verbose {
		LevelFilter::Debug
	} else {
		LevelFilter::Info
	};

	let color_choice = if no_color {
		ColorChoice::Never
	} else {
		ColorChoice::Auto
	};

	niji_console::init(level, color_choice);

	let app = handle!(NijiApp::init());

	match args.subcommand() {
		Some(("apply", _)) => cmd_apply(&app),
		Some(("theme", args)) => cmd_theme(&app, args),
		_ => unreachable!()
	}
}

fn cmd_apply(app: &NijiApp) {
	handle!(app.apply())
}

fn cmd_theme(app: &NijiApp, args: &ArgMatches) {
	match args.subcommand() {
		Some(("get", _)) => cmd_theme_get(app),
		Some(("show", args)) => cmd_theme_show(app, args),
		Some(("set", args)) => cmd_theme_set(app, args),
		Some(("list", _)) => cmd_theme_list(app),
		Some(("reset", _)) => cmd_theme_reset(app),
		_ => unreachable!()
	}
}

fn cmd_theme_get(app: &NijiApp) {
	let theme = handle!(app.current_theme());
	println!("{}", theme.name);
}

fn cmd_theme_show(app: &NijiApp, args: &ArgMatches) {
	let name = args.get_one::<String>("name");

	let resolved_name;

	let theme = match name {
		Some(name) => {
			resolved_name = name.clone();
			handle!(app.get_theme(name))
		}
		None => {
			let theme = handle!(app.current_theme());
			resolved_name = theme.name;
			theme.values
		}
	};

	println!("Theme \"{resolved_name}\":");
	println!();
	println!("{theme}")
}

fn cmd_theme_set(app: &NijiApp, args: &ArgMatches) {
	let name = args.get_one::<String>("name").unwrap().as_str();
	let apply = *args.get_one::<bool>("apply").unwrap();

	handle!(app.set_theme(name));
	if apply {
		handle!(app.apply());
	}
}

fn cmd_theme_list(app: &NijiApp) {
	let mut empty = true;

	for theme in app.list_themes() {
		empty = false;
		println!("{theme}")
	}

	if empty {
		error!("No usable themes were found");
	}
}

fn cmd_theme_reset(app: &NijiApp) {
	handle!(app.reset_theme())
}
