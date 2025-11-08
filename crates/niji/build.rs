use std::{env, fs, path::PathBuf};

use anyhow::Context;
use clap_complete::shells;

pub mod syntax {
	include!("./src/cli/syntax.rs");
}

fn main() -> anyhow::Result<()> {
	let Some(outdir) = env::var_os("OUT_DIR") else {
		println!("cargo:warning=No out dir set, completions will not be generated");
		return Ok(());
	};

	let comp_dir = PathBuf::from(outdir).join("../../../completions");
	fs::create_dir_all(&comp_dir)?;

	let mut cmd = syntax::build_cmd();

	clap_complete::generate_to(shells::Bash, &mut cmd, syntax::NAME, &comp_dir)
		.context("Failed to generate bash completions")?;
	clap_complete::generate_to(shells::Zsh, &mut cmd, syntax::NAME, &comp_dir)
		.context("Failed to generate zsh completions")?;
	clap_complete::generate_to(shells::Fish, &mut cmd, syntax::NAME, &comp_dir)
		.context("Failed to generate fish completions")?;

	Ok(())
}
