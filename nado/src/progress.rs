use indicatif::{ProgressBar, ProgressStyle};

// Constant
const PB_STYLE: &str =
    "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";

/// Wrapper over the ProgressBar
#[derive(Debug)]
pub struct ProgressBuilder {
    pb: ProgressBar,
}

impl ProgressBuilder {
    /// Create a new progress bar
    ///
    /// # Arguments
    ///
    /// * `size` - u64
    pub fn new(size: u64) -> Self {
        let pb = ProgressBar::new(size);
        if let Ok(style) = ProgressStyle::with_template(PB_STYLE) {
            pb.set_style(style);
        }

        Self { pb }
    }

    /// Increase the value of the progress bar
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Self
    /// * `value` - u64
    pub fn inc(&mut self, value: u64) {
        self.pb.inc(value);
    }

    /// Clear the progress bar
    pub fn clear(&mut self) {
        self.pb.finish_and_clear();
    }
}
