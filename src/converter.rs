use crate::cli::Args;
use crate::error::ConversionError;
use crate::openssl::{PemFormatter, PfxParser};
use crate::output::{OutputConfig, OutputHandler, ProgressReporter};
use std::fs;
use std::path::Path;

/// Main conversion function that orchestrates the PFX to PEM conversion
pub fn convert_pfx_to_pem(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Set up output handling
    let output_config = OutputConfig::from_args(&args);
    let mut output = OutputHandler::new(output_config.clone());
    let progress = ProgressReporter::new(&output_config);

    // Start the conversion process
    progress.start_conversion();

    output.info("Starting PFX to PEM conversion...")?;
    output.info(&format!("Input file: {}", args.pfx))?;

    // Create output directory
    let output_dir = args.output_dir();
    progress.reading_file(&args.pfx);

    fs::create_dir_all(output_dir).map_err(|e| {
        progress.error(&format!("Failed to create output directory: {}", e));
        ConversionError::DirectoryCreation(output_dir.to_string(), e)
    })?;

    output.info(&format!("Output directory: {}", output_dir))?;

    // Parse the PFX file
    output.status("Converting PFX to PEM format...")?;
    progress.parsing();

    let parsed = PfxParser::parse_file(&args.pfx, args.password()).map_err(|e| {
        progress.error(&format!("Failed to parse PFX: {}", e));
        e
    })?;

    output.info("Successfully parsed PFX file")?;

    // Show basic cert info in verbose mode, detailed table will be shown in summary
    if output_config.verbose {
        output.info("Certificate information:")?;
        let cert_info_text = parsed.cert_info();
        for line in cert_info_text.lines() {
            output.info(line)?;
        }
    }

    output.print_cert_info(&parsed)?;

    if parsed.has_chain() {
        output.info(&format!(
            "Found {} additional certificates in chain",
            parsed.chain_length()
        ))?;
    }

    // Set up file paths
    let key_path = Path::new(output_dir).join(args.key_filename());
    let cert_path = Path::new(output_dir).join(args.cert_filename());
    let combined_path = Path::new(output_dir).join(args.combined_filename());

    // Convert and save private key
    progress.extracting_key();
    let private_key_pem = PemFormatter::private_key_to_pem(&parsed)?;
    write_file(&key_path, &private_key_pem)?;
    output.success(&format!("Private key saved to: {}", key_path.display()))?;

    // Convert and save certificate
    progress.extracting_cert();
    let cert_pem = PemFormatter::certificate_to_pem(&parsed)?;
    write_file(&cert_path, &cert_pem)?;
    output.success(&format!("Certificate saved to: {}", cert_path.display()))?;

    // Handle certificate chain if requested
    if args.chain && parsed.has_chain() {
        progress.extracting_chain(parsed.chain_length());
        save_certificate_chain(&parsed, output_dir, &mut output)?;
    }

    // Create combined file if requested
    if args.combined {
        progress.writing_files();
        let combined_pem = PemFormatter::combined_to_pem(&parsed, args.chain)?;
        write_file(&combined_path, &combined_pem)?;
        output.success(&format!(
            "Combined PEM saved to: {}",
            combined_path.display()
        ))?;
    }

    // Complete the process
    progress.complete();

    // Print the beautiful summary
    output.print_summary(&args, &parsed)?;

    Ok(())
}

/// Save certificate chain files
fn save_certificate_chain(
    parsed: &crate::openssl::ParsedPfx,
    output_dir: &str,
    output: &mut OutputHandler,
) -> Result<(), ConversionError> {
    // Save complete chain in one file
    let chain_pem = PemFormatter::chain_to_pem(parsed)?;
    let chain_path = Path::new(output_dir).join("certificate_chain.pem");
    write_file(&chain_path, &chain_pem)?;
    output
        .success(&format!(
            "Certificate chain saved to: {}",
            chain_path.display()
        ))
        .map_err(|e| ConversionError::FileWrite("output".to_string(), e))?;

    // Save individual chain certificates
    let chain_certs_pem = PemFormatter::chain_certs_to_pem(parsed)?;
    for (i, cert_pem) in chain_certs_pem.iter().enumerate() {
        let cert_path = Path::new(output_dir).join(format!("chain_cert_{}.pem", i + 1));
        write_file(&cert_path, cert_pem)?;
        output
            .info(&format!(
                "Chain certificate {} saved to: {}",
                i + 1,
                cert_path.display()
            ))
            .map_err(|e| ConversionError::FileWrite("output".to_string(), e))?;
    }

    Ok(())
}

/// Write data to a file with proper error handling
fn write_file<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<(), ConversionError> {
    let path = path.as_ref();
    fs::write(path, data).map_err(|e| ConversionError::FileWrite(path.display().to_string(), e))
}
