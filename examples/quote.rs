// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use eyre::Result;

use sdk_off_chain as sdk;
use tracing::*;
use tracing_subscriber;

/// Show how to get a quote from the Darklake DEX.
///
/// This example shows how to get a quote from the Darklake DEX.
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = sdk::Config::builder()
        .network(sdk::Network::Mainnet)
        .url("http://localhost:50051")?
        .is_final_url(true)
        .build()?;

    let mut client = sdk::Client::new(config).await?;

    let quote = client
        .get_quote(sdk::QuoteRequest {
            token_mint_x: "DdLxrGFs2sKYbbqVk76eVx9268ASUdTMAhrsqphqDuX".to_string(),
            token_mint_y: "So11111111111111111111111111111111111111112".to_string(),
            amount_in: 1000000000000000000,
            is_swap_x_to_y: true,
        })
        .await?;

    info!("Quote: {:?}", quote);

    Ok(())
}
