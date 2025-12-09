mod types;
use candid::{Nat, Principal};
use types::*;

// "74ncn-fqaaa-aaaaq-aaasa-cai"
const MINTER: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x24, 0x01, 0x01]);

// "73mez-iiaaa-aaaaq-aaasq-cai"
const LEDGER: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x25, 0x01, 0x01]);

#[ic_cdk::query]
fn greet() -> String {
    format!("minter: {}, ledger: {}", MINTER.to_text(), LEDGER.to_text())
}

#[ic_cdk::update]
async fn burn(e8s: u64) -> Result<BlockIndex, BurnError> {
    let arg = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: MINTER,
            subaccount: None,
        },
        amount: Nat::from(e8s),
        fee: None,
        memo: None,
        created_at_time: None,
    };
    let call_result = ic_cdk::call::Call::unbounded_wait(LEDGER, "icrc1_transfer")
        .with_arg(arg)
        .await;

    let transfer_result: Result<BlockIndex, TransferError> = match call_result {
        Ok(response) => response.candid().unwrap(),
        Err(err) => {
            return Err(BurnError::CallReject(format!("{:?}", err)));
        }
    };

    match transfer_result {
        Ok(ok) => Ok(ok),
        Err(err) => Err(BurnError::TransferError(err)),
    }
}

ic_cdk::export_candid!();
