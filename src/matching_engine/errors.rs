use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MatchingEngineError {
    OrderbookNotFound(String),
    InsufficientLiquidity,
    InvalidOrderSize,
    InvalidOrderPrice,
    InvalidOrderType(String),
    InvalidOrderCancellation(String),
    OrderNotFound(String),
    OrderAlreadyFilled,
    TradingPairNotFound(String),
    InternalError(String),
}

impl fmt::Display for MatchingEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatchingEngineError::OrderbookNotFound(pair) => {
                write!(f, "Orderbook not found for trading pair: {}", pair)
            }
            MatchingEngineError::InsufficientLiquidity => {
                write!(f, "Insufficient liquidity to fill order")
            }
            MatchingEngineError::InvalidOrderSize => {
                write!(f, "Invalid order size")
            }
            MatchingEngineError::InvalidOrderPrice => {
                write!(f, "Invalid order price")
            }
            MatchingEngineError::InvalidOrderType(msg) => {
                write!(f, "Invalid order type: {}", msg)
            }
            MatchingEngineError::InvalidOrderCancellation(msg) => {
                write!(f, "Invalid order cancellation: {}", msg)
            }
            MatchingEngineError::OrderNotFound(id) => {
                write!(f, "Order not found: {}", id)
            }
            MatchingEngineError::OrderAlreadyFilled => {
                write!(f, "Order is already filled")
            }
            MatchingEngineError::TradingPairNotFound(pair) => {
                write!(f, "Trading pair not found: {}", pair)
            }
            MatchingEngineError::InternalError(msg) => {
                write!(f, "Internal error: {}", msg)
            }
        }
    }
}

impl Error for MatchingEngineError {}

pub type MatchingEngineResult<T> = Result<T, MatchingEngineError>;
