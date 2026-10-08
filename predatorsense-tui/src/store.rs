use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::tui::state::lighting::profile::LightingProfile;

const PROFILE_FILE: &str = "lighting.toml";

/// `$XDG_STATE_HOME/predatorsense/lighting.toml`, falling back to
/// `~/.local/state/predatorsense/lighting.toml`.
///
/// A relative `XDG_STATE_HOME` is ignored per the XDG spec — otherwise the
/// profile would follow the shell's working directory around.
pub fn default_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))
        .ok_or("neither XDG_STATE_HOME nor HOME is set")?;

    Ok(base.join("predatorsense").join(PROFILE_FILE))
}

pub fn load(path: &Path) -> Result<LightingProfile, Box<dyn std::error::Error>> {
    // Load the file first
    let mut f = File::open(path)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;

    // then use toml to deserialize
    let profile: LightingProfile = toml::from_str(&content)?;

    Ok(profile)
}

pub fn save(profile: &LightingProfile, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = toml::to_string(profile)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write to a sibling temp file, flush it to disk, then rename over the
    // target. rename(2) is atomic, so a crash leaves either the previous
    // profile or the new one — never a truncated file.
    let tmp = path.with_extension("toml.tmp");

    let mut file = File::create(&tmp)?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?;
    drop(file);

    fs::rename(&tmp, path)?;

    Ok(())
}
