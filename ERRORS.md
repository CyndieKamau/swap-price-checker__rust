# List of all the errors I encountered building this project, and how I solved them;


# 1. Mismatched Types

My function `simulate_swap` was to return a `Result<f64, SwapError>` , but it returned nothing -> `()`.

The code snippet;

```rust
//simulate the token swap
    pub fn simulate_swap(&self, from: TokenType, to: TokenType, amount: f64) -> Result<f64, SwapError> {

        // the "?" will return early with an Err if the pair isn't supported
        self.check_token_pair(from, to)?;       

        for pair in &self.token_pairs {

            if pair.from_token == from && pair.to_token == to {

                if pair.liquidity as f64 >= amount {

                    return Ok(amount * pair.swap_rate);
                } else {

                    return Err(SwapError::NotEnoughLiquidity);
                }
            }
        }
    }
```

The error;

```sh
error[E0308]: mismatched types
   --> src/main.rs:128:9
    |
123 |       pub fn simulate_swap(&self, from: TokenType, to: TokenType, amount: f64) -> Result<f64, SwapError> {
    |                                                                                   ---------------------- expected `Result<f64, SwapError>` because of return type
...
128 | /         for pair in &self.token_pairs {
129 | |
130 | |             if pair.from_token == from && pair.to_token == to {
131 | |
...   |
139 | |             }
140 | |         }
    | |_________^ expected `Result<f64, SwapError>`, found `()`
    |
 = note:   expected enum `Result<f64, SwapError>`
            found unit type `()`
note: the function expects a value to always be returned, but loops might run zero times
   --> src/main.rs:128:9
    |
128 |         for pair in &self.token_pairs {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this might have zero elements to iterate on
...
134 |                     return Ok(amount * pair.swap_rate);
    |                     ---------------------------------- if the loop doesn't execute, this value would never get returned
...
137 |                     return Err(SwapError::NotEnoughLiquidity);
    |                     ----------------------------------------- if the loop doesn't execute, this value would never get returned
 = help: return a value for the case when the loop has zero elements to iterate on, or consider changing the return type to account for that possibility
help: try adding an expression at the end of the block
    |
140 ~         }
141 +         Ok(())
```

## Issue

First the program is checking whether a token pair exists, `from: TokenType`, `to: TokenType`, so if a user wants to swap from usdc to usdt it checks if both from and to exists as a swapping pair. 

The `?` function returns an error is the pair doesn't.

```rust
self.check_token_pair(from, to)?;
```

We then enter into a loop, where if the token to be swapped from matches our from (`from: TokenType`), and token swapped to matches to (`to: TokenType`);

```rust
for pair in &self.token_pairs {
        if pair.from_token == from && pair.to_token == to {
        ....
        }
....
}    
```

and the liquidity pools of these tokens are bigger than the amount being swapped;

```rust
    if pair.liquidity as f64 >= amount {
                return Ok(amount * pair.swap_rate);
            } else {
                return Err(SwapError::NotEnoughLiquidity);

            }
    }
```  
Then it would return an `Ok()` and performing the swap function, but if the liquidity pool is not enough, return an error of `NotEnoughLiquidity`.

Notice I didn't handle the situation where the loop finishes but finds no match pair (despite having returned early an error if matching pair is not found).

That's what is bringing the error, as we promised the function will either return an amount(f64), or an error (SwapError).

## Solution

Add a return statement after the for loop to handle the Error where a token pair is not supported.

```rust
pub fn simulate_swap(&self, from: TokenType, to: TokenType, amount: f64) -> Result<f64, SwapError> {

    // the "?" will return early with an Err if the pair isn't supported
    self.check_token_pair(from, to)?;       

    for pair in &self.token_pairs {
        if pair.from_token == from && pair.to_token == to {
            if pair.liquidity as f64 >= amount {
                return Ok(amount * pair.swap_rate);
            } else {
                return Err(SwapError::NotEnoughLiquidity);
            }
        }
    }

    // If the loop completes without returning, it means no matching pair was found
    Err(SwapError::TokenPairNotSupported)
}


```

# 2. the trait bound `TokenType: Eq` is not satisfied

This error touches on traits in Rust.

Let's check out my enum, `TokenType`;

```rust
#[derive(Debug, PartialEq, Clone, Copy)]
//To represent our two tokens, USDC and USDT
enum TokenType {

   USDC,
   USDT,
   BUSD,

}
```

So for my `TokenType` enum, I had implemented the `Debug, PartialEq, Clone, Copy` traits in my `Derive` macro;

* `Debug` for displaying the enum contents (meant for programmers)

