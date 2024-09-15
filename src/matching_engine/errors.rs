use std::fmt;

#[derive(Debug)]
pub enum MatchingEngineError {
    OrderbookNotFound(String),
    InsufficientLiquidity,
    InvalidOrder,
    // Add more error types as needed
}

impl fmt::Display for MatchingEngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatchingEngineError::OrderbookNotFound(pair) => write!(f, "Orderbook not found for pair: {}", pair),
            MatchingEngineError::InsufficientLiquidity => write!(f, "Insufficient liquidity to fill the order"),
            MatchingEngineError::InvalidOrder => write!(f, "Invalid order"),
        }
    }
}

impl std::error::Error for MatchingEngineError {}

pub type MatchingEngineResult<T> = Result<T, MatchingEngineError>;