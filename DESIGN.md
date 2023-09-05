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



