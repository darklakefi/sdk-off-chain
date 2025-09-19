// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use eyre::Result;

use darklake_sdk_off_chain as sdk;
use solana_sdk::signature::Keypair;
use tracing::*;
use tracing_subscriber;

/// Show how to get a quote from the Darklake DEX.
///
/// This example shows how to get a quote from the Darklake DEX.
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = sdk::Config::builder()
        .network(sdk::Network::Devnet)
        .url("http://localhost:50051")?
        .is_final_url(true)
        .build()?;

    let mut client = sdk::Client::new(config).await?;

    let secret: [u8; 32] = [
        73, 119, 63, 0, 6, 178, 225, 187, 108, 179, 236, 246, 77, 91, 48, 8, 92, 241, 232, 101,
        215, 54, 149, 16, 172, 166, 236, 71, 237, 30, 204, 226,
    ];
    let keypair = Keypair::new_from_array(secret);
    // Wallet address for demo purposes only.
    let wallet_address = "4bRZeVcTPFTFwcL8hgbEX3gdzQ37v2vr2GSmvR7X4Asp".to_string();

    let add_liquidity = client
        .add_liquidity(sdk::AddLiquidityRequest {
            token_mint_x: "DdLxrGFs2sKYbbqVk76eVx9268ASUdTMAhrsqphqDuX".to_string(),
            token_mint_y: "HXsKnhXPtGr2mq4uTpxbxyy7ZydYWJwx4zMuYPEDukY".to_string(),
            user_address: wallet_address,
            amount_lp: 20,
            max_amount_x: 1000,
            max_amount_y: 1000,
            ref_code: "test".to_string(),
            label: "test".to_string(),
        })
        .await?;

    info!("Add liquidity: {:?}", add_liquidity);

    // response.unsigned_transaction should be sent to the wallet for the signing.
    // This code simulates the signed process
    let signed_tx_base64 =
        get_base64_signed_transaction(&add_liquidity.unsigned_transaction, &keypair).await?;

    Ok(())
}
