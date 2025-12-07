//core orderbook logic goes here, states, and updates 
//rust is automatically set as private, so set everything as pub to make it public

use std::fmt; 

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Bid,
    Ask,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub id: i32, 
    pub owner: String,
    pub price: f64, 
    pub quantity: i32, 
    pub order_type: Type,
}

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