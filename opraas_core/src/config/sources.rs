use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourcesConfig {
    #[serde(default = "defaults::op_repo_url")]
    pub op_repo_url: String,
    #[serde(default = "defaults::op_repo_tag")]
    pub op_repo_tag: String,
    #[serde(default = "defaults::op_repo_target")]
    pub op_repo_target: String,

    #[serde(default = "defaults::op_geth_repo_repo")]
    pub op_geth_repo_url: String,
    #[serde(default = "defaults::op_geth_repo_tag")]
    pub op_geth_repo_tag: String,
    #[serde(default = "defaults::op_geth_repo_target")]
    pub op_geth_repo_target: String,
}

mod defaults {
    // optimism repo
    pub fn op_repo_url() -> String {
        "https://github.com/ethereum-optimism/optimism.git".to_string()
    }
    pub fn op_repo_tag() -> String {
        "v1.9.3".to_string()
    }
    pub fn op_repo_target() -> String {
        "optimism".to_string()
    }

    // op-geth repo
    pub fn op_geth_repo_repo() -> String {
        "https://github.com/ethereum-optimism/op-geth.git".to_string()
    }
    pub fn op_geth_repo_tag() -> String {
        "v1.101315.3".to_string()
    }
    pub fn op_geth_repo_target() -> String {
        "op-geth".to_string()
    }
}
