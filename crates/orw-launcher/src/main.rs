mod app;
mod cli;

use clap::Parser;
use cli::Cli;
use eframe::egui;
use tracing::info;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    if cli.command.is_some() || std::env::var("ORW_LAUNCHER_CLI").is_ok() {
        return cli::run(cli);
    }

    info!("starting OpenRailWorks launcher UI");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 720.0])
            .with_title("OpenRailWorks"),
        ..Default::default()
    };
    eframe::run_native(
        "OpenRailWorks",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(app::LauncherApp::new(cc)))
        }),
    )
    .map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(())
}
