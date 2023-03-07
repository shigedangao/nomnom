use indicatif::{ProgressBar, ProgressStyle};
use crate::error::Error;

pub(crate) struct IndicHandler<'a> {
    pb: ProgressBar,
    max_size: u64,
    end_msg: &'a str
}

impl<'a> IndicHandler<'a> {
    /// Create a new Indic Handler
    /// 
    /// # Arguments
    /// 
    /// * `size` - u64
    /// * `end_msg` - &'a str
    pub fn new(size: u64, end_msg: &'a str) -> IndicHandler<'a> {
        let pb = ProgressBar::new(size);

        IndicHandler {
            pb,
            max_size: size - 1,
            end_msg
        }
    }

    /// Set the style of the progress bar
    pub fn set_style(&mut self) -> Result<&mut Self, Error> {
        let style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} {msg}")
            .map_err(|err| Error::Indic(err.to_string()))?
            .progress_chars("#>-");

        self.pb.set_style(style);

        Ok(self)
    }

    /// Increase the progress bar & display a message when completed
    pub fn increase(&mut self) {
        self.pb.inc(1);

        if self.pb.position() == self.max_size {
            self.pb.finish_with_message(self.end_msg.to_string());
        }
    }
}
