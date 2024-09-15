# Trade Engine in Rust

## Overview

This project implements a simple trade engine in Rust, aimed at providing a hands-on learning experience with the Rust programming language. The trade engine supports basic functionalities such as placing limit orders, managing an order book, and handling trading pairs.

## Features

- **Order Management**: Supports bid and ask orders with limit pricing.
- **Order Book**: Maintains a record of active orders for trading pairs.
- **Trading Pairs**: Allows the creation and management of trading pairs (e.g., BTC/USD).
- **Market Orders**: Facilitates the filling of market orders against limit orders.

## Getting Started

### Prerequisites

- Rust (version 1.56 or later)
- Cargo (Rust's package manager)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/trade-engine.git
   cd trade-engine
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the project:
   ```bash
   cargo run
   ```

## Project Structure

- `src/main.rs`: The entry point of the application, where the main trading logic is executed.
- `src/matching_engine/`: Contains the core matching engine logic, including order placement and market management.
- `src/matching_engine/orderbook.rs`: Implements the order book functionality, managing bids and asks.
- `src/matching_engine/engine.rs`: Defines the trading engine and its operations.
- `Cargo.toml`: The manifest file for Rust's package manager, specifying dependencies and project metadata.

## Dependencies

- `rust_decimal`: A library for handling decimal numbers, crucial for financial calculations.
- `rust_decimal_macros`: Provides macros for easier decimal number creation.

## Testing

The project includes unit tests to ensure the functionality of the order book and limit orders. You can run the tests using:

```bash
   cargo test
```
