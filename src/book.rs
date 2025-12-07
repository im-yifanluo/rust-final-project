use std::{collections::BTreeMap, vec::Vec};

use crate::order::{Order, Type};

// pub struct Order {
//     pub id: i32, 
//     pub owner: String,
//     pub price: f64, 
//     pub quantity: i32, 
//     pub order_type: Type,
// }

pub struct Book {
    name: String,
    bids: BTreeMap<i32, Order>,
    asks: BTreeMap<i32, Order>,
    orders: Vec<Order>,
    owners: Vec<String>,
}

impl Book {
    pub fn new(name: String) -> Book {
        Book {
            name,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            orders: Vec::new(),
            owners: Vec::new(),
        }
    }

    pub fn get_orders(&self) -> &Vec<Order> {
        &self.orders
    }

    pub fn add_order(&mut self, order_type: Type, id: i32, owner: String, price: f64, quantity: i32) {
        // add order logic
        let new_order = Order {
            id: id,
            owner: owner.clone(),
            price: price,
            quantity: quantity,
            order_type: order_type.clone(),
        };

        self.orders.push(new_order.clone());
        self.owners.push(owner);

        if order_type == Type::Bid {
            self.bids.insert(id, new_order);
        } else {
            self.asks.insert(id, new_order);
        }
    }

    pub fn add_bid(&mut self, id: i32, owner: String, price: f64, quantity: i32) {
        self.add_order(Type::Bid, id, owner, price, quantity);
    }

    pub fn add_ask(&mut self, id: i32, owner: String, price: f64, quantity: i32) {
        self.add_order(Type::Ask, id, owner, price, quantity);
    }
}

 

