// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

mod common;

use eyre::Result;
use solana_sdk::signature::Keypair;

use darklake_sdk_off_chain::{
    self as sdk, SendSignedTransactionAndCheckStatusRequest, SendSignedTransactionResponse,
    TradeStatus,
};
use tokio::sync::mpsc;
use tracing::*;
use tracing_subscriber;

use crate::common::common::create_base64_signed_transaction;

/// Show how to run a swap transaction using Darklake DEX.
///
/// This example shows how to run a swap transaction using Darklake DEX. Instead of having two calls, one for sending the signed transaction and one for checking the status, this example shows how to do both in one call.
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = sdk::Config::builder()
        .network(sdk::Network::Devnet)
        .url("http://localhost:50051")?
        .is_final_url(true)
        .build()?;

    let mut client = sdk::Client::new(config).await?;

    // Generate keypair from wallet secrets, This secret key is for demo purposes only.
    let secret: [u8; 32] = [
        73, 119, 63, 0, 7, 178, 225, 183, 108, 179, 236, 246, 77, 91, 48, 8, 92, 241, 232, 101,
        215, 54, 149, 16, 178, 166, 238, 71, 237, 30, 204, 226,
    ];
    let keypair = Keypair::new_from_array(secret);
    // Wallet address for demo purposes only.
    let wallet_address = "4bRZeMcTPFTFwdL8hgbEX3gdzQ37v2vr2GSmvR0X4Bsp".to_string();

    let request = sdk::CreateUnsignedTransactionRequest::builder(
        &wallet_address,
        "DdLxrGFs2sKYbbqVk76eVx9268ASUdTMAhrsqphqDuX",
        "HXsKnhXPtGr2mq4uTpxbxyy7ZydYWJwx4zMuYPEDukY",
        1000000,
        100000,
    )
    .build();

    let response = client.create_unsigned_transaction(request).await?;
    let trade_id = response.trade_id;

    // response.unsigned_transaction should be sent to the wallet for the signing.
    // This code simulates the signed process
    let signed_tx_base64 =
        create_base64_signed_transaction(&response.unsigned_transaction, &keypair).await?;
    // then the signed transaction should be sent to the Darklake DEX for the execution.

    let (tx_signed, mut rx_signed) = mpsc::channel::<SendSignedTransactionResponse>(10);
    let (tx_status, mut rx_status) = mpsc::channel::<TradeStatus>(10);

    let request = SendSignedTransactionAndCheckStatusRequest::builder(
        &signed_tx_base64,
        &trade_id,
        tx_signed,
    )
    .tx_status(tx_status)
    .build();

    let listener = tokio::spawn(async move {
        info!("Listening for trade status updates...");
        while let Some(status_update) = rx_status.recv().await {
            info!("Received status update: {:?}", status_update);
        }
    });

    let listener_signed = tokio::spawn(async move {
        info!("Listening for signed transaction response...");
        while let Some(signed_response) = rx_signed.recv().await {
            info!(
                "Received signed transaction response: {:?}",
                signed_response
            );
        }
    });

    let trade_result = client
        .send_signed_transaction_and_check_status(request)
        .await?;

    info!("Trade result: {:?}", trade_result);

    listener.await?;
    listener_signed.await?;
    Ok(())
}
