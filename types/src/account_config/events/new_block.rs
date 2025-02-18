// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::account_address::AccountAddress;
use anyhow::Result;
use move_deps::move_core_types::{ident_str, identifier::IdentStr, move_resource::MoveStructType};
use serde::{Deserialize, Serialize};

/// Struct that represents a NewBlockEvent.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewBlockEvent {
    epoch: u64,
    round: u64,
    previous_block_votes: Vec<bool>,
    proposer: AccountAddress,
    failed_proposer_indices: Vec<u64>,
    timestamp: u64,
}

impl NewBlockEvent {
    pub fn round(&self) -> u64 {
        self.round
    }

    pub fn proposer(&self) -> AccountAddress {
        self.proposer
    }

    pub fn proposed_time(&self) -> u64 {
        self.timestamp
    }

    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        bcs::from_bytes(bytes).map_err(Into::into)
    }

    #[cfg(any(test, feature = "fuzzing"))]
    pub fn new(
        epoch: u64,
        round: u64,
        previous_block_votes: Vec<bool>,
        proposer: AccountAddress,
        failed_proposer_indices: Vec<u64>,
        timestamp: u64,
    ) -> Self {
        Self {
            epoch,
            round,
            previous_block_votes,
            proposer,
            failed_proposer_indices,
            timestamp,
        }
    }
}

impl MoveStructType for NewBlockEvent {
    const MODULE_NAME: &'static IdentStr = ident_str!("Block");
    const STRUCT_NAME: &'static IdentStr = ident_str!("NewBlockEvent");
}
