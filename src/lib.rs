#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Address, Env, String, symbol_short, Vec};

#[contract]
pub struct PaymentContract;

#[contractimpl]
impl PaymentContract {
    pub fn send_payment(env: Env, from: Address, to: Address, amount: i128, message: String) -> Vec<String> {
        // Require authorization from the sender
        from.require_auth();

        // Store the message in contract storage
        env.storage().instance().set(&symbol_short!("msg"), &message);

        // Log the payment details
        env.events().publish(("payment", from, to), amount);

        // Return success status
        vec![&env, String::from_str(&env, "success")]
    }

    pub fn get_message(env: Env) -> String {
        env.storage().instance().get(&symbol_short!("msg")).unwrap_or(String::from_str(&env, "No message"))
    }
}