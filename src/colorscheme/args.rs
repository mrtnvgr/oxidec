use super::commands::generate::backends::Backend;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    #[structopt(name = "set", about = "[name] set colorscheme")]
    Set(Set),
    #[structopt(name = "remove", about = "<name> remove colorscheme")]
    Remove(Remove),
    #[structopt(name = "import", about = "<file-path> import colorscheme")]
    Import(Import),
    #[structopt(name = "list", about = "list colorschemes")]
    List(List),
    #[structopt(name = "status", about = "colorscheme status")]
    Status(Status),
    #[structopt(name = "generate", about = "generate colorscheme from wallpaper")]
    Generate(Generate),
    #[structopt(name = "reload", about = "reload colorscheme")]
    Reload,
}

#[derive(StructOpt)]
pub struct Set {
    pub name: Option<String>,
}

#[derive(StructOpt)]
pub struct Remove {
    pub name: String,
}

#[derive(StructOpt)]
pub struct Import {
    #[structopt(parse(from_os_str))]
    pub file_path: PathBuf,
}

#[derive(StructOpt)]
pub struct List {
    #[structopt(long)]
    pub json: bool,
}

#[derive(StructOpt)]
pub struct Status {
    #[structopt(long)]
    pub json: bool,
}

#[derive(StructOpt)]
pub struct Generate {
    #[structopt(long = "backend", default_value = "imagemagick", case_insensitive = true, possible_values = &Backend::variants())]
    pub backend: Backend,
    #[structopt(long)]
    pub light: bool,
}
