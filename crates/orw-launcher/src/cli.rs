use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};
use orw_core::{
    catalog_matches_benchmarks, scan_routes, InstallError, RailWorksInstall,
    RouteScanOptions,
};
use orw_vfs::RailWorksVfs;

#[derive(Parser)]
#[command(
    name = "orw-launcher",
    version,
    about = "OpenRailWorks launcher (CLI mirrors the GUI)"
)]
pub struct Cli {
    #[arg(long, env = "RAILWORKS_PATH", global = true)]
    pub railworks_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Doctor,
    Routes {
        #[arg(long)]
        json: bool,
    },
    #[command(name = "ap-count")]
    ApCount,
}

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Some(Commands::Doctor) => cmd_doctor(cli.railworks_path.as_deref())?,
        Some(Commands::Routes { json }) => cmd_routes(cli.railworks_path.as_deref(), json)?,
        Some(Commands::ApCount) => cmd_ap_count(cli.railworks_path.as_deref())?,
        None => {
            anyhow::bail!("pass a subcommand in CLI mode (doctor, routes, ap-count)")
        }
    }
    Ok(())
}

fn open_install(path: Option<&std::path::Path>) -> anyhow::Result<RailWorksInstall> {
    RailWorksInstall::discover(path).map_err(|e| match e {
        InstallError::PathNotConfigured => anyhow::anyhow!(
            "{e}\nConfigure railworks_path in Settings or set RAILWORKS_PATH"
        ),
        other => anyhow::anyhow!("{other}"),
    })
}

fn cmd_doctor(path: Option<&std::path::Path>) -> anyhow::Result<()> {
    let install = open_install(path)?;
    let vfs = RailWorksVfs::open(&install)?;
    let aps = vfs.list_ap_archives()?;
    println!("OpenRailWorks launcher (CLI)");
    println!("  root:     {}", install.root.display());
    println!("  layout:   {:?}", install.layout);
    println!("  routes:   {}", install.routes_dir.display());
    println!("  .ap files: {}", aps.len());
    println!("  status:   OK");
    Ok(())
}

fn cmd_routes(path: Option<&std::path::Path>, json: bool) -> anyhow::Result<()> {
    let install = open_install(path)?;
    let catalog = scan_routes(&install, &RouteScanOptions::default()).context("scan routes")?;
    if json {
        println!("{}", serde_json::to_string_pretty(&catalog)?);
        return Ok(());
    }
    let hits = catalog_matches_benchmarks(&catalog);
    println!(
        "{} routes ({} benchmark hits)",
        catalog.routes.len(),
        hits.len()
    );
    for route in &catalog.routes {
        println!("  {}", route.id);
    }
    Ok(())
}

fn cmd_ap_count(path: Option<&std::path::Path>) -> anyhow::Result<()> {
    let install = open_install(path)?;
    let vfs = RailWorksVfs::open(&install)?;
  println!("{}", vfs.list_ap_archives()?.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_requires_subcommand() {
        let cli = Cli {
            railworks_path: None,
            command: None,
        };
        assert!(run(cli).is_err());
    }
}
