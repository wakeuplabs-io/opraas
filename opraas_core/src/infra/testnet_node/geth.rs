use super::testnet_node::TTestnetNode;
use crate::infra::eth_rpc::{EthRpc, JsonRpc};
use crate::system::execute_command;
use serde_json::json;
use std::{process::Command, thread, time};

const DOCKER_IMAGE: &str = "ethereum/client-go:v1.13.4";
const CONTAINER_NAME: &str = "geth-testnet-node";
const MAX_TIMEOUT: u64 = 30;

pub struct GethTestnetNode {
    eth_rpc: Box<dyn EthRpc>,
}

// implementations ==============================================

impl GethTestnetNode {
    pub fn new() -> Self {
        Self {
            eth_rpc: Box::new(JsonRpc::new()),
        }
    }
}

impl TTestnetNode for GethTestnetNode {
    fn start(&self, chain_id: u32, port: u64) -> Result<(), Box<dyn std::error::Error>> {
        if chain_id != 1337 {
            return Err("Unsupported chain id".into());
        }

        execute_command(Command::new("docker").args(["pull", DOCKER_IMAGE]), false)?;

        execute_command(
            Command::new("docker").args([
                "run",
                "-d",
                "--rm",
                "-p",
                &format!("{}:8545", port),
                "--name",
                CONTAINER_NAME,
                DOCKER_IMAGE,
                "--dev",
                "--http",
                "--http.api=eth,debug",
                &format!("--http.port={}", port),
                "--http.corsdomain=*",
                "--http.vhosts=*",
                "--http.addr=0.0.0.0",
                "--verbosity=4",
                "--gcmode=archive",
                "--dev.gaslimit=30000000",
                "--dev.period=1",
                "--rpc.allow-unprotected-txs",
                "--state.scheme=path",
            ]),
            true,
        )?;

        // Wait for node to start
        let timeout_duration = time::Duration::from_secs(MAX_TIMEOUT);
        let start_time = time::Instant::now();

        // Fetch the chain ID synchronously
        let rpc_url = format!("http://127.0.0.1:{}", port);

        loop {
            if start_time.elapsed() >= timeout_duration {
                return Err(format!(
                    "Timeout reached: Node did not respond within {} seconds.",
                    MAX_TIMEOUT
                )
                .into());
            }

            match self
                .eth_rpc
                .send_rpc_request(&rpc_url, 1, "eth_chainId", vec![])
            {
                Ok(_) => {
                    break; /*  geth is ready */
                }
                Err(_) => { /* unable to connect, retry */ }
            }

            // Wait before retrying
            thread::sleep(time::Duration::from_secs(2));
        }

        let accounts = self
            .eth_rpc
            .send_rpc_request(&rpc_url, 2, "eth_accounts", vec![])?;
        let send_from = accounts
            .get("result")
            .and_then(|v| v.get(0))
            .unwrap()
            .as_str()
            .unwrap();

        // fund dev accounts
        let dev_accounts = [
            "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
            "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC",
            "0x90F79bf6EB2c4f870365E785982E1f101E93b906",
            "0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65"
        ];
        for account in dev_accounts {
            self.eth_rpc.send_rpc_request(
                &rpc_url,
                1,
                "eth_sendTransaction",
                vec![json!( {
                    "from": send_from,
                    "to": account,
                    "value": "0x56bc75e2d63100000",
                })],
            )?;
        }
       
        Ok(())
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = execute_command(Command::new("docker").arg("stop").arg(CONTAINER_NAME), true);
        let _ = execute_command(Command::new("docker").arg("rm").arg(CONTAINER_NAME), true);

        Ok(())
    }
}
