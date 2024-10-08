use rust_decimal::Decimal;
use super::orderbook::{Order, Orderbook};
use super::errors::{MatchingEngineError, MatchingEngineResult};
use std::collections::HashMap;
// BTC/USD => BTC = BASE AND USD = QUOTE
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }

    pub fn to_string(self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("open new orderbook for market {:?}", pair.to_string())
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        price: Decimal,
        order: Order,
    ) -> MatchingEngineResult<()> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order)?;
                println!("Placed limit order at price level {}", price);
                Ok(())
            }
            None => Err(MatchingEngineError::OrderbookNotFound(pair.to_string())),
        }
    }
}
