use std::fmt;

#[derive(Debug)]
pub enum ConversionError {
    /// File not found or cannot be read
    FileNotFound(String),
    /// Failed to read file contents
    FileRead(String, std::io::Error),
    /// Invalid PFX/P12 file format
    InvalidFormat(String),
    /// Wrong password or password required
    Authentication(String),
    /// Failed to create output directory
    DirectoryCreation(String, std::io::Error),
    /// Failed to write output file
    FileWrite(String, std::io::Error),
    /// OpenSSL error during conversion
    Ssl(openssl::error::ErrorStack),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::FileNotFound(path) => {
                write!(f, "PFX file not found: {}", path)
            }
            ConversionError::FileRead(path, err) => {
                write!(f, "Failed to read PFX file '{}': {}", path, err)
            }
            ConversionError::InvalidFormat(msg) => {
                write!(f, "Invalid PFX file format: {}", msg)
            }
            ConversionError::Authentication(msg) => {
                write!(f, "Authentication failed: {}", msg)
            }
            ConversionError::DirectoryCreation(path, err) => {
                write!(f, "Failed to create output directory '{}': {}", path, err)
            }
            ConversionError::FileWrite(path, err) => {
                write!(f, "Failed to write file '{}': {}", path, err)
            }
            ConversionError::Ssl(err) => {
                write!(f, "SSL/TLS error: {}", err)
            }
        }
    }
}

impl std::error::Error for ConversionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConversionError::FileRead(_, err) => Some(err),
            ConversionError::DirectoryCreation(_, err) => Some(err),
            ConversionError::FileWrite(_, err) => Some(err),
            ConversionError::Ssl(err) => Some(err),
            _ => None,
        }
    }
}

impl From<openssl::error::ErrorStack> for ConversionError {
    fn from(err: openssl::error::ErrorStack) -> Self {
        ConversionError::Ssl(err)
    }
}
