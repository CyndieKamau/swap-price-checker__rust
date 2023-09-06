
use std::io;
use std::collections::HashMap;
use rand::Rng;


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
    BalanceNotFound,
    UserNotFound,
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

    user_wallet_address: String,
    from_token: Token,
    to_token: Token,
    amount: f64,      //amount of from_token to swap

}

struct SwapResult {

    exchange_name: Exchange,
    received_amount: f64,
    slippage: f64
}

#[derive(Clone)]
struct User {

    network: Network,
    wallet_address: String,
    balances: HashMap<TokenType, f64>,

}

struct UserDatabase {

    users: Vec<User>,
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
        return Err(SwapError::TokenPairNotSupported)
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



//Implementing user logic

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

    // Function to check if user has sufficient balance of a given token
    pub fn has_sufficient_balance(&self, token: TokenType, amount: f64) -> bool {


        match self.balances.get(&token) {

            Some(balance) => *balance >= amount,
            None => false,
        }
    }


    // Deduct the specified amount from the user's balance for a given token
    pub fn deduct_balance(&mut self, token: TokenType, amount: f64) -> Result<(), SwapError> {

        if self.has_sufficient_balance(token, amount) {

            if let Some(balance) = self.balances.get_mut(&token) {

                *balance -= amount;
                Ok(())
            } else {

                Err(SwapError::BalanceNotFound)
            }
        } else {

            Err(SwapError::InsufficientBalance)
        }
    }


    // Add the specified amount to the user's balance for a given token
    pub fn add_balance(&mut self, token: TokenType, amount: f64) {

        *self.balances.entry(token).or_insert(0.0) += amount;
    }    
}

//Managing multiple users

impl UserDatabase {

    //initialize a new empty db

    pub fn new_db()  -> Self{

        UserDatabase {users: Vec::new()}
    }

    //Add new user to the db

    pub fn add_user(&mut self, user: User) {

        self.users.push(user);
    }

    //fetch a user in db. Returns Option as user might not be in db

    pub fn get_user_by_address(&mut self, address: &str) -> Option<&mut User> {

        self.users.iter_mut().find(|user| &user.wallet_address == address)
    }

    // Fetch a mutable reference to a user by wallet address. This allows you to update the user's details.

    pub fn get_user_by_address_mut(&mut self, address: &str) -> Option<&mut User> {

        self.users.iter_mut().find(|user| user.wallet_address == address)
    }

    // Remove a user from database
    
    pub fn remove_user_by_address(&mut self, address: &str) {

        if let Some(index) = self.users.iter().position(|user| user.wallet_address == address) {

            self.users.remove(index);
        }
    }

}


//Logic for performing a swap
fn perform_swap(swap: &Swap, exchanges: &[ExchangeData], user_db: &mut UserDatabase) -> Result<SwapResult, SwapError> {
    // 1. Fetch the user
    let user = match user_db.get_user_by_address_mut(&swap.user_wallet_address) {
        Some(u) => u,
        None => return Err(SwapError::UserNotFound),
    };

    // 2. Ensure the user has enough balance for the swap
    if !user.has_sufficient_balance(swap.from_token.token_type, swap.amount) {
        return Err(SwapError::InsufficientBalance);
    }

    let mut best_result: Option<SwapResult> = None;

    // 3. Loop through each exchange
    for exchange in exchanges {
        let result = exchange.simulate_swap(swap.from_token.token_type, swap.to_token.token_type, swap.amount);
        
        match result {
            Ok(received_amount) => {
                if best_result.is_none() || received_amount > best_result.as_ref().unwrap().received_amount {
                    best_result = Some(SwapResult {
                        exchange_name: exchange.exchange_name,
                        received_amount,
                        slippage: 0.01, // Assuming a fixed slippage of 1% for now
                    });
                }
            },
            Err(_) => continue,
        }
    }

    // 4. If a suitable exchange is found, proceed with the swap
    if let Some(best_swap) = best_result {
        user.deduct_balance(swap.from_token.token_type, swap.amount)?;
        user.add_balance(swap.to_token.token_type, best_swap.received_amount);
        Ok(best_swap)
    } else {
        Err(SwapError::TokenPairNotSupported)
    }
}

