use clap::{Arg, ArgAction, ArgMatches, Command};
use log::{error, LevelFilter};
use niji_console::ColorChoice;

use crate::app::NijiApp;

const AUTHOR: &str = "Lina Roether <lina.roether@proton.me>";

macro_rules! handle {
	($expr:expr, $cleanup:expr) => {
		match $expr {
			Ok(val) => val,
			Err(err) => {
				log::error!("{err:?}");

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
				.global(true)
				.help("Disables all log messages")
		)
		.arg(
			Arg::new("verbose")
				.long("verbose")
				.short('v')
				.action(ArgAction::SetTrue)
				.conflicts_with("quiet")
				.global(true)
				.help("Prints additional debug output")
		)
		.arg(
			Arg::new("no_color")
				.long("no-color")
				.short('b')
				.action(ArgAction::SetTrue)
				.global(true)
				.help("Disable color output")
		)
		.subcommand(
			Command::new("apply")
				.about("Apply (or re-apply) the current theme and and configuration")
				.arg(
					Arg::new("modules")
						.long("module")
						.short('M')
						.action(ArgAction::Append)
						.help(
							"The module to apply the config to. Can be set multiple times to \
							 apply to multiple modules. If not set, all active modules will be \
							 applied."
						)
				)
				.arg(
					Arg::new("no_reload")
						.long("no-reload")
						.short('k')
						.action(ArgAction::SetTrue)
						.help(
							"Do not reload the module targets to apply the changes immediately. \
							 Changes will only take effect after a restart."
						)
				)
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
					Command::new("preview")
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
							Arg::new("no_apply")
								.long("no-apply")
								.short('n')
								.action(ArgAction::SetTrue)
								.help("Don't apply the theme after setting it")
								.conflicts_with("no_reload")
						)
						.arg(
							Arg::new("no_reload")
								.long("no-reload")
								.short('k')
								.action(ArgAction::SetTrue)
								.help(
									"Do not reload the module targets to apply the changes \
									 immediately. Changes will only take effect after a restart."
								)
						)
				)
				.subcommand(Command::new("list").about("List the names of available themes"))
				.subcommand(Command::new("unset").about(
					"Unset the current theme. Note that this will not make any changes to the \
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
		Some(("apply", args)) => cmd_apply(&app, args),
		Some(("theme", args)) => cmd_theme(&app, args),
		_ => unreachable!()
	}
}

fn cmd_apply(app: &NijiApp, args: &ArgMatches) {
	let no_reload = args.get_one::<bool>("no_reload").unwrap();
	let modules: Option<Vec<String>> = args
		.get_many::<String>("modules")
		.map(|v| v.cloned().collect());

	handle!(app.apply(!no_reload, modules.as_deref()))
}

fn cmd_theme(app: &NijiApp, args: &ArgMatches) {
	match args.subcommand() {
		Some(("get", _)) => cmd_theme_get(app),
		Some(("show", args)) => cmd_theme_preview(app, args),
		Some(("set", args)) => cmd_theme_set(app, args),
		Some(("list", _)) => cmd_theme_list(app),
		Some(("unset", _)) => cmd_theme_unset(app),
		_ => unreachable!()
	}
}

fn cmd_theme_get(app: &NijiApp) {
	let theme = handle!(app.get_current_theme());
	niji_console::println!("{}", theme.name.unwrap());
}

fn cmd_theme_preview(app: &NijiApp, args: &ArgMatches) {
	let name = args.get_one::<String>("name");
	let no_color = args.get_one::<bool>("no_color").unwrap();

	if *no_color {
		error!(
			"Theme display is not supported in no-color mode. You can query the theme name by \
			 using `niji theme get`."
		);
		return;
	}

	let theme = match name {
		Some(name) => handle!(app.get_theme(name)),

		None => handle!(app.get_current_theme())
	};

	niji_console::println!("Theme \"{}\":", theme.name.as_ref().unwrap());
	niji_console::println!();
	niji_console::println!("{theme}")
}

fn cmd_theme_set(app: &NijiApp, args: &ArgMatches) {
	let name = args.get_one::<String>("name").unwrap().as_str();
	let no_apply = *args.get_one::<bool>("no_apply").unwrap();
	let no_reload = *args.get_one::<bool>("no_reload").unwrap();

	handle!(app.set_current_theme(name));
	if !no_apply {
		handle!(app.apply(!no_reload, None));
	}
}

fn cmd_theme_list(app: &NijiApp) {
	let mut empty = true;

	for theme in app.list_themes() {
		empty = false;
		niji_console::println!("{theme}")
	}

	if empty {
		error!("No usable themes were found");
	}
}

fn cmd_theme_unset(app: &NijiApp) {
	handle!(app.unset_current_theme())
}
