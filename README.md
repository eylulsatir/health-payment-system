# health-payment-system
# Health Payment System

This project is a **Health Payment System** that allows users to send payments using Stellar's native token (XLM) and attach short messages to those payments. It includes features such as regular payments, balance checking, bulk payments, and viewing transaction history.

## Features

- **XLM Transfer**: Users can transfer Stellar's native token (XLM) to other users.
- **Message with Payments**: Attach short messages with each payment transaction.
- **Check Balance**: View the user's current XLM balance.
- **Bulk Payments**: Send XLM to multiple addresses in a single transaction.
- **View Transaction History**: View the list of past transactions and the messages attached.

## Extra Features

- **Scheduled Payments**: Schedule recurring payments to users at specified intervals.
- **Transaction History**: View a detailed history of payments and messages.

## Project Structure

- **lib.rs**: The main logic of the Health Payment System is implemented here. It contains functions for executing payments, scheduling payments, and checking balances.
- **Cargo.toml**: Defines the project's dependencies and build configurations.

## How to Run the Project

### 1. Clone the Repository

```bash
git clone git@github.com:eylulsatir/health-payment-system.git
cd health-payment-system
