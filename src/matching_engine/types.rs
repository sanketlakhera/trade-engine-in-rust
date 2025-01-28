use rust_decimal::Decimal;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    GTC, // Good Till Cancelled
    IOC, // Immediate Or Cancel
    FOK, // Fill Or Kill
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub trading_pair: TradingPair,
    pub side: OrderSide,
    pub price: Option<Decimal>, // None for market orders
    pub size: Decimal,
    pub filled_size: Decimal,
    pub order_type: OrderType,
    pub timestamp: SystemTime,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    pub base: String,
    pub quote: String,
}

impl Order {
    pub fn new_limit(
        id: impl Into<String>,
        trading_pair: TradingPair,
        side: OrderSide,
        price: Decimal,
        size: impl Into<Decimal>,
        order_type: OrderType,
    ) -> Self {
        Order {
            id: id.into(),
            trading_pair,
            side,
            price: Some(price),
            size: size.into(),
            filled_size: Decimal::ZERO,
            order_type,
            timestamp: SystemTime::now(),
        }
    }

    pub fn new_market(
        id: impl Into<String>,
        trading_pair: TradingPair,
        side: OrderSide,
        size: impl Into<Decimal>,
    ) -> Self {
        Order {
            id: id.into(),
            trading_pair,
            side,
            price: None,
            size: size.into(),
            filled_size: Decimal::ZERO,
            order_type: OrderType::IOC,
            timestamp: SystemTime::now(),
        }
    }

    pub fn is_filled(&self) -> bool {
        self.filled_size >= self.size
    }

    pub fn remaining_size(&self) -> Decimal {
        self.size - self.filled_size
    }
}

impl TradingPair {
    pub fn new(base: impl Into<String>, quote: impl Into<String>) -> Self {
        TradingPair {
            base: base.into(),
            quote: quote.into(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub id: String,
    pub trading_pair: TradingPair,
    pub price: Decimal,
    pub size: Decimal,
    pub maker_order_id: String,
    pub taker_order_id: String,
    pub timestamp: SystemTime,
}

impl Trade {
    pub fn new(
        trading_pair: TradingPair,
        price: Decimal,
        size: Decimal,
        maker_order_id: String,
        taker_order_id: String,
    ) -> Self {
        Trade {
            id: Uuid::new_v4().to_string(),
            trading_pair,
            price,
            size,
            maker_order_id,
            taker_order_id,
            timestamp: SystemTime::now(),
        }
    }
}
