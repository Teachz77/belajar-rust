pub fn basic_if_else(){
    let transaction_amount = 200;

    if transaction_amount > 0 {
        println!("Transaction Valid")
    } else {
        println!("Transaction invalid")
    }
}

pub fn match_example(member: u8){
    let team_7 = match member {
        1 => "Kakashi",
        2 => "Naruto",
        3 => "Sasuke",
        4 => "Sakura",
        _ => "Invalid member",
    };

    println!("Member Name: {}", team_7);
}

pub fn while_loop_example(){
    let mut pending_transactions = 0;

    while pending_transactions < 5 {
        println!("Processing transaction: {}", pending_transactions + 1);
        pending_transactions += 1
    }

    println!("All transactions processed")
}

pub fn for_loop_example(){

    let staking_rewards = [10, 20, 30, 40, 50];

    for reward in staking_rewards.iter(){
        println!("Validator Reward: {}", reward);
    }

    for block in 1..5 {
        println!("Produced block number: {}", block);
    }
}

pub fn infinite_loop_example(){
    let mut attempts = 0;

    loop{
        println!("Checking blockchain state... attempt: {}", attempts + 1);
        attempts += 1;

        if attempts == 3 {
            println!("Breaking the loop after 3 attempts");
            break;
        }
    }
}

pub fn match_pattern_example(number: i32){
    match number {
        1 => println!("Laptop Asus"),
        2 | 3 | 4 | 5 => println!("Laptop Buatan China"),
        6..=10 => println!("Laptop Buatan Korea & Jepang"),
        _ => println!("Unrecognized Laptop"),
    }
}

pub fn let_if_example(reputation_score: i32){
    let reputation_level = if reputation_score >= 90 {
        "High Reputation"
    } else if reputation_score >= 80 {
        "Good Reputation"
    } else if reputation_score >= 70 {
        "Average Reputation"
    } else if reputation_score >= 60 {
        "Low Reputation"
    } else {
        "Poor Reputation"
    };

    println!("Reputation Score: {}, Reputation Level: {}", reputation_score, reputation_level);
}

pub fn match_return_example(status_code: i32) -> &'static str {
    match status_code {
        200 => "Transaction Successful",
        400 => "Transaction Not Found",
        500 => "Blockchain Error",
        _ => "Unknown Status",
    }
}

pub fn demo(){
    println!("\n");
    basic_if_else();

    println!("\n");
    match_example(3);

    println!("\n");
    while_loop_example();

    println!("\n");
    for_loop_example();

    println!("\n");
    infinite_loop_example();

    println!("\n");
    match_pattern_example(2);

    println!("\n");
    let_if_example(79);

    println!("\n");
    let status_message = match_return_example(300);
    println!("Status Message: {}", status_message);
}