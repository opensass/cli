use anyhow::Result;
use clap::Parser;
use opensass::cli::{Cli, Command};
use opensass::cmds::add::run_add;
use opensass::utils::log::setup_logging;
use tokio::task;

/// The main entry point of `opensass`.
#[tokio::main]
async fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    setup_logging()?;
    match args.cmd {
        Some(Command::Add(cmd)) => {
            task::spawn_blocking(move || run_add(&cmd.name, &cmd.features)).await??;
        }
        None => {}
    }
    Ok(())
}
