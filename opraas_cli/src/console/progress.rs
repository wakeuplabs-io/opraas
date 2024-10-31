use indicatif::{ProgressBar, ProgressStyle};
use opraas_core::progress::ProgressTracker;

pub struct ConsoleProgressTracker  {
    bar: ProgressBar,
}

impl ConsoleProgressTracker  {
    pub fn new(message: &str) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        bar.set_message(message.to_string());
        bar.enable_steady_tick(std::time::Duration::from_millis(100));

        ConsoleProgressTracker { bar }
    }

    pub fn finish(self, message: &str) {
        self.bar.finish_with_message(message.to_string());
    }
}

impl ProgressTracker for ConsoleProgressTracker {
    fn set_length(&self, length: u64) {
        if length > 0 {
            self.bar.set_length(length);
            self.bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                    .unwrap()
                    .progress_chars("#>-"),
            );
        } // else we're already spinner
    }

    fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }
}


