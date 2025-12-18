use super::errors::{MatchingEngineError, MatchingEngineResult};
use super::market_data::{MarketDataAggregator, OrderBookLevel};
use super::orderbook::Orderbook;
use super::trade::TradeHistory;
use super::types::{Order, OrderSide, OrderType, TradingPair};
use super::validation::OrderValidator;
use rust_decimal::Decimal;
use std::collections::HashMap;

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
    trade_history: TradeHistory,
    market_data: MarketDataAggregator,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
            trade_history: TradeHistory::new(),
            market_data: MarketDataAggregator::new(),
        }
    }

    pub fn add_market(&mut self, pair: TradingPair) {
        if !self.orderbooks.contains_key(&pair) {
            self.orderbooks.insert(pair.clone(), Orderbook::new());
            println!("Opened new orderbook for market {}", pair.to_string());
        }
    }

    pub fn place_order(&mut self, order: Order) -> MatchingEngineResult<()> {
        // Validate the order
        OrderValidator::validate_new_order(&order)?;

        let trading_pair = order.trading_pair.clone();
        let orderbook = self
            .orderbooks
            .get_mut(&trading_pair)
            .ok_or_else(|| MatchingEngineError::OrderbookNotFound(trading_pair.to_string()))?;

        let result = match order.order_type {
            OrderType::IOC => {
                let mut order = order;
                let trades = orderbook.match_order(&mut order)?;
                Ok((trades, None))
            }
            OrderType::FOK => {
                let mut order = order;
                if !orderbook.can_fill_completely(&order) {
                    return Err(MatchingEngineError::InsufficientLiquidity);
                }
                let trades = orderbook.match_order(&mut order)?;
                Ok((trades, None))
            }
            OrderType::GTC => {
                let mut order = order;
                let trades = orderbook.match_order(&mut order)?;
                if !order.is_filled() {
                    Ok((trades, Some((order.price.unwrap(), order))))
                } else {
                    Ok((trades, None))
                }
            }
        }?;

        // Process trades and add remaining order if any
        let (trades, remaining_order) = result;
        self.process_trades(trades, &trading_pair);

        if let Some((price, order)) = remaining_order {
            let orderbook = self
                .orderbooks
                .get_mut(&trading_pair)
                .ok_or_else(|| MatchingEngineError::OrderbookNotFound(trading_pair.to_string()))?;
            orderbook.add_order(price, order)?;
        }

        Ok(())
    }

    fn process_trades(&mut self, trades: Vec<(Order, Order, Decimal)>, pair: &TradingPair) {
        for (maker, taker, price) in trades {
            let size = maker.filled_size.min(taker.filled_size);

            // Record the trade
            self.trade_history
                .record_trade(pair.clone(), price, size, &maker, &taker);

            // Update market data
            self.market_data
                .update_market_stats(pair.clone(), Some(price), size, 1);
        }

        // Update order book snapshot
        if let Some(orderbook) = self.orderbooks.get(pair) {
            let (bids, asks) = orderbook.get_aggregated_levels();
            self.market_data.update_order_book(
                pair.clone(),
                bids.into_iter()
                    .map(|(price, size, count)| OrderBookLevel {
                        price,
                        size,
                        order_count: count,
                    })
                    .collect(),
                asks.into_iter()
                    .map(|(price, size, count)| OrderBookLevel {
                        price,
                        size,
                        order_count: count,
                    })
                    .collect(),
            );
        }
    }

    pub fn cancel_order(
        &mut self,
        pair: &TradingPair,
        order_id: String,
    ) -> MatchingEngineResult<()> {
        let orderbook = self
            .orderbooks
            .get_mut(pair)
            .ok_or_else(|| MatchingEngineError::OrderbookNotFound(pair.to_string()))?;

        orderbook.cancel_order(&order_id)
    }

    pub fn get_order_book_snapshot(
        &self,
        pair: &TradingPair,
    ) -> Option<(
        Vec<(Decimal, Decimal, usize)>,
        Vec<(Decimal, Decimal, usize)>,
    )> {
        self.orderbooks
            .get(pair)
            .map(|ob| ob.get_aggregated_levels())
    }

    pub fn get_market_stats(&self, pair: &TradingPair) -> Option<super::market_data::MarketStats> {
        self.market_data.get_market_stats(pair)
    }

    pub fn get_trades(&self, pair: &TradingPair) -> Vec<super::types::Trade> {
        self.trade_history.get_trades_by_pair(pair)
    }
}


