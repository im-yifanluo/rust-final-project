//core orderbook logic goes here, states, and updates 
//rust is automatically set as private, so set everything as pub to make it public
use crate::types::{Order, Type, Trade};

use std::fmt;

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tp = match self.order_type {
            Type::Bid => "BUY", 
            Type::Ask => "SELL",
        };
        write!(f, "{} | {} | {} @ ${} | Quantity: {}", self.id, self.owner, tp, self.price_dollar(), self.quantity)
    }
}

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (bid id) has traded {} shares for ${} with {} (ask id)", self.bid_order_id, self.quantity, self.price_dollar(), self.ask_order_id)
    }
}

impl Order {
    // Function that converts the price in cents to dollar
    pub fn price_dollar(&self) -> f64 {
        self.price_cent as f64 / 100.0
    }
}
