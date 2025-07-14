use crate::error::ConversionError;
use clap::Parser;
use crate::secure::SecurePassword;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    name = "forge",
    author = "Nick Hudson <nick.hudson@gmail.com>",
    version = "0.1.1",
    about = "Convert PFX/P12 certificate files to PEM format",
    long_about = "A Rust-based tool for converting PFX (PKCS#12) certificate files to PEM format. \
                  Supports password-protected files, certificate chains, and various output options."
)]
pub struct Args {
    /// Path to the PFX/P12 file
    #[arg(long, required = true, help = "Path to the PFX/P12 certificate file")]
    pub pfx: String,

    /// Password for the PFX/P12 file
    #[arg(long, help = "Password for the PFX file (if password-protected)")]
    pub password: Option<String>,

    /// Output directory for PEM files (defaults to current directory)
    #[arg(long, help = "Output directory for generated PEM files")]
    out: Option<String>,

    /// Generate a combined PEM file with both certificate and private key
    #[arg(
        long,
        help = "Create a combined PEM file containing both private key and certificate"
    )]
    pub combined: bool,

    /// Custom filename for the private key (defaults to private_key.pem)
    #[arg(long, help = "Custom filename for the private key output")]
    pub key_file: Option<String>,

    /// Custom filename for the certificate (defaults to certificate.pem)
    #[arg(long, help = "Custom filename for the certificate output")]
    pub cert_file: Option<String>,

    /// Custom filename for the combined file (defaults to certificate_with_key.pem)
    #[arg(long, help = "Custom filename for the combined PEM file")]
    pub combined_file: Option<String>,

    /// Extract all certificates in the chain (not just the main certificate)
    #[arg(long, help = "Extract and save the complete certificate chain")]
    pub chain: bool,

    /// Verbose output
    #[arg(long, help = "Enable verbose output with detailed information")]
    pub verbose: bool,
}

impl Args {
    /// Get the password, wrapped in a secure container that zeroizes memory when dropped
    pub fn password(&self) -> SecurePassword {
        SecurePassword::from_option(&self.password)
    }

    /// Get the output directory, defaulting to current directory
    pub fn output_dir(&self) -> &str {
        self.out.as_deref().unwrap_or(".")
    }

    /// Get the private key filename, with default
    pub fn key_filename(&self) -> &str {
        self.key_file.as_deref().unwrap_or("private_key.pem")
    }

    /// Get the certificate filename, with default
    pub fn cert_filename(&self) -> &str {
        self.cert_file.as_deref().unwrap_or("certificate.pem")
    }

    /// Get the combined file filename, with default
    pub fn combined_filename(&self) -> &str {
        self.combined_file
            .as_deref()
            .unwrap_or("certificate_with_key.pem")
    }

    /// Validate all input arguments before starting conversion
    pub fn validate(&self) -> Result<(), ConversionError> {
        // Validate PFX file path
        let pfx_path = Path::new(&self.pfx);

        // Check if the path exists
        if !pfx_path.exists() {
            return Err(ConversionError::FileNotFound(self.pfx.clone()));
        }

        // Check if it's a file
        if !pfx_path.is_file() {
            return Err(ConversionError::InvalidFormat(format!(
                "'{}' is not a file",
                self.pfx
            )));
        }

        // Validate file extension (if available)
        if let Some(ext) = pfx_path.extension().and_then(|e| e.to_str()) {
            let ext = ext.to_lowercase();
            if ext != "pfx" && ext != "p12" {
                return Err(ConversionError::InvalidFileExtension(ext));
            }
        }

        // Validate output directory
        let out_dir = Path::new(self.output_dir());
        if out_dir.exists() && !out_dir.is_dir() {
            return Err(ConversionError::InvalidFormat(format!(
                "Output path '{}' exists but is not a directory",
                self.output_dir()
            )));
        }

        // Password validation (if explicitly provided)
        if let Some(ref pwd) = self.password {
            if pwd.is_empty() {
                // If password is explicitly provided but empty, warn user
                // Since empty password is valid in some cases, we don't return an error
                eprintln!(
                    "Warning: Empty password provided. If the PFX file requires a password, conversion might fail."
                );
            }
        }

        Ok(())
    }
}
