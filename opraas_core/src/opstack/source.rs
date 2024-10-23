use crate::utils;
use std::{error::Error, path::Path};
use crate::config::sources::Source;

const OPT_URL: &str = "https://github.com/ethereum-optimism/optimism";
const OPT_TAG: &str = "op-node/v1.3.1";
const OPT_FIXES: &[&str] = &["2e57472890f9fea39cde72537935393b068d3e0f", "5252c82f607af81f6cb741a370425eaf26280892"];
const GETH_URL: &str = "https://github.com/ethereum-optimism/op-geth.git";
const GETH_TAG: &str = "v1.101315.3";

// https://github.com/ethereum-optimism/optimism/archive/refs/tags/op-node/v1.3.1.zip

pub async fn download<P: AsRef<Path>>(
    source: &Source,
) -> Result<(),   Box<dyn Error>> {

    utils::git::download_release(
        &source.release_tag,
        &"src/op_node",
    ).await?;


    Ok(())
}
