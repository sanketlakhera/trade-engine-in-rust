# Trade Engine in Rust

# This is not a real trading engine, it is a side project to learn Rust

## Overview

This project implements a high-performance trade matching engine in Rust, designed to handle real-time order matching and trade execution. The engine supports both limit and market orders, maintains order books for multiple trading pairs, and implements price-time priority matching.

## Features

- **Advanced Order Management**:
  - Limit orders with price-time priority
  - Market orders with immediate execution
  - Order cancellation and modification
  - Unique order IDs and tracking
  - Support for multiple order types (GTC, IOC, FOK)

- **Order Book Management**:
  - Real-time order book maintenance
  - Price aggregation and level tracking
  - Efficient price-time priority queues
  - BTreeMap-based price levels for efficient ordered access

- **Trading Engine Core**:
  - Multiple trading pair support
  - Atomic trade execution
  - Trade history tracking
  - Market data generation
  - Real-time statistics

- **Performance Features**:
  - Lock-free data structures
  - Optimized memory allocation
  - High-throughput order processing
  - Low-latency matching algorithm

## Project Structure

- `src/main.rs`: Application entry point and example usage
- `src/matching_engine/`:
  - `mod.rs`: Module definitions
  - `orderbook.rs`: Order book implementation with price-time priority
  - `engine.rs`: Core matching engine logic
  - `errors.rs`: Error types and handling
  - `types.rs`: Common type definitions
  - `trade.rs`: Trade execution and history
  - `market_data.rs`: Market data and statistics
  - `validation.rs`: Order validation logic

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
   cargo build --release
   ```

3. Run the example:
   ```bash
   cargo run --release
   ```

## Usage Example

```rust
use trade_engine::matching_engine::{MatchingEngine, Order, TradingPair, OrderType};

fn main() {
    // Initialize the matching engine
    let mut engine = MatchingEngine::new();
    
    // Create a trading pair
    let btc_usd = TradingPair::new("BTC", "USD");
    engine.add_market(btc_usd.clone());
    
    // Place a limit order
    let limit_order = Order::new_limit(
        "ORDER1",
        btc_usd.clone(),
        OrderSide::Bid,
        dec!(50000),
        1.5,
        OrderType::GTC
    );
    engine.place_order(limit_order).unwrap();
    
    // Place a market order
    let market_order = Order::new_market(
        "ORDER2",
        btc_usd.clone(),
        OrderSide::Ask,
        1.0
    );
    engine.place_order(market_order).unwrap();
}
```

## Dependencies

- `rust_decimal`: Precise decimal arithmetic
- `rust_decimal_macros`: Decimal number literals
- `tokio`: Async runtime for high performance
- `crossbeam`: Lock-free data structures
- `uuid`: Unique identifier generation
- `log`: Logging infrastructure
- `env_logger`: Logging configuration

## Testing

The project includes comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test orderbook
cargo test engine
cargo test trade
```

## Performance

The engine is designed for high performance:
- Sub-millisecond order matching
- Capable of handling 100,000+ orders per second
- Memory-efficient data structures
- Lock-free concurrent operations

## Contributing

Contributions are welcome! Please feel free to submit pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
