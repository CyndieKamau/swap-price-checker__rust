use std::io;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Clone, Copy)]
//To represent our two tokens, USDC and USDT
enum TokenType {

   USDC,
   USDT,
   BUSD,

}

//list of the exchanges
#[derive(Debug, PartialEq, Clone, Copy)]
enum Exchange {

   Uniswap,
   Carbon,
   CowSwap,
   Matcha,
   Sushi,
}

//list of networks
#[derive(Debug, PartialEq, Clone, Copy)]
enum Network {

   Ethereum,
   BNBChain,
   Polygon,   
}

//Error handling
#[derive(Debug)]
enum SwapError {

    TokenPairNotSupported,
    NotEnoughLiquidity,
    InsufficientBalance,
    IncorrectNetwork,
}


struct Token {

    token_type: TokenType,
}

//data for each token pair (USDC to USDT, USDC to DAI etc)
struct TokenPairData {

    from_token: TokenType,
    to_token: TokenType,
    swap_rate: f64,    //swap rate for tokens
    liquidity: u64,    //liquidity pool for the tokens
}

//Data to be fetched from the mock exchanges
struct ExchangeData {

    exchange_name: Exchange,   //eg uniswap etc
    supported_network: Network,   //network the swap operates on
    token_pairs: Vec<TokenPairData>,    // Each exchange now has a list of token pairs it supports
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

struct User {

    network: Network,
    wallet_address: String,
    balances: HashMap<TokenType, f64>,

}


//implementing functionalities 

impl ExchangeData {


    //checks if a token pair exists in the exchanges
    pub fn check_token_pair(&self, from: TokenType, to: TokenType) -> Result<bool, SwapError> {

        for pair in &self.token_pairs {

            // If we find a match for both 'from' and 'to' tokens, pair is supported
            if pair.from_token == from && pair.to_token == to {

                return Ok(true);
            }
        }

        //If loop completes without returning, the pair is not supported
        Err(SwapError::TokenPairNotSupported)
    }


    //simulate the token swap
    pub fn simulate_swap(&self, from: TokenType, to: TokenType, amount: f64) -> Result<f64, SwapError> {

        // the "?" will return early with an Err if the pair isn't supported
        self.check_token_pair(from, to)?       

        for pair in &self.token_pairs {

            if pair.from_token == from && pair.to_token == to {

                if pair.liquidity as f64 >= amount {

                    return Ok(amount * pair.swap_rate);
                } else {

                    return Err(SwapError::NotEnoughLiquidity);
                }
            }
        }

        //This is just a safety net for precaution
        Err(SwapError::TokenPairNotSupported)
    }


