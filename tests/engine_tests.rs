use trade_engine::matching_engine::{MatchingEngine, MatchingEngineResult, Order, OrderSide, OrderType, TradingPair};
use rust_decimal_macros::dec;

#[test]
fn test_place_limit_order() -> MatchingEngineResult<()> {
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC", "USD");
    engine.add_market(pair.clone());

    let order = Order::new_limit(
        "order1".to_string(),
        pair.clone(),
        OrderSide::Bid,
        dec!(50000),
        dec!(1.0),
        OrderType::GTC,
    );

    engine.place_order(order)?;

    let (bids, _) = engine.get_order_book_snapshot(&pair).unwrap();
    assert_eq!(bids.len(), 1);
    assert_eq!(bids[0].0, dec!(50000));
    assert_eq!(bids[0].1, dec!(1.0));

    Ok(())
}

#[test]
fn test_market_order_matching() -> MatchingEngineResult<()> {
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC", "USD");
    engine.add_market(pair.clone());

    // Place a limit order
    let limit_order = Order::new_limit(
        "order1".to_string(),
        pair.clone(),
        OrderSide::Ask,
        dec!(50000),
        dec!(1.0),
        OrderType::GTC,
    );
    engine.place_order(limit_order)?;

    // Place a market order
    let market_order = Order::new_market(
        "order2".to_string(),
        pair.clone(),
        OrderSide::Bid,
        dec!(1.0),
    );
    engine.place_order(market_order)?;

    // Check trades
    let trades = engine.get_trades(&pair);
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].price, dec!(50000));
    assert_eq!(trades[0].size, dec!(1.0));

    Ok(())
}
