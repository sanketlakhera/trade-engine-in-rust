use crate::matching_engine::{MatchingEngine, Order, OrderSide, OrderType, TradingPair};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[path = "../src/matching_engine/mod.rs"]
mod matching_engine;

fn place_limit_orders(c: &mut Criterion) {
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC", "USD");
    engine.add_market(pair.clone());

    c.bench_function("place_limit_order", |b| {
        b.iter(|| {
            let order = Order::new_limit(
                black_box("order1".to_string()),
                black_box(pair.clone()),
                black_box(OrderSide::Bid),
                black_box(dec!(50000)),
                black_box(dec!(1.0)),
                black_box(OrderType::GTC),
            );
            engine.place_order(order).unwrap();
        })
    });
}

fn match_market_orders(c: &mut Criterion) {
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC", "USD");
    engine.add_market(pair.clone());

    // Add some liquidity first
    for i in 0..100 {
        let price = dec!(50000) + Decimal::from(i * 10);
        let order = Order::new_limit(
            format!("ask{}", i),
            pair.clone(),
            OrderSide::Ask,
            price,
            dec!(1.0),
            OrderType::GTC,
        );
        engine.place_order(order).unwrap();
    }

    c.bench_function("match_market_order", |b| {
        b.iter(|| {
            let order = Order::new_market(
                black_box("market1".to_string()),
                black_box(pair.clone()),
                black_box(OrderSide::Bid),
                black_box(dec!(0.5)),
            );
            engine.place_order(order).unwrap();
        })
    });
}

fn order_book_updates(c: &mut Criterion) {
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC", "USD");
    engine.add_market(pair.clone());

    c.bench_function("order_book_updates", |b| {
        b.iter(|| {
            // Add and cancel orders rapidly
            for i in 0..10 {
                let order = Order::new_limit(
                    format!("order{}", i),
                    pair.clone(),
                    OrderSide::Bid,
                    dec!(50000),
                    dec!(1.0),
                    OrderType::GTC,
                );
                engine.place_order(order).unwrap();
            }

            for i in 0..5 {
                engine.cancel_order(&pair, format!("order{}", i)).unwrap();
            }

            engine.get_order_book_snapshot(&pair);
        })
    });
}

criterion_group!(
    benches,
    place_limit_orders,
    match_market_orders,
    order_book_updates
);
criterion_main!(benches);
