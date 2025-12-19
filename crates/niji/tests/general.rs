mod common;

#[test]
fn install() {
	let fixture = common::setup();

	let out = fixture
		.niji()
		.arg("--version")
		.output()
		.expect("Failed to get niji version");

	assert_eq!(String::from_utf8_lossy(&out.stdout), "niji 0.5.0");
}
