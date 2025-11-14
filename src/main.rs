mod order;
use crate::order::Order;

fn main() {
    println!("Hello, world!");

    let user1 =  Order {
        id: 1,   
        price: 30.40, 
        quantity: 8, 
        order_history: Vec::new(), 
    };

    println!("{}", user1.id);
    println!("{}", user1.price); 
    println!("{}", user1.quantity);
}
