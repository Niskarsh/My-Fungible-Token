use near_sdk::near;
use near_sdk::json_types::Base64VecU8;

#[near(serializers = [borsh, json])]
pub struct FungibleTokenMetadata {
    spec: String,
    name: String,
    symbol: String,
    decimals: u8,
    icon: Option<String>,
    reference: Option<String>,
    reference_hash: Option<Base64VecU8>,
}