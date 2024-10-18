use std::path::Path;
use std::process::Command;
use crate::opstack::network::NetworkConfig;

pub fn build<P: AsRef<Path>>(source: &P) -> Result<(), String> {
    let build_out = Command::new("make")
        .arg("build-contracts")
        .current_dir(source)
        .output()
        .expect("Failed to execute build command");
    if !build_out.status.success() {
        let error_message = String::from_utf8_lossy(&build_out.stderr);
        return Err(format!("Error building source: {}", error_message));
    }
    
    Ok(())
}

pub fn deploy<P: AsRef<Path>>(source: &P, cfg: &NetworkConfig) -> Result<(), String> {
    println!("Deploying contracts...");

    // write config where script needs to find it, maybe inside deployments folder where artifacts will also be...

    // run deploy script

//     DEPLOYMENT_OUTFILE=deployments/artifact.json \
// DEPLOY_CONFIG_PATH=<PATH_TO_MY_DEPLOY_CONFIG> \
//   forge script scripts/deploy/Deploy.s.sol:Deploy \
//   --broadcast --private-key $PRIVATE_KEY \
//   --rpc-url $ETH_RPC_URL
// ```

// The `IMPL_SALT` env var can be used to set the `create2` salt for deploying the implementation
// contracts.
    
    Ok(())
}
