use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

//eg: val: foo  then error will be formatted as "Custom Error val: Foo"
    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
