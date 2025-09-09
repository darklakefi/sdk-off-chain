pub mod grpc_client;

pub enum ClientType {
    Grpc,
}

pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
