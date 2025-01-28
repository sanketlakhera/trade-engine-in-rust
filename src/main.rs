mod matching_engine;

use matching_engine::{MatchingEngine, Order, OrderSide, OrderType, TradingPair};
use rust_decimal_macros::dec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the matching engine
    let mut engine = MatchingEngine::new();

    // Create a trading pair
    let btc_usd = TradingPair::new("BTC", "USD");
    engine.add_market(btc_usd.clone());

    println!("Created BTC/USD market");

    // Place some limit orders
    let limit_orders = vec![
        Order::new_limit(
            "ask1".to_string(),
            btc_usd.clone(),
            OrderSide::Ask,
            dec!(50000),
            dec!(1.0),
            OrderType::GTC,
        ),
        Order::new_limit(
            "ask2".to_string(),
            btc_usd.clone(),
            OrderSide::Ask,
            dec!(50100),
            dec!(2.0),
            OrderType::GTC,
        ),
        Order::new_limit(
            "bid1".to_string(),
            btc_usd.clone(),
            OrderSide::Bid,
            dec!(49900),
            dec!(1.5),
            OrderType::GTC,
        ),
    ];

    for order in limit_orders {
        let id = order.id.clone();
        let side = order.side.clone();
        let size = order.size;
        let price = order.price;
        engine.place_order(order)?;
        println!(
            "Placed {} {} order for {} BTC at ${}",
            id,
            match side {
                OrderSide::Bid => "bid",
                OrderSide::Ask => "ask",
            },
            size,
            price.unwrap()
        );
    }

    // Place a market buy order
    let market_order = Order::new_market(
        "market1".to_string(),
        btc_usd.clone(),
        OrderSide::Bid,
        dec!(0.5),
    );
    engine.place_order(market_order)?;
    println!("Placed market buy order for 0.5 BTC");

    // Get and display market stats
    if let Some(stats) = engine.get_market_stats(&btc_usd) {
        println!("\nMarket Stats:");
        println!("Last Price: ${}", stats.last_price.unwrap());
        println!("24h Volume: {} BTC", stats.volume_24h);
        println!("24h Trades: {}", stats.num_trades_24h);
    }

    // Get and display order book
    if let Some((bids, asks)) = engine.get_order_book_snapshot(&btc_usd) {
        println!("\nOrder Book:");
        println!("Asks:");
        for (price, size, count) in asks.iter().rev() {
            println!("${}: {} BTC ({} orders)", price, size, count);
        }
        println!("Bids:");
        for (price, size, count) in bids.iter() {
            println!("${}: {} BTC ({} orders)", price, size, count);
        }
    }

    // Get and display recent trades
    let trades = engine.get_trades(&btc_usd);
    println!("\nRecent Trades:");
    for trade in trades {
        println!(
            "Price: ${}, Size: {} BTC, Maker: {}, Taker: {}",
            trade.price, trade.size, trade.maker_order_id, trade.taker_order_id
        );
    }

    Ok(())
}
