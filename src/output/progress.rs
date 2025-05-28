use crate::output::OutputConfig;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Handles progress reporting during conversion
pub struct ProgressReporter {
    bar: Option<ProgressBar>,
}

impl ProgressReporter {
    pub fn new(config: &OutputConfig) -> Self {
        let bar = if config.interactive {
            Some(Self::create_progress_bar(config.use_colors))
        } else {
            None
        };

        Self { bar }
    }

    /// Start the conversion process
    pub fn start_conversion(&self) {
        if let Some(ref bar) = self.bar {
            bar.set_message("Initializing conversion...");
            bar.set_position(0);
        }
    }

    /// Update progress for file reading
    pub fn reading_file(&self, filename: &str) {
        if let Some(ref bar) = self.bar {
            bar.set_message(format!("Reading PFX file: {}", filename));
            bar.set_position(1);
        }
    }

    /// Update progress for parsing
    pub fn parsing(&self) {
        if let Some(ref bar) = self.bar {
            bar.set_message("Parsing PFX structure...");
            bar.set_position(2);
        }
    }

    /// Update progress for key extraction
    pub fn extracting_key(&self) {
        if let Some(ref bar) = self.bar {
            bar.set_message("Extracting private key...");
            bar.set_position(3);
        }
    }

    /// Update progress for certificate extraction
    pub fn extracting_cert(&self) {
        if let Some(ref bar) = self.bar {
            bar.set_message("Extracting certificate...");
            bar.set_position(4);
        }
    }

    /// Update progress for chain extraction
    pub fn extracting_chain(&self, count: usize) {
        if let Some(ref bar) = self.bar {
            bar.set_message(format!("Extracting certificate chain ({} certs)...", count));
            bar.set_position(5);
        }
    }

    /// Update progress for file writing
    pub fn writing_files(&self) {
        if let Some(ref bar) = self.bar {
            bar.set_message("Writing PEM files...");
            bar.set_position(6);
        }
    }

    /// Complete the process
    pub fn complete(&self) {
        if let Some(ref bar) = self.bar {
            bar.set_message("Conversion completed!");
            bar.set_position(7);
            bar.finish();
        }
    }

    /// Handle errors
    pub fn error(&self, message: &str) {
        if let Some(ref bar) = self.bar {
            bar.abandon_with_message(format!("Error: {}", message));
        }
    }

    /// Create a styled progress bar
    fn create_progress_bar(use_colors: bool) -> ProgressBar {
        let bar = ProgressBar::new(7);

        if use_colors {
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("█▉▊▋▌▍▎▏  "),
            );
        } else {
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner} [{elapsed_precise}] [{bar:40}] {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("#>-"),
            );
        }

        bar.enable_steady_tick(Duration::from_millis(100));
        bar
    }
}

impl Drop for ProgressReporter {
    fn drop(&mut self) {
        if let Some(ref bar) = self.bar {
            bar.finish_and_clear();
        }
    }
}
