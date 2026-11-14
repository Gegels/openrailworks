use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "orw-game", about = "OpenRailWorks session window (started from the launcher)")]
struct GameCli {
    #[arg(long)]
    session: PathBuf,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();
    let cli = GameCli::parse();
    orw_launcher::run_game(cli.session)
}