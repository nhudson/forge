mod formatter;
mod progress;

pub use formatter::OutputFormatter;
pub use progress::ProgressReporter;

use crate::cli::Args;
use crate::openssl::ParsedPfx;
use colored::*;
use console::Term;
use std::io::{self, Write};

/// Configuration for output formatting
#[derive(Debug, Clone)]
pub struct OutputConfig {
    pub use_colors: bool,
    pub verbose: bool,
    pub interactive: bool,
}

impl OutputConfig {
    pub fn from_args(args: &Args) -> Self {
        let term = Term::stdout();
        Self {
            use_colors: term.features().colors_supported(),
            verbose: args.verbose,
            interactive: term.features().is_attended(),
        }
    }
}

/// Main output handler
pub struct OutputHandler {
    config: OutputConfig,
    term: Term,
}

impl OutputHandler {
    pub fn new(config: OutputConfig) -> Self {
        Self {
            config,
            term: Term::stdout(),
        }
    }

    /// Print a status message
    pub fn status(&mut self, message: &str) -> io::Result<()> {
        if self.config.use_colors {
            writeln!(self.term, "{} {}", "⚡".bright_yellow(), message.bold())?;
        } else {
            writeln!(self.term, "=> {message}")?;
        }
        Ok(())
    }

    /// Print a success message
    pub fn success(&mut self, message: &str) -> io::Result<()> {
        if self.config.use_colors {
            writeln!(self.term, "{} {}", "✓".bright_green(), message)?;
        } else {
            writeln!(self.term, "✓ {message}")?;
        }
        Ok(())
    }

    /// Print an info message (only in verbose mode)
    pub fn info(&mut self, message: &str) -> io::Result<()> {
        if self.config.verbose {
            if self.config.use_colors {
                writeln!(self.term, "{} {}", "ℹ".bright_blue(), message.dimmed())?;
            } else {
                writeln!(self.term, "i {message}")?;
            }
        }
        Ok(())
    }

    /// Print a formatted summary
    pub fn print_summary(&mut self, args: &Args, parsed: &ParsedPfx) -> io::Result<()> {
        let formatter = OutputFormatter::new(&self.config);
        formatter.print_summary(args, parsed, &mut self.term)
    }

    /// Print certificate information
    pub fn print_cert_info(&mut self, parsed: &ParsedPfx) -> io::Result<()> {
        if self.config.verbose {
            let formatter = OutputFormatter::new(&self.config);
            formatter.print_cert_info(parsed, &mut self.term)?;
        }
        Ok(())
    }
}
