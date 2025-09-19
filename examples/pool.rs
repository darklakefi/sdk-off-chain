// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

mod common;

use eyre::Result;

use darklake_sdk_off_chain as sdk;
use solana_sdk::signature::Keypair;
use tracing::*;
use tracing_subscriber;

use crate::common::common::create_base64_signed_transaction;

/// Show how to init a pool from the Darklake DEX.
///
/// This example shows how to init a pool from the Darklake DEX.
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
        73, 119, 63, 0, 6, 178, 225, 187, 110, 179, 236, 246, 77, 91, 48, 8, 92, 241, 232, 101,
        215, 54, 149, 16, 172, 166, 236, 73, 237, 30, 204, 226,
    ];
    let keypair = Keypair::new_from_array(secret);
    // Wallet address for demo purposes only.
    let wallet_address = "4bRZeVcTPFTFwcL8hgbEX3udzQ37v5vr2GSmvR7X4Asp".to_string();

    let pool = client
        .init_pool(sdk::InitPoolRequest {
            token_mint_x: "DdLxrGFs2sKYbbqVk76eVx9268ASUdTMAhrsqphqDuX".to_string(),
            token_mint_y: "HXsKnhXPtGr2mq4uTpxbxyy7ZydYWJwx4zMuYPEDukY".to_string(),
            user_address: wallet_address.clone(),
            amount_x: 20,
            amount_y: 20,
            ref_code: "test".to_string(),
            label: "test".to_string(),
        })
        .await?;

    info!("Init pool: {:?}", pool);

    // response.unsigned_transaction should be sent to the wallet for the signing and execution.
    // This code simulates the signed process
    let _ = create_base64_signed_transaction(&pool.unsigned_transaction, &keypair).await?;

    Ok(())
}
