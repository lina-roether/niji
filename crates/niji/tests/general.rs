use assert_cmd::cargo::cargo_bin_cmd;

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
