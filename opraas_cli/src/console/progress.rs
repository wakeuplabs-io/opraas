use indicatif::{ProgressBar, ProgressStyle};
use opraas_core::progress::ProgressTracker;

pub struct ConsoleProgressTracker<'a>  {
    bar: ProgressBar,
    finish_msg: &'a str,
}

impl<'a> ConsoleProgressTracker<'a>  {
    pub fn new_progress_bar(message: &str, finish_msg: &'a str) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        bar.set_message(message.to_string());

        ConsoleProgressTracker { bar, finish_msg }
    }

    pub fn new_spinner_tracker(message: &str, finish_msg: &'a str) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner} {msg}")
                .unwrap(),
        );
        bar.set_message(message.to_string());
        bar.enable_steady_tick(std::time::Duration::from_millis(100));

        ConsoleProgressTracker { bar, finish_msg }
    }
}

impl<'a> ProgressTracker for ConsoleProgressTracker<'a> {
    fn set_length(&self, length: u64) {
        if length > 0 {
            self.bar.set_length(length);
            self.bar.set_style(
                ProgressStyle::default_bar()
                    .template("{msg} [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
                    .unwrap()
                    .progress_chars("#>-"),
            );
        } else {
            // if we were on a  progress bar, switch to a spinner as there's nothing to show. This shouldn't be called by spinner
            self.bar.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner} {msg}")
                    .unwrap(),
            );
            self.bar
                .enable_steady_tick(std::time::Duration::from_millis(100));
        }
    }

    fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }

    fn finish(&self) {
        self.bar.finish_with_message(self.finish_msg.to_string());
    }
}


