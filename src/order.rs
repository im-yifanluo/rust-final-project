//core orderbook logic goes here, states, and updates 
//rust is automatically set as private, so set everything as pub to make it public
use crate::types::{Order, Type};

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tp = match self.order_type {
            Type::Bid => "BUY", 
            Type::Ask => "SELL",
        };
        write!(f, "{} | {} | {} @ {} | {}", self.id, self.owner, tp, self.price, self.quantity)
    }
}

impl Type {
    fn eq(&self, other: Type) -> bool {
        match (self, other) {
            (Type::Bid, Type::Bid) => true,
            (Type::Ask, Type::Ask) => true,
            _ => false,
        }
    }
}

impl Order {
    pub fn new(id: i32, owner: String, price_cent: i32, quantity: i32, order_type: Type) -> Order {
        Order {
            id,
            owner,
            price_cent,
            quantity,
            order_type,
        }
    }

    // Function that converts the price in cents to dollar
    pub fn price_dollar(&self) -> f64 {
        self.price_cent as f64 / 100.0
    }
}