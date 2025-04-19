//! This module contains the CLI functionalities for interacting with the OpenSASS ecosystem.
//!
//! The CLI allows you to add OpenSASS components to your WASM projects by downloading them
//! from crates.io, importing their dependencies, features, and related files.

use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Args, Parser, Subcommand};

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .usage(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser, Debug, Clone)]
#[command(
    author = "Mahmoud Harmouch",
    version,
    name = "os",
    propagate_version = true,
    styles = styles(),
    help_template = r#"{before-help}{name} {version}
{about}
{usage-heading} {usage}

{all-args}{after-help}

AUTHORS:
    {author}
"#,
    about = r#"
 ██████  ██████  ███████ ███    ██ ███████  █████  ███████ ███████
██    ██ ██   ██ ██      ████   ██ ██      ██   ██ ██      ██     
██    ██ ██████  █████   ██ ██  ██ ███████ ███████ ███████ ███████
██    ██ ██      ██      ██  ██ ██      ██ ██   ██      ██      ██
 ██████  ██      ███████ ██   ████ ███████ ██   ██ ███████ ███████

`os` is a command-line interface for downloading and importing OpenSASS
components into your WASM projects.

FUNCTIONALITIES:
  - Download OpenSASS components from crates.io.
  - Automatically detect and import required dependencies and feature flags.
  - Copy relevant source files into your local project for easy integration.

EXAMPLES:
  Add the `i18nrs` component with `yew` feature:
    os add i18nrs yew

For more information, visit: https://github.com/opensass/cli
"#
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Add an OpenSASS component to your project
    Add(AddArgs),
}

#[derive(Args, Debug, Clone)]
pub struct AddArgs {
    /// The name of the OpenSASS component to add
    pub name: String,

    /// List of features to include
    pub features: String,
}
