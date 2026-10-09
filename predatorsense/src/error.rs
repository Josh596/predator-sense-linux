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
}

pub type Result<T> = std::result::Result<T, Error>;
