use std::{collections::BTreeMap, vec::Vec, cmp::Reverse};

use crate::{types::{Book, Order, Trade, Type}};

impl Trade {
    pub fn new(bid_order_id: i32, ask_order_id: i32, price: i32, quantity: i32) -> Trade {
        Trade {
            bid_order_id,
            ask_order_id,
            price,
            quantity,
        }
    }
}

impl Book {
    pub fn new(name: String) -> Book {
        Book {
            name,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            orders: Vec::new(),
            trades: Vec::new(),
        }
    }
    /*
    Orders are executed as limit orders, meaning that you buy or sell a security at a specified price or better.
    An incoming order is attempted to be executed before it is stored in the order book.
    If the order, or some part of it, can be executed, trades are created and returned.
    If the order cannot be fully executed, it, or the remaining part of it, is stored on the order book
    */

    // ============ Adding Orders =============
    pub fn add_order(&mut self, order_type: Type, id: i32, owner: String, price: i32, quantity: i32) -> Option<Vec<Trade>> {
        // add order logic
        if order_type == Type::Bid {
            return self.add_bid(id, owner, price, quantity);
        }

        return self.add_ask(id, owner, price, quantity);
    }

    pub fn add_bid(&mut self, id: i32, owner: String, price: i32, quantity: i32) -> Option<Vec<Trade>> {
        let new_order = Order {
            id: id,
            owner: owner.clone(),
            price_cent: price,
            quantity: quantity,
            order_type: Type::Bid,
        };

        self.orders.push(new_order.clone());
        return self.match_bid(new_order); // Call matching
    }

    pub fn add_ask(&mut self, id: i32, owner: String, price: i32, quantity: i32) -> Option<Vec<Trade>> {
        let new_order = Order {
            id: id,
            owner: owner.clone(),
            price_cent: price,
            quantity: quantity,
            order_type: Type::Ask,
        };

        self.orders.push(new_order.clone());
        return self.match_ask(new_order); // Call matching
    }

