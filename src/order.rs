//core orderbook logic goes here, states, and updates 
use std::vec::Vec; 
//rust is automatically set as private, so set everything as pub to make it public
pub struct Order {
    pub id: i32, 
    pub price: f64, 
    pub quantity: i32, 
    pub order_history: Vec<String>, 
}