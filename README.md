# Transaction Manager Demo
This application is a simple transaction manager that takes in a CSV of transactions and aggregates them in-order to produce a CSV representing the final state of each client account. Functionality is organized in the following modules.

## Modules

### input_parser
The input parser, as you might expect, parses the CSV input into a vec of transactions.

### client_account_manager
The `ClientAccountManager` struct accepts a chronological sequence of transactions via the `process_transaction` method. Each time this method is called, a new transaction is handled. A new client account will be created if one matching the provided ID did not exist prior. If the transaction ID has already been used, the transaction will be ignored.

Most of the application's business logic lives here: all transaction validations and adjustments to client accounts are handled in the relevant methods (`deposit`, `withdrawal`, etc) in this struct's impl.

### client_account
The `ClientAccount` struct represents the state of a single client account, containing information on available and held funds as well as the whether or not the client is allowed to receive new transactions (in the event of a chargeback, the `is_locked` flag will be set to prevent new transactions).

### transaction
The `Transaction` struct represents a single transaction.

## Sample Data
Six sample CSVs are available in the `sample_data` directory. These were used to manually test core functionality (valid and invalid deposits and withdrawals, disputing and resolving transactions, and freezing accounts due to chargebacks).

To see error logs from sample data in the output CSV file (such as denoting whether a withdrawal failed due to insufficient funds), uncomment line 25 in `main.rs`.

## Future Improvements
One of the more obvious areas for improvement if this was a production application would be to handle error messages as error codes rather than raw strings; as it currently stands, the messages are useful for developers but otherwise unwieldy to use in code.

Similarly, there's a TODO comment to the effect of "actually dump these error logs somewhere". Since actually outputting them would result in an invalid CSV file, right now it's only useful for manually testing (still valuable, but definitely not ideal).