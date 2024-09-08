#![no_std]

use core::panic;

use soroban_sdk::{
contract, contractimpl, contracttype, token, unwrap::UnwrapOptimized, Address, Env
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer,
}

#[derive(Clone)]
#[contracttype]
pub struct Offer{
    pub seller: Address,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_price:u32,
    pub buy_price:u32,
}

#[contract]
pub struct SingleOffer;

#[contractimpl]
impl SingleOffer{
    pub fn create(e:Env, seller:Address, sell_token:Address, buy_token:Address, sell_price:u32, buy_price:u32){
        if e.storage().instance().has(&DataKey::Offer){
            panic!("Offer is already created");
        }
        if buy_price == 0 || sell_price == 0{
            panic!("Zero price is not allowed")
        }
        seller.require_auth();
        write_offer(
            &e,
            &Offer{
                seller,
                sell_token,
                buy_token,
                sell_price,
                buy_price
            },
        );
    };
    pub fn trade (e:Env, buyer:Address, buy_token_amount:i128, min_sell_token_amount:i128){
        buyer.require_auth();
        let offer = load_offer(&e);
        let sell_token_client = token::Client::new(&e, &offer.sell_token);
        let buy_token_client = token::Client::new(&e, &offer.buy_token);

        let sell_token_amount = buy_token_amount.checked_mul(offer.sell_price as i128).unwrap_optimized()/ offer.buy_price as i128;

        if sell_token_amount < min_sell_token_amount{
            panic!("price is too low");
        }

        let contract = e.current_contract_address();

        buy_token_client.transfer(&buyer, &contract, &buy_token_amount);
        sell_token_client.transfer(&contract, &buyer, &sell_token_amount);
        buy_token_client.transfer(&contract, &offer.seller, &buy_token_amount);
    }
}