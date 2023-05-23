use home::home_dir;
use rand::seq::IteratorRandom;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter)]
pub enum Folder {
    Root,
    Colorschemes,
    Templates,
    Reloaders,
    Wallpapers,
    Themes,
}

impl Folder {
    fn path(&self) -> PathBuf {
        let mut root = home_dir().expect("Failed to get HOME directory");
        root.push(".config/oxidec");

        match self {
            Self::Root => root,
            Self::Colorschemes => root.join("colorschemes"),
            Self::Templates => root.join("templates"),
            Self::Reloaders => root.join("reloaders"),
            Self::Wallpapers => root.join("wallpapers"),
            Self::Themes => root.join("themes"),
        }
    }

    fn force_extension(&self, path: PathBuf) -> PathBuf {
        match self {
            Self::Colorschemes | Self::Themes => path.with_extension("json"),
            _ => path,
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        let extension = self.force_extension(Path::new(&name).to_path_buf());
        let entry = extension.to_string_lossy().to_string();
        self.path().join(entry).exists()
    }

    pub fn random_file(&self) -> Option<PathBuf> {
        let mut rng = rand::thread_rng();
        let files = fs::read_dir(self.path()).expect("Failed to read the dir contents");
        let entry = files.choose(&mut rng)?.ok()?;
        Some(entry.path())
    }

    pub fn copy(&self, path: &PathBuf) -> io::Result<()> {
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
            .map(|e| e.file_stem().unwrap())
            .map(|e| e.to_str().unwrap().to_owned())
            .collect()
    }

    pub fn list_names(&self) -> Vec<String> {
        let list = self.list();
        list.iter()
            .map(|e| e.file_name().unwrap())
            .map(|e| e.to_str().unwrap().to_owned())
            .collect()
    }

    pub fn get(&self, entry: &str) -> io::Result<PathBuf> {
        // TODO: refactor?
        let path = self.force_extension(self.path().join(entry));
        if path.exists() {
            return Ok(path);
        }

        for file in self.list() {
            let file_name = file.file_name();
            let file_stem = file.file_stem();

            let file_name = file_name.unwrap_or_default().to_str().unwrap();
            let file_stem = file_stem.unwrap_or_default().to_str().unwrap();

            if file_name == entry || file_stem == entry {
                return Ok(file);
            }
        }

        Err(io::Error::from(io::ErrorKind::NotFound))
    }

    pub fn build_path(&self, name: &str) -> PathBuf {
        // TODO: refactor?
        self.force_extension(self.path().join(name))
    }
}

pub fn ensure_config_exists() {
    for folder in Folder::iter() {
        let path = folder.path();
        fs::create_dir_all(path).expect("Failed to create config directories");
    }
}
