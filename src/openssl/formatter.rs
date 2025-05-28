use crate::error::ConversionError;
use crate::openssl::ParsedPfx;
use openssl::x509::X509;

/// Formatter for converting certificates and keys to PEM format
pub struct PemFormatter;

impl PemFormatter {
    /// Convert private key to PEM format
    pub fn private_key_to_pem(parsed: &ParsedPfx) -> Result<Vec<u8>, ConversionError> {
        let pem_data = parsed
            .private_key
            .private_key_to_pem_pkcs8()
            .map_err(ConversionError::from)?;

        // Validate the generated PEM
        if !Self::validate_pem(&pem_data) {
            return Err(ConversionError::InvalidFormat(
                "Generated private key PEM is invalid".to_string(),
            ));
        }

        Ok(pem_data)
    }

    /// Convert certificate to PEM format
    pub fn certificate_to_pem(parsed: &ParsedPfx) -> Result<Vec<u8>, ConversionError> {
        let pem_data = parsed.certificate.to_pem().map_err(ConversionError::from)?;

        // Validate the generated PEM
        if !Self::validate_pem(&pem_data) {
            return Err(ConversionError::InvalidFormat(
                "Generated certificate PEM is invalid".to_string(),
            ));
        }

        Ok(pem_data)
    }

    /// Convert a single certificate to PEM format
    pub fn cert_to_pem(cert: &X509) -> Result<Vec<u8>, ConversionError> {
        let pem_data = cert.to_pem().map_err(ConversionError::from)?;

        // Validate the generated PEM
        if !Self::validate_pem(&pem_data) {
            return Err(ConversionError::InvalidFormat(
                "Generated certificate PEM is invalid".to_string(),
            ));
        }

        Ok(pem_data)
    }

    /// Convert certificate chain to PEM format (all certificates concatenated)
    pub fn chain_to_pem(parsed: &ParsedPfx) -> Result<Vec<u8>, ConversionError> {
        let mut chain_pem = Self::certificate_to_pem(parsed)?;

        for cert in &parsed.chain {
            let cert_pem = Self::cert_to_pem(cert)?;
            chain_pem.extend_from_slice(&cert_pem);
        }

        // Validate the complete chain PEM
        if !Self::validate_pem(&chain_pem) {
            return Err(ConversionError::InvalidFormat(
                "Generated certificate chain PEM is invalid".to_string(),
            ));
        }

        Ok(chain_pem)
    }

    /// Convert individual chain certificates to PEM format
    pub fn chain_certs_to_pem(parsed: &ParsedPfx) -> Result<Vec<Vec<u8>>, ConversionError> {
        parsed.chain.iter().map(Self::cert_to_pem).collect()
    }

    /// Create a combined PEM with private key and certificate(s)
    pub fn combined_to_pem(
        parsed: &ParsedPfx,
        include_chain: bool,
    ) -> Result<Vec<u8>, ConversionError> {
        let mut combined = Self::private_key_to_pem(parsed)?;

        if include_chain && parsed.has_chain() {
            let chain_pem = Self::chain_to_pem(parsed)?;
            combined.extend_from_slice(&chain_pem);
        } else {
            let cert_pem = Self::certificate_to_pem(parsed)?;
            combined.extend_from_slice(&cert_pem);
        }

        // Validate the combined PEM
        if !Self::validate_pem(&combined) {
            return Err(ConversionError::InvalidFormat(
                "Generated combined PEM is invalid".to_string(),
            ));
        }

        Ok(combined)
    }

    /// Validate PEM format (basic check)
    pub fn validate_pem(pem_data: &[u8]) -> bool {
        let pem_str = match std::str::from_utf8(pem_data) {
            Ok(s) => s,
            Err(_) => return false,
        };

        // Check for PEM markers - should contain at least one BEGIN/END pair
        let has_begin = pem_str.contains("-----BEGIN");
        let has_end = pem_str.contains("-----END");

        if !has_begin || !has_end {
            return false;
        }

        // Count BEGIN and END markers - they should match
        let begin_count = pem_str.matches("-----BEGIN").count();
        let end_count = pem_str.matches("-----END").count();

        begin_count == end_count && begin_count > 0
    }
}
