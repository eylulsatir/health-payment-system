#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec, vec, symbol_short};

#[contract]
pub struct HealthPaymentSystem;

#[contractimpl]
impl HealthPaymentSystem {
    /// Function to schedule a payment to a user
    /// Allows scheduling automatic payments to a specified user at regular intervals.
    pub fn schedule_payment(env: Env, to: Address, amount: i128, interval: u32) {
        // Check if the transfer amount is positive
        if amount <= 0 {
            panic!("Transfer amount must be positive");
        }
        // Create a vector to store the payment history (can be used later)
        let mut history = vec![&env];
        history.push_back((to, amount, interval));
        // In a real implementation, you would store this history
    }

    /// Function for one-time payments
    /// Sends the specified amount of XLM to the user.
    pub fn execute_payment(env: Env, to: Address, amount: i128, message: String) {
        // Create a token client for the transfer operation
        let token_id = env.current_contract_address();
        let token = soroban_sdk::token::Client::new(&env, &token_id);
        
        // Check if the amount is positive
        if amount <= 0 {
            panic!("Amount to be sent must be positive");
        }
        
        // Execute the transfer
        token.transfer(&env.current_contract_address(), &to, &amount);
        
        // Send a short message to the user (messages can be stored)
        env.events().publish((symbol_short!("payment"), to, amount), message);
    }

    /// Function for bulk payments
    /// Sends the specified amounts of XLM to multiple users.
    pub fn schedule_bulk_payment(env: Env, to: Vec<Address>, amounts: Vec<i128>, _interval: u32) {
        // Check if the number of recipients matches the number of amounts
        if to.len() != amounts.len() {
            panic!("Recipients and amounts lists must be of equal length.");
        }
        
        let token_id = env.current_contract_address();
        let token = soroban_sdk::token::Client::new(&env, &token_id);
        
        // Execute transfer for each recipient
        for (recipient, amount) in to.iter().zip(amounts.iter()) {
            // Check if each amount is positive
            if amount <= 0 {
                panic!("Each transfer amount must be positive");
            }
            token.transfer(&env.current_contract_address(), &recipient, &amount);
        }
    }

    /// Function to check balance
    /// Returns the current XLM balance of the user.
    pub fn check_balance(env: Env, user: Address) -> i128 {
        let token_id = env.current_contract_address();
        let token = soroban_sdk::token::Client::new(&env, &token_id);
        token.balance(&user)
    }

    /// Function to view transaction history
    /// Lists the past payments and messages the user has made.
    pub fn view_transaction_history(env: Env, user: Address) -> Vec<(Address, i128, String)> {
        // For demonstration purposes, a static history is returned, but in a real implementation,
        // this could be stored and retrieved from a database.
        let mut history = vec![&env];
        history.push_back((user.clone(), 1000, String::from_str(&env, "First payment")));
        history.push_back((user.clone(), 2000, String::from_str(&env, "Regular payment")));
        history
    }
}
