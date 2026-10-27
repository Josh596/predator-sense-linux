use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::profile::LightingProfile;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    /// A fresh directory under the system temp dir, deleted when dropped.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let dir = std::env::temp_dir()
                .join(format!("predatorsense-test-{}-{name}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    // default_path

    fn set_env(key: &str, value: Option<&OsString>) {
        // SAFETY: only `default_path_follows_xdg_rules` touches these variables,
        // and nothing else in this crate's tests reads them.
        unsafe {
            match value {
                Some(v) => std::env::set_var(key, v),
                None => std::env::remove_var(key),
            }
        }
    }

    /// All cases live in one test: tests run in parallel, and the environment
    /// is shared by the whole process.
    #[test]
    fn default_path_follows_xdg_rules() {
        let saved_xdg = std::env::var_os("XDG_STATE_HOME");
        let saved_home = std::env::var_os("HOME");

        set_env("XDG_STATE_HOME", Some(&"/xdg".into()));
        set_env("HOME", Some(&"/home/me".into()));
        assert_eq!(
            default_path().unwrap(),
            PathBuf::from("/xdg/predatorsense/lighting.toml"),
            "absolute XDG_STATE_HOME wins"
        );

        set_env("XDG_STATE_HOME", Some(&"relative/xdg".into()));
        assert_eq!(
            default_path().unwrap(),
            PathBuf::from("/home/me/.local/state/predatorsense/lighting.toml"),
            "relative XDG_STATE_HOME is ignored"
        );

        set_env("XDG_STATE_HOME", None);
        assert_eq!(
            default_path().unwrap(),
            PathBuf::from("/home/me/.local/state/predatorsense/lighting.toml"),
            "falls back to HOME"
        );

        set_env("HOME", None);
        assert!(default_path().is_err(), "neither variable set");

        set_env("XDG_STATE_HOME", saved_xdg.as_ref());
        set_env("HOME", saved_home.as_ref());
    }

    // load

    #[test]
    fn load_reads_valid_file() {
        let dir = TempDir::new("load-valid");
        let path = dir.0.join("lighting.toml");
        fs::write(&path, toml::to_string(&LightingProfile::default()).unwrap()).unwrap();

        assert_eq!(load(&path).unwrap(), LightingProfile::default());
    }

    #[test]
    fn load_missing_file_is_error() {
        let dir = TempDir::new("load-missing");
        assert!(load(&dir.0.join("nope.toml")).is_err());
    }

    #[test]
    fn load_invalid_toml_is_error() {
        let dir = TempDir::new("load-invalid");
        let path = dir.0.join("lighting.toml");
        fs::write(&path, "this is = = not toml").unwrap();

        assert!(load(&path).is_err());
    }

    // save

    #[test]
    fn save_creates_missing_parent_directories() {
        let dir = TempDir::new("save-parents");
        let path = dir.0.join("a/b/lighting.toml");

        save(&LightingProfile::default(), &path).unwrap();

        assert!(path.exists());
    }

    #[test]
    fn save_leaves_no_temp_file_behind() {
        let dir = TempDir::new("save-tmp");
        let path = dir.0.join("lighting.toml");

        save(&LightingProfile::default(), &path).unwrap();

        assert!(!path.with_extension("toml.tmp").exists());
    }

    #[test]
    fn save_overwrites_existing_file() {
        let dir = TempDir::new("save-overwrite");
        let path = dir.0.join("lighting.toml");
        fs::write(&path, "old contents").unwrap();

        save(&LightingProfile::default(), &path).unwrap();

        assert_eq!(load(&path).unwrap(), LightingProfile::default());
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = TempDir::new("round-trip");
        let path = dir.0.join("lighting.toml");
        let profile: LightingProfile =
            toml::from_str("[keyboard]\nbrightness = 40\n[logo]\nspeed = 3\n").unwrap();

        save(&profile, &path).unwrap();

        assert_eq!(load(&path).unwrap(), profile);
    }
}
