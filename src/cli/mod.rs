use clap::Parser;
use crate::secure::SecurePassword;

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
}
