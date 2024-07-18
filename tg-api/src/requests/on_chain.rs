use dotenv::dotenv;
use ethers::{
    prelude::*,
    providers::{Http, Middleware, Provider},
    types::{U256, U64},
};
use std::convert::TryFrom;

/// Type to query on chain info
#[derive(Debug, Clone)]
pub(crate) struct OnChainInfoQuery {
    provider: Provider<Http>,
}

impl OnChainInfoQuery {
    pub(crate) fn new(_chain_id: u64) -> anyhow::Result<Self> {
        dotenv().ok();
        //let provider = match chain_id {
        //    137 => {
        //        let rpc_url = std::env::var("POLYGON_RPC_URL")?;
        //        let provider = Provider::<Http>::try_from(rpc_url)?;
        //        log::info!("Connected to Polygon mainnet");
        //        provider
        //    }
        //    1 => {
        //        let rpc_url = std::env::var("ETH_RPC_URL")?;
        //        let provider = Provider::<Http>::try_from(rpc_url)?;
        //        log::info!("Connected to Ethereum mainnet");
        //        provider
        //    }
        //    _ => return Err(anyhow!("Unsupported chain id: {}", chain_id)),
        let rpc_url = std::env::var("ETH_RPC_URL")?;
        let provider = Provider::<Http>::try_from(rpc_url)?;

        Ok(Self { provider })
    }

    /// Gets the block number and gas fee
    pub(crate) async fn query_info(&self) -> anyhow::Result<(U64, U256)> {
        let block_number = self.provider.get_block_number().await?;

        let gas_price = self.provider.get_gas_price().await?;

        Ok((block_number, gas_price))
    }
}

/// Helper function to query the block number and gas fee from supported networks
pub(crate) async fn get_on_chain_info() -> anyhow::Result<String> {
    let eth_provider = OnChainInfoQuery::new(1).unwrap();
    let (eth_block_number, eth_gas_price) = eth_provider.query_info().await.unwrap();
    let eth_gas_price = eth_gas_price / 1_000_000_000u64;
    let message = format!(
        "*Ethereum*\n*Gas:* {} Gwei  ═  *Block:* {}\n",
        eth_gas_price, eth_block_number
    );
    Ok(message)
}

pub(crate) async fn get_on_chain_info_start() -> anyhow::Result<String> {
    let wallet1 = LocalWallet::new(&mut rand::thread_rng());
    let address1 = wallet1.address();

    let wallet2 = LocalWallet::new(&mut rand::thread_rng());
    let address2 = wallet2.address();

    let wallet3 = LocalWallet::new(&mut rand::thread_rng());
    let address3 = wallet3.address();
    let eth_provider = OnChainInfoQuery::new(1).unwrap();
    let (eth_block_number, eth_gas_price) = eth_provider.query_info().await.unwrap();
    let eth_gas_price = eth_gas_price / 1_000_000_000u64;
    let message = format!(
        "*Ethereum*\n*Gas:* {} Gwei  ═  *Block:* {}\n\n *Wallet 1* {}\n *Wallet2* {}\n*Wallet 3* {}",
        eth_gas_price, eth_block_number,  address1, address2, address3
    );
    Ok(message)
}
