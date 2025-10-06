use std::{
	fs::{read_dir, ReadDir},
	io,
	path::{Path, PathBuf},
};

pub struct SubPathIter(ReadDir);

impl SubPathIter {
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
		Ok(Self(read_dir(path)?))
	}
}

impl Iterator for SubPathIter {
	type Item = Result<PathBuf, io::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		Some(self.0.next()?.map(|entry| entry.path()))
	}
}

pub struct FindSubPathsIter<I> {
	search_path_iter: I,
	sub_path_iter: Option<SubPathIter>,
}

impl<P, I> FindSubPathsIter<I>
where
	P: AsRef<Path>,
	I: Iterator<Item = P>,
{
	pub fn new(search_path_iter: I) -> Self {
		Self {
			search_path_iter,
			sub_path_iter: None,
		}
	}
}

impl<P, I> Iterator for FindSubPathsIter<I>
where
	P: AsRef<Path>,
	I: Iterator<Item = P>,
{
	type Item = PathBuf;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if self.sub_path_iter.is_none() {
				self.sub_path_iter = Some(
					self.search_path_iter
						.find_map(|search_path| SubPathIter::new(search_path).ok())?,
				);
			}

			let path = self
				.sub_path_iter
				.as_mut()
				.unwrap()
				.find_map(|path| path.ok());

			if path.is_some() {
				return path;
			} else {
				self.sub_path_iter = None;
			}
		}
	}
}

pub struct FileIter<I>(I);

impl<P, I> FileIter<I>
where
	P: AsRef<Path>,
	I: Iterator<Item = P>,
{
	#[inline]
	pub fn new(inner: I) -> Self {
		Self(inner)
	}
}

impl<P, I> Iterator for FileIter<I>
where
	P: AsRef<Path>,
	I: Iterator<Item = P>,
{
	type Item = P;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.by_ref().find(|path| path.as_ref().is_file())
	}
}

pub struct DirIter<I>(I);

impl<P, I> DirIter<I>
where
	P: AsRef<Path>,
	I: Iterator<Item = P>,
{
	#[inline]
	pub fn new(inner: I) -> Self {
		Self(inner)
	}
}

impl<P, I> Iterator for DirIter<I>
where
	P: AsRef<Path>,
	I: Iterator<Item = P>,
{
	type Item = P;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.by_ref().find(|path| path.as_ref().is_dir())
	}
}

pub type FindFilesIter<I> = FileIter<FindSubPathsIter<I>>;

pub fn find_files<P, S>(search_paths: S) -> FindFilesIter<S::IntoIter>
where
	P: AsRef<Path>,
	S: IntoIterator<Item = P>,
{
	FileIter::new(FindSubPathsIter::new(search_paths.into_iter()))
}

pub type FindDirsIter<I> = DirIter<FindSubPathsIter<I>>;

pub fn find_dirs<P, S>(search_paths: S) -> FindDirsIter<S::IntoIter>
where
	P: AsRef<Path>,
	S: IntoIterator<Item = P>,
{
	DirIter::new(FindSubPathsIter::new(search_paths.into_iter()))
}
