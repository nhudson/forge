use crate::cli::Args;
use crate::openssl::ParsedPfx;
use crate::output::OutputConfig;
use colored::*;
use console::Term;
use std::io::{self, Write};
use tabled::{
    Table, Tabled,
    settings::{Alignment, Modify, Style, object::Rows},
};

/// Handles detailed TUI output formatting
pub struct OutputFormatter {
    config: OutputConfig,
}

#[derive(Tabled)]
struct FileOutput {
    #[tabled(rename = "Type")]
    file_type: String,
    #[tabled(rename = "Filename")]
    filename: String,
    #[tabled(rename = "Location")]
    location: String,
    #[tabled(rename = "Status")]
    status: String,
}

#[derive(Tabled)]
struct CertInfo {
    #[tabled(rename = "Property")]
    property: String,
    #[tabled(rename = "Value")]
    value: String,
}

impl OutputFormatter {
    pub fn new(config: &OutputConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Print a beautifully formatted summary
    pub fn print_summary(
        &self,
        args: &Args,
        parsed: &ParsedPfx,
        term: &mut Term,
    ) -> io::Result<()> {
        // Header
        self.print_header("Conversion Summary", term)?;

        // Create table data
        let mut files = vec![
            FileOutput {
                file_type: "Private Key".to_string(),
                filename: args.key_filename().to_string(),
                location: args.output_dir().to_string(),
                status: if self.config.use_colors {
                    "✓ Created".green().to_string()
                } else {
                    "✓ Created".to_string()
                },
            },
            FileOutput {
                file_type: "Certificate".to_string(),
                filename: args.cert_filename().to_string(),
                location: args.output_dir().to_string(),
                status: if self.config.use_colors {
                    "✓ Created".green().to_string()
                } else {
                    "✓ Created".to_string()
                },
            },
        ];

        // Add combined file if requested
        if args.combined {
            files.push(FileOutput {
                file_type: "Combined PEM".to_string(),
                filename: args.combined_filename().to_string(),
                location: args.output_dir().to_string(),
                status: if self.config.use_colors {
                    "✓ Created".green().to_string()
                } else {
                    "✓ Created".to_string()
                },
            });
        }

        // Add chain files if requested
        if args.chain && parsed.has_chain() {
            files.push(FileOutput {
                file_type: "Certificate Chain".to_string(),
                filename: "certificate_chain.pem".to_string(),
                location: args.output_dir().to_string(),
                status: if self.config.use_colors {
                    "✓ Created".green().to_string()
                } else {
                    "✓ Created".to_string()
                },
            });

            for i in 0..parsed.chain_length() {
                files.push(FileOutput {
                    file_type: format!("Chain Cert {}", i + 1),
                    filename: format!("chain_cert_{}.pem", i + 1),
                    location: args.output_dir().to_string(),
                    status: if self.config.use_colors {
                        "✓ Created".green().to_string()
                    } else {
                        "✓ Created".to_string()
                    },
                });
            }
        }

        // Create and style the table
        let mut table = Table::new(&files);
        table
            .with(Style::rounded())
            .with(Modify::new(Rows::first()).with(Alignment::center()));

        if self.config.use_colors {
            writeln!(term, "{}", table.to_string().bright_white())?;
        } else {
            writeln!(term, "{table}")?;
        }

        // Statistics box
        self.print_stats_box(parsed, term)?;

        // Footer
        if self.config.use_colors {
            writeln!(
                term,
                "\n{} Conversion completed successfully!",
                "🎉".bright_green()
            )?;
        } else {
            writeln!(term, "\n✓ Conversion completed successfully!")?;
        }

        Ok(())
    }

    /// Print certificate information in a formatted way
    pub fn print_cert_info(&self, parsed: &ParsedPfx, term: &mut Term) -> io::Result<()> {
        self.print_header("Certificate Information", term)?;

        let cert_info = parsed.certificate_info();

        let cert_data = vec![
            CertInfo {
                property: "Subject".to_string(),
                value: cert_info.subject,
            },
            CertInfo {
                property: "Issuer".to_string(),
                value: cert_info.issuer,
            },
            CertInfo {
                property: "Serial Number".to_string(),
                value: cert_info.serial_number,
            },
            CertInfo {
                property: "Valid From".to_string(),
                value: cert_info.not_before,
            },
            CertInfo {
                property: "Valid Until".to_string(),
                value: cert_info.not_after,
            },
            CertInfo {
                property: "Signature Algorithm".to_string(),
                value: cert_info.signature_algorithm,
            },
        ];

        let mut table = Table::new(&cert_data);
        table
            .with(Style::rounded())
            .with(Modify::new(Rows::first()).with(Alignment::center()));

        if self.config.use_colors {
            writeln!(term, "{}", table.to_string().bright_white())?;
        } else {
            writeln!(term, "{table}")?;
        }

        Ok(())
    }

    /// Print a stylized header
    fn print_header(&self, title: &str, term: &mut Term) -> io::Result<()> {
        let width = 60;
        let padding = (width - title.len() - 2) / 2;

        if self.config.use_colors {
            writeln!(term, "\n{}", "═".repeat(width).bright_cyan())?;
            writeln!(
                term,
                "{}{}{}{}",
                "║".bright_cyan(),
                " ".repeat(padding),
                title.bold().bright_white(),
                " ".repeat(width - padding - title.len() - 2) + "║"
            )?;
            writeln!(term, "{}", "═".repeat(width).bright_cyan())?;
        } else {
            writeln!(term, "\n{}", "=".repeat(width))?;
            writeln!(
                term,
                "|{}{}{}|",
                " ".repeat(padding),
                title,
                " ".repeat(width - padding - title.len() - 2)
            )?;
            writeln!(term, "{}", "=".repeat(width))?;
        }

        Ok(())
    }

    /// Print statistics in a box
    fn print_stats_box(&self, parsed: &ParsedPfx, term: &mut Term) -> io::Result<()> {
        writeln!(term)?;

        if self.config.use_colors {
            writeln!(
                term,
                "{}",
                "┌─ Statistics ─────────────────────────────────┐".bright_blue()
            )?;
            writeln!(
                term,
                "{} Files generated: {}                        {}",
                "│".bright_blue(),
                if parsed.has_chain() { "5+" } else { "2-3" }.bright_yellow(),
                "│".bright_blue()
            )?;
            writeln!(
                term,
                "{} Certificate chain: {}                      {}",
                "│".bright_blue(),
                if parsed.has_chain() {
                    format!("{} certificates", parsed.chain_length() + 1).bright_green()
                } else {
                    "No chain".bright_red()
                },
                "│".bright_blue()
            )?;
            writeln!(
                term,
                "{} Private key format: {}                     {}",
                "│".bright_blue(),
                "PKCS#8 PEM".bright_green(),
                "│".bright_blue()
            )?;
            writeln!(
                term,
                "{}",
                "└─────────────────────────────────────────────┘".bright_blue()
            )?;
        } else {
            writeln!(term, "┌─ Statistics ─────────────────────────────────┐")?;
            writeln!(
                term,
                "│ Files generated: {}                        │",
                if parsed.has_chain() { "5+" } else { "2-3" }
            )?;
            writeln!(
                term,
                "│ Certificate chain: {}                      │",
                if parsed.has_chain() {
                    format!("{} certificates", parsed.chain_length() + 1)
                } else {
                    "No chain".to_string()
                }
            )?;
            writeln!(term, "│ Private key format: PKCS#8 PEM             │")?;
            writeln!(term, "└─────────────────────────────────────────────┘")?;
        }

        Ok(())
    }
}
