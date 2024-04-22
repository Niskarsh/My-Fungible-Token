use near_sdk::{json_types::U128, near, AccountId};
// use near_sdk::collections::LazyOption;

pub mod ft_core;
pub mod metadata;
pub mod storage;

use crate::metadata::FungibleTokenMetadata;


#[near(contract_state)]
pub struct Contract {
    // owner_id: AccountId,
    // total_supply: U128,
    metadata: Option<FungibleTokenMetadata>
}

#[near]
impl Contract {
    #[init]
    pub fn new (
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata
    ) -> Self{
        Self { metadata: Some(metadata) }
    }
}