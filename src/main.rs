use clap::{Parser, Subcommand};
use colored::control;
use squaredle_cli::{display, fetcher};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(long, global = true)]
    pub json: bool,

    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    Daily,
}

pub fn run(cli: Cli) -> anyhow::Result<()> {
    if cli.no_color || std::env::var_os("NO_COLOR").is_some() {
        control::set_override(false);
    }

    match &cli.command {
        Some(Commands::Daily) | None => {
            let data = fetcher::fetch_today_puzzle()?;

            if cli.json {
                let formatted = display::format_json(&data)?;
                println!("{}", formatted);
            } else {
                display::print_puzzle(&data);
            }
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(cli)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::parse_from(&["test", "daily", "--json"]);
        assert_eq!(cli.command, Some(Commands::Daily));
        assert!(cli.json);
        assert!(!cli.no_color);
    }

    #[test]
    fn test_cli_parsing_no_color() {
        let cli = Cli::parse_from(&["test", "--no-color"]);
        assert_eq!(cli.command, None);
        assert!(!cli.json);
        assert!(cli.no_color);
    }
}
