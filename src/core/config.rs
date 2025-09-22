// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::client::ClientType;
use eyre::{Result, eyre};
use std::fmt;
use url::Url;

/// Represents the network to use for the client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Mainnet,
    Devnet,
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Network::Mainnet => write!(f, "mainnet"),
            Network::Devnet => write!(f, "devnet"),
        }
    }
}

/// Configuration for the client.
///
/// This struct holds all the necessary settings to create and configure
/// a client for the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct Config {
    /// The blockchain network to connect to (e.g., Mainnet, Devnet).
    ///
    /// This is used to determine the correct host for the gRPC service.
    pub network: Network,
    /// The base URL of the gRPC service.
    ///
    /// The final URL will be constructed by prepending the `network` to this URL.
    /// This URL will be provided by Darklake Labs.
    pub url: Url,
    /// The type of client to create (e.g., gRPC).
    pub client_type: ClientType,
    /// A flag to indicate whether the `url` is the final URL to use.
    ///
    /// If `true`, the `network` field will be ignored and the `url` will be
    /// used directly for the connection.
    pub is_final_url: bool,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

/// Builder for the `Config` struct.
///
/// This struct provides a fluent interface for creating and configuring
/// a `Config` instance.
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    /// The blockchain network to connect to (e.g., Mainnet, Devnet).
    ///
    /// This is used to determine the correct host for the gRPC service.
    pub network: Network,
    /// The base URL of the gRPC service.
    ///
    /// The final URL will be constructed by prepending the `network` to this URL.
    pub url: Url,
    /// The type of client to create (e.g., gRPC).
    ///
    /// This is used to determine the correct client to create.
    pub client_type: ClientType,
    /// A flag to indicate whether the `url` is the final URL to use.
    ///
    /// If `true`, the `network` field will be ignored and the `url` will be
    /// used directly for the connection.
    pub is_final_url: bool,
}

impl ConfigBuilder {
    /// Creates a new `ConfigBuilder` instance.
    ///
    /// This is used to create a new `ConfigBuilder` instance.
    pub fn new() -> Self {
        Self {
            network: Network::Mainnet,
            url: Url::parse("https://localhost").unwrap(),
            client_type: ClientType::Grpc,
            is_final_url: false,
        }
    }

    /// Sets the network to use for the client.
    ///
    /// This is used to determine the correct host for the gRPC service.
    ///
    /// # Returns
    ///
    /// Returns the `ConfigBuilder` instance.
    pub fn network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    /// Sets the base URL of the gRPC service.
    ///
    /// This is used to determine the correct host for the gRPC service.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid.
    pub fn url(mut self, url: &str) -> Result<Self> {
        self.url = Url::parse(url).map_err(|e| eyre!("Invalid URL: {}", e))?;
        Ok(self)
    }

    /// Sets the flag to indicate whether the `url` is the final URL to use.
    ///
    /// If `true`, the `network` field will be ignored and the `url` will be
    /// used directly for the connection.
    ///
    /// # Returns
    ///
    /// Returns the `ConfigBuilder` instance.
    pub fn is_final_url(mut self, is_final_url: bool) -> Self {
        self.is_final_url = is_final_url;
        self
    }

    /// Builds the `Config` instance.
    ///
    /// This is used to build the `Config` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid.
    ///
    /// # Returns
    ///
    /// Returns the `Config` instance.
    pub fn build(self) -> Result<Config> {
        let mut dest_url = self.url.clone();
        if !self.is_final_url {
            let new_host = match self.url.host() {
                Some(url::Host::Domain(host)) => {
                    format!("{}.{}", self.network, host)
                }
                Some(url::Host::Ipv4(ip)) => ip.to_string(),
                Some(url::Host::Ipv6(ip)) => ip.to_string(),
                _ => {
                    return Err(eyre!("Invalid host: no host component found"));
                }
            };

            dest_url = Url::parse(&format!("{}://{}", self.url.scheme(), new_host))
                .map_err(|e| eyre!("Invalid URL: {:?}", e))?;

            if let Some(port) = self.url.port() {
                dest_url
                    .set_port(Some(port))
                    .map_err(|e| eyre!("Failed to set port: {:?}", e))?;
            }
        }
        Ok(Config {
            network: self.network,
            url: dest_url,
            client_type: self.client_type,
            is_final_url: self.is_final_url,
        })
    }
}
