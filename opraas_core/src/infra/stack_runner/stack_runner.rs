use std::collections::HashMap;

pub trait TStackRunner {
    fn run(
        &self,
        values_file: &str,
        overrides: HashMap<&str, &str>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
}
