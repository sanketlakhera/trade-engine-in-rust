use super::errors::{MatchingEngineError, MatchingEngineResult};
use super::types::{Order, OrderSide, OrderType};
use rust_decimal::Decimal;

pub struct OrderValidator;

impl OrderValidator {
    pub fn validate_new_order(order: &Order) -> MatchingEngineResult<()> {
        // Validate size
        if order.size <= Decimal::ZERO {
            return Err(MatchingEngineError::InvalidOrderSize);
        }

        // Validate price for limit orders
        if let Some(price) = order.price {
            if price <= Decimal::ZERO {
                return Err(MatchingEngineError::InvalidOrderPrice);
            }
        }

        // Validate order type specific rules
        match order.order_type {
            OrderType::GTC => Self::validate_gtc_order(order),
            OrderType::IOC => Self::validate_ioc_order(order),
            OrderType::FOK => Self::validate_fok_order(order),
        }
    }

    fn validate_gtc_order(order: &Order) -> MatchingEngineResult<()> {
        // GTC orders must have a price (limit orders only)
        if order.price.is_none() {
            return Err(MatchingEngineError::InvalidOrderType(
                "GTC orders must be limit orders".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_ioc_order(_order: &Order) -> MatchingEngineResult<()> {
        // IOC orders can be either market or limit
        Ok(())
    }

    fn validate_fok_order(order: &Order) -> MatchingEngineResult<()> {
        // FOK orders must have a price (limit orders only)
        if order.price.is_none() {
            return Err(MatchingEngineError::InvalidOrderType(
                "FOK orders must be limit orders".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_order_cancellation(order: &Order, is_filled: bool) -> MatchingEngineResult<()> {
        if is_filled {
            return Err(MatchingEngineError::OrderAlreadyFilled);
        }

        match order.order_type {
            OrderType::GTC => Ok(()),
            _ => Err(MatchingEngineError::InvalidOrderCancellation(
                "Only GTC orders can be cancelled".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matching_engine::types::TradingPair;
    use rust_decimal_macros::dec;

    #[test]
    fn test_validate_gtc_order() {
        let pair = TradingPair::new("BTC", "USD");

        // Valid GTC order
        let valid_order = Order::new_limit(
            "order1",
            pair.clone(),
            OrderSide::Bid,
            dec!(50000),
            dec!(1.0),
            OrderType::GTC,
        );
        assert!(OrderValidator::validate_new_order(&valid_order).is_ok());

        // Invalid GTC order (market order)
        let invalid_order = Order {
            id: "order2".to_string(),
            trading_pair: pair,
            side: OrderSide::Bid,
            price: None,
            size: dec!(1.0),
            filled_size: Decimal::ZERO,
            order_type: OrderType::GTC,
            timestamp: std::time::SystemTime::now(),
        };
        assert!(OrderValidator::validate_new_order(&invalid_order).is_err());
    }

    #[test]
    fn test_validate_order_size() {
        let pair = TradingPair::new("BTC", "USD");

        // Invalid order size
        let invalid_order = Order::new_limit(
            "order1",
            pair,
            OrderSide::Bid,
            dec!(50000),
            dec!(0),
            OrderType::GTC,
        );
        assert!(matches!(
            OrderValidator::validate_new_order(&invalid_order),
            Err(MatchingEngineError::InvalidOrderSize)
        ));
    }

    #[test]
    fn test_validate_order_cancellation() {
        let pair = TradingPair::new("BTC", "USD");

        // Valid GTC order cancellation
        let gtc_order = Order::new_limit(
            "order1",
            pair.clone(),
            OrderSide::Bid,
            dec!(50000),
            dec!(1.0),
            OrderType::GTC,
        );
        assert!(OrderValidator::validate_order_cancellation(&gtc_order, false).is_ok());

        // Invalid IOC order cancellation
        let ioc_order = Order::new_market("order2", pair, OrderSide::Ask, dec!(1.0));
        assert!(OrderValidator::validate_order_cancellation(&ioc_order, false).is_err());
    }
}
