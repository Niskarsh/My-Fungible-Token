use near_sdk::near;
use near_sdk::json_types::Base64VecU8;

#[near(serializers = [borsh, json])]
pub struct FungibleTokenMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

/// The specific version of the standard we're using
pub const FT_METADATA_SPEC: &str = "ft-1.0.0";