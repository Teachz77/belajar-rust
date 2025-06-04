use core::error;

enum TransactionType {
    Transfer,
    Mint,
    Burn,
    Stake,
}

enum ContractEvent {
    ContractDeployed,
    ContractTerminated,
    TokenTransfer{
        from: String,
        to: String,
        amount: u64,
    },
    OracleUpdate {
        price: f64,
    },
}

enum TransactionStatus {
    Pending, 
    Confirmed(u32),
    Failed(String),
}

impl TransactionStatus {
    fn display_status(&self) {
        match self {
            TransactionStatus::Pending => println!("Transaction is currently pending."),
            TransactionStatus::Confirmed(block) => {
                println!("Transaction confirmed in block: {}", block)
            }
            TransactionStatus::Failed(error) => println!("Transaction failed due to: {}", error),
        }
    }
}

enum ContractLifeCycle {
    Initialization,
    Active { participants: u32},
    Paused,
    Terminated,
}

impl ContractLifeCycle {
    fn display_state(&self) {
        match self {
            ContractLifeCycle::Initialization => println!("Smart contract is being initialized."),
            ContractLifeCycle::Active { participants } => {
                println!("Contract is active with {} participants", participants)
            }
            ContractLifeCycle::Paused => println!("Contract is currently paused."),
            ContractLifeCycle::Terminated => println!("Contract has been terminated."),
        }
    }
}


pub fn demo(){
    let tx_type = TransactionType::Mint;
    match tx_type {
        TransactionType::Transfer => println!("Processing a token transfer"),
        TransactionType::Mint => println!("Minting new tokens"),
        TransactionType::Stake => println!("Staking tokens for rewards"),
        TransactionType::Burn => println!("Burning tokens from supply"),
    }

    let transfer_event = ContractEvent::TokenTransfer { 
        from: String::from("0x19a0b46abaaa672387f9f3e0957fa2fe42c5e0fe"), 
        to: String::from("0xa97cb363977c03523e64b1dec1584b5083222e01"), 
        amount: 200,
    };

    let oracle_event = ContractEvent::OracleUpdate { price: 1234.6 };

    match transfer_event {
        ContractEvent::ContractDeployed => println!("Smart contract deployed"),
        ContractEvent::ContractTerminated => println!("Smart contract terminated"),
        ContractEvent::TokenTransfer { from, to, amount } => {
            println!("Transfer of {} tokens from {} to {}.", amount, from, to)
        }
        ContractEvent::OracleUpdate { price } => {
            println!("Oracle updated price to: ${}", price)
        }
    }

    match oracle_event {
        ContractEvent::OracleUpdate { price } => println!("New price from oracle: ${}", price),
        _ => println!("Unhandled event"),
    }

    let tx1 = TransactionStatus::Pending;
    let tx2 = TransactionStatus::Confirmed(12345);
    let tx3 = TransactionStatus::Failed("Invalid signature".to_string());

    tx1.display_status();
    tx2.display_status();
    tx3.display_status();

    let contract = ContractLifeCycle::Active { participants: 100 };
    let paused_contract = ContractLifeCycle::Paused;
    let terminated_contract = ContractLifeCycle::Terminated;

    contract.display_state();
    paused_contract.display_state();
    terminated_contract.display_state();
}