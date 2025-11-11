use anyhow::{Context, anyhow};
use log::{debug, info, warn};
use niji_console::prompt;
use std::{
	collections::hash_map::DefaultHasher,
	fs::{self, File},
	hash::{Hash, Hasher},
	io::{BufReader, Read},
	path::{Path, PathBuf},
};

const CHECKSUM_XATTR: &str = "user.niji.checksum";

pub fn write(path: &Path, string: &str) -> anyhow::Result<()> {
	if !path.exists() {
		debug!("Creating new managed file at {}", path.display());
		init_new_file(path, string)
	} else {
		manage_existing_file(path, string)
	}
}

fn init_new_file(path: &Path, string: &str) -> anyhow::Result<()> {
	fs::write(path, string).context(format!("Failed to write to {}", path.display()))?;
	set_managed(path.to_path_buf())?;

	info!("niji now manages {}", path.display());

	Ok(())
}

fn manage_existing_file(path: &Path, string: &str) -> anyhow::Result<()> {
	let current_hash = hash_contents(path)?;
	debug!("{} has current hash {current_hash}", path.display());

	if let Some(known_hash) = get_managed_hash(path)? {
		if current_hash == known_hash {
			debug!("Writing to managed file at {}", path.display());
			fs::write(path, string).context(format!("Failed to write to {}", path.display()))?;
			set_managed(path.to_path_buf())?;
			return Ok(());
		} else {
			debug!("File contents of {} have changed", path.display())
		}
	} else {
		debug!("{} is not in the managed files table", path.display())
	}

	backup_and_replace(path, string, current_hash)
}

fn backup_and_replace(path: &Path, string: &str, hash: u64) -> anyhow::Result<()> {
	let backup_path = get_backup_path(path, hash);

	warn!(
		"In order to apply your configuration, niji needs to write to {}. This would overwrite a \
		 previous version of that file that is not managed by niji. You can choose to let niji \
		 overwrite the file, or cancel the process. If you overwrite the file, you may choose to \
		 back up the previous version.",
		path.display(),
	);
	if !prompt!(default: false, "Overwrite {}?", path.display()) {
		return Err(anyhow!(
			"Writing to {} was cancelled by the user",
			path.display()
		));
	}

	if prompt!(default: true, "Backup {} to {}", path.display(), backup_path.display()) {
		fs::copy(path, &backup_path)
			.context(format!("Failed to write to {}", backup_path.display()))?;
	}

	init_new_file(path, string)?;

	info!("Backup created at {}", backup_path.display());

	Ok(())
}

fn get_backup_path(path: &Path, hash: u64) -> PathBuf {
	let date = chrono::offset::Local::now().date_naive();
	let file_name = format!(
		"{}.backup-{date}-{hash}",
		path.file_name().unwrap().to_string_lossy()
	);

	path.parent().unwrap().join(file_name)
}

fn set_managed(path: PathBuf) -> anyhow::Result<()> {
	let path = path.canonicalize()?;
	let hash = hash_contents(&path)?;

	debug!("Hash for newly managed file {} is {hash}", path.display());
	xattr::set(path, CHECKSUM_XATTR, &hash.to_ne_bytes())
		.context("Failed to set file attribute")?;
	Ok(())
}

fn get_managed_hash(path: &Path) -> anyhow::Result<Option<u64>> {
	let path = path.canonicalize()?;

	let Some(data) = xattr::get(&path, CHECKSUM_XATTR).context("Failed to read file attribute")?
	else {
		return Ok(None);
	};

	let Ok(bytes) = <[u8; 8]>::try_from(data) else {
		log::warn!(
			"Invalid managed hash attribute on {}! removing.",
			path.display()
		);
		xattr::remove(&path, CHECKSUM_XATTR).context("Failed to remove file attribute")?;

		return Ok(None);
	};

	Ok(Some(u64::from_ne_bytes(bytes)))
}

fn hash_contents(path: &Path) -> anyhow::Result<u64> {
	let file = BufReader::new(File::open(path)?);
	let mut hasher = DefaultHasher::new();
	for byte in file.bytes() {
		byte?.hash(&mut hasher);
	}
	Ok(hasher.finish())
}

#[cfg(test)]
mod tests {
	use tempfile::tempdir;

	use super::*;

	#[test]
	fn write_new() {
		let tempdir = tempdir().unwrap();

		write(&tempdir.path().join("test.txt"), "Test").unwrap();

		assert!(
			xattr::get(tempdir.path().join("test.txt"), "user.niji.checksum")
				.unwrap()
				.is_some()
		);
	}

	#[test]
	fn write_existing_managed() {
		let tempdir = tempdir().unwrap();

		write(&tempdir.path().join("test.txt"), "AAA").unwrap();
		write(&tempdir.path().join("test.txt"), "Test").unwrap();

		assert_eq!(
			fs::read_to_string(tempdir.path().join("test.txt")).unwrap(),
			"Test"
		);
	}

	#[test]
	fn write_existing_unmanaged() {
		let tempdir = tempdir().unwrap();
		fs::write(tempdir.path().join("test.txt"), "AAA").unwrap();

		// This will fail because the prompt can't complete in a test environment
		write(&tempdir.path().join("test.txt"), "Test").unwrap_err();

		assert_eq!(
			fs::read_to_string(tempdir.path().join("test.txt")).unwrap(),
			"AAA"
		);
	}
}
