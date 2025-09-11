// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use bincode::{deserialize, serialize};
use eyre::Result;
use solana_sdk::{signature::Keypair, transaction::Transaction};

use darklake_sdk_off_chain::{
    self as sdk, SendSignedTransactionAndCheckStatusRequest, SendSignedTransactionResponse,
    TradeStatus,
};
use tokio::sync::mpsc;
use tracing::*;
use tracing_subscriber;

use base64::prelude::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::hash::Hash;
use std::io::ErrorKind;

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
        73, 119, 63, 0, 7, 178, 225, 187, 108, 179, 236, 246, 77, 91, 48, 8, 92, 241, 232, 101,
        215, 54, 149, 16, 178, 166, 236, 71, 237, 30, 204, 226,
    ];
    let keypair = Keypair::new_from_array(secret);
    // Wallet address for demo purposes only.
    let wallet_address = "4bRZeVcTPFTFwdL8hgbEX3gdzQ37v2vr2GSmvR7X4Bsp".to_string();

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
        get_base64_signed_transaction(&response.unsigned_transaction, &keypair).await?;
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

async fn get_recent_blockhash() -> Result<Hash> {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let recent_blockhash: Hash = rpc_client.get_latest_blockhash().await?;
    Ok(recent_blockhash)
}

async fn get_base64_signed_transaction(
    unsigned_transaction: &str,
    keypair: &Keypair,
) -> Result<String> {
    let transaction = deserialize_transaction(&unsigned_transaction)?;
    info!("Deserialized transaction: {:?}", transaction);
    let recent_blockhash: Hash = get_recent_blockhash().await?;
    let signed_tx = sign_transaction(transaction, &[&keypair], recent_blockhash)?;
    info!("Signed transaction: {:?}", signed_tx);
    let signed_tx_base64 = encode_transaction_to_base64(&signed_tx)?;
    Ok(signed_tx_base64)
}

fn deserialize_transaction(base64_str: &str) -> Result<Transaction> {
    // Decode the Base64 string into a byte vector.
    let decoded_bytes = BASE64_STANDARD
        .decode(base64_str)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;

    // Use `bincode::deserialize` directly on the byte slice.
    let transaction = deserialize(&decoded_bytes)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

    Ok(transaction)
}

fn sign_transaction(
    mut transaction: Transaction,
    signers: &[&Keypair],
    recent_blockhash: Hash,
) -> Result<Transaction> {
    transaction.sign(signers, recent_blockhash);
    Ok(transaction)
}

fn encode_transaction_to_base64(transaction: &Transaction) -> Result<String> {
    // Use `bincode::serialize` to convert the transaction to a byte vector.
    let encoded_bytes = serialize(transaction)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

    // Encode the byte vector to a Base64 string.
    let base64_string = BASE64_STANDARD.encode(&encoded_bytes);

    Ok(base64_string)
}
