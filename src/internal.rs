use std::error::Error;
// use Contract;
use crate::{ Contract, ContractExt };
// use self::Contract;
use near_primitives::types::Balance;
use near_sdk::{json_types::U128, near, AccountId};


#[near]
impl Contract {
    /// Fn to deposit initial FT supply to owner
    pub(crate) fn internal_deposit(
        &mut self,
        account_id: &AccountId,
        amount: Balance,
    ) -> Result<(), Box<dyn Error>> {
        let balance = self
            .accounts
            .get(account_id)
            .unwrap_or(&Balance::from(U128::from(0)))
            .clone();

        let new_balance = balance.checked_add(amount);
        match new_balance {
            Some(new_balance) => {
                self.accounts.insert(account_id.clone(), new_balance);
                Ok(())
            },
            None => Err("Balance Overflow".into())
        }
    }
}
