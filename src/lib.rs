pub mod account;
pub mod error;
pub mod grpc_cli;

pub const DERIVATION_PATH: &str = "m/44'/118'/0'/0/";
pub const COSMOS_GRPC: &str = "https://cosmos-grpc.publicnode.com:443";
// or "https://cosmoshub.grpc.kjnodes.com:443";

// we should randomly choose one validator, this is cosmostation
pub const VALIDATOR_ADDRESS: &str = "cosmosvaloper1jst8q8flpn94u9uvkpae8mrkk3a5pjhxx529z2";
pub const CHAIN_ID: &str = "cosmoshub-4";
pub const DENOM: &str = "uatom";

// stake
pub const MINIMUM_ATOM_AMOUNT: u128 = 15_000_000;
pub const MINIMUM_ATOM_LEFT: u128 = 500_000;
pub const MAXIMUM_ATOM_LEFT: u128 = 1_000_000;

pub const TEST_MNEMONIC_CODE: &str =
    "praise hurt canyon trade tilt danger sketch echo muffin grace pottery fringe";
