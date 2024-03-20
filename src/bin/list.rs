use cosmos::account::AccountGenerator;
use cosmos::error::MyResult;
use cosmos::TEST_MNEMONIC_CODE;

#[tokio::main]
async fn main() -> MyResult<()> {
    let ag = AccountGenerator::new(TEST_MNEMONIC_CODE)?;
    for idx in 0..100 {
        let account = ag.get_account_from_index(idx)?;
        let account_id = account.account_id()?;

        println!("{idx}: {account_id}");
    }

    Ok(())
}
