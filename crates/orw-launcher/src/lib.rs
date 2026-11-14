pub mod app;
pub mod cli;
pub mod game;
pub mod preview;

use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use eframe::egui;
use tracing::info;

pub fn run_launcher() -> anyhow::Result<()> {
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
            if let Some(rs) = cc.wgpu_render_state.as_ref() {
                let gpu = preview::GameCamGpu::new(&rs.device, rs.target_format);
                rs.renderer.write().callback_resources.insert(gpu);
            }
            Ok(Box::new(app::LauncherApp::new(cc)))
        }),
    )
    .map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(())
}

pub fn run_game(session_file: PathBuf) -> anyhow::Result<()> {
    game::run(session_file)
}