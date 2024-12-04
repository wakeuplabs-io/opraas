use std::{collections::HashMap, path::Path};

use crate::domain::Release;

pub trait TReleaseRunner {
    fn run(
        &self,
        release: &Release,
        volume: &Path,
        env: HashMap<&str, String>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
