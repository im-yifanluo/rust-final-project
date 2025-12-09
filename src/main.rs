mod order;
mod book;

use crate::order::Order;
use crate::book::Book;

use std::io; 

fn print_menu() {
    println!(); 
    println!("===============================");
    println!(" Order Book Menu ");
    println!("===============================");
    println!("What would you like to do today?"); 
    println!(" 1) View Order Book \n 2) Buy \n 3) Sell \n 4) Quit\n"); 
}

fn handle_buy(book: &mut Book, id: &mut i32) {
    println!("Please enter order in format: <qty> <price> <username>"); 
    
    let mut input: String = String::new(); 
    println!(">>>");
    io::stdin().read_line(&mut input).expect("Invalid Format, pelase enter in <qty> <price> <username> (eg.: 3 50 Suhaan)"); 

    let parts: Vec<&str> = input.trim().split_whitespace().collect(); 
    if parts.len() != 3 {
        println!("Invalid Format, pelase enter in <qty> <price> (eg.: 3 50 Suhaan)");
        return; 
    }

    let quantity : i32 = parts[0].trim().parse().expect("Invalid Quantity");
    let price : f64 = parts[1].trim().parse().expect("Invalid Price");
    let owner : &str = parts[2].trim();

    //must add owner functionality later
    book.add_bid(*id, owner.to_string(), price, quantity);

    println!("Your buy order has been added!"); 
    *id += 1; 
}

fn handle_sell(book: &mut Book, id: &mut i32) {
    println!("Please enter order in format: <qty> <price> <username>"); 
    
    let mut input: String = String::new(); 
    println!(">>>");
    io::stdin().read_line(&mut input).expect("Invalid Format, pelase enter in <qty> <price> (eg.: 3 50 Suhaan)"); 

    let parts: Vec<&str> = input.trim().split_whitespace().collect(); 
    if parts.len() != 3 {
        println!("Invalid Format, pelase enter in <qty> <price> (eg.: 3 50 Suhaan)");
        return; 
    }

    let quantity : i32 = parts[0].trim().parse().expect("Invalid Quantity");
    let price : f64 = parts[1].trim().parse().expect("Invalid Price");
    let owner : &str = parts[2].trim();

    //must add owner functionality later
    book.add_ask(*id, owner.to_string(), price, quantity);

    println!("Your ask order has been added!"); 
    *id += 1; 
}

fn view_history(book : &Book) {
    println!(" ===== Transaction History ===== "); 
    for ord in book.get_orders() {
        println!("{}", ord); 
    }
}

fn main() {
    // Test code for order
    println!("Welcome to the Order Book designed and implemented by Suhaan Khan and Yifan Luo!");

    let mut new_order_book: Book = Book::new(
        "APPL".to_string(),
    );

    let mut id: i32 = 1; 
    
    loop {
        print_menu(); 
        let mut input: String = String::new(); 
        println!(">>>");
        io::stdin().read_line(&mut input).expect("Invalid Character Entered: Please enter a number from 1-4"); 

        let num: i32 = input.trim().parse().expect("Invalid Character Entered: Please enter a number from 1-4"); 
        match num {
            1 => view_history(&new_order_book), 
            2 => handle_buy(&mut new_order_book, &mut id),  
            3 => handle_sell(&mut new_order_book, &mut id), 
            4 => {
                    println!("See you later...");
                    break; 
                 },
            _ => println!("Invalid Character Entered: Please enter a number from 1-4"), 
        }
    }
}