   //Generate mock data for the exchanges
    pub fn mock_swap_data() -> Vec<Self> {

       let uniswap_pairs = vec![

           TokenPairData {
           
               from_token: TokenType::USDT,
               to_token: TokenType::USDC,
               swap_rate: 1.001,
               liquidity: 50_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDT,
               to_token: TokenType::BUSD,
               swap_rate: 1.0005,
               liquidity: 45_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDC,
               to_token: TokenType::USDT,
               swap_rate: 0.999,
               liquidity: 49_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDC,
               to_token: TokenType::BUSD,
               swap_rate: 1.0002,
               liquidity: 40_000_000,
           },

           TokenPairData {

               from_token: TokenType::BUSD,
               to_token: TokenType::USDT,
               swap_rate: 1.0003,
               liquidity: 44_000_000,
           },

           TokenPairData {

               from_token: TokenType::BUSD,
               to_token: TokenType::USDC,
               swap_rate: 0.9998,
               liquidity: 39_000_000,
           }
       ];

       //token pairs data for Carbon exchange
       let carbon_pairs = vec![
           TokenPairData {
           
               from_token: TokenType::USDT,
               to_token: TokenType::USDC,
               swap_rate: 1.002,
               liquidity: 48_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDT,
               to_token: TokenType::BUSD,
               swap_rate: 1.001,
               liquidity: 42_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDC,
               to_token: TokenType::USDT,
               swap_rate: 0.998,
               liquidity: 47_500_000,
           },

           TokenPairData {

               from_token: TokenType::USDC,
               to_token: TokenType::BUSD,
               swap_rate: 1.0015,
               liquidity: 38_000_000,
           },

           TokenPairData {

               from_token: TokenType::BUSD,
               to_token: TokenType::USDT,
               swap_rate: 1.0008,
               liquidity: 43_500_000,
           },

           TokenPairData {

               from_token: TokenType::BUSD,
               to_token: TokenType::USDC,
               swap_rate: 0.9995,
               liquidity: 37_500_000,
           }
       ];

       //token pairs for CowSwap exchange
       let cowswap_pairs = vec![
       
           TokenPairData {
           
               from_token: TokenType::USDT,
               to_token: TokenType::USDC,
               swap_rate: 1.003,
               liquidity: 36_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDT,
               to_token: TokenType::BUSD,
               swap_rate: 1.0005,
               liquidity: 32_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDC,
               to_token: TokenType::USDT,
               swap_rate: 0.997,
               liquidity: 34_500_000,
           },

           TokenPairData {

               from_token: TokenType::USDC,
               to_token: TokenType::BUSD,
               swap_rate: 1.0013,
               liquidity: 30_000_000,
           },

           TokenPairData {

               from_token: TokenType::BUSD,
               to_token: TokenType::USDT,
               swap_rate: 1.0009,
               liquidity: 33_000_000,
           },

           TokenPairData {

               from_token: TokenType::BUSD,
               to_token: TokenType::USDC,
               swap_rate: 0.9997,
               liquidity: 29_000_000,
           }
       ];


       //token pairs for Matcha Exchange
       let matcha_pairs = vec![
       
           TokenPairData {
           
               from_token: TokenType::USDT,
               to_token: TokenType::USDC,
               swap_rate: 1.004,
               liquidity: 42_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDT,
               to_token: TokenType::BUSD,
               swap_rate: 1.0006,
               liquidity: 37_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDC,
               to_token: TokenType::USDT,
               swap_rate: 0.996,
               liquidity: 40_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDC,
               to_token: TokenType::BUSD,
               swap_rate: 1.0011,
               liquidity: 35_000_000,
           },

           TokenPairData {

               from_token: TokenType::BUSD,
               to_token: TokenType::USDT,
               swap_rate: 1.0010,
               liquidity: 38_000_000,
           },

           TokenPairData {

               from_token: TokenType::BUSD,
               to_token: TokenType::USDC,
               swap_rate: 0.9995,
               liquidity: 36_500_000,
           }
       ];


       //token pairs for Sushi exchange
       let sushi_pairs = vec![
       
           TokenPairData {
           
               from_token: TokenType::USDT,
               to_token: TokenType::USDC,
               swap_rate: 1.005,
               liquidity: 48_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDT,
               to_token: TokenType::BUSD,
               swap_rate: 1.0007,
               liquidity: 44_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDC,
               to_token: TokenType::USDT,
               swap_rate: 0.995,
               liquidity: 46_000_000,
           },

           TokenPairData {

               from_token: TokenType::USDC,
               to_token: TokenType::BUSD,
               swap_rate: 1.0010,
               liquidity: 43_000_000,
           },

           TokenPairData {

               from_token: TokenType::BUSD,
               to_token: TokenType::USDT,
               swap_rate: 1.0015,
               liquidity: 45_000_000,
           },

           TokenPairData {

               from_token: TokenType::BUSD,
               to_token: TokenType::USDC,
               swap_rate: 0.9998,
               liquidity: 42_500_000,
           }
       ];

        vec![

            ExchangeData {

                exchange_name: Exchange::Uniswap,
                supported_network: Network::Ethereum,
                token_pairs: uniswap_pairs,
                
            },

            ExchangeData {

                exchange_name: Exchange::Carbon,
                supported_network: Network::Ethereum,
                token_pairs: carbon_pairs,
            },

            ExchangeData {

                exchange_name: Exchange::CowSwap,
                supported_network: Network::Ethereum,
                token_pairs: cowswap_pairs,
            },

            ExchangeData {

                exchange_name: Exchange::Matcha,
                supported_network: Network::Ethereum,
                token_pairs: matcha_pairs,
            },

            ExchangeData {

               exchange_name: Exchange::Sushi,
               supported_network: Network::Ethereum,
               token_pairs: sushi_pairs,
            },

            
        ]

    }

}


//To work on, figure out how to get different swap rates for token pairs instead of one swap rate.

fn main() {
    println!("Hello, world!");
}
