use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourcesConfig {
    pub node: Source,
    pub geth: Source,
    pub contracts: Source,
    pub batcher: Source,
    pub proposer: Source,
    pub explorer: Source,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub release_tag: String,
}
