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
}

pub type Result<T> = std::result::Result<T, Error>;
