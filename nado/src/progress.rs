use indicatif::ProgressBar;

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
        Self {
            pb: ProgressBar::new(size),
        }
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
