mod command;
mod config;
mod fuzzel;
mod rbw;
mod utils;

use clap::Parser;
use clap::Subcommand;

use crate::rbw::app;
use crate::utils::health;

#[derive(Subcommand, Debug)]
enum Commands {
    Show,
    Health,
}

#[derive(Parser, Debug)]
#[command(
    name = "frbw",
    version,
    about = "Bitwarden in fuzzel",
    long_about = "A tool that allows you to use rbw with fuzzel"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Show => {
            app::init()?;
        }
        Commands::Health => {
            health::check_deps()?;
        }
    }

    Ok(())
}
