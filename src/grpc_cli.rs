use std::str::FromStr;

use cosmos_sdk_proto::cosmos::{
    auth::v1beta1::{
        query_client::QueryClient as AccountQueryClient, BaseAccount, QueryAccountRequest,
    },
    bank::v1beta1::{query_client::QueryClient as BalanceQueryClient, QueryBalanceRequest},
    tx::v1beta1::{service_client::ServiceClient, BroadcastMode, BroadcastTxRequest},
};
use cosmrs::{
    staking::MsgDelegate,
    tendermint::chain::id::Id,
    tx::{self, Fee, Msg, SignDoc, SignerInfo},
    AccountId, Coin, Denom,
};
use prost::Message;
use rand::Rng;

use crate::account::Account;
use crate::error::MyResult;
use crate::{CHAIN_ID, DENOM, VALIDATOR_ADDRESS};

#[allow(dead_code)]
pub struct GRPCCli {
    grpc_uri: String,
}

#[allow(dead_code)]
impl GRPCCli {
    pub fn new(grpc_uri: String) -> Self {
        Self { grpc_uri }
    }

    async fn get_account(&self, account_id: &AccountId) -> MyResult<BaseAccount> {
        let mut qc = AccountQueryClient::connect(self.grpc_uri.clone()).await?;

        let res = qc
            .account(QueryAccountRequest {
                address: account_id.to_string(),
            })
            .await?
            .into_inner()
            .account
            .unwrap();

        let acc = BaseAccount::decode(res.value.as_slice()).unwrap();
        println!("acc: {:?}", acc);

        Ok(acc)
    }

    pub async fn get_sequence_number(&self, account_id: &AccountId) -> MyResult<u64> {
        Ok(match self.get_account(account_id).await {
            Ok(account) => account.sequence,
            Err(_) => 0u64,
        })
    }

    async fn get_account_number(&self, account_id: &AccountId) -> MyResult<u64> {
        Ok(self.get_account(account_id).await?.account_number)
    }

    pub async fn get_balance(&self, account_id: &AccountId) -> MyResult<u128> {
        let mut qc = BalanceQueryClient::connect(self.grpc_uri.clone()).await?;

        let res = qc
            .balance(QueryBalanceRequest {
                address: account_id.to_string(),
                denom: DENOM.to_string(),
            })
            .await?
            .into_inner()
            .balance
            .unwrap();

        println!("coin: {:?}", res.amount);
        Ok(res.amount.parse::<u128>().unwrap_or(0))
    }

    pub async fn stake(&self, account: &Account, staking_amount: &Coin) -> MyResult<bool> {
        let delegation_msg = MsgDelegate {
            delegator_address: account.account_id()?,
            validator_address: AccountId::from_str(VALIDATOR_ADDRESS)?,
            amount: staking_amount.clone(),
        }
        .to_any()?;

        let gas = 1_000_000u64;
        let mut rng = rand::thread_rng();
        let fee = Fee::from_amount_and_gas(
            Coin {
                denom: Denom::from_str(DENOM).unwrap(),
                amount: rng.gen_range(8000u128..8450u128),
            },
            gas,
        );

        let tx_body = tx::BodyBuilder::new().msg(delegation_msg).finish();
        let acc = self.get_account(&account.account_id()?).await?;

        let auth_info =
            SignerInfo::single_direct(Some(account.public_key()), acc.sequence).auth_info(fee);

        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &Id::from_str(CHAIN_ID)?,
            acc.account_number,
        )?;

        let tx_raw = sign_doc.sign(account.private_key()).unwrap();

        let mut client = ServiceClient::connect(self.grpc_uri.clone()).await?;
        let broadcast_request = BroadcastTxRequest {
            tx_bytes: tx_raw.to_bytes()?,
            mode: BroadcastMode::Sync as i32,
        };

        let response = client
            .broadcast_tx(broadcast_request)
            .await?
            .into_inner()
            .tx_response
            .unwrap();

        println!("broadcast_tx result: {:?}", response);
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmrs::{Coin, Denom};
    use rand::Rng;

    use crate::account::AccountGenerator;
    use crate::error::MyResult;
    use crate::grpc_cli::GRPCCli;
    use crate::{
        COSMOS_GRPC, DENOM, MAXIMUM_ATOM_LEFT, MINIMUM_ATOM_AMOUNT, MINIMUM_ATOM_LEFT,
        TEST_MNEMONIC_CODE,
    };

    #[tokio::test]
    async fn test_get_sequence_number() -> MyResult<()> {
        let ag = AccountGenerator::new(TEST_MNEMONIC_CODE)?;
        for index in 0..10 {
            let account = ag.get_account_from_index(index)?;

            let cli = GRPCCli::new(COSMOS_GRPC.to_string());
            let res = cli.get_sequence_number(&account.account_id()?).await?;

            let balance = cli.get_balance(&account.account_id()?).await?;
            println!("balance: {:?}", balance);
            if balance <= MINIMUM_ATOM_AMOUNT {
                println!("...");
                return Ok(());
            }

            let mut rng = rand::thread_rng();
            let left = rng.gen_range(MINIMUM_ATOM_LEFT..MAXIMUM_ATOM_LEFT);
            let left = left - left % 10_000;

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
}
