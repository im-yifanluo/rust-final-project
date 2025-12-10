mod order;
mod book;
mod types;

use crate::types::{Order, Type, Book, Trade};

use rand::Rng; 
use std::io; 

fn print_menu() {
    println!(); 
    println!("===============================");
    println!(" Order Book Menu ");
    println!("===============================");
    println!("What would you like to do today?"); 
    println!(" 1) View Order Book \n 2) Buy <qty> <price> \n 3) Sell <qty> <price> \n 4) Quit\n"); 
}

fn handle_buy(book: &mut Book) {
    let mut rng = rand::thread_rng(); 
    let id: i32 = rng.gen_range(1..=100); 

    let owner : &str = "User"; 

    println!("Please enter order in format: <qty> <price>"); 
    
    let mut input: String = String::new(); 
    println!(">>>");
    io::stdin().read_line(&mut input).expect("Invalid Format, pelase enter in <qty> <price> (eg.: 3 50)"); 

    let parts: Vec<&str> = input.trim().split_whitespace().collect(); 
    if parts.len() != 2 {
        println!("Invalid Format, pelase enter in <qty> <price> (eg.: 3 50)");
        return; 
    }

    let quantity : i32 = parts[0].trim().parse().expect("Invalid Quantity");
    let price : i32 = parts[1].trim().parse().expect("Invalid Price");

    //must add owner functionality later
    book.add_bid(id, owner.to_string(), price, quantity);

    println!("Your buy order has been added!"); 
}

fn main() {
    // Test code for order
    println!("Welcome to the Order Book designed and implemented by Suhaan Khan and Yifan Luo!");

    let mut new_order_book: Book = Book::new(
        "APPL".to_string(),
    );

    new_order_book.add_ask(1, "Yifan".to_string(), 267, 100);
    
    
    print_menu(); 
    let mut input: String = String::new(); 
    println!(">>>");
    io::stdin().read_line(&mut input).expect("Invalid Character Entered: Please enter a number from 1-4"); 

    let num: i32 = input.trim().parse().expect("Invalid Character Entered: Please enter a number from 1-4"); 
    match num {
        1 => println!("You selected View Order Book!"), 
        2 => handle_buy(&mut new_order_book),  
        3 => println!("You selected Sell!"), 
        4 => println!("See you later..."),
        _ => println!("Invalid Character Entered: Please enter a number from 1-4"), 
    }
}