use std::{env::join_paths, fs, process::Command};

use tempfile::{TempDir, tempdir};

#[derive(Debug)]
pub struct TestFixture {
	dir: TempDir,
}

impl TestFixture {
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn niji(&self) -> Command {
		let mut command = Command::new(self.dir.path().join("usr/bin/niji"));
		command.envs([
			(
				"XDG_CONFIG_HOME",
				self.dir.path().join("home/testuser/.config").as_os_str(),
			),
			(
				"XDG_DATA_HOME",
				self.dir
					.path()
					.join("home/testuser/.local/share")
					.as_os_str(),
			),
			(
				"XDG_STATE_HOME",
				self.dir
					.path()
					.join("home/testuser/.local/state")
					.as_os_str(),
			),
			(
				"XDG_CACHE_HOME",
				self.dir.path().join("home/testuser/.cache").as_os_str(),
			),
			(
				"XDG_RUNTIME_DIR",
				self.dir.path().join("run/user/1000").as_os_str(),
			),
			(
				"XDG_DATA_DIRS",
				&join_paths([
					self.dir.path().join("usr/local/share"),
					self.dir.path().join("usr/share"),
				])
				.unwrap(),
			),
			(
				"XDG_CONFIG_DIRS",
				self.dir.path().join("etc/xdg").as_os_str(),
			),
		]);
		command
	}
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn setup() -> TestFixture {
	let dir = tempdir().expect("Failed to create temporary directory for test fixture");

	fs::create_dir_all(dir.path().join("home/testuser/.config")).unwrap();
	fs::create_dir_all(dir.path().join("home/testuser/.local/share")).unwrap();
	fs::create_dir_all(dir.path().join("home/testuser/.local/state")).unwrap();
	fs::create_dir_all(dir.path().join("home/testuser/.cache")).unwrap();
	fs::create_dir_all(dir.path().join("usr/bin")).unwrap();
	fs::create_dir_all(dir.path().join("usr/share")).unwrap();
	fs::create_dir_all(dir.path().join("usr/local/share")).unwrap();
	fs::create_dir_all(dir.path().join("run/user/1000")).unwrap();

	Command::new("just")
		.args(["install", &dir.path().to_string_lossy()])
		.status()
		.expect("Failed to install niji in integration test");

	TestFixture { dir }
}
