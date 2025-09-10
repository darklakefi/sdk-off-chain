// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use eyre::Result;
use solana_sdk::signature::{Keypair, Signer};

use sdk_off_chain as sdk;
use tracing::*;
use tracing_subscriber;

use base64::prelude::*;
use bincode::config;
use bincode::serde::decode_from_slice;
use solana_sdk::transaction::VersionedTransaction;
use solana_sdk::transport::TransportError;
use std::io::{Cursor, ErrorKind};

/// Show how to run a swap transaction using Darklake DEX.
///
/// This example shows how to run a swap transaction using Darklake DEX.
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = sdk::Config::builder()
        .network(sdk::Network::Devnet)
        .url("http://localhost:50051")?
        .is_final_url(true)
        .build()?;

    let mut client = sdk::Client::new(config).await?;

    // Generate a random wallet address
    let keypair = Keypair::new();
    let wallet_address = keypair.pubkey().to_string();

    let request = sdk::CreateUnsignedTransactionRequest::builder()
        .user_address(&wallet_address)
        .token_mint_x("DdLxrGFs2sKYbbqVk76eVx9268ASUdTMAhrsqphqDuX")
        .token_mint_y("So11111111111111111111111111111111111111112")
        .amount_in(1000000000000000000)
        .min_out(900000000000000000)
        .build();

    let response = client.create_unsigned_transaction(request).await?;

    info!("Unsigned transaction: {:?}", response.unsigned_transaction);
    // response.unsigned_transaction should be sent to the wallet for the signing.
    // This code simulate the signed process
    let transaction = deserialize_transaction(&response.unsigned_transaction)?;
    info!("Deserialized transaction: {:?}", transaction);
    let signed_tx = sign_transaction(transaction, &[&keypair])?;
    info!("Signed transaction: {:?}", signed_tx);
    let signed_tx_base64 = encode_transaction_to_base64(&signed_tx)?;
    // then the signed transaction should be sent to the Darklake DEX for the execution.

    info!("Signed transaction base64: {:?}", signed_tx_base64);
    let request = sdk::SendSignedTransactionRequest::builder()
        .signed_transaction(&signed_tx_base64)
        .trade_id(&response.trade_id)
        .build();

    let response = client.send_signed_transaction(request).await?;

    info!("Signed transaction response: {:?}", response);

    Ok(())
}

fn deserialize_transaction(base64_str: &str) -> Result<VersionedTransaction> {
    let decoded_bytes = BASE64_STANDARD
        .decode(base64_str)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;

    let (transaction, _): (VersionedTransaction, _) =
        decode_from_slice(&decoded_bytes, config::standard())
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

    Ok(transaction)
}

fn sign_transaction(
    transaction: VersionedTransaction,
    signers: &[&Keypair],
) -> Result<VersionedTransaction> {
    // The `try_new` method is the correct way to create a signed VersionedTransaction.
    // It takes a message and a slice of signers.
    let signed_tx = VersionedTransaction::try_new(transaction.message, signers).map_err(|e| {
        eprintln!("Failed to sign transaction: {:?}", e);
        std::io::Error::new(ErrorKind::Other, "Failed to sign transaction")
    })?;

    Ok(signed_tx)
}

fn encode_transaction_to_base64(transaction: &VersionedTransaction) -> Result<String> {
    // 1. Serialize the VersionedTransaction's message to a byte vector.
    // The serialize method returns a Vec<u8>, so we assign it directly.
    let encoded_bytes = transaction.message.serialize();

    // 2. Encode the byte vector to a Base64 string.
    let base64_string = BASE64_STANDARD.encode(&encoded_bytes);

    Ok(base64_string)
}
