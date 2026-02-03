use crate::client::BlindPay;
use crate::error::Result;
use crate::types::Network;
use serde::{Deserialize, Serialize};

pub mod blockchain;
pub mod offramp;

pub struct WalletsResources {
    client: BlindPay,
}

impl WalletsResources {
    pub(crate) fn new(client: BlindPay) -> Self {
        Self { client }
    }

    pub fn blockchain(&self) -> blockchain::BlockchainWalletsResource {
        blockchain::BlockchainWalletsResource::new(self.client.clone())
    }

    pub fn offramp(&self) -> offramp::OfframpWalletsResource {
        offramp::OfframpWalletsResource::new(self.client.clone())
    }
}
