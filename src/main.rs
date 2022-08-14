mod client_account;
mod client_account_manager;
mod input_parser;
mod transaction;

use client_account_manager::*;
use input_parser::*;

fn main() {
    // Read the input file and  get a list of transactions.
    let mut input_parser = InputParser::new();
    input_parser.read_file_from_args().unwrap();
    let transactions = input_parser.get_transactions();

    // Create an account manager and process the list of transactions.
    let mut client_account_manager = ClientAccountManager::new();
    for transaction in transactions {
        // TODO: Actually log these error messages somewhere instead of intentionally ignoring them.
        if let Err(_msg) = client_account_manager.process_transaction(
            transaction.transaction_type.as_str(),
            transaction.client,
            transaction.tx,
            transaction.amount,
        ) {
            // println!("{}", _msg);
        };
    }

    // Output the final client list.
    println!("client, available, held, total, locked");
    let client_list = client_account_manager.get_client_list();
    for client in client_list {
        println!(
            "{}, {}, {}, {}, {}",
            client.get_id(),
            client.get_available_amount(),
            client.get_held_amount(),
            client.get_total_amount(),
            client.get_is_locked()
        );
    }
}
