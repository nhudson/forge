use crate::error::ConversionError;
use crate::openssl::ParsedPfx;
use openssl::pkcs12::Pkcs12;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

/// Parser for PFX/P12 files
pub struct PfxParser;

impl PfxParser {
    /// Parse a PFX file from a file path
    pub fn parse_file<P: AsRef<Path>>(
        path: P,
        password: &str,
    ) -> Result<ParsedPfx, ConversionError> {
        let path = path.as_ref();

        // Check if file exists
        if !path.exists() {
            return Err(ConversionError::FileNotFound(path.display().to_string()));
        }

        // Verify that the file is readable
        if let Err(e) = fs::metadata(path) {
            return Err(ConversionError::FileRead(
                path.display().to_string(),
                std::io::Error::new(std::io::ErrorKind::PermissionDenied, e),
            ));
        }

        // Validate file extension (if it has one)
        if let Some(ext) = path.extension().and_then(OsStr::to_str) {
            let ext = ext.to_lowercase();
            if ext != "pfx" && ext != "p12" {
                return Err(ConversionError::InvalidFileExtension(ext));
            }
        }

        // Read the file
        let pfx_data =
            fs::read(path).map_err(|e| ConversionError::FileRead(path.display().to_string(), e))?;

        // Validate file size
        if pfx_data.is_empty() {
            return Err(ConversionError::InvalidFormat("File is empty".to_string()));
        }

        Self::parse_bytes(&pfx_data, password)
    }

    /// Parse PFX data from bytes
    pub fn parse_bytes(data: &[u8], password: &str) -> Result<ParsedPfx, ConversionError> {
        // Validate input data
        if data.is_empty() {
            return Err(ConversionError::InvalidFormat(
                "Empty PFX data provided".to_string(),
            ));
        }

        // Parse the PKCS12 structure
        let pkcs12 = Pkcs12::from_der(data).map_err(|e| {
            ConversionError::InvalidFormat(format!("Failed to parse PFX structure: {e}"))
        })?;

        // Extract the contents with the provided password
        let parsed = pkcs12.parse2(password).map_err(|e| {
            if password.is_empty() {
                ConversionError::Authentication(format!(
                    "Failed to parse PFX file: {e}. This file may require a password. Use --password option."))
            } else {
                ConversionError::Authentication(format!(
                    "Failed to parse PFX file with provided password: {e}"))
            }
        })?;

        Ok(ParsedPfx::from(parsed))
    }
}
