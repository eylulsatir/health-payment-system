# Health Payment System on Stellar

This project implements a Health Payment System using Stellar's blockchain technology. It leverages Soroban, Stellar's smart contract platform, to enable secure and efficient healthcare-related payments.

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Setup and Installation](#setup-and-installation)
- [Deployment](#deployment)
- [Usage](#usage)
- [Advantages of Blockchain in Healthcare](#advantages-of-blockchain-in-healthcare)
- [Contributing](#contributing)
- [License](#license)

## Overview

The Health Payment System allows users to send payments using Stellar's native token (XLM) and attach short messages to those payments. It's designed to facilitate transparent, secure, and efficient financial transactions in the healthcare sector.

## Features

- **XLM Transfer**: Send Stellar's native token (XLM) to other users.
- **Messaging**: Attach short messages to payment transactions.
- **Balance Checking**: View current XLM balance of users.
- **Bulk Payments**: Execute multiple payments in a single transaction.
- **Transaction History**: View detailed history of past transactions and messages.
- **Scheduled Payments**: Set up recurring payments at specified intervals.

## Technology Stack

- [Rust](https://www.rust-lang.org/)
- [Soroban](https://soroban.stellar.org/)
- [Stellar Blockchain](https://www.stellar.org/)

## Project Structure

- `src/lib.rs`: Core implementation of the Health Payment System.
- `Cargo.toml`: Project dependencies and build configurations.

## Setup and Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/eylulsatir/health-payment-system.git
   cd health-payment-system

## Deployment

The contract is deployed on Stellar's testnet. Contract ID: `CC5BMHCE66N6WX7L2YN5YNQCRJLSVO5N3BJBBOFFAZUKOJK6YBY66CQR`

To deploy your own instance:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/health_payment_system.wasm \
  --source YOUR_SECRET_KEY \
  --network testnet

## Usage
soroban contract invoke \
  --id CC5BMHCE66N6WX7L2YN5YNQCRJLSVO5N3BJBBOFFAZUKOJK6YBY66CQR \
  --source YOUR_SECRET_KEY \
  --network testnet \
  -- \
  schedule_payment \
  --to RECIPIENT_ADDRESS \
  --amount 1000000000 \
  --interval 86400

## Advantages of Blockchain in Healthcare
* **Enhanced Data Security**: Secure storage and sharing of sensitive patient data.
* **Increased Transparency**: Transparent and easily traceable medical records.
* **Improved Efficiency**: Automation of processes leading to reduced costs and time savings.
* **Interoperability**: Seamless data exchange between different healthcare providers.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
