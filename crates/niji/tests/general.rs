use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn version() {
	let cmd = cargo_bin_cmd!("niji").arg("--version").assert();
	cmd.success().stdout("niji 0.5.1\n");
}

#[test]
fn help_option() {
	let cmd = cargo_bin_cmd!("niji").arg("--help").assert();
	cmd.success();
}

#[test]
fn help_command() {
	let cmd = cargo_bin_cmd!("niji").arg("help").assert();
	cmd.success();
}
