use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};
use orw_core::{
    catalog_matches_benchmarks, scan_routes, InstallError, RailWorksInstall, RouteScanOptions,
};
use tracing::info;

#[derive(Parser)]
#[command(
    name = "orw-catalog",
    version,
    about = "OpenRailWorks — read-only catalog of your Train Simulator Classic installation"
)]
struct Cli {
    /// Path to RailWorks / TSC installation root (overrides RAILWORKS_PATH and config file).
    #[arg(long, env = "RAILWORKS_PATH", global = true)]
    railworks_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify installation layout and print summary.
    Doctor,
    /// List routes under Content/Routes (or Routes).
    Routes {
        #[arg(long)]
        json: bool,
        #[arg(long, help = "Include folders without Tracks.bin or scenarios")]
        include_incomplete: bool,
    },
    /// Highlight benchmark routes (Ringbahn Berlin, Südast, …).
    Benchmarks,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    let path = cli.railworks_path.as_deref();

    match cli.command {
        Commands::Doctor => cmd_doctor(path)?,
        Commands::Routes { json, include_incomplete } => {
            cmd_routes(path, json, include_incomplete)?
        }
        Commands::Benchmarks => cmd_benchmarks(path)?,
    }

    Ok(())
}

fn open_install(path: Option<&std::path::Path>) -> anyhow::Result<RailWorksInstall> {
    RailWorksInstall::discover(path).map_err(|e| match e {
        InstallError::PathNotConfigured => anyhow::anyhow!(
            "{e}\nSet --railworks-path, RAILWORKS_PATH, or railworks_path in ~/.config/openrailworks/config.toml"
        ),
        other => anyhow::anyhow!("{other}"),
    })
}

fn cmd_doctor(path: Option<&std::path::Path>) -> anyhow::Result<()> {
    let install = open_install(path)?;
    info!(root = %install.root.display(), "installation found");
    println!("OpenRailWorks doctor");
    println!("  root:         {}", install.root.display());
    println!("  layout:       {:?}", install.layout);
    println!("  routes dir:   {}", install.routes_dir.display());
    if let Some(packages) = &install.packages_dir {
        println!("  packages dir: {}", packages.display());
    } else {
        println!("  packages dir: (not found)");
    }
    println!("  status:       OK");
    Ok(())
}

fn cmd_routes(
    path: Option<&std::path::Path>,
    json: bool,
    include_incomplete: bool,
) -> anyhow::Result<()> {
    let install = open_install(path)?;
    let catalog = scan_routes(
        &install,
        &RouteScanOptions {
            include_incomplete,
        },
    )
    .context("scan routes")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&catalog)?);
        return Ok(());
    }

    println!(
        "Routes under {} ({} found, {} .ap under Packages)\n",
        catalog.install_root.display(),
        catalog.routes.len(),
        catalog.package_count
    );

    for route in &catalog.routes {
        let flags = format!(
            "{}{} scenarios={}",
            if route.has_tracks_bin { "tracks " } else { "" },
            if route.has_route_properties { "props " } else { "" },
            route.scenario_count
        );
        if route.display_name != route.id {
            println!("  {:<48} {}", route.display_name, flags.trim());
            println!("    {:<46}", route.id);
        } else {
            println!("  {:<40} {}", route.id, flags.trim());
        }
    }

    Ok(())
}

fn cmd_benchmarks(path: Option<&std::path::Path>) -> anyhow::Result<()> {
    let install = open_install(path)?;
    let catalog = scan_routes(&install, &RouteScanOptions::default())?;
    let hits = catalog_matches_benchmarks(&catalog);

    if hits.is_empty() {
        println!(
            "No benchmark routes matched (Ringbahn / Berlin / Südast). Install DLC and re-run, or check folder names."
        );
        println!("Total routes scanned: {}", catalog.routes.len());
        return Ok(());
    }

    println!("Benchmark routes in your installation:");
    for id in hits {
        if let Some(route) = catalog.routes.iter().find(|r| r.id == id) {
            println!(
                "  {}  (tracks={}, scenarios={})",
                route.id, route.has_tracks_bin, route.scenario_count
            );
        }
    }
    Ok(())
}