* `PartialEq` for [Partial Equivalence Relation](https://en.wikipedia.org/wiki/Partial_equivalence_relation); something about two items here can be compared, but it doesn't mean x ==x

* `Clone`  We can make a copy of the enum and its variants (USDC, USDT, BUSD) when we need to

* `Copy` If the compiler makes a bit-for-bit copy of this item's memory representation, the result is a valid new item.


Let's go through the error;

```sh

error[E0277]: the trait bound `TokenType: Eq` is not satisfied
    --> src/main.rs:468:22
     |
468  |             balances.insert(*token, (rand::random::<u64>() % 10_000) as f64); // Randomly assigns 0 to 9999 tokens
     |                      ^^^^^^ the trait `Eq` is not implemented for `TokenType`
     |
note: required by a bound in `HashMap::<K, V, S>::insert`
    --> /home/hp/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/collections/hash/map.rs:733:8
     |
733  |     K: Eq + Hash,
     |        ^^ required by this bound in `HashMap::<K, V, S>::insert`
...
1102 |     pub fn insert(&mut self, k: K, v: V) -> Option<V> {
     |            ------ required by a bound in this associated function
help: consider annotating `TokenType` with `#[derive(Eq)]`
     |
8    + #[derive(Eq)]
9    | enum TokenType {

```

The error interestingly occured not in my enum, but while implementing my `User` logic;

```rust
impl User {

    //create a new user with random balances
    pub fn new(network: Network, wallet_address: String) -> Self {

        let mut balances = HashMap::new();
    

    
        // Generate random balances for each token
        for token in [TokenType::USDC, TokenType::USDT, TokenType::BUSD].iter() {

            balances.insert(*token, (rand::random::<u64>() % 10_000) as f64); // Randomly assigns 0 to 9999 tokens
        }

        User {

            network,
            wallet_address,
            balances
        }
    }

//rest of my logic

}
```

Note: The compiler had already given me the answer;

```sh
help: consider annotating `TokenType` with `#[derive(Eq)]`
```

But let's go through why the error came by while implementing my `User` logic.

## Issue

In Rust we have _data types_; Scalar (int, float, bool, char etc), and Compound(array[], tuple()).

Then we have data structures called _collections_;

* Vector (`Vec<T>`) -> stores a list of values of the same type (T) next to each other in memory.

* String (N.B Rust has different string types String, str etc) -> a collection of characters (UTF-8 encoded)

* Hash Map (`HashMap<K, V>` -> Stores values with their appropiate keys. 

N.B. The difference between _collections_ with arrays and tuples  is in collections data is stored in the heap,in arrays and tuples stack. 


For my struct `User`, the user's balance was stored in a hashmap, since the user's wallet address was to hold usdc, usdt and busd, and the `TokenType` was the key for getting the balances.

```rust
#[derive(Clone)]
struct User {

    network: Network,
    wallet_address: String,
    balances: HashMap<TokenType, f64>,

}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
//To represent our two tokens, USDC and USDT
enum TokenType {

   USDC,
   USDT,
   BUSD,

}

``` 

So when I initialized an empty HashMap while implementing the `User` struct;

```rust

//Implementing user logic

impl User {

    //create a new user with random balances
    pub fn new(network: Network, wallet_address: String) -> Self {

        let mut balances = HashMap::new();     //here's where I initialized the Hash Map 
    

    
        // Generate random balances for each token
        for token in [TokenType::USDC, TokenType::USDT, TokenType::BUSD].iter() {

            balances.insert(*token, (rand::random::<u64>() % 10_000) as f64); // Randomly assigns 0 to 9999 tokens
        }

        User {

            network,
            wallet_address,
            balances
        }
    }

```

I then iterated over each token and  generated random balances for each `rand::random()` crate, then inserted each balance in our `balances` Hash Map;

For each token in `TokenType` {

    randomly assign a number from 0 to 9999 to represent token balance;
    insert each of these tokens to the `balances` Hash Map;
    use the token as key to the hashmap, to find out user's balance(randomly generated) for that specific token.

}

```rust
...
// Generate random balances for each token
        for token in [TokenType::USDC, TokenType::USDT, TokenType::BUSD].iter() {

            balances.insert(*token, (rand::random::<u64>() % 10_000) as f64); // Randomly assigns 0 to 9999 tokens
        }

...
```

Since the `TokenType` enum was being used as the key to the `balances` Hash Map, it was not implementing the `Eq` trait needed to be a key in the map.

I had derived the `PartialEq` trait, but did not add the `Eq` trait.

## Solution

```rust
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
//To represent our tokens
enum TokenType {

   USDC,
   USDT,
   BUSD,

}
```

