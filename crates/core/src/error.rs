use thiserror::Error;

/// Starblox errors.
#[derive(Debug, Error)]
pub enum StarbloxError {
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("TLS: {0}")]
    Tls(#[from] rustls::Error),

    #[error("IMAP: {0}")]
    Imap(#[from] async_imap::error::Error),
}

pub type Result<T> = std::result::Result<T, StarbloxError>;
