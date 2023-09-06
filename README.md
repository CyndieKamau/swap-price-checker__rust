# Cyndie Dex Mock DeFi Tool


```sh
              Welcome to ...


                                          
  ___  _  _  __ _  ____  __  ____    ____  ____  _  _ 
 / __)( \/ )(  ( \(    \(  )(  __)  (    \(  __)( \/ )
( (__  )  / /    / ) D ( )(  ) _)    ) D ( ) _)  )  ( 
 \___)(__/  \_)__)(____/(__)(____)  (____/(____)(_/\_)


              Thank you for choosing Cyndie Dex!!!

              Cyndie Dex is designed to be user-friendly and to guide you through the world of token swaps.

              Here's what you should know about Cyndie Dex;

              1. **Decentralized Exchanges (DEX)**: Cyndie Dex aggregates prices from various decentralized exchanges 
              to ensure you get the best rates.

              2. **Swapping Tokens**: At the heart of this tool, you can seamlessly swap tokens from one type to another. 
              Simply input the desired amount, and Cyndie Dex will find you the best rates and even calculate potential slippage!
           
              ⚠️ ----N.B.!!!--- Cyndie Dex is a Mock Simulator; the token prices do not reflect current prices. DYOR!!            
  
              🦀 Built with Love in Rust 🦀
    


```



# Introduction

Cyndie Dex is a mock Decentralized Exchange (DEX) tool designed to simulate cryptocurrency token swaps. 

It allows users to interact with different tokens, execute swaps, and determine the best exchange rate across multiple exchanges.

While this tool operates in a mock environment, it provides valuable insights into the workings of a real-world DEX.



## Project's Objectives

* Compare swap prices from multiple mock exchanges -> (Uniswap, Cowswap, Carbon, Sushi, Matcha) 

* Present the best swap rate for the user.

N.B. The mock tool initially focuses on USDC, USDT and BUSD.


## Features

* **User Registration:** Allows users to enter their wallet address and select a preferred network.

* **Token Swapping:** Enables users to swap between different cryptocurrency tokens and view the best exchange rate available.

* **Dynamic Pricing:** Cyndie Dex fetches mock data from multiple exchanges to determine the best swap rate for users.

* **Interactive UI:**  Simple and user-friendly interface that guides users through the swap process.




# Getting Started

## Prerequisites

You need to have rust installed in your system.

For installation purposes [click here.](https://www.rust-lang.org/tools/install)


## Installation

1. Clone the repository

```sh
$ git clone https://github.com/CyndieKamau/swap-price-checker__rust.git

```

2. Navigate to project's directory to build the project

```sh
$ cd swap-price-checker

$ cargo build
```


Successful installation looks like this;

```sh

Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `/home/hp/Desktop/swap-price-checker__rust/swap-price-checker/target/debug/swap-price-checker`


 
                        Welcome to ...


                                          
  ___  _  _  __ _  ____  __  ____    ____  ____  _  _ 
 / __)( \/ )(  ( \(    \(  )(  __)  (    \(  __)( \/ )
( (__  )  / /    / ) D ( )(  ) _)    ) D ( ) _)  )  ( 
 \___)(__/  \_)__)(____/(__)(____)  (____/(____)(_/\_)


              Thank you for choosing Cyndie Dex!!!

              Cyndie Dex is designed to be user-friendly and to guide you through the world of token swaps.

              Here's what you should know about Cyndie Dex;

              1. **Decentralized Exchanges (DEX)**: Cyndie Dex aggregates prices from various decentralized exchanges 
              to ensure you get the best rates.

              2. **Swapping Tokens**: At the heart of this tool, you can seamlessly swap tokens from one type to another. 
              Simply input the desired amount, and Cyndie Dex will find you the best rates and even calculate potential slippage!
           
              ⚠️ ----N.B.!!!--- Cyndie Dex is a Mock Simulator; the token prices do not reflect current prices. DYOR!!            
  
              🦀 Built with Love in Rust 🦀
    
Please enter your wallet address (or type 'exit' to quit):
0x123fghu900mr56hjkuAxnm890nrop4iklm233nkk
Please select a network (1. Ethereum, 2. BNBChain, 3. Polygon):
1
User created with random balances!
--- User Menu ---
1. View balances
2. Initiate swap
3. Exit
Select an option:

1
Your balances are: {USDC: 1767.0, BUSD: 4600.0, USDT: 7628.0}
--- User Menu ---
1. View balances
2. Initiate swap
3. Exit
Select an option:

2
Choose the token you want to swap FROM:
1. USDC
2. USDT
3. BUSD

3
Choose the token you want to swap TO:
1. USDC
2. USDT
3. BUSD

1
Enter the amount you want to swap:
1500
Swap Successful! Best exchange: Uniswap. Received amount: 1499.7 with slippage of 1%
--- User Menu ---
1. View balances
2. Initiate swap
3. Exit
Select an option:

3
Please enter your wallet address (or type 'exit' to quit):
exit


```


