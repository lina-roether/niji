use clap::ArgMatches;
use log::{LevelFilter, error};
use niji_console::ColorChoice;
mod syntax;

use crate::{app::NijiApp, cli::syntax::build_cmd};

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
	let matches = build_cmd().get_matches();
	cmd(&matches);
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
		_ => unreachable!(),
	}
}

fn cmd_apply(app: &NijiApp, args: &ArgMatches) {
	let no_reload = args.get_one::<bool>("no_reload").unwrap();
	let modules: Option<Vec<String>> = args
		.get_many::<String>("modules")
		.map(|v| v.cloned().collect());

	handle!(app.apply(!no_reload, modules.as_deref()));
}

fn cmd_theme(app: &NijiApp, args: &ArgMatches) {
	match args.subcommand() {
		Some(("get", _)) => cmd_theme_get(app),
		Some(("preview", args)) => cmd_theme_preview(app, args),
		Some(("set", args)) => cmd_theme_set(app, args),
		Some(("list", _)) => cmd_theme_list(app),
		Some(("unset", _)) => cmd_theme_unset(app),
		_ => unreachable!(),
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

		None => handle!(app.get_current_theme()),
	};

	niji_console::println!("Theme \"{}\":", theme.name.as_ref().unwrap());
	niji_console::println!();
	niji_console::println!("{theme}");
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
		niji_console::println!("{theme}");
	}

	if empty {
		error!("No usable themes were found");
	}
}

fn cmd_theme_unset(app: &NijiApp) {
	handle!(app.unset_current_theme());
}
