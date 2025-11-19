use clap::{Parser, Args, Subcommand};

#[derive(Parser)]
#[command(about = "A simple package manager wrapper using lua", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install packages outlined in lua files within your config directory
    #[command(visible_aliases = ["i", "add"])]
    Install,
    /// List all packages known by q-pkg-manager
    #[command(visible_aliases = ["l", "ls"])]
    List(ListArgs)
}

#[derive(Args)]
struct ListArgs {
    /// List only installed packages
    #[arg(short, long, default_value_t = false)]
    installed: bool
}
