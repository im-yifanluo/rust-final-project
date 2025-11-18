//core orderbook logic goes here, states, and updates 
//rust is automatically set as private, so set everything as pub to make it public

#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Bid,
    Ask,
}

#[derive(Clone)]
pub struct Order {
    pub id: i32, 
    pub owner: String,
    pub price: f64, 
    pub quantity: i32, 
    pub order_type: Type,
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