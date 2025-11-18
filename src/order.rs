//core orderbook logic goes here, states, and updates 
use std::vec::Vec; 
use std::cmp::Ordering; 
use std::time::{SystemTime, UNIX_EPOCH};
//rust is automatically set as private, so set everything as pub to make it public

pub enum OrderType {
    Buy, 
    Sell,
}

#[derive(Clone)]
pub struct OrderHistoryEvent {
    pub description: String, 
    pub timestamp: u128, 
}
pub struct Order {
    pub id: i32, 
    pub price: f64, 
    pub quantity: u32, 
    pub order_history: Vec<OrderHistoryEvent>, 
    pub timestamp: u128, 
    pub order_type: OrderType, 
    pub limit: f64, //maybe add a limit? 
}

impl Order {
    pub fn new(id: i32,
    price: f64, 
    quantity: u32, 
    timestamp: u128, 
    order_type: OrderType, 
    limit: f64) -> Self {
        Self {
            id, 
            price, 
            quantity, 
            timestamp, 
            order_history: Vec::new(), 
            order_type, 
            limit, 
        }
    }

    pub fn increase_quantity(&mut self, amount: u32) {
        self.quantity += amount;
        let event = OrderHistoryEvent {
            description: format!("Quantity has been increased by {}", amount),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH,).unwrap().as_millis(),
        }; 
        self.order_history.push(event); 
    }

    pub fn reduce_quantity(&mut self, amount: u32) {
        if (amount > self.quantity) {
            self.quantity = 0; 
        } else {
            self.quantity -= amount;
        }
        let event = OrderHistoryEvent {
            description: format!("Quantity has been reduced by {}", amount),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH,).unwrap().as_millis(),  
        }; 
        self.order_history.push(event); 
    }

    pub fn order_is_filled(&mut self) -> bool {
        return self.quantity == 0
    } 

     //needs to implement clone trait to use clone
    pub fn get_order_history(&self) -> Vec<OrderHistoryEvent> {
        self.order_history.clone()
    }

    pub fn add_order_history(&mut self, info: String) {
        let event = OrderHistoryEvent {
            description: info, 
            timestamp: self.timestamp,   
        };
        self.order_history.push(event);
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// impl PartialOrd for Order {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other)) //calls the ord trait which I still need to implment
//     }
// }

//add more comparisons later
