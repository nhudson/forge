mod formatter;
pub mod parser;

pub use formatter::PemFormatter;
pub use parser::PfxParser;

use openssl::pkcs12::ParsedPkcs12_2 as ParsedPkcs12;
use openssl::pkey::PKey;
use openssl::pkey::Private;
use openssl::x509::X509;

/// Represents the contents of a parsed PFX file
#[derive(Debug)]
pub struct ParsedPfx {
    /// The private key
    pub private_key: PKey<Private>,
    /// The main certificate
    pub certificate: X509,
    /// Additional certificates in the chain (if any)
    pub chain: Vec<X509>,
}

impl From<ParsedPkcs12> for ParsedPfx {
    fn from(parsed: ParsedPkcs12) -> Self {
        Self {
            private_key: parsed.pkey.expect("Private key is required"),
            certificate: parsed.cert.expect("Certificate is required"),
            chain: parsed
                .ca
                .map(|stack| stack.into_iter().collect())
                .unwrap_or_default(),
        }
    }
}

/// Certificate information structure
#[derive(Debug, Clone)]
pub struct CertificateInfo {
    pub subject: String,
    pub issuer: String,
    pub serial_number: String,
    pub not_before: String,
    pub not_after: String,
    pub signature_algorithm: String,
}

impl ParsedPfx {
    /// Get certificate information as a formatted string (legacy method)
    pub fn cert_info(&self) -> String {
        let info = self.certificate_info();
        format!("Subject: {}\nIssuer: {}", info.subject, info.issuer)
    }

    /// Get detailed certificate information
    pub fn certificate_info(&self) -> CertificateInfo {
        let cert = &self.certificate;
        CertificateInfo {
            subject: format!("{:?}", cert.subject_name()),
            issuer: format!("{:?}", cert.issuer_name()),
            serial_number: cert
                .serial_number()
                .to_bn()
                .unwrap()
                .to_hex_str()
                .unwrap()
                .to_string(),
            not_before: cert.not_before().to_string(),
            not_after: cert.not_after().to_string(),
            signature_algorithm: cert.signature_algorithm().object().to_string(),
        }
    }

    /// Check if this PFX contains a certificate chain
    pub fn has_chain(&self) -> bool {
        !self.chain.is_empty()
    }

    /// Get the number of certificates in the chain
    pub fn chain_length(&self) -> usize {
        self.chain.len()
    }
}