// `user_menu` provides an interactive interface to the user
//The function allows the user to interact with their account by providing
//multiple options, such as viewing balances or initiating a swap transaction.
fn user_menu(wallet_address: &str, exchanges: &[ExchangeData], user_db: &mut UserDatabase) {
    
    loop {
        println!("--- User Menu ---");
        println!("1. View balances");
        println!("2. Initiate swap");
        println!("3. Exit");
        println!("Select an option:");
        println!();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
               
                let user = user_db.get_user_by_address_mut(wallet_address).unwrap();
                
                // View balances
                println!("Your balances are: {:?}", user.balances);
            },
            "2" => {
                // Initiate swap 
                let from_token = select_token("Choose the token you want to swap FROM:");
                let to_token = select_token("Choose the token you want to swap TO:");

                //incase user chooses the same token to swap.
                if from_token == to_token {
                    println!("Both source and destination tokens are the same. Please try again.");
                    continue;
                }

                println!("Enter the amount you want to swap:");
                let mut amount_input = String::new();
                io::stdin().read_line(&mut amount_input).expect("Failed to read line");
                let amount: f64 = match amount_input.trim().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Invalid amount. Please try again.");
                        continue;
                    }
                };

                // Now use the perform_swap function
                let swap = Swap {
                    from_token: Token {
                        token_type: from_token,
                    },
                    to_token: Token {
                        token_type: to_token,
                    },
                    amount,
                    user_wallet_address: wallet_address.to_string()
                };

                match perform_swap(&swap, exchanges, user_db) {
                    Ok(result) => {
                        println!("Swap Successful! Best exchange: {:?}. Received amount: {} with slippage of {}%",
                            result.exchange_name, result.received_amount, result.slippage * 100.0);
                    },
                    Err(error) => {
                        println!("Swap failed: {:?}", error);
                    }
                }
            },
            "3" => {
                break;
            },
            _ => {
                println!("Invalid choice.");
            }
        }
    }
}


//This function is for selecting the token you want to swap FROM and TO
fn select_token(prompt: &str) -> TokenType {
    loop {
        println!("{}", prompt);
        println!("1. USDC");
        println!("2. USDT");
        println!("3. BUSD");
        println!();


        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => return TokenType::USDC,
            "2" => return TokenType::USDT,
            "3" => return TokenType::BUSD,
            _ => println!("Invalid choice. Please try again.")
        }
    }
}



fn main() {

    println!(r#"

 
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
    "#);
    
    let exchanges = ExchangeData::mock_swap_data();
    let mut user_db = UserDatabase::new_db();

    loop {
        // Ask the user for their wallet address
        println!("Please enter your wallet address (or type 'exit' to quit):");
        let mut wallet_address = String::new();
        io::stdin().read_line(&mut wallet_address).expect("Failed to read line");

        if wallet_address.trim() == "exit" {
            break;
        }

        // Check if this user already exists in user_db
        let user_exists = user_db.get_user_by_address(wallet_address.trim()).is_some();

        if user_exists {
            user_menu(wallet_address.trim(), &exchanges, &mut user_db);
        } else {
            // If its a New user, ask for their network
            println!("Please select a network (1. Ethereum, 2. BNBChain, 3. Polygon):");
            let mut network_input = String::new();
            io::stdin().read_line(&mut network_input).expect("Failed to read line");
            let network = match network_input.trim() {
                "1" => Network::Ethereum,
                "2" => Network::BNBChain,
                "3" => Network::Polygon,
                _ => {
                    println!("Invalid selection");
                    continue;
                }
            };

            // Check if the network is Ethereum. If user chooses another network they get an error.
            if network != Network::Ethereum {
                println!("Error: {:?}", SwapError::IncorrectNetwork);
                continue;
            }

            // Create new user with random balances and add to user_db
            let new_user = User::new(network, wallet_address.trim().to_string());
            user_db.add_user(new_user);
            println!("User created with random balances!");

            user_menu(wallet_address.trim(), &exchanges, &mut user_db);
        }
    }
}




