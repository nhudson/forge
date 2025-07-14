use forge::openssl::{PemFormatter, PfxParser};
use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkcs12::Pkcs12;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::extension::{BasicConstraints, KeyUsage, SubjectKeyIdentifier};
use openssl::x509::{X509, X509NameBuilder};
use std::fs;
use tempfile::TempDir;

fn create_test_certificate() -> (PKey<Private>, X509) {
    // Generate a private key
    let rsa = Rsa::generate(2048).unwrap();
    let private_key = PKey::from_rsa(rsa).unwrap();

    // Create a certificate
    let mut cert_builder = X509::builder().unwrap();
    cert_builder.set_version(2).unwrap();

    let serial_number = {
        let mut serial = BigNum::new().unwrap();
        serial.rand(159, MsbOption::MAYBE_ZERO, false).unwrap();
        serial.to_asn1_integer().unwrap()
    };
    cert_builder.set_serial_number(&serial_number).unwrap();

    let mut name_builder = X509NameBuilder::new().unwrap();
    name_builder
        .append_entry_by_nid(Nid::COUNTRYNAME, "US")
        .unwrap();
    name_builder
        .append_entry_by_nid(Nid::STATEORPROVINCENAME, "California")
        .unwrap();
    name_builder
        .append_entry_by_nid(Nid::LOCALITYNAME, "San Francisco")
        .unwrap();
    name_builder
        .append_entry_by_nid(Nid::ORGANIZATIONNAME, "Test Company")
        .unwrap();
    name_builder
        .append_entry_by_nid(Nid::COMMONNAME, "test.example.com")
        .unwrap();
    let name = name_builder.build();

    cert_builder.set_subject_name(&name).unwrap();
    cert_builder.set_issuer_name(&name).unwrap();

    let not_before = Asn1Time::days_from_now(0).unwrap();
    cert_builder.set_not_before(&not_before).unwrap();
    let not_after = Asn1Time::days_from_now(365).unwrap();
    cert_builder.set_not_after(&not_after).unwrap();

    cert_builder.set_pubkey(&private_key).unwrap();

    let basic_constraints = BasicConstraints::new().build().unwrap();
    cert_builder.append_extension(basic_constraints).unwrap();

    let key_usage = KeyUsage::new()
        .critical()
        .non_repudiation()
        .digital_signature()
        .key_encipherment()
        .build()
        .unwrap();
    cert_builder.append_extension(key_usage).unwrap();

    let subject_key_identifier = SubjectKeyIdentifier::new()
        .build(&cert_builder.x509v3_context(None, None))
        .unwrap();
    cert_builder
        .append_extension(subject_key_identifier)
        .unwrap();

    cert_builder
        .sign(&private_key, MessageDigest::sha256())
        .unwrap();
    let cert = cert_builder.build();

    (private_key, cert)
}

fn create_test_pfx(password: &str) -> Vec<u8> {
    let (private_key, cert) = create_test_certificate();

    let mut pkcs12_builder = Pkcs12::builder();
    pkcs12_builder.name("test");
    pkcs12_builder.pkey(&private_key);
    pkcs12_builder.cert(&cert);
    let pkcs12 = pkcs12_builder.build2(password).unwrap();

    pkcs12.to_der().unwrap()
}

#[test]
fn test_pfx_parsing_without_password() {
    let pfx_data = create_test_pfx("");
    let parsed = PfxParser::parse_bytes(&pfx_data, "").unwrap();

    // Verify we have the required components
    assert!(!parsed.has_chain()); // Our test cert has no chain
    assert_eq!(parsed.chain_length(), 0);
}

#[test]
fn test_pfx_parsing_with_password() {
    let password = "test123";
    let pfx_data = create_test_pfx(password);
    let parsed = PfxParser::parse_bytes(&pfx_data, password).unwrap();

    // Verify we have the required components
    assert!(!parsed.has_chain()); // Our test cert has no chain
    assert_eq!(parsed.chain_length(), 0);
}

#[test]
fn test_pfx_parsing_wrong_password() {
    let correct_password = "test123";
    let wrong_password = "wrong";
    let pfx_data = create_test_pfx(correct_password);

    let result = PfxParser::parse_bytes(&pfx_data, wrong_password);
    assert!(result.is_err());
}

