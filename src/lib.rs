
pub mod ft_core;
pub mod metadata;
pub mod storage;

// pub mod my_ft {


pub use metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC};
// pub(in self::test_default_metadata_for_ft_contract) use crate::metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC};

use near_sdk::{json_types::U128, near, AccountId, PanicOnDefault};
#[near(contract_state)]
#[derive(Debug, PanicOnDefault)]
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

#[near]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.as_ref().unwrap().clone()
    }
}
// }
