use home::home_dir;
use rand::seq::IteratorRandom;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter)]
pub enum Directory {
    Root,
    Colorschemes,
    Templates,
    Reloaders,
    Wallpapers,
    Themes,
}

impl Directory {
    fn path(&self) -> PathBuf {
        let mut root = home_dir().expect("Failed to get HOME directory");
        root.push(".config/oxidec");
        root.push(self.get_self_handle());
        root
    }

    fn get_self_handle(&self) -> String {
        match self {
            Self::Root => "",
            Self::Colorschemes => "colorschemes",
            Self::Templates => "templates",
            Self::Reloaders => "reloaders",
            Self::Wallpapers => "wallpapers",
            Self::Themes => "themes",
        }
        .to_owned()
    }

    fn force_extension(&self, path: PathBuf) -> PathBuf {
        match self {
            Self::Colorschemes | Self::Themes => path.with_extension("json"),
            Self::Root | Self::Templates | Self::Reloaders | Self::Wallpapers => path,
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        let extension = self.build_path(name);
        let entry = extension.to_string_lossy().to_string();
        self.path().join(entry).exists()
    }

    fn random_file(&self) -> Option<PathBuf> {
        let mut rng = rand::thread_rng();
        let files = fs::read_dir(self.path()).expect("Failed to read the dir contents");
        let entry = files.choose(&mut rng)?.ok()?;
        Some(entry.path())
    }

    pub fn random_entry(&self) -> String {
        let error_message = format!("There are no {}.", self.get_self_handle());
        let file = self.random_file().expect(&error_message);
        file.to_string_lossy().into()
    }

    pub fn copy(&self, path: &Path) -> io::Result<()> {
        let from_path = fs::canonicalize(path)?;
        let file_name = path.file_name().unwrap_or_default();
        let to_path = self.path().join(file_name);

        if from_path != to_path {
            fs::copy(from_path, to_path)?;
        }

        Ok(())
    }

    pub fn remove(&self, entry: &str) -> io::Result<()> {
        fs::remove_file(self.get(entry)?)
    }

    pub fn list(&self) -> Vec<PathBuf> {
        let entries = fs::read_dir(self.path()).unwrap().flatten();
        let paths = entries.map(|e| e.path());
        let mut files = paths.filter(|p| p.is_file()).collect::<Vec<PathBuf>>();

        files.sort_by_key(|path| PathBuf::from(path.file_name().unwrap()));

        files
    }

    pub fn list_stems(&self) -> Vec<String> {
        let list = self.list();
        list.iter()
            .filter_map(|e| e.file_stem())
            .map(|e| e.to_string_lossy().into())
            .collect()
    }

    pub fn list_names(&self) -> Vec<String> {
        let list = self.list();
        list.iter()
            .filter_map(|e| e.file_name())
            .map(|e| e.to_string_lossy().into())
            .collect()
    }

    pub fn get(&self, entry: &str) -> io::Result<PathBuf> {
        let path = self.build_path(entry);
        if path.exists() {
            return Ok(path);
        }

        for file in self.list() {
            let file_name = file.file_name();
            let file_stem = file.file_stem();

            let file_name = file_name.unwrap_or_default().to_string_lossy();
            let file_stem = file_stem.unwrap_or_default().to_string_lossy();

            if file_name == entry || file_stem == entry {
                return Ok(file);
            }
        }

        Err(io::Error::from(io::ErrorKind::NotFound))
    }

    pub fn build_path(&self, name: &str) -> PathBuf {
        self.force_extension(self.path().join(name))
    }
}

pub fn ensure_config_exists() {
    for directory in Directory::iter() {
        let path = directory.path();
        fs::create_dir_all(path).expect("Failed to create config directories");
    }
}
