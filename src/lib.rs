// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

//! The **Darklake SDK for off-chain integrations** is a library that provides a set of tools for off-chain integrations.
//!
//! Specifically, the [sdk-off-chain](https://github.com/darklakefi/sdk-off-chain) project makes it easy for developers to integrate with the Darklake DEX.
//!
//! To start, please see below for [installation instructions](https://github.com/darklakefi/sdk-off-chain#installation) and [usage](https://github.com/darklakefi/sdk-off-chain#usage).
//!
//! This project is part of a wider integration project composed by:
//! - [sdk-off-chain](https://github.com/darklakefi/sdk-off-chain)
//! - [sdk-on-chain](https://github.com/darklakefi/sdk-on-chain)
//!
//! ### Usage
//!
//! To use the SDK, you need to create a client and then use the client to get a quote. To create the client you need a URL we will provide you with.
//!
//!   A very simple starter example, which just outputs a quote:
//!   ```rust
//!   use eyre::Result;
//!   use sdk_off_chain as sdk;
//!   use tracing::*;
//!   use tracing_subscriber;

//!   #[tokio::main]
//!   async fn main() -> Result<()> {
//!   tracing_subscriber::fmt::init();

//!   let config = sdk::Config::builder()
//!     .network(sdk::Network::Mainnet)
//!     .url("http://localhost:50051")?
//!     .is_final_url(true)
//!     .build()?;

//!   let mut client = sdk::Client::new(config).await?;

//!   let quote = client
//!     .get_quote(sdk::QuoteRequest {
//!         token_mint_x: "DdLxrGFs2sKYbbqVk76eVx9268ASUdTMAhrsqphqDuX".to_string(),
//!         token_mint_y: "So11111111111111111111111111111111111111112".to_string(),
//!         amount_in: 1000000000000000000,
//!         is_swap_x_to_y: true,
//!     })
//!   .await?;

//!   info!("Quote: {:?}", quote);

//!   Ok(())
//!   }
//!   ```
pub mod integrations_pb {
    tonic::include_proto!("darklake.v1");
}

pub mod client;
pub mod core;
pub mod models;

pub use client::Client;
pub use core::config::{Config, Network};
pub use models::{
    CreateUnsignedTransactionRequest, CreateUnsignedTransactionResponse, QuoteRequest,
    QuoteResponse, SendSignedTransactionRequest, SendSignedTransactionResponse,
};
