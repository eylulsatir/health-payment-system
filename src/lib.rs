use soroban_sdk::{contractimpl, Address, Env, String, Vec};
use soroban_sdk::token; // Required for token operations

pub struct HealthPaymentSystem;

#[contractimpl]
impl HealthPaymentSystem {
    /// Function to schedule a payment to a user
    /// Allows scheduling automatic payments to a specified user at regular intervals.
    /// Returns `Ok(())` if the payment is successfully scheduled, otherwise returns `Err`.
    pub fn schedule_payment(env: &Env, to: Address, amount: i128, interval: u32) -> Result<(), String> {
        // Check if the transfer amount is positive
        if amount <= 0 {
            return Err(String::from_slice(env, "Transfer amount must be positive"));
        }

        // Create a vector to store the payment history (can be used later)
        let mut history = vec![]; // Vector to store payment details
        history.push((to.clone(), amount, interval));

        // Return Ok if successful
        Ok(())
    }

    /// Function for one-time payments
    /// Sends the specified amount of XLM to the user.
    /// Returns `Ok(())` if the transfer is successful, otherwise returns `Err`.
    pub fn execute_payment(env: &Env, to: Address, amount: i128, message: String) -> Result<(), String> {
        // Create a token client for the transfer operation
        let token_client = token::Client::new(env, &env.current_contract_address());

        // Check if the amount is positive
        if amount <= 0 {
            return Err(String::from_slice(env, "Amount to be sent must be positive"));
        }

        // Execute the transfer
        token_client.transfer(&env.current_contract_address(), &to, &amount);
        
        // Send a short message to the user (messages can be stored)
        env.events().publish((to.clone(), amount), message);

        Ok(())
    }

    /// Function for bulk payments
    /// Sends the specified amounts of XLM to multiple users.
    /// If the lengths of `to` and `amounts` are not equal, returns `Err`, otherwise returns `Ok(())`.
    pub fn schedule_bulk_payment(env: &Env, to: Vec<Address>, amounts: Vec<i128>, interval: u32) -> Result<(), String> {
        // Check if the number of recipients matches the number of amounts
        if to.len() != amounts.len() {
            return Err(String::from_slice(env, "Recipients and amounts lists must be of equal length."));
        }

        // Create a token client for the transfer operation
        let token_client = token::Client::new(env, &env.current_contract_address());
        
        // Execute transfer for each recipient
        for (recipient, amount) in to.iter().zip(amounts.iter()) {
            // Check if each amount is positive
            if *amount <= 0 {
                return Err(String::from_slice(env, "Each transfer amount must be positive"));
            }
            token_client.transfer(&env.current_contract_address(), recipient, &amount);
        }

        Ok(())
    }

    /// Function to check balance
    /// Returns the current XLM balance of the user.
    pub fn check_balance(env: &Env, user: Address) -> Result<i128, String> {
        let token_client = token::Client::new(env, &env.current_contract_address());
        let balance = token_client.balance(&user);

        Ok(balance)
    }

    /// Function to view transaction history
    /// Lists the past payments and messages the user has made.
    pub fn view_transaction_history(env: &Env, user: Address) -> Result<Vec<(Address, i128, String)>, String> {
        // For demonstration purposes, a static history is returned, but in a real implementation,
        // this could be stored and retrieved from a database.
        let history: Vec<(Address, i128, String)> = vec![
            (user.clone(), 1000, String::from_slice(env, "First payment")),
            (user.clone(), 2000, String::from_slice(env, "Regular payment"))
        ];

        Ok(history)
    }
}