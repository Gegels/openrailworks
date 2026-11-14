use std::path::PathBuf;
use std::process::Command;

use orw_core::{load_user_config, UserConfig};
use serde::Serialize;

use crate::session::LaunchSession;
use crate::EngineError;

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct LaunchBundle {
    pub session: LaunchSession,
    pub config: UserConfig,
}

fn sibling_binary(stem: &str) -> PathBuf {
    let name = if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem.to_string()
    };
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join(&name)))
        .unwrap_or_else(|| PathBuf::from(name))
}

/// Write session + launcher settings and start `orw-game` (openOMSI-style separate game window).
pub fn launch_session(session: &LaunchSession) -> Result<(), EngineError> {
    let config = load_user_config();
    let bundle = LaunchBundle {
        session: session.clone(),
        config,
    };
    let path = std::env::temp_dir().join(format!(
        "openrailworks-session-{}.json",
        std::process::id()
    ));
    let json = serde_json::to_string(&bundle).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;
    std::fs::write(&path, json)?;

    let game = sibling_binary("orw-game");
    if !game.is_file() {
        return Err(EngineError::GameBinaryMissing(game));
    }

    Command::new(&game)
        .arg("--session")
        .arg(&path)
        .spawn()
        .map(|_| ())
        .map_err(EngineError::Io)
}