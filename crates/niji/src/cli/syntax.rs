use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    bin_name = "niji",
    author = "Lina Roether <lina.roether@proton.me>",
    version = env!("CARGO_PKG_VERSION"),
    about = "An extensible desktop theming utility",
)]
pub struct Niji {
	#[command(flatten)]
	pub global_args: GlobalArgs,

	#[command(subcommand)]
	pub command: NijiCommand,
}

#[derive(Args, Debug)]
pub struct GlobalArgs {
	#[arg(
		short = 'q',
		long = "quiet",
		global = true,
		conflicts_with = "verbose",
		help = "Disables all log messages"
	)]
	pub quiet: bool,

	#[arg(
		short = 'v',
		long = "verbose",
		global = true,
		conflicts_with = "quiet",
		help = "Prints additional debug output"
	)]
	pub verbose: bool,

	#[arg(
		short = 'b',
		long = "no-color",
		global = true,
		help = "Disable colored output"
	)]
	pub no_color: bool,
}

#[derive(Subcommand, Debug)]
pub enum NijiCommand {
	Apply(Apply),
	Theme(Theme),
	Accent(Accent),
}

#[derive(Args, Debug)]
pub struct ApplyArgs {
	#[arg(
		short = 'k',
		long = "no-reload",
		help = "Do not reload the module targets to apply the changes immediately. Changes will \
		        only take effect after a restart."
	)]
	pub no_reload: bool,

	#[arg(
		short = 'i',
		long = "ignore-deps",
		help = "Ignore missing module dependencies"
	)]
	pub ignore_deps: bool,
}

#[derive(Args, Debug)]
pub struct UpdateArgs {
	#[arg(
		short = 'n',
		long = "no-apply",
		help = "Don't apply the new styles after setting them. The changes won't be visible until \
		        applied."
	)]
	pub no_apply: bool,

	#[command(flatten)]
	pub apply_args: ApplyArgs,
}

#[derive(Parser, Debug)]
#[command(about = "Apply (or re-apply) the current theme and configuration")]
pub struct Apply {
	#[arg(
		short = 'M',
		long = "module",
		help = "The module to apply the config to. Can be set multiple times to apply to multiple \
		        modules. If not set, all active modules will be applied."
	)]
	pub modules: Vec<String>,

	#[command(flatten)]
	pub apply_args: ApplyArgs,
}

#[derive(Parser, Debug)]
#[command(
	about = "Perform actions related to themes, such as changing the theme or listing available \
	         themes"
)]
pub struct Theme {
	#[command(subcommand)]
	pub command: ThemeCommand,
}

#[derive(Subcommand, Debug)]
pub enum ThemeCommand {
	Get(ThemeGet),
	Preview(ThemePreview),
	Set(ThemeSet),
	List(ThemeList),
	Unset(ThemeUnset),
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum PaletteColor {
	Pink,
	Red,
	Orange,
	Yellow,
	Green,
	Teal,
	Blue,
	Purple,
	Black,
	White,
}

#[derive(Args, Debug)]
pub struct AccentParams {
	#[arg(short = 'A', long = "accent", help = "Set the accent color to use")]
	pub accent: Option<PaletteColor>,
}

#[derive(Parser, Debug)]
#[command(about = "Get the name of the current theme")]
pub struct ThemeGet;

#[derive(Parser, Debug)]
#[command(about = "Display a preview of a theme in the terminal")]
pub struct ThemePreview {
	#[arg(help = "The name of the theme to preview. Defaults to the current theme if not set.")]
	pub name: Option<String>,

	#[command(flatten)]
	pub accent_args: AccentParams,
}

#[derive(Parser, Debug)]
#[command(about = "Change the current theme")]
pub struct ThemeSet {
	#[arg(help = "THe name of the theme to change to")]
	pub name: String,

	#[command(flatten)]
	pub update_args: UpdateArgs,

	#[command(flatten)]
	pub accent_args: AccentParams,
}

#[derive(Parser, Debug)]
#[command(about = "List the names of available themes")]
pub struct ThemeList;

#[derive(Parser, Debug)]
#[command(about = "Unset the current theme. This will cause an error on the next application.")]
pub struct ThemeUnset;

#[derive(Parser, Debug)]
#[command(about = "Query or set the current accent color")]
pub struct Accent {
	#[command(subcommand)]
	pub command: AccentCommand,
}

#[derive(Subcommand, Debug)]
pub enum AccentCommand {
	Get(AccentGet),
	Set(AccentSet),
	Unset(AccentUnset),
}

#[derive(Parser, Debug)]
#[command(about = "Print the name of the current accent color")]
pub struct AccentGet;

#[derive(Parser, Debug)]
#[command(about = "Set the current accent color")]
pub struct AccentSet {
	#[arg(help = "The name of the palette color to use")]
	pub color: PaletteColor,

	#[command(flatten)]
	pub update_args: UpdateArgs,
}

#[derive(Parser, Debug)]
#[command(
	about = "Unset the current accent color. This will cause an error on the next application."
)]
pub struct AccentUnset;
