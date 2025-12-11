mod order;
mod book;
mod types;

use crate::types::{Book};

use std::io; 

fn print_menu(book: &Book) {
    println!(); 
    println!("===============================");
    println!("{} Order Book", book.name);
    println!("===============================");
    println!("What would you like to do today?"); 
    println!(" 1) Buy \n 2) Sell \n 3) View Live Bids \n 4) View Live Asks \n 5) Past Orders \n 6) Past Trades \n 7) Quit\n"); 
}

fn handle_buy(book: &mut Book, id: &mut i32) {
    println!("Please enter order in format: <qty> <price_in_cents> <username>"); 
    
    let mut input: String = String::new(); 
    println!(">>>");
    io::stdin().read_line(&mut input).expect("Invalid Format, pelase enter in <qty> <price_in_cents> <username> (eg.: 3 50 Suhaan)"); 

    let parts: Vec<&str> = input.trim().split_whitespace().collect(); 
    if parts.len() != 3 {
        println!("Invalid Format, pelase enter in <qty> <price_in_cents> <username> (eg.: 3 50 Suhaan)");
        return; 
    }

    let quantity : i32 = parts[0].trim().parse().expect("Invalid Quantity");
    let price_cent : i32 = parts[1].trim().parse().expect("Invalid Price");
    let owner : &str = parts[2].trim();

    let trades_vec = book.add_bid(*id, owner.to_string(), price_cent, quantity);

    match trades_vec {
        Some(trades) => {
            for trade in trades {
                 println!("A trade has successfully occurred between {} (bid id) and {} (ask id) for {} shares at ${:.2}!",
                    trade.bid_order_id, 
                    trade.ask_order_id,
                    trade.quantity,
                    trade.price as f64 / 100 as f64, 
                );
            }
        }
        None => {
          println!("Your buy order has been added!"); 
        }
    }
    *id += 1; 
}

fn handle_sell(book: &mut Book, id: &mut i32) {
    println!("Please enter order in format: <qty> <price_in_cents> <username>"); 
    
    let mut input: String = String::new(); 
    println!(">>>");
    io::stdin().read_line(&mut input).expect("Invalid Format, pelase enter in <qty> <price_in_cents> <username> (eg.: 3 50 Suhaan)"); 

    let parts: Vec<&str> = input.trim().split_whitespace().collect(); 
    if parts.len() != 3 {
        println!("Invalid Format, pelase enter in <qty> <price_in_cents> <username> (eg.: 3 50 Suhaan)");
        return; 
    }

    let quantity : i32 = parts[0].trim().parse().expect("Invalid Quantity");
    let price_cent : i32 = parts[1].trim().parse().expect("Invalid Price");
    let owner : &str = parts[2].trim();

    let trades_vec = book.add_ask(*id, owner.to_string(), price_cent, quantity);

     match trades_vec {
        Some(trades) => {
            for trade in trades {
                 println!("A trade has successfully occurred between {} (buyer) and {} (seller) for {} shares at ${:.2}!",
                    trade.bid_order_id, 
                    trade.ask_order_id,
                    trade.quantity,
                    trade.price as f64 / 100 as f64, 
                );
            }
        }
        None => {
          println!("Your ask order has been added!"); 
        }
    }
    *id += 1; 
}

fn view_bids(book: &Book) {
    println!(" ===== Bids ===== "); 
    for (price, ord) in book.get_bids() {
        let price = price.0; 
        println!("Price: ${} | Order ID: {}", price as f64 / 100 as f64, ord); 
    }
}

fn view_asks(book: &Book) {
    println!(" ===== Asks ===== "); 
    for (price, ord) in book.get_asks() {
        println!("Price: ${} | Order ID: {}", *price as f64 / 100 as f64, ord); 
    }
}

fn view_history(book : &Book) {
    println!(" ===== Transaction History ===== "); 
    for ord in book.get_orders() {
        println!("{}", ord); 
    }
}

fn view_trades(book: &Book) {
    println!(" ===== Trades History ===== "); 
    for trade in book.get_trades() {
        println!("{}", trade); 
    }
}

fn main() {
    // Test code for order
    println!("Welcome to the Order Book designed and implemented by Suhaan Khan and Yifan Luo!");

    let mut new_order_book: Book = Book::new(
        "FMLT".to_string(),
    );

    let mut id: i32 = 1; 
    
    loop {
        print_menu(&mut new_order_book); 
        let mut input: String = String::new(); 
        println!(">>>");
        io::stdin().read_line(&mut input).expect("Invalid Character Entered: Please enter a number from 1-4"); 

        let num: i32 = input.trim().parse().expect("Invalid Character Entered: Please enter a number from 1-4"); 
        match num {
            1 => handle_buy(&mut new_order_book, &mut id), 
            2 => handle_sell(&mut new_order_book, &mut id),
            3 => view_bids(&new_order_book),
            4 => view_asks(&new_order_book),  
            5 => view_history(&new_order_book), 
            6 => view_trades(&new_order_book), 
            7 => {
                    println!("See you later...");
                    break; 
                 },
            _ => println!("Invalid Character Entered: Please enter a number from 1-4"), 
        }
    }
}