pub trait ProgressTracker: std::marker::Sync {
    fn set_length(&self, length: u64);
    fn inc(&self, delta: u64);
}
