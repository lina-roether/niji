use assert_cmd::cargo::cargo_bin_cmd;
use tempfile::tempdir;

const BUILTIN_THEMES: &[&str] = &[
	"catppuccin-frappe",
	"catppuccin-latte",
	"dracula",
	"gruvbox",
	"gruvbox-light",
	"tokyonight",
];

#[test]
fn help_option() {
	cargo_bin_cmd!("niji")
		.args(["theme", "--help"])
		.assert()
		.success();
}

#[test]
fn help_command() {
	cargo_bin_cmd!("niji")
		.args(["help", "theme"])
		.assert()
		.success();
}

#[test]
fn list_themes_global() {
	let config_dir = tempdir().unwrap();

	cargo_bin_cmd!("niji")
		.env(
			"XDG_DATA_DIRS",
			concat!(env!("CARGO_MANIFEST_DIR"), "/tests/theme.in"),
		)
		.env("XDG_CONFIG_HOME", config_dir.path().as_os_str())
		.args(["-v", "theme", "list"])
		.assert()
		.success()
		.stdout(
			BUILTIN_THEMES
				.iter()
				.fold(String::new(), |acc, s| format!("{acc}{s}\n")),
		);
}

#[test]
fn list_themes_local() {
	cargo_bin_cmd!("niji")
		.env("XDG_DATA_DIRS", "")
		.env(
			"XDG_CONFIG_HOME",
			concat!(env!("CARGO_MANIFEST_DIR"), "/tests/theme.in"),
		)
		.args(["-v", "theme", "list"])
		.assert()
		.success()
		.stdout(
			BUILTIN_THEMES
				.iter()
				.fold(String::new(), |acc, s| format!("{acc}{s}\n")),
		);
}

#[test]
fn preview_themes() {
	for theme in BUILTIN_THEMES {
		cargo_bin_cmd!("niji")
			.env("XDG_DATA_DIRS", "")
			.env(
				"XDG_CONFIG_HOME",
				concat!(env!("CARGO_MANIFEST_DIR"), "/tests/theme.in"),
			)
			.args(["-v", "theme", "preview", theme, "--accent", "red"])
			.assert()
			.success();
	}
}
