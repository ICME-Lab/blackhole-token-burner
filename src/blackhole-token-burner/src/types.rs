use candid::{CandidType, Nat, Principal};
use ic_cdk::call::CandidDecodeFailed;
use serde::{Deserialize, Serialize};

pub type Tokens = Nat;
pub type BlockIndex = Nat;

#[derive(CandidType, Deserialize, Debug, Serialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

#[derive(CandidType, Deserialize, Debug, Serialize)]
pub struct Subaccount([u8; 32]);

#[derive(CandidType, Deserialize, Debug, Serialize)]
pub struct TransferArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub amount: Tokens,
    pub fee: Option<Tokens>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub enum TransferError {
    BadFee(BadFee),
    BadBurn(BadBurn),
    InsufficientFunds(InsufficientFunds),
    TooOld,
    CreatedInFuture(CreatedInFuture),
    TemporarilyUnavailable,
    Duplicate(Duplicate),
    GenericError(GenericError),
}

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub struct BadFee {
    pub expected_fee: Tokens,
}

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub struct BadBurn {
    pub min_burn_amount: Tokens,
}

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub struct InsufficientFunds {
    pub balance: Tokens,
}

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub struct InsufficientAllowance {
    pub allowance: Tokens,
}

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub struct CreatedInFuture {
    pub ledger_time: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub struct Duplicate {
    pub duplicate_of: BlockIndex,
}

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub struct GenericError {
    pub error_code: Nat,
    pub message: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, Serialize)]
pub enum BurnError {
    TransferError(TransferError),
    CallReject(String),
}
