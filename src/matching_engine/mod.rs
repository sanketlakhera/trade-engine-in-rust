pub mod engine;
pub mod errors;
pub mod market_data;
pub mod orderbook;
pub mod trade;
pub mod types;
pub mod validation;

pub use engine::MatchingEngine;
pub use errors::{MatchingEngineError, MatchingEngineResult};
pub use market_data::{MarketDataAggregator, MarketStats, OrderBookSnapshot};
pub use orderbook::Orderbook;
pub use trade::TradeHistory;
pub use types::{Order, OrderSide, OrderType, Trade, TradingPair};
pub use validation::OrderValidator;
