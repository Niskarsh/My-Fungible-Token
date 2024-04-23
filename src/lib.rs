use std::ops::Deref;

use metadata::FungibleTokenMetadataProvider;
use near_sdk::{json_types::U128, near, AccountId};
// use near_sdk::collections::LazyOption;

pub mod ft_core;
pub mod metadata;
pub mod storage;

use crate::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};

#[near(contract_state)]
pub struct Contract {
    // owner_id: AccountId,
    // total_supply: U128,
    metadata: Option<FungibleTokenMetadata>,
}

#[near]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        Self {
            metadata: Some(metadata),
        }
    }

    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "My Fungible Token".to_string(),
                symbol: "myFT".to_string(),
                decimals: 24,
                icon: None,
                reference: None,
                reference_hash: None,
            },
        )
    }
}

impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.as_ref().unwrap().clone()
    }
}