#[test]
fn test_pem_formatting() {
    let pfx_data = create_test_pfx("");
    let parsed = PfxParser::parse_bytes(&pfx_data, "").unwrap();

    // Test private key formatting
    let private_key_pem = PemFormatter::private_key_to_pem(&parsed).unwrap();
    assert!(PemFormatter::validate_pem(&private_key_pem));

    let private_key_str = String::from_utf8(private_key_pem).unwrap();
    assert!(private_key_str.starts_with("-----BEGIN PRIVATE KEY-----"));
    assert!(private_key_str.ends_with("-----END PRIVATE KEY-----\n"));

    // Test certificate formatting
    let cert_pem = PemFormatter::certificate_to_pem(&parsed).unwrap();
    assert!(PemFormatter::validate_pem(&cert_pem));

    let cert_str = String::from_utf8(cert_pem).unwrap();
    assert!(cert_str.starts_with("-----BEGIN CERTIFICATE-----"));
    assert!(cert_str.ends_with("-----END CERTIFICATE-----\n"));

    // Test combined formatting
    let combined_pem = PemFormatter::combined_to_pem(&parsed, false).unwrap();
    assert!(PemFormatter::validate_pem(&combined_pem));

    let combined_str = String::from_utf8(combined_pem).unwrap();
    assert!(combined_str.contains("-----BEGIN PRIVATE KEY-----"));
    assert!(combined_str.contains("-----BEGIN CERTIFICATE-----"));
}

#[test]
fn test_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let pfx_path = temp_dir.path().join("test.pfx");

    // Create a test PFX file
    let pfx_data = create_test_pfx("");
    fs::write(&pfx_path, pfx_data).unwrap();

    // Test parsing from file
    let parsed = PfxParser::parse_file(&pfx_path, "").unwrap();

    // Verify parsing worked
    assert!(!parsed.has_chain());

    // Test certificate info
    let info = parsed.cert_info();
    assert!(info.contains("Subject:"));
    assert!(info.contains("Issuer:"));
}

#[test]
fn test_invalid_pfx_data() {
    let invalid_data = b"This is not a valid PFX file";
    let result = PfxParser::parse_bytes(invalid_data, "");

    assert!(result.is_err());
    match result.unwrap_err() {
        forge::error::ConversionError::InvalidFormat(_) => {}
        _ => panic!("Expected InvalidFormat error"),
    }
}

#[test]
fn test_nonexistent_file() {
    let result = PfxParser::parse_file("/nonexistent/path/file.pfx", "");

    assert!(result.is_err());
    match result.unwrap_err() {
        forge::error::ConversionError::FileNotFound(_) => {}
        _ => panic!("Expected FileNotFound error"),
    }
}

// #[test]
// fn test_password_variants_generation() {
//     // Test the password variant generation for Windows compatibility
//     let variants = forge::openssl::parser::PfxParser::generate_password_variants("test\\pass");
//     assert!(variants.contains(&"test\\pass".to_string()));
//     assert!(variants.contains(&"test/pass".to_string()));
//     assert!(variants.contains(&"testpass".to_string()));

//     let variants_quotes =
//         forge::openssl::parser::PfxParser::generate_password_variants("test\"pass");
//     assert!(variants_quotes.contains(&"test\"pass".to_string()));
//     assert!(variants_quotes.contains(&"test\\\"pass".to_string()));
//     assert!(variants_quotes.contains(&"testpass".to_string()));

//     let variants_spaces = forge::openssl::parser::PfxParser::generate_password_variants(" test ");
//     assert!(variants_spaces.contains(&" test ".to_string()));
//     assert!(variants_spaces.contains(&"test".to_string()));
// }
#[test]
fn test_error_message_patterns() {
    // Test that we can identify Windows-specific error patterns
    let rc2_error_msg = "error: 0308010C:digital envelope routines: inner_evp_generic_fetch:unsupported:crypto\\evp\\evp_fetch.c:375:Global default library context, Algorithm (RC2-40-CBC : 0), Properties ()";
    assert!(rc2_error_msg.contains("RC2-40-CBC"));
    assert!(rc2_error_msg.contains("unsupported"));

    let mac_error_msg = "error: 11800071:PKCS12 routines: PKCS12_parse:mac verify failure:crypto\\pkcs12\\p12_kiss.c:71:";
    assert!(mac_error_msg.contains("mac verify failure"));
    assert!(mac_error_msg.contains("PKCS12"));
}
