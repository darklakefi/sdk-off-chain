// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::io::ErrorKind;

use base64::{Engine, prelude::BASE64_STANDARD};
use bincode::{deserialize, serialize};
use eyre::Result;

use solana_sdk::{signature::Keypair, signer::Signer, transaction::VersionedTransaction};
use tracing::*;

pub async fn create_base64_signed_transaction(
    unsigned_transaction: &str,
    keypair: &Keypair,
) -> Result<String> {
    let transaction = deserialize_versioned_transaction(&unsigned_transaction)?;
    info!("Deserialized transaction: {:?}", transaction);
    let signed_tx = sign_versioned_transaction(transaction, &[&keypair])?;
    info!("Signed transaction: {:?}", signed_tx);
    let signed_tx_base64 = encode_versioned_transaction_to_base64(&signed_tx)?;
    Ok(signed_tx_base64)
}

pub fn deserialize_versioned_transaction(base64_str: &str) -> Result<VersionedTransaction> {
    // Decode the Base64 string into a byte vector.
    let decoded_bytes = BASE64_STANDARD
        .decode(base64_str)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;

    // Use `bincode::deserialize` to handle the versioned transaction.
    // This will correctly parse the version prefix and message.
    let transaction: VersionedTransaction = deserialize(&decoded_bytes)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

    Ok(transaction)
}

pub fn sign_versioned_transaction(
    mut transaction: VersionedTransaction,
    signers: &[&Keypair],
) -> Result<VersionedTransaction> {
    // Get the message from the transaction.
    let message_bytes = transaction.message.serialize();

    // Sign the message with all signers.
    let signatures: Vec<solana_sdk::signature::Signature> = signers
        .iter()
        .map(|signer| signer.sign_message(&message_bytes))
        .collect();

    // Update the signatures on the transaction.
    transaction.signatures = signatures;

    Ok(transaction)
}

pub fn encode_versioned_transaction_to_base64(
    transaction: &VersionedTransaction,
) -> Result<String> {
    let encoded_bytes = serialize(transaction)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

    let base64_string = BASE64_STANDARD.encode(&encoded_bytes);

    Ok(base64_string)
}
