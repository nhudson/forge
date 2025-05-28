//! # forge
//!
//! A Rust library and command-line tool for converting PFX/P12 certificate files to PEM format.
//!
//! ## Features
//!
//! - Convert PFX/P12 files to PEM format
//! - Support for password-protected files
//! - Extract certificate chains
//! - Generate combined PEM files
//! - Pure Rust implementation using OpenSSL bindings
//!
//! ## Usage as a Library
//!
//! ```rust,no_run
//! use forge::openssl::{PfxParser, PemFormatter};
//!
//! // Parse a PFX file
//! let parsed = PfxParser::parse_file("certificate.pfx", "password")?;
//!
//! // Convert to PEM format
//! let private_key_pem = PemFormatter::private_key_to_pem(&parsed)?;
//! let certificate_pem = PemFormatter::certificate_to_pem(&parsed)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod cli;
pub mod converter;
pub mod error;
pub mod openssl;
pub mod output;

// Re-export commonly used types
pub use error::ConversionError;
pub use openssl::{ParsedPfx, PemFormatter, PfxParser};
