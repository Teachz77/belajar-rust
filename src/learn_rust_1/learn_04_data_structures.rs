use std::collections::HashMap;

pub fn array_examples(){
    let block_hashes: [u64; 5] = [123456, 7890112, 345678, 901234, 567890];
    println!("Block hashes array: {:?}", block_hashes);
    println!("First block hash: {}", block_hashes[0]);
    println!("Last block hash: {}", block_hashes[block_hashes.len() - 1]);

    let slice = &block_hashes[1..4];
    println!("Sliced block hashes: {:?}", slice);
}

pub fn vector_example(){
    let mut transaction_ids: Vec<&str> = vec!["tx1", "tx2", "tx3"];
    transaction_ids.push("tx4");
    transaction_ids.push("tx5");
    println!("After adding transactions: {:?}", transaction_ids);

    transaction_ids.pop();
    println!("After removing last transactions: {:?}", transaction_ids);

    for tx in &transaction_ids {
        println!("Transaction ID: {}", tx);
    }

    if let Some(first_tx) = transaction_ids.get(0) {
        println!("First transaction ID: {}", first_tx);
    }
}

pub fn tupple_example(){
    let user_info: (&str, i32, f64) = ("Ria", 30, 2.5);
    println!("User info tupple: {:?}", user_info);

    println!("Username: {}", user_info.0);
    println!("Age: {}", user_info.1);
    println!("Token Balance: {}", user_info.2);

    let (name, age, balance) = user_info;
    println!("Destructure info: Name = {}, Age = {}, Balance = {}", 
        name, age, balance
    );
}

pub fn hash_map_example(){
    let mut balanceOf: HashMap<&str, u32> = HashMap::new();

    balanceOf.insert("0x742d35Cc6634C0532925a3b844Bc454e4438f44e", 1000);
    balanceOf.insert("0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199", 2500);
    balanceOf.insert("0xdD2FD4581271e230360230F9337D5c0430Bf44C0", 750);

    println!("User balance hashmap: {:#?}", balanceOf);

    match balanceOf.get("0x742d35Cc6634C0532925a3b844Bc454e4438f44e") {
        Some(balance) => println!("User & balance: {}", balance),
        None => println!("No balance found for this user")
    }

    for (user, balance) in &balanceOf {
        println!("{}'s balance: {}", user, balance);
    }

    balanceOf.entry("0x742d35Cc6634C0532925a3b844Bc454e4438f44e").and_modify(|balance|*balance -= 50);
    println!("Update Ria's balance: {:#?}", balanceOf);

    balanceOf.remove("0x742d35Cc6634C0532925a3b844Bc454e4438f44e");
    println!("After removing user: {:#?}", balanceOf);

}

pub fn nested_data_strutures_example(){
    let mut user_transactions: HashMap<&str, Vec<&str>> = HashMap::new();

    user_transactions.insert("0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199", vec!["tx1", "tx2", "tx3"]);
    user_transactions.insert("0xdD2FD4581271e230360230F9337D5c0430Bf44C0", vec!["tx4", "tx5"]);

    println!("Users transaction hashmap: {:#?}", user_transactions);

    if let Some(transactions) = user_transactions.get("0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199") {
        println!("User's transactions: {:#?}", transactions);
    }

    if let Some(transactions) = user_transactions.get_mut("0xdD2FD4581271e230360230F9337D5c0430Bf44C0"){
        transactions.push("tx6");
    }
    println!("Users transactions hashmap: {:#?}", user_transactions);
}

pub fn demo(){
    array_examples();
    vector_example();
    tupple_example();
    hash_map_example();
    nested_data_strutures_example();
}