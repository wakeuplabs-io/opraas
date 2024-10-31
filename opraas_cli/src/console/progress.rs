use indicatif::{ProgressBar, ProgressStyle};
use opraas_core::ProgressTracker;

pub struct ConsoleProgressBar<'a>  {
    bar: ProgressBar,
    finish_msg: &'a str,
}

impl<'a> ConsoleProgressBar<'a>  {
    pub fn new_progress_bar(message: &str, finish_msg: &'a str) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        bar.set_message(message.to_string());

        ConsoleProgressBar { bar, finish_msg }
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

        ConsoleProgressBar { bar, finish_msg }
    }
}

impl<'a> ProgressTracker for ConsoleProgressBar<'a> {
    fn set_length(&self, length: u64) {
        if length > 0 {
            self.bar.set_length(length);
        } else {
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



// fn main() {
//     let bar = ConsoleProgressBar::new_spinner_tracker("Processing...", "Done!");
//     opraas_core::download(&bar);
//     bar.finish();
// }
