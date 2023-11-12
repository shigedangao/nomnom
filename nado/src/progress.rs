use indicatif::ProgressBar;

#[derive(Debug)]
pub struct ProgressBuilder {
    pb: ProgressBar
}

impl ProgressBuilder {
    pub fn new(size: u64) -> Self {
        Self {
            pb: ProgressBar::new(size)
        }
    }

    pub fn inc(&mut self, value: u64) {
        self.pb.inc(value);
    }

    pub fn clear(&mut self) {
        self.pb.finish_and_clear();
    }
}
