use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("device {vid:04x}:{pid:04x} not found")]
    NotFound { vid: u16, pid: u16 },

    #[error("permission denied — add a udev rule for hidraw or run as root")]
    PermissionDenied,

    #[error("invalid charge limit: {0}")]
    InvalidLimit(&'static str),

    #[error(transparent)]
    Hid(#[from] hidapi::HidError),

    #[error("{role} device unavailable: {reason}")]
    Unavailable { role: &'static str, reason: String },

    #[error("unknown performance mode byte: {0:#04x}")]
    UnknownPerfMode(u8),

    #[error("neither XDG_STATE_HOME nor HOME is set")]
    NoStateDir,

    #[error("could not read {path}: {source}")]
    ProfileRead {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("could not write {path}: {source}")]
    ProfileWrite {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("invalid profile {path}: {source}")]
    ProfileParse {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error(transparent)]
    ProfileSerialize(#[from] toml::ser::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found_shows_ids_as_four_digit_hex() {
        let err = Error::NotFound {
            vid: 0x0cf2,
            pid: 0x5130,
        };
        assert_eq!(err.to_string(), "device 0cf2:5130 not found");
    }

    #[test]
    fn unknown_perf_mode_shows_byte_as_hex() {
        assert_eq!(
            Error::UnknownPerfMode(0x07).to_string(),
            "unknown performance mode byte: 0x07"
        );
    }
}
