# Core functionalities of `ExchangeData`;

* Store mock data about exchanges

* Find out if a token swap exists on the exchange

* Simulate a token swap on the exchange to know how much we will get in return


# `User` struct Logic

* Generate random balances when a user is created (`rand` crate)

* Check if user has sufficient tokens for a swap

* Deducting from user's balance after a successful swap

* Adding to user's balance when they receive tokens after swap


# `UserDatabase` Logic

* Helps in managing multiple users

* Can lookup users based on their wallet addresses

## UserDatabase tradeoffs;

I went with `HashMap` over `Vector` database for managing multiple users as;

* Hashmaps have faster lookups than vector dbs

* Faster insertions and deletions via the key(wallet addresses)

* I had no need to maintain a specific order of users

## Disadvantage

* Will use up more memory than a vector db



User Requests Swap: The user provides details about the swap they want to perform, including from_token, to_token, and the amount they want to swap.

Find Best Exchange Rate: Iterate over all ExchangeData to find the best rate for the specified swap. This is determined by checking the token_pairs in each exchange.

Check Liquidity: Ensure that the chosen exchange has enough liquidity for the swap.

Perform Swap and Update User Balances: Deduct the swapped amount from the user's balance and add the received amount.

Calculate Slippage: Slippage is the difference between the expected price of a trade and the price at which the trade is executed. To keep things simple, you might consider a constant slippage percentage based on the liquidity available or the size of the swap.

Return Swap Result: Return details about the swap, including the exchange used, the received amount, and the slippage.




