use anyhow::{anyhow, Context};
use log::{debug, info, warn};
use niji_console::prompt;
use std::{
	collections::{hash_map::DefaultHasher, HashMap},
	fs::{self, File},
	hash::{Hash, Hasher},
	io::{BufReader, Read},
	path::{Path, PathBuf},
	rc::Rc,
};

use crate::files::Files;

pub struct FileManager {
	files: Rc<Files>,
}

impl FileManager {
	pub fn new(files: Rc<Files>) -> anyhow::Result<Self> {
		if !files.managed_files_file().exists() {
			fs::write(files.managed_files_file(), "").context(format!(
				"Failed to write to {}",
				files.managed_files_file().display(),
			))?;
		}

		Ok(Self { files })
	}

	pub fn write_managed(&self, path: &Path, string: &str) -> anyhow::Result<()> {
		let mut managed_files = self.managed_files()?;

		if !path.exists() {
			debug!("Creating new managed file at {}", path.display());
			self.init_new_file(&mut managed_files, path, string)
		} else {
			self.manage_existing_file(&mut managed_files, path, string)
		}
	}

	fn init_new_file(
		&self,
		managed_files: &mut HashMap<PathBuf, u64>,
		path: &Path,
		string: &str,
	) -> anyhow::Result<()> {
		fs::write(path, string).context(format!("Failed to write to {}", path.display()))?;
		self.set_managed(managed_files, path.to_path_buf())?;

		info!("niji now manages {}", path.display());

		Ok(())
	}

	fn manage_existing_file(
		&self,
		managed_files: &mut HashMap<PathBuf, u64>,
		path: &Path,
		string: &str,
	) -> anyhow::Result<()> {
		let current_hash = Self::hash_contents(path)?;
		debug!("{} has current hash {current_hash}", path.display());

		if let Some(known_hash) = self.get_known_hash(managed_files, path)? {
			if current_hash == known_hash {
				debug!("Writing to managed file at {}", path.display());
				fs::write(path, string)
					.context(format!("Failed to write to {}", path.display()))?;
				self.set_managed(managed_files, path.to_path_buf())?;
				return Ok(());
			} else {
				debug!("File contents of {} have changed", path.display())
			}
		} else {
			debug!("{} is not in the managed files table", path.display())
		}

		self.backup_and_replace(managed_files, path, string, current_hash)
	}

	fn backup_and_replace(
		&self,
		managed_files: &mut HashMap<PathBuf, u64>,
		path: &Path,
		string: &str,
		hash: u64,
	) -> anyhow::Result<()> {
		let backup_path = Self::get_backup_path(path, hash);

		warn!(
			"In order to apply your configuration, niji needs to write to {}. This would \
			 overwrite a previous version of that file that is not managed by niji. You can \
			 choose to let niji overwrite the file, or cancel the process. If you overwrite the \
			 file, the previous version will be backed up to {}.",
			path.display(),
			backup_path.display()
		);
		if !prompt!(default: false, "Backup and overwrite {}?", path.display()) {
			return Err(anyhow!(
				"Writing to {} was cancelled by the user",
				path.display()
			));
		}

		fs::copy(path, &backup_path)
			.context(format!("Failed to write to {}", backup_path.display()))?;

		self.init_new_file(managed_files, path, string)?;

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

	fn set_managed(
		&self,
		managed_files: &mut HashMap<PathBuf, u64>,
		path: PathBuf,
	) -> anyhow::Result<()> {
		let path = path.canonicalize()?;
		let hash = Self::hash_contents(&path)?;

		debug!("Hash for newly managed file {} is {hash}", path.display());
		managed_files.insert(path.clone(), hash);
		self.write_managed_files(managed_files)
	}

	fn get_known_hash(
		&self,
		managed_files: &HashMap<PathBuf, u64>,
		path: &Path,
	) -> anyhow::Result<Option<u64>> {
		let path = path.canonicalize()?;

		Ok(managed_files.get(&path).copied())
	}

	fn hash_contents(path: &Path) -> anyhow::Result<u64> {
		let file = BufReader::new(File::open(path)?);
		let mut hasher = DefaultHasher::new();
		for byte in file.bytes() {
			byte?.hash(&mut hasher);
		}
		Ok(hasher.finish())
	}

	fn managed_files(&self) -> anyhow::Result<HashMap<PathBuf, u64>> {
		let mut managed_files = HashMap::new();

		let mut reader = csv::ReaderBuilder::new()
			.has_headers(false)
			.from_path(self.files.managed_files_file())
			.context("Error while reading from managed_files.csv")?;

		for result in reader.deserialize::<(PathBuf, u64)>() {
			let (path, hash) = result?;
			managed_files.insert(path, hash);
		}
		Ok(managed_files)
	}

	fn write_managed_files(&self, managed_files: &HashMap<PathBuf, u64>) -> anyhow::Result<()> {
		let mut writer = csv::Writer::from_path(self.files.managed_files_file())
			.context("Error while writing to managed_files.csv")?;
		for (path, hash) in managed_files.iter() {
			writer.serialize((path, hash))?;
		}

		Ok(())
	}
}
