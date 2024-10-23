use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct SourcesConfig {
    #[serde(default = "defaults::op_batcher")]
    pub op_batcher: Source,
    #[serde(default = "defaults::op_node")]
    pub op_node: Source,
    #[serde(default = "defaults::op_proposer")]
    pub op_proposer: Source,
    #[serde(default = "defaults::op_contracts")]
    pub op_contracts: Source,
    #[serde(default = "defaults::op_geth")]
    pub op_geth: Source,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub release_tag: String,
    pub build: Vec<String>,
}


mod defaults {

    pub fn op_batcher() -> super::Source {
        super::Source {
            release_tag: "v0.4.0".to_string(),
            build: vec![],
        }
    }

    pub fn op_node() -> super::Source {
        super::Source {
            release_tag: "v0.4.0".to_string(),
            build: vec![],
        }
    }

    pub fn op_proposer() -> super::Source {
        super::Source {
            release_tag: "v0.4.0".to_string(),
            build: vec![],
        }
    }

    pub fn op_contracts() -> super::Source {
        super::Source {
            release_tag: "v0.4.0".to_string(),
            build: vec![],
        }
    }

    pub fn op_geth() -> super::Source {
        super::Source {
            release_tag: "v0.4.0".to_string(),
            build: vec![],
        }
    }
}
