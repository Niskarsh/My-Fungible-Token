
pub mod ft_core;
pub mod metadata;
pub mod storage;
pub mod internal;

// pub mod my_ft {
    
pub use metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC, ICON_BASE_64_ENCODED};
use near_sdk::{json_types::U128, near, store::LookupMap, AccountId, PanicOnDefault};
use near_primitives::types::Balance;
// pub crate::Contract;
#[near(contract_state)]
#[derive(Debug, PanicOnDefault)]
pub(crate) struct Contract {
    accounts: LookupMap<AccountId, Balance>,
    total_supply: Balance,
    metadata: Option<FungibleTokenMetadata>,
}

#[near]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        Self {
            accounts: LookupMap::new(b"a"),
            total_supply: Balance::from(total_supply),
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
                icon: Some(ICON_BASE_64_ENCODED.to_string()),
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
