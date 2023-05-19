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
        }
    }

    fn force_extension(&self, path: PathBuf) -> PathBuf {
        match self {
            Self::Colorschemes => path.with_extension("json"),
            Self::Wallpapers => todo!(),
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

    pub fn list(&self) -> io::Result<Vec<PathBuf>> {
        fs::read_dir(self.path())?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()
    }

    pub fn get(&self, entry: &str) -> io::Result<PathBuf> {
        let path = self.force_extension(self.path().join(entry));
        if path.exists() {
            return Ok(path);
        }

        for file in self.list()? {
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
}

pub fn ensure_config_exists() {
    for folder in Folder::iter() {
        let path = folder.path();
        fs::create_dir_all(path).expect("Failed to create config directories");
    }
}
