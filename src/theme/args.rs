use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    #[structopt(name = "set", about = "[name] set theme")]
    Set(Set),
    #[structopt(name = "save", about = "<name> save theme")]
    Save(Save),
    #[structopt(name = "remove", about = "<name> remove theme")]
    Remove(Remove),
    #[structopt(name = "list", about = "list themes")]
    List(List),
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
pub struct List {
    #[structopt(long)]
    pub json: bool,
}

#[derive(StructOpt)]
pub struct Save {
    pub name: String,
}
