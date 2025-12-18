use trade_engine::matching_engine::{Order, Orderbook, OrderSide, OrderType, TradingPair};
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

#[test]
fn test_limit_order_price_enforcement() -> Result<(), Box<dyn std::error::Error>> {
    let mut orderbook = Orderbook::new();
    let pair = TradingPair::new("BTC", "USD");

    // 1. Add a Sell Limit order at $110
    let sell_order = Order::new_limit(
        "sell_order".to_string(),
        pair.clone(),
        OrderSide::Ask,
        dec!(110),
        dec!(1.0),
        OrderType::GTC,
    );
    orderbook.add_order(dec!(110), sell_order)?;

    // 2. Try to match a Buy Limit order at $100
    // Expected: Should NOT match because Buy Price ($100) < Sell Price ($110)
    let mut buy_order = Order::new_limit(
        "buy_order".to_string(),
        pair,
        OrderSide::Bid,
        dec!(100),
        dec!(1.0),
        OrderType::GTC,
    );

    let trades = orderbook.match_order(&mut buy_order)?;

    // If bug exists, this might be 1. Correct behavior is 0.
    assert_eq!(trades.len(), 0, "Limit order matched against worse price!");
    assert_eq!(buy_order.filled_size, dec!(0), "Buy order should not be filled");

    Ok(())
}
