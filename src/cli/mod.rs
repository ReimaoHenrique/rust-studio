pub mod help;
pub mod run;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Run,
    Help,
}
