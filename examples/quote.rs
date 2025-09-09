// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use eyre::Result;

use sdk_off_chain as sdk;

/// Show how to get a quote from the Darklake DEX.
///
/// This example shows how to get a quote from the Darklake DEX.
#[tokio::main]
async fn main() -> Result<()> {
    let config = sdk::Config::builder()
        .network(sdk::Network::Mainnet)
        .url("https://api.darklake.fi")
        .build()?;

    let client = sdk::Client::new(config);

    let quote = client
        .quote(sdk::QuoteRequest {
            token_mint_x: "9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump".to_string(),
            token_mint_y: "So11111111111111111111111111111111111111112".to_string(),
            amount_in: 1000000000000000000,
            is_swap_x_to_y: true,
        })
        .await?;

    println!("Quote: {:?}", quote);

    Ok(())
}
