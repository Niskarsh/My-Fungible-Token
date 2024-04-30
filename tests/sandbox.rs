use std::error::Error;
use near_sdk::serde_json::json;
use near_gas::NearGas;

#[path ="../src/metadata.rs"]
mod metadata;
mod common;
use metadata::{ FT_METADATA_SPEC, FungibleTokenMetadata, ICON_BASE_64_ENCODED };
// use testing::my_ft::metadata::FT_METADATA_SPEC;
#[tokio::test]
async fn test_default_metadata_for_ft_contract() -> Result<(), Box<dyn Error>> {
    let (accounts, contract) = common::prepare_dev_env().await?;
    // Default Contract Init
    let _ = contract
        .call("new_default_meta")
        .args_json(json!({
            "owner_id": contract.id().clone().to_string(),
            "total_supply": "0"
        }))
        .gas(NearGas::from_tgas(300))
        .transact()
        .await?;
    // println!("{:?}", outcome);

    let default_metadata = accounts[0]
        .view(contract.id(), "ft_metadata")
        .args_json(json!({}))
        .await?
        .json::<FungibleTokenMetadata>()?;

    assert_eq!(
        default_metadata,
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: "My Fungible Token".to_string(),
            symbol: "myFT".to_string(),
            decimals: 24,
            icon: Some(ICON_BASE_64_ENCODED.to_string()),
            reference: None,
            reference_hash: None,
        }
    );
    Ok(())
}
