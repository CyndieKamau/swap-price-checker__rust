
#[derive(Debug, PartialEq, Clone, Copy)]
//To represent our two tokens, USDC and USDT
enum TokenType {

   USDC,
   USDT,

}

//list of the exchanges
#[derive(Debug, PartialEq, Clone, Copy)]
enum Exchange {

   Uniswap,
   PancakeSwap,
   CowSwap,
   Matcha,
   Sushi,
}


struct Token {

    token_type: TokenType,
}

//Data to be fetched from the mock exchanges
struct ExchangeData {

    exchange_name: Exchange,   //eg uniswap etc
    swap_rate: f64,          //swap rate for usdc & usdt
    liquidity: u64,          //liquidity pool for the tokens
}

//when user decides to swap
struct Swap {

    from_token: Token,
    to_token: Token,
    amount: f64,      //amount of from_token to swap

}

struct SwapResult {

    exchange_name: Exchange,
    received_amount: f64,
    slippage: f64
}



fn main() {
    println!("Hello, world!");
}
