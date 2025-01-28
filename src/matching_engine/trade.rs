use super::types::{Order, Trade, TradingPair};
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TradeHistory {
    trades: Vec<Trade>,
    trades_by_pair: HashMap<TradingPair, Vec<Trade>>,
}

impl TradeHistory {
    pub fn new() -> Self {
        TradeHistory {
            trades: Vec::new(),
            trades_by_pair: HashMap::new(),
        }
    }

    pub fn record_trade(
        &mut self,
        trading_pair: TradingPair,
        price: Decimal,
        size: Decimal,
        maker_order: &Order,
        taker_order: &Order,
    ) -> Trade {
        let trade = Trade::new(
            trading_pair.clone(),
            price,
            size,
            maker_order.id.clone(),
            taker_order.id.clone(),
        );

        self.trades.push(trade.clone());
        self.trades_by_pair
            .entry(trading_pair)
            .or_default()
            .push(trade.clone());

        trade
    }

    pub fn get_trades_by_pair(&self, pair: &TradingPair) -> Vec<Trade> {
        self.trades_by_pair
            .get(pair)
            .map(|trades| trades.clone())
            .unwrap_or_default()
    }

    pub fn get_last_price(&self, pair: &TradingPair) -> Option<Decimal> {
        self.trades_by_pair
            .get(pair)
            .and_then(|trades| trades.last())
            .map(|trade| trade.price)
    }

    pub fn get_volume_24h(&self, pair: &TradingPair) -> Decimal {
        let now = std::time::SystemTime::now();
        let one_day = std::time::Duration::from_secs(24 * 60 * 60);

        self.trades_by_pair
            .get(pair)
            .map(|trades| {
                trades
                    .iter()
                    .filter(|trade| {
                        trade
                            .timestamp
                            .duration_since(now)
                            .map(|duration| duration < one_day)
                            .unwrap_or(false)
                    })
                    .map(|trade| trade.size)
                    .sum()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matching_engine::types::{OrderSide, OrderType};
    use rust_decimal_macros::dec;

    #[test]
    fn test_record_trade() {
        let mut history = TradeHistory::new();
        let pair = TradingPair::new("BTC", "USD");

        let maker = Order::new_limit(
            "order1",
            pair.clone(),
            OrderSide::Bid,
            dec!(50000),
            dec!(1.0),
            OrderType::GTC,
        );

        let taker = Order::new_market("order2", pair.clone(), OrderSide::Ask, dec!(1.0));

        let trade = history.record_trade(pair.clone(), dec!(50000), dec!(1.0), &maker, &taker);

        assert_eq!(trade.price, dec!(50000));
        assert_eq!(trade.size, dec!(1.0));
        assert_eq!(trade.maker_order_id, "order1");
        assert_eq!(trade.taker_order_id, "order2");

        let trades = history.get_trades_by_pair(&pair);
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].price, dec!(50000));
    }
}
