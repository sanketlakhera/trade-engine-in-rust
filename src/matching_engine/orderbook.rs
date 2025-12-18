#![allow(dead_code)]
use super::errors::{MatchingEngineError, MatchingEngineResult};
use super::types::{Order, OrderSide};
use rust_decimal::Decimal;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Orderbook {
    asks: BTreeMap<Decimal, Limit>,
    bids: BTreeMap<Decimal, Limit>,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
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
            // Check if price is acceptable for limit orders
            if let Some(order_price) = order.price {
                match order.side {
                    OrderSide::Bid => {
                        if limit_price_level.price > order_price {
                            break;
                        }
                    }
                    OrderSide::Ask => {
                        if limit_price_level.price < order_price {
                            break;
                        }
                    }
                }
            }

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
        
        // Iterating over limits in price priority order
        match order.side {
            OrderSide::Bid => {
                // Buying: check asks (lowest price first)
                for limit in self.asks.values() {
                    for maker_order in &limit.orders {
                        remaining_size -= maker_order.remaining_size();
                        if remaining_size <= Decimal::ZERO {
                            return true;
                        }
                    }
                }
            },
            OrderSide::Ask => {
                // Selling: check bids (highest price first)
                for limit in self.bids.values().rev() {
                    for maker_order in &limit.orders {
                        remaining_size -= maker_order.remaining_size();
                        if remaining_size <= Decimal::ZERO {
                            return true;
                        }
                    }
                }
            }
        };

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
        // Asks are sorted ascending by key (price) in BTreeMap
        self.asks.values_mut().collect()
    }

    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        // Bids need to be sorted descending by price
        self.bids.values_mut().rev().collect()
    }

    pub fn get_aggregated_levels(
        &self,
    ) -> (
        Vec<(Decimal, Decimal, usize)>,
        Vec<(Decimal, Decimal, usize)>,
    ) {
        // Bids: Descending order (highest price first)
        let bids = self
            .bids
            .iter()
            .rev()
            .map(|(price, limit)| (*price, limit.total_volume(), limit.orders.len()))
            .collect::<Vec<_>>();

        // Asks: Ascending order (lowest price first)
        let asks = self
            .asks
            .iter()
            .map(|(price, limit)| (*price, limit.total_volume(), limit.orders.len()))
            .collect::<Vec<_>>();

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


