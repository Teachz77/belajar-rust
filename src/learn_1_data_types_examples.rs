use std::io;
use rand::Rng;

pub fn guess_number(){
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your number");

    println!("The secret number: {secret_number}");

    let mut guess: String = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Read the line");

    println!("You guessed: {}", guess);
}

pub fn primitive_data_types(){
    let y = 5;
    let x = 7;

    println!("x = {x} and y + 2 = {}", y + 2);
}

pub fn arithmetic_operation(){
    let account_balance: i32 = 1000;
    let transaction_ammount: i32 = 250;

    println!("account balance: {} transaction ammount: {}", account_balance, transaction_ammount);

    println!("New Account Balance: {}", account_balance - transaction_ammount);

    let gas_price: f64 = 0.00000012;
    let gas_used: f64 = 21000.0;

    println!("Gas Price: {}, Gas Used: {}", gas_price, gas_used);
    println!("Total gas fee: {:.8}", gas_price * gas_used);
}

pub fn logical_operations(){
    let is_staking: bool = true;
    let has_sufficient_balance: bool = false;

    println!("is staking: {}, has sufficient balance: {}", is_staking, has_sufficient_balance);

    println!("can perform staking: {}", is_staking && has_sufficient_balance);

    println!("can either perform staking or withdraw: {}", is_staking || has_sufficient_balance);

    println!("Negative staking status: !is_staking = {}", !is_staking);
}

pub fn variable_shadowing_and_conversion(){
    let account_balance: i32 = 500;
    println!("Initial account balance: {}", account_balance);

    let account_balance: i32 = account_balance + 100;
    println!("Update account balance: {}", account_balance);

    let gas_fee: f64 = 0.0025;
    let gas_fee_init: i32 = gas_fee as i32;
    println!("Gas fee (f64): {}, Converted to lamports: {}", gas_fee, gas_fee_init);

    let block_height: i32 = 10200;
    let block_height_str: String = block_height.to_string();

    println!("block height: {}, converted to string: {}", block_height, block_height_str);
}

pub fn mutability_example(){
    let token_supply: i32 = 1_000_000;

    let mut user_balance: i32 = 500;
    println!("Before transaction: {}", user_balance);

    user_balance -= 50;
    println!("After transaction: {}", user_balance);
}

pub fn tuple_destructuring_example(){
    let transaction_info = ("Transfer", 200, 0.002);

    let (tx_type, tx_amount, tx_fee) = transaction_info;

    println!("Transaction Type: {}, Amount: {}, Fee:{}", tx_type, tx_amount, tx_fee);

    println!("Transaction Type: {}, Amount: {}, Fee:{}", transaction_info.0, transaction_info.1, transaction_info.2);
}

pub fn demo(){
    println!("\n");
    guess_number();

    println!("\n");
    primitive_data_types();

    println!("\n");
    arithmetic_operation();

    println!("\n");
    logical_operations();

    println!("\n");
    variable_shadowing_and_conversion();

    println!("\n");
    mutability_example();

    println!("\n");
    tuple_destructuring_example();
}