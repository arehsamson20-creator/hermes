use soroban_sdk::{Address, Env, String};

const ZERO_ACCOUNT_ADDRESS: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";
const ZERO_CONTRACT_ADDRESS: &str = "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4";

pub(crate) fn is_valid_admin_address(env: &Env, admin: &Address) -> bool {
    let zero_account = Address::from_string(&String::from_str(env, ZERO_ACCOUNT_ADDRESS));
    let zero_contract = Address::from_string(&String::from_str(env, ZERO_CONTRACT_ADDRESS));
    admin != &zero_account && admin != &zero_contract
}
