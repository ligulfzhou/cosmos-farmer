use crate::{error::MyResult, DERIVATION_PATH};
use bip39::{Language, Mnemonic};
use cosmrs::crypto::PublicKey;
use cosmrs::{bip32::DerivationPath, crypto::secp256k1::SigningKey, AccountId};

pub struct Account(SigningKey);

impl Account {
    pub fn private_key(&self) -> &SigningKey {
        &self.0
    }

    pub fn public_key(&self) -> PublicKey {
        self.0.public_key()
    }

    pub fn account_id(&self) -> MyResult<AccountId> {
        Ok(self.0.public_key().account_id("cosmos")?)
    }

    pub fn address(&self) -> MyResult<String> {
        Ok(self.account_id()?.to_string())
    }
}

pub struct AccountGenerator<'a> {
    mnemonic_code: &'a str,
    seed: [u8; 64],
}

impl<'a> AccountGenerator<'a> {
    pub fn new(mnemonic_code: &'a str) -> MyResult<Self> {
        let mnemonic = Mnemonic::parse_in_normalized(Language::English, mnemonic_code)?;
        let seed = mnemonic.to_seed("");

        Ok(Self {
            mnemonic_code,
            seed,
        })
    }

    pub fn get_account_from_index(&self, index: u32) -> MyResult<Account> {
        let derivation_path = format!("{}{}", DERIVATION_PATH, index).parse::<DerivationPath>()?;
        let signing_key = SigningKey::derive_from_path(self.seed, &derivation_path)?;
        Ok(Account(signing_key))
    }

    pub fn mnemonic_code(&self) -> &str {
        self.mnemonic_code
    }

    pub fn seed(&self) -> [u8; 64] {
        self.seed
    }
}

#[cfg(test)]
mod tests {
    use crate::account::AccountGenerator;
    use crate::error::MyResult;
    use crate::TEST_MNEMONIC_CODE;
    use cosmrs::AccountId;
    use std::str::FromStr;

    #[test]
    fn test_gen_account() -> MyResult<()> {
        let pairs = [
            (0, "cosmos1698et8zhydk8clqpcvqkufht6pmefk2v6n3kjz"),
            (1, "cosmos1n9he59hvh53p7tzw3ck2lv2nnht9dxe59c8dlt"),
            (2, "cosmos19fchj4haghqk2s2nw0cmtvd7zrkua2y2h98eg5"),
            (3, "cosmos1xzcrz27p8lt9v9jgazrg7s5x95ww0wsldwfrmh"),
        ];
        let ag = AccountGenerator::new(TEST_MNEMONIC_CODE)?;
        for pair in pairs {
            let account = ag.get_account_from_index(pair.0)?;
            let account_id = account.account_id()?;

            assert_eq!(account_id, AccountId::from_str(pair.1)?);
        }

        Ok(())
    }
}
