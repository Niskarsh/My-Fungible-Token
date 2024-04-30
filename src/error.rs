use std::{error::Error, fmt::{Display, Formatter, Result}};

use near_sdk::FunctionError;
use anyhow::anyhow;

#[derive(Debug)]
pub enum MyFtError {
    BalanceOverflow,
}

impl Display for MyFtError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MyFtError::BalanceOverflow => write!(f, "Balance Overflow")
        }
    }
}

impl Error for MyFtError {}

impl AsRef<str> for MyFtError {
    fn as_ref(&self) -> &'static str {
        let static_var: &'static str = stringify!(self.to_string());
        static_var
        // self.to_string().to_owned().as_str()
    }
}