    // ============ Making Trades (Core logic of Matching Engine) =============
    pub fn match_bid(&mut self, bid_order: Order) -> Option<Vec<Trade>> {

        // Case 1: the asks book is empty
        if self.asks.is_empty(){
            self.bids.insert(Reverse(bid_order.price_cent), bid_order);
            return None;
        }

        let lowest_ask = self.asks.first_key_value().unwrap();

        // Case 2: If the prices do not cross
        if &bid_order.price_cent < lowest_ask.0 {
            self.bids.insert(Reverse(bid_order.price_cent), bid_order);
            return None;
        }

        // Case 3: If the prices cross and quantity are equal
        if lowest_ask.1.quantity == bid_order.quantity {
            let ask_price_cent = lowest_ask.1.price_cent;

            let new_trade = Trade {
                bid_order_id: bid_order.id,
                ask_order_id: lowest_ask.1.id,
                price: lowest_ask.1.price_cent,
                quantity: bid_order.quantity,
            };

            let mut new_trades: Vec<Trade> = Vec::new();
            new_trades.push(new_trade.clone());
            self.trades.push(new_trade);
            self.asks.remove(&ask_price_cent);
            
            return Some(new_trades);
        }

        // Case 4: If the prices cross and the ask quantity is greater than bid 
        if lowest_ask.1.quantity > bid_order.quantity {
            let ask_id = lowest_ask.1.id;
            let ask_owner = lowest_ask.1.owner.clone();
            let ask_price_cent = lowest_ask.1.price_cent;
            let ask_quantity = lowest_ask.1.quantity;

            let new_trade = Trade {
                bid_order_id: bid_order.id,
                ask_order_id: ask_id,
                price: ask_price_cent,
                quantity: bid_order.quantity,
            };

            self.asks.remove(&ask_price_cent);

            let new_order = Order {
                id: ask_id,
                owner: ask_owner,
                price_cent: ask_price_cent,
                quantity: ask_quantity - bid_order.quantity,
                order_type: Type::Ask,
            };

            self.asks.insert(ask_price_cent, new_order);
            let mut new_trades: Vec<Trade> = Vec::new();
            new_trades.push(new_trade.clone());
            self.trades.push(new_trade);
            return Some(new_trades);
        }

        // Case 5: If price crosses and the ask quantity is lower than the bid
        let mut new_trades: Vec<Trade> = Vec::new();
        let mut curr_quantity = bid_order.quantity;

        while curr_quantity > 0 {
            let lowest_ask = match self.asks.first_key_value() {
                Some(ask) => ask,
                None => break,
            };

            if bid_order.price_cent < *lowest_ask.0 {
                break;
            }

            let ask_id = lowest_ask.1.id;
            let ask_owner = lowest_ask.1.owner.clone();
            let ask_price_cent = lowest_ask.1.price_cent;
            let ask_quantity = lowest_ask.1.quantity;
            
            if curr_quantity >= ask_quantity {
                let new_trade = Trade {
                    bid_order_id: bid_order.id,
                    ask_order_id: ask_id,
                    price: ask_price_cent,
                    quantity: ask_quantity,
                };

                self.asks.remove(&ask_price_cent);

                new_trades.push(new_trade.clone());
                self.trades.push(new_trade);

                curr_quantity -= ask_quantity;
            } else {
                let new_trade = Trade {
                    bid_order_id: bid_order.id,
                    ask_order_id: ask_id,
                    price: ask_price_cent,
                    quantity: curr_quantity,
                };

                self.asks.remove(&ask_price_cent);

                let new_order = Order {
                    id: ask_id,
                    owner: ask_owner,
                    price_cent: ask_price_cent,
                    quantity: ask_quantity - curr_quantity,
                    order_type: Type::Ask,
                };

                curr_quantity = 0;

                self.asks.insert(ask_price_cent, new_order);
                new_trades.push(new_trade.clone());
                self.trades.push(new_trade);
            }
        }

        if curr_quantity > 0 {
            let new_order = Order {
                id: bid_order.id,
                owner: bid_order.owner.clone(),
                price_cent: bid_order.price_cent,
                quantity: curr_quantity,
                order_type: Type::Bid,
            };

            self.bids.insert(Reverse(bid_order.price_cent), new_order);
        }
        
        Some(new_trades)
    }


    
    pub fn match_ask(&mut self, ask_order: Order) -> Option<Vec<Trade>> {
        // Case 1: the asks book is empty
        if self.bids.is_empty(){
            self.asks.insert(ask_order.price_cent, ask_order);
            return None;
        }

        let highest_bid = self.bids.first_key_value().unwrap();

        // Case 2: If the prices do not cross
        if ask_order.price_cent >highest_bid.1.price_cent {
            self.asks.insert(ask_order.price_cent, ask_order);
            return None;
        }

        // Case 3: If the prices cross and quantity are equal
        if highest_bid.1.quantity == ask_order.quantity {
            let bid_price_cent = highest_bid.1.price_cent;

            let new_trade = Trade {
                bid_order_id: highest_bid.1.id,
                ask_order_id: ask_order.id,
                price: highest_bid.1.price_cent,
                quantity: ask_order.quantity,
            };

            let mut new_trades: Vec<Trade> = Vec::new();
            new_trades.push(new_trade.clone());
            self.trades.push(new_trade);
            self.bids.remove(&Reverse(bid_price_cent));
            
            return Some(new_trades);
        }

        // Case 4: If the prices cross and the ask quantity is greater than bid 
        if highest_bid.1.quantity > ask_order.quantity {
            let bid_id = highest_bid.1.id;
            let bid_owner = highest_bid.1.owner.clone();
            let bid_price_cent = highest_bid.1.price_cent;
            let bid_quantity = highest_bid.1.quantity;

            let new_trade = Trade {
                bid_order_id: bid_id,
                ask_order_id: ask_order.id,
                price: bid_price_cent,
                quantity: ask_order.quantity,
            };

            self.bids.remove(&Reverse(bid_price_cent));

            let new_order = Order {
                id: bid_id,
                owner: bid_owner,
                price_cent: bid_price_cent,
                quantity: bid_quantity - ask_order.quantity,
                order_type: Type::Bid,
            };

            self.bids.insert(Reverse(bid_price_cent), new_order);
            let mut new_trades: Vec<Trade> = Vec::new();
            new_trades.push(new_trade.clone());
            self.trades.push(new_trade);
            return Some(new_trades);
        }

        // Case 5: If price crosses and the ask quantity is lower than the bid
        let mut new_trades: Vec<Trade> = Vec::new();
        let mut curr_quantity = ask_order.quantity;

        while curr_quantity > 0 {
            let highest_bid = match self.bids.first_key_value() {
                Some(bid) => bid,
                None => break,
            };

            if ask_order.price_cent > highest_bid.1.price_cent {
                break;
            }

            let bid_id = highest_bid.1.id;
            let bid_owner = highest_bid.1.owner.clone();
            let bid_price_cent = highest_bid.1.price_cent;
            let bid_quantity = highest_bid.1.quantity;
            
            if curr_quantity >= bid_quantity {
                let new_trade = Trade {
                    bid_order_id: bid_id,
                    ask_order_id: ask_order.id,
                    price: bid_price_cent,
                    quantity: bid_quantity,
                };

                self.bids.remove(&Reverse(bid_price_cent));

                new_trades.push(new_trade.clone());
                self.trades.push(new_trade);

                curr_quantity -= bid_quantity;
            } else {
                let new_trade = Trade {
                    bid_order_id: bid_id,
                    ask_order_id: ask_order.id,
                    price: bid_price_cent,
                    quantity: curr_quantity,
                };

                self.bids.remove(&Reverse(bid_price_cent));

                let new_order = Order {
                    id: bid_id,
                    owner: bid_owner,
                    price_cent: bid_price_cent,
                    quantity: bid_quantity - curr_quantity,
                    order_type: Type::Bid,
                };

                curr_quantity = 0;

                self.bids.insert(Reverse(bid_price_cent), new_order);
                new_trades.push(new_trade.clone());
                self.trades.push(new_trade);
            }
        }

        if curr_quantity > 0 {
            let new_order = Order {
                id: ask_order.id,
                owner: ask_order.owner.clone(),
                price_cent: ask_order.price_cent,
                quantity: curr_quantity,
                order_type: Type::Ask,
            };

            self.asks.insert(ask_order.price_cent, new_order);
        }
        
        Some(new_trades)
    }
}

 

