use clap::Parser;
mod cli;

#[derive(Parser)]
#[command(name = "cargo-rust-studio")]
#[command(about = "Rust database client with web interface", version = "0.1.0")]
struct Cli {
    #[arg(skip)]
    _cargo_subcommand: Option<String>,
    
    #[command(subcommand)]
    command: Option<cli::Commands>,
}

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "rust-studio" {
        args.remove(1);
    }
    
    let cli = Cli::parse_from(args);

    match cli.command {
        Some(cli::Commands::Run) => cli::run::run_server().await,
        Some(cli::Commands::Help) => cli::help::print_help(),
        None => cli::help::print_help(),
    }
}
