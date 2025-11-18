mod order;
mod book;

use crate::order::Order;
use crate::book::Book;

fn main() {
    // Test code for order
    println!("Welcome to the Order Book designed and implemented by Yifan Luo and Suhaan Khan");

    let mut new_order_book: Book = Book::new(
        "APPL".to_string(),
    );

    new_order_book.add_ask(1, "Yifan".to_string(), 267.44, 100);
}