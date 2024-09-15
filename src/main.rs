mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order};
use rust_decimal_macros::dec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let price = Price::new(50.5);
    // let mut limit = Limit::new(65);
    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_bob = Order::new(BidOrAsk::Bid, 2.5);
    // let sell_order = Order::new(BidOrAsk::Ask, 2.5);
    // limit.add_order(buy_order);
    // limit.add_order(sell_order);
    let mut orderbook = matching_engine::orderbook::Orderbook::new();
    orderbook.add_limit_order(dec!(4.4), buy_order);
    orderbook.add_limit_order(dec!(4.4), buy_order_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_limit_order(dec!(20.00), sell_order);

    // println!("==> {:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order_1 = Order::new(BidOrAsk::Bid, 6.5);
    // let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    engine.place_limit_order(pair, dec!(10.000), buy_order_1)?;

    Ok(())
}
