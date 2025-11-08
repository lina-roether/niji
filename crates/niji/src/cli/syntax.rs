use clap::{Arg, ArgAction, Command};

pub const NAME: &str = "niji";
pub const AUTHOR: &str = "Lina Roether <lina.roether@proton.me>";

pub fn build_cmd() -> Command {
	Command::new(NAME)
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
				.help("Disables all log messages"),
		)
		.arg(
			Arg::new("verbose")
				.long("verbose")
				.short('v')
				.action(ArgAction::SetTrue)
				.conflicts_with("quiet")
				.global(true)
				.help("Prints additional debug output"),
		)
		.arg(
			Arg::new("no_color")
				.long("no-color")
				.short('b')
				.action(ArgAction::SetTrue)
				.global(true)
				.help("Disable color output"),
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
							 applied.",
						),
				)
				.arg(
					Arg::new("no_reload")
						.long("no-reload")
						.short('k')
						.action(ArgAction::SetTrue)
						.help(
							"Do not reload the module targets to apply the changes immediately. \
							 Changes will only take effect after a restart.",
						),
				),
		)
		.subcommand(
			Command::new("theme")
				.about(
					"Perform actions related to themes, such as changing the theme or listing \
					 available themes",
				)
				.subcommand_required(true)
				.subcommand(Command::new("get").about("Get the name of the current theme"))
				.subcommand(
					Command::new("preview")
						.about("Display a preview of a theme in the console")
						.arg(Arg::new("name").help(
							"The theme to preview. Defaults to the current theme if not set.",
						)),
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
								.conflicts_with("no_reload"),
						)
						.arg(
							Arg::new("no_reload")
								.long("no-reload")
								.short('k')
								.action(ArgAction::SetTrue)
								.help(
									"Do not reload the module targets to apply the changes \
									 immediately. Changes will only take effect after a restart.",
								),
						),
				)
				.subcommand(Command::new("list").about("List the names of available themes"))
				.subcommand(Command::new("unset").about(
					"Unset the current theme. Note that this will not make any changes to the \
					 emitted files!",
				)),
		)
}
