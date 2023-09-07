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

We then enter into a loop, where if the token to be swapped from matches our from (`from: TokenType), and token swapped to matches to (`to: TokenType`);

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

