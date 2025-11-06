# CS 128 Honors Final Project: Order Book

Group Name: KL Coders

Group Members: Yifan Luo (yifan65), Suhaan Khan (suhaank2)

## Project Introduction
Consider when you want to buy a stock at a specific price, how is your trade fulfilled by the exchange? This is done by an order book, which lists the current buy orders (bids) and the sell orders (asks) for a specific asset. This can include stocks, crypto, commodities, futures, derivatives, or essentially anything that can be traded, and this mechanism is used by nearly every financial exchange/institution.

Modern orderbooks show the price buyers and sellers are willing pay and the units that they are willing to buy or sell. The exchange uses this information to match the interested buyers and sellers, making trades possible.

The scope of this project is to implement a limit order book and simple trade matching in Rust. By doing this project, we will be able to gain new insights on how markets work and how trades are executed. Additionally, implementing this with Rust allows high performance, low latency, while ensuring memory safety. 


## Technical Overview
We will be implementing two major components: a console based limit order book for a specific asset, and a matching engine to facilitate the trades.

The following structs/implementations would need to be created:
 - struct Order: Contains the information of each specific order.
 - struct Book: Contains all the orders of bids and asks.
 - impl Book: This is the matching engine that matches and facilitates the orders.
 - struct Trade: This includes the information of each trade.

Additionally, a command-line interface (CLI) will need to be implemented to display the order book.

Timeline:
 - Checkpoint 1: Finish Order, Book, and basic CLI that takes inputs.
 - Checkpoint 2: FInish matching engine and Trade, with real time trades and matching.

## Possible Challenges

This is a technically exciting and challenging project with many components that require speed and performacne. Due to this, many challenges can arise:
 - The matching engine is difficult to implement, and it is challenging to determine when an order should be executed.
 - THe matching engine may also have issues at fast paced environments, when orders are added/cancelled quickly.
 - Input/output and implementing CLI is new to us and we will need to learn this.

## References
What is an order book: https://www.coinbase.com/learn/advanced-trading/what-is-an-order-book

How to Build an Exchange: https://www.janestreet.com/tech-talks/building-an-exchange/