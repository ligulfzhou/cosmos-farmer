use cosmos::account::AccountGenerator;
use cosmos::error::MyResult;
use cosmos::grpc_cli::GRPCCli;
use cosmos::{
    COSMOS_GRPC, DENOM, MAXIMUM_ATOM_LEFT, MINIMUM_ATOM_AMOUNT, MINIMUM_ATOM_LEFT,
    TEST_MNEMONIC_CODE,
};
use cosmrs::{Coin, Denom};
use rand::Rng;
use std::str::FromStr;

#[tokio::main]
async fn main() -> MyResult<()> {
    let ag = AccountGenerator::new(TEST_MNEMONIC_CODE)?;
    let cli = GRPCCli::new(COSMOS_GRPC.to_string());

    for index in 0..10 {
        let account = ag.get_account_from_index(index)?;
        let balance = cli.get_balance(&account.account_id()?).await?;
        println!("balance: {:?}", balance);
        if balance <= MINIMUM_ATOM_AMOUNT {
            println!("...");
            return Ok(());
        }

        let left = {
            let mut rng = rand::thread_rng();
            let ran = rng.gen_range(MINIMUM_ATOM_LEFT..MAXIMUM_ATOM_LEFT);
            ran - ran % 10_000
        };

        println!(
            "balance: {}, left: {}, amount: {}",
            balance,
            left,
            balance - left
        );

        cli.stake(
            &account,
            &Coin {
                denom: Denom::from_str(DENOM)?,
                amount: balance - left,
            },
        )
        .await?;
    }

    Ok(())
}
