use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Action {
    #[command(name = "set", about = "[name] set state")]
    Set(Set),
    #[command(name = "save", about = "<name> save state")]
    Save(Save),
    #[command(name = "remove", about = "<name> remove state")]
    Remove(Remove),
    #[command(name = "list", about = "list states")]
    List(List),
}

#[derive(Args)]
pub struct Set {
    pub name: Option<String>,
}

#[derive(Args)]
pub struct Remove {
    pub name: String,
}

#[derive(Args)]
pub struct List {
    #[arg(long)]
    pub json: bool,
}

#[derive(Args)]
pub struct Save {
    pub name: String,
}
