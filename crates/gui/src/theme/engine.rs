use super::Theme;
use dashmap::DashSet;
use directories::ProjectDirs;
use parking_lot::RwLock;
use std::{io, sync::Arc};
use tokio::{
	fs::{create_dir_all, read_dir, File},
	io::{AsyncReadExt, BufReader},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("could not create project directory")]
	NoProjectDir,
	#[error("an io error occurred: {0}")]
	IOError(#[from] io::Error),
	#[error("could not parse theme file: {0}")]
	SerdeError(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct ThemeEngine {
	themes: Arc<RwLock<DashSet<Theme>>>,
}

impl ThemeEngine {
	pub fn new() -> Self {
		Self {
			themes: Arc::new(RwLock::new(DashSet::new())),
		}
	}

	pub fn get_themes(&self) -> Arc<RwLock<DashSet<Theme>>> { Arc::clone(&self.themes) }

	pub async fn load_themes(&mut self) -> Result<()> {
		let project_dir =
			ProjectDirs::from("com", "STBoyden", "codectrl").ok_or(Error::NoProjectDir)?;

		let theme_directory = format!("{}/themes", project_dir.data_dir().display());
		create_dir_all(&theme_directory).await?;

		while let Some(dir_entry) = read_dir(&theme_directory).await?.next_entry().await? {
			if dir_entry.path().extension().is_none() {
				continue;
			}

			let theme_file = File::open(dir_entry.path()).await?;
			let mut theme_file_reader = BufReader::new(theme_file);

			let mut data = String::new();
			theme_file_reader.read_to_string(&mut data).await?;

			let theme: Theme = serde_json::from_str(&data)?;

			self.themes.write().insert(theme);
		}

		Ok(())
	}
}
