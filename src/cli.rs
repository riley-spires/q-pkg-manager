use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(about = "A simple package manager wrapper using lua", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install packages outlined in lua files within your config/packages directory
    #[command(visible_aliases = ["i", "add"])]
    Install,
    /// List all packages known by nexus
    #[command(visible_aliases = ["l", "ls"])]
    List(ListArgs),

    /// Uninstall packages that are no longer outlined within your config/packages directory
    #[command(visible_aliases = ["rm", "remove", "uninstall", "p", "r"])]
    Purge,

    /// Update all installed packages known by nexus
    #[command(visible_aliases = ["upgrade", "u", "refresh"])]
    Update,
}

#[derive(Args)]
pub struct ListArgs {
    /// List only installed packages
    #[arg(short, long, default_value_t = false)]
    pub installed: bool,
}
