use std::{collections::BTreeMap, vec::Vec, cmp::Reverse};

#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Bid,
    Ask,
}

// A limit order
#[derive(Clone)]
pub struct Order {
    pub id: i32, 
    pub owner: String,
    pub price_cent: i32, // The price of an order is an integer to be usable in BTreeMap
    pub quantity: i32, 
    pub order_type: Type,
}

// An order book containing orders
pub struct Book {
    pub name: String, // Name of the book (stock, crypto, etc.)
    pub bids: BTreeMap<Reverse<i32>, Order>, // Actual live book bids, in reversed BTreeMap to keep highest bid at front
    pub asks: BTreeMap<i32, Order>, // Actual live book asks, in BTreeMap to keep lowest ask at front
    pub orders: Vec<Order>, // All historical orders ever placed
    pub trades: Vec<Trade>, // All historical trades ever made
}

// A trade is created after orders are matched
#[derive(Clone)]
pub struct Trade {
    pub bid_order_id: i32,
    pub ask_order_id: i32,
    pub price: i32,
    pub quantity: i32,
}