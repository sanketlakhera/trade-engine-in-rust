#![allow(dead_code)]
use super::errors::{MatchingEngineError, MatchingEngineResult};
use super::types::{Order, OrderSide};
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn match_order(
        &mut self,
        order: &mut Order,
    ) -> MatchingEngineResult<Vec<(Order, Order, Decimal)>> {
        let mut trades = Vec::new();
        let limits = match order.side {
            OrderSide::Bid => self.ask_limits(),
            OrderSide::Ask => self.bid_limits(),
        };

        for limit_price_level in limits {
            let orders = &mut limit_price_level.orders;
            let mut i = 0;
            while i < orders.len() {
                let maker_order = &mut orders[i];
                let trade_size = maker_order.remaining_size().min(order.remaining_size());

                if trade_size > Decimal::ZERO {
                    maker_order.filled_size += trade_size;
                    order.filled_size += trade_size;
                    trades.push((maker_order.clone(), order.clone(), limit_price_level.price));
                }

                if order.is_filled() {
                    break;
                }
                i += 1;
            }

            // Remove filled orders
            orders.retain(|o| !o.is_filled());

            if order.is_filled() {
                break;
            }
        }

        Ok(trades)
    }

    pub fn can_fill_completely(&self, order: &Order) -> bool {
        let mut remaining_size = order.size;
        let limits = match order.side {
            OrderSide::Bid => self.asks.values().collect::<Vec<&Limit>>(),
            OrderSide::Ask => self.bids.values().collect::<Vec<&Limit>>(),
        };

        for limit in limits {
            for maker_order in &limit.orders {
                remaining_size -= maker_order.remaining_size();
                if remaining_size <= Decimal::ZERO {
                    return true;
                }
            }
        }

        false
    }

    pub fn add_order(&mut self, price: Decimal, order: Order) -> MatchingEngineResult<()> {
        match order.side {
            OrderSide::Bid => {
                self.bids
                    .entry(price)
                    .or_insert_with(|| Limit::new(price))
                    .add_order(order);
            }
            OrderSide::Ask => {
                self.asks
                    .entry(price)
                    .or_insert_with(|| Limit::new(price))
                    .add_order(order);
            }
        }
        Ok(())
    }

    pub fn cancel_order(&mut self, order_id: &str) -> MatchingEngineResult<()> {
        let mut found = false;

        // Search in bids
        for limit in self.bids.values_mut() {
            if limit.cancel_order(order_id) {
                found = true;
                break;
            }
        }

        // Search in asks if not found in bids
        if !found {
            for limit in self.asks.values_mut() {
                if limit.cancel_order(order_id) {
                    found = true;
                    break;
                }
            }
        }

        if found {
            Ok(())
        } else {
            Err(MatchingEngineError::OrderNotFound(order_id.to_string()))
        }
    }

    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| a.price.cmp(&b.price));
        limits
    }

    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| b.price.cmp(&a.price));
        limits
    }

    pub fn get_aggregated_levels(
        &self,
    ) -> (
        Vec<(Decimal, Decimal, usize)>,
        Vec<(Decimal, Decimal, usize)>,
    ) {
        let mut bids = self
            .bids
            .iter()
            .map(|(price, limit)| (*price, limit.total_volume(), limit.orders.len()))
            .collect::<Vec<_>>();
        bids.sort_by(|a, b| b.0.cmp(&a.0));

        let mut asks = self
            .asks
            .iter()
            .map(|(price, limit)| (*price, limit.total_volume(), limit.orders.len()))
            .collect::<Vec<_>>();
        asks.sort_by(|a, b| a.0.cmp(&b.0));

        (bids, asks)
    }
}

#[derive(Debug)]
pub struct Limit {
    pub price: Decimal,
    pub orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: Decimal) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn total_volume(&self) -> Decimal {
        self.orders.iter().map(|order| order.remaining_size()).sum()
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }

    fn cancel_order(&mut self, order_id: &str) -> bool {
        if let Some(pos) = self.orders.iter().position(|o| o.id == order_id) {
            self.orders.remove(pos);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::super::types::{OrderType, TradingPair};
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_match_market_order() -> Result<(), Box<dyn std::error::Error>> {
        let mut orderbook = Orderbook::new();
        let pair = TradingPair::new("BTC", "USD");

        // Add some limit orders
        let limit_order = Order::new_limit(
            "order1".to_string(),
            pair.clone(),
            OrderSide::Ask,
            dec!(50000),
            dec!(1.0),
            OrderType::GTC,
        );
        orderbook.add_order(dec!(50000), limit_order)?;

        // Create a market buy order
        let mut market_order =
            Order::new_market("order2".to_string(), pair, OrderSide::Bid, dec!(0.5));

        // Match the order
        let trades = orderbook.match_order(&mut market_order)?;

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].2, dec!(50000)); // Price
        assert_eq!(market_order.filled_size, dec!(0.5));

        Ok(())
    }

    #[test]
    fn test_can_fill_completely() -> Result<(), Box<dyn std::error::Error>> {
        let mut orderbook = Orderbook::new();
        let pair = TradingPair::new("BTC", "USD");

        // Add a limit order
        let limit_order = Order::new_limit(
            "order1".to_string(),
            pair.clone(),
            OrderSide::Ask,
            dec!(50000),
            dec!(1.0),
            OrderType::GTC,
        );
        orderbook.add_order(dec!(50000), limit_order)?;

        // Check if we can fill a smaller order
        let market_order = Order::new_market(
            "order2".to_string(),
            pair.clone(),
            OrderSide::Bid,
            dec!(0.5),
        );
        assert!(orderbook.can_fill_completely(&market_order));

        // Check if we can fill an equal sized order
        let market_order = Order::new_market(
            "order3".to_string(),
            pair.clone(),
            OrderSide::Bid,
            dec!(1.0),
        );
        assert!(orderbook.can_fill_completely(&market_order));

        // Check if we can fill a larger order
        let market_order = Order::new_market("order4".to_string(), pair, OrderSide::Bid, dec!(1.5));
        assert!(!orderbook.can_fill_completely(&market_order));

        Ok(())
    }
}
