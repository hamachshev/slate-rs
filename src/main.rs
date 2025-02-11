use anyhow::{Context, Result};
use std::{fs, path::Path};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        name: Option<String>,

        #[command(subcommand)]
        command: Option<NewCommands>,
    },
}

#[derive(Subcommand)]
enum NewCommands {
    Theme { name: String },
    Page { name: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, command } => {
            if let Some(path) = name {
                fs::create_dir_all(format!("{path}/content")).context("creating new dirs")?;
                fs::File::create_new(format!("{path}/content/index.md"))
                    .context("creating index.md")?;
                println!("Created {}! cd into {} to get started", path, path);
            }

            match command {
                Some(NewCommands::Theme { name }) => {
                    if Path::new("themes/").exists() {
                        fs::create_dir(name).context("creating theme dir")?;
                    } else {
                        fs::create_dir_all(format!("themes/{}", name))
                            .context("creating themes and theme dir")?;
                    }
                    fs::File::create_new(format!("themes/{}/index.html", name))
                        .context("creating theme index")?;
                }
                Some(NewCommands::Page { name }) => {
                    anyhow::ensure!(Path::new("content/").exists());

                    fs::File::create_new(format!("content/{name}"))
                        .context("writing new page file")?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
