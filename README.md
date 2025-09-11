# sdk-off-chain

[![GitHub License](https://img.shields.io/github/license/darklakefi/sdk-off-chain)](https://github.com/darklakefi/sdk-off-chain/blob/develop/LICENSE)

The **Darklake SDK for off-chain integrations** is a library that provides a set of tools for off-chain integrations.

Specifically, the [sdk-off-chain](https://github.com/darklakefi/sdk-off-chain) project makes it easy for developers to integrate with the Darklake DEX.

To start, please see below for [installation instructions](https://github.com/darklakefi/sdk-off-chain#installation) and [usage](https://github.com/darklakefi/sdk-off-chain#usage).

This project is part of a wider integration project composed by:
- [sdk-off-chain](https://github.com/darklakefi/sdk-off-chain)
- [sdk-on-chain](https://github.com/darklakefi/sdk-on-chain)

### Installation

Stable releases of the **sdk-off-chain** codebase are published to **crates.io** under the package name [darklake-sdk-off-chain](https://crates.io/crates/darklake-sdk-off-chain), and can be easily installed with cargo.

For general usage, the following steps should be sufficient to get the latest stable version using the [Package Installer for Rust](https://github.com/rust-lang/cargo):

- **\[option 1]** using `cargo add`

    ```bash
    $ cargo add darklake-sdk-off-chain
    ```

- **\[option 2]** specifying it in Cargo.toml file

    
    ```toml
    [dependencies]
    darklake-sdk-off-chain = "~0.2"
    ```
    
To use the bleeding edge instead of the stable version, the dependecies section should be modified like this:

```
[dependencies]
darklake-sdk-off-chain = { git = "https://github.com/darklakefi/sdk-off-chain.git", branch = "develop" }
```

The instructions above assume a Linux-type system. However, the steps should be identical on Windows and MacOS platforms.

See [The Cargo Book](https://doc.rust-lang.org/cargo/index.html) for more details on how to use cargo.

### Usage

To use the SDK, you need to create a client and then use the client to get a quote. To create the client you need a URL we will provide you with.

A very simple starter example, which just outputs a quote:
```rust
  use eyre::Result;
  use darklake_sdk_off_chain as sdk;
  use tracing::*;
  use tracing_subscriber;

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
```

For more examples, please see the [examples](https://github.com/darklakefi/sdk-off-chain/tree/develop/examples) directory.