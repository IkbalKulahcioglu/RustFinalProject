#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Address, Env, String, Symbol, Vec};

#[contract]
pub struct PaymentContract;

#[contractimpl]
impl PaymentContract {
    pub fn send_payment(env: Env, to: Address, amount: i128, message: String) -> Vec<Symbol> {
        // Get the sender's address
        let sender = env.current_contract_address();
        
        // Transfer XLM from sender to recipient
        sender.require_auth();
        env.host().transfer(&sender, &to, &amount);

        // Store the message in contract storage
        env.storage().instance().set(&Symbol::short("msg"), message);

        // Return success status
        vec![&env, Symbol::short("success")]
    }

    pub fn get_message(env: Env) -> String {
        env.storage().instance().get(&Symbol::short("msg")).unwrap_or(String::from_str(&env, "No message"))
    }
}