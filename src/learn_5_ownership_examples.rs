
pub fn ownership_example(){
    let token_owner: String = String::from("Alice"); //Store in memory location called "4th avenue"
    let new_owner: String = token_owner; //Is referring to a value stored in "4th avenue"

    println!("\nNew Token Owner: {}", new_owner);
}

pub fn cloning_example(){
    let transaction_id = String::from("tx456");
    let transaction_copy = transaction_id.clone();

    println!("Original Transaction ID: {}", transaction_id);
    println!("Cloned Transaction ID: {}", transaction_copy);
}

pub fn copy_trait_example(){
    let token_amount = 100;
    let token_copy = token_amount;

    println!("\nOriginal Token Amount: {}, Copied Token Amount: {}", token_amount, token_copy);
}

pub fn borrowing_example(){
    let contract_state = String::from("Contract active");
    print_borrow(&contract_state);
    println!("\nAfter borrowing, contract state: {}", contract_state);
}

pub fn print_borrow(contract: &String){
    println!("Borrowed contract state: {}", contract);
}

pub fn mutable_borrowing_example(){
    let mut contract_state_mutable = String::from("Contract Pending");
    modify_state(&mut contract_state_mutable);

    println!("\nAfter modifying, Contract state: {}", contract_state_mutable);
}

pub fn modify_state(contract: &mut String){
    contract.push_str(" and now active");
}

// pub fn dangling_reference_example(){
//     let invalid_reference = invalid_borrow();
//     println!("{}", invalid_reference);
// }

// pub fn invalid_borrow() -> &String{
//     let s: String = String::from("I will be dropped ");
//     &s
// }


pub fn demo() {
    ownership_example();
    cloning_example();
    copy_trait_example();
    borrowing_example();
    mutable_borrowing_example();
}