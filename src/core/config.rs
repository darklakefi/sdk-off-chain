use eyre::{Result, eyre};
use url::Url;

use crate::client::ClientType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Mainnet,
    Devnet,
}

pub struct Config {
    pub network: Network,
    pub url: Url,
    pub client_type: ClientType,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

pub struct ConfigBuilder {
    pub network: Network,
    pub url: Url,
    pub client_type: ClientType,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            network: Network::Mainnet,
            url: Url::parse("https://localhost").unwrap(),
            client_type: ClientType::Grpc,
        }
    }

    pub fn network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    pub fn url(mut self, url: &str) -> Result<Self> {
        self.url = Url::parse(url).map_err(|e| eyre!("Invalid URL: {}", e))?;
        Ok(self)
    }

    pub fn build(self) -> Config {
        Config {
            network: self.network,
            url: self.url,
            client_type: self.client_type,
        }
    }
}
