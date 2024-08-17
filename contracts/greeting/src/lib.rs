#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec};
use soroban_sdk::token::TokenClient;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Token
}


#[contract]
pub struct TransferContract;

#[contractimpl]
impl TransferContract {

    pub fn transfer_xlm(env: Env, from: Address, to: Address, amount: i128) -> bool {
        from.require_auth();
        assert!(amount > 0, "amount must be positive");
        let xlm_address_str = String::from_str(
            &env,
            "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
        );
        let xlm_address = Address::from_string(&xlm_address_str);
        env.storage()
            .persistent()
            .set(&DataKey::Token, &xlm_address);
        let token = TokenClient::new(&env, &xlm_address);
        token.transfer(&from, &to, &amount);
        true
    }
} 

mod test;
