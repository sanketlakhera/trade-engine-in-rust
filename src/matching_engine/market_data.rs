use super::types::TradingPair;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct OrderBookLevel {
    pub price: Decimal,
    pub size: Decimal,
    pub order_count: usize,
}

#[derive(Debug, Clone)]
pub struct OrderBookSnapshot {
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone)]
pub struct MarketStats {
    pub last_price: Option<Decimal>,
    pub volume_24h: Decimal,
    pub high_24h: Option<Decimal>,
    pub low_24h: Option<Decimal>,
    pub num_trades_24h: u64,
    pub timestamp: SystemTime,
}

#[derive(Debug, Default)]
pub struct MarketDataAggregator {
    order_book_snapshots: HashMap<TradingPair, Vec<OrderBookSnapshot>>,
    market_stats: HashMap<TradingPair, MarketStats>,
}

impl MarketDataAggregator {
    pub fn new() -> Self {
        MarketDataAggregator {
            order_book_snapshots: HashMap::new(),
            market_stats: HashMap::new(),
        }
    }

    pub fn update_order_book(
        &mut self,
        pair: TradingPair,
        bids: Vec<OrderBookLevel>,
        asks: Vec<OrderBookLevel>,
    ) {
        let snapshot = OrderBookSnapshot {
            bids,
            asks,
            timestamp: SystemTime::now(),
        };

        self.order_book_snapshots
            .entry(pair)
            .or_default()
            .push(snapshot);
    }

    pub fn update_market_stats(
        &mut self,
        pair: TradingPair,
        price: Option<Decimal>,
        volume: Decimal,
        num_trades: u64,
    ) {
        let now = SystemTime::now();
        let stats = self.market_stats.entry(pair).or_insert(MarketStats {
            last_price: None,
            volume_24h: Decimal::ZERO,
            high_24h: None,
            low_24h: None,
            num_trades_24h: 0,
            timestamp: now,
        });

        if let Some(price) = price {
            stats.last_price = Some(price);
            match (stats.high_24h, stats.low_24h) {
                (Some(high), Some(low)) => {
                    stats.high_24h = Some(std::cmp::max(high, price));
                    stats.low_24h = Some(std::cmp::min(low, price));
                }
                _ => {
                    stats.high_24h = Some(price);
                    stats.low_24h = Some(price);
                }
            }
        }

        stats.volume_24h += volume;
        stats.num_trades_24h += num_trades;
        stats.timestamp = now;
    }

    pub fn get_market_stats(&self, pair: &TradingPair) -> Option<MarketStats> {
        self.market_stats.get(pair).cloned()
    }

    pub fn get_order_book_snapshot(&self, pair: &TradingPair) -> Option<OrderBookSnapshot> {
        self.order_book_snapshots
            .get(pair)
            .and_then(|snapshots| snapshots.last())
            .cloned()
    }

    pub fn cleanup_old_data(&mut self) {
        let now = SystemTime::now();
        let one_day = Duration::from_secs(24 * 60 * 60);

        for snapshots in self.order_book_snapshots.values_mut() {
            snapshots.retain(|snapshot| {
                snapshot
                    .timestamp
                    .elapsed()
                    .map(|age| age < one_day)
                    .unwrap_or(false)
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_market_stats_update() {
        let mut aggregator = MarketDataAggregator::new();
        let pair = TradingPair::new("BTC", "USD");

        // Update with first trade
        aggregator.update_market_stats(pair.clone(), Some(dec!(50000)), dec!(1.0), 1);
        let stats = aggregator.get_market_stats(&pair).unwrap();
        assert_eq!(stats.last_price, Some(dec!(50000)));
        assert_eq!(stats.volume_24h, dec!(1.0));
        assert_eq!(stats.high_24h, Some(dec!(50000)));
        assert_eq!(stats.low_24h, Some(dec!(50000)));
        assert_eq!(stats.num_trades_24h, 1);

        // Update with second trade at higher price
        aggregator.update_market_stats(pair.clone(), Some(dec!(51000)), dec!(2.0), 1);
        let stats = aggregator.get_market_stats(&pair).unwrap();
        assert_eq!(stats.last_price, Some(dec!(51000)));
        assert_eq!(stats.volume_24h, dec!(3.0));
        assert_eq!(stats.high_24h, Some(dec!(51000)));
        assert_eq!(stats.low_24h, Some(dec!(50000)));
        assert_eq!(stats.num_trades_24h, 2);
    }
}
