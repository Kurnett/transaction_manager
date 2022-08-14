use std::collections::HashMap;

use super::client_account::*;
use super::transaction::*;

pub struct ClientAccountManager {
  clients: HashMap<u16, ClientAccount>,
  transactions: HashMap<u32, Transaction>,
}

impl ClientAccountManager {
  pub fn new() -> ClientAccountManager {
    ClientAccountManager {
      clients: HashMap::new(),
      transactions: HashMap::new(),
    }
  }

  pub fn process_transaction(
    &mut self,
    transaction_type: &str,
    client_id: u16,
    transaction_id: u32,
    amount: Option<f32>,
  ) -> Result<(), String> {
    // Check whether the transaction has a valid type. If it does, create a Transaction. Otherwise, return.
    let transaction_type = TransactionType::get_from_str(transaction_type);
    if let None = transaction_type {
      return Err("Invalid transaction type, ignoring.".to_owned());
    }
    let transaction = Transaction {
      transaction_type: transaction_type.unwrap(),
      id: transaction_id,
      client_id,
      amount,
      is_disputed: false,
    };

    // If the client is locked, ignore any transaction attempts.
    let client = self.get_or_create_client_account(client_id);
    if client.get_is_locked() == true {
      return Err("Transaction failed because client is locked.".to_owned());
    }

    // Process the transaction.
    let result = match &transaction.transaction_type {
      TransactionType::Deposit => self.deposit(&transaction),
      TransactionType::Withdrawal => self.withdrawal(&transaction),
      TransactionType::Dispute => self.dispute(&transaction),
      TransactionType::Resolve => self.resolve(&transaction),
      TransactionType::Chargeback => self.chargeback(&transaction),
    };

    result
  }

  fn get_or_create_client_account(&mut self, client_id: u16) -> &mut ClientAccount {
    // Check if the client exists. If it doesn't already exist, create it.
    self
      .clients
      .entry(client_id)
      .or_insert(ClientAccount::new(client_id))
  }

  pub fn get_client_list(&self) -> Vec<ClientAccount> {
    self.clients.values().cloned().collect()
  }

  fn deposit(&mut self, transaction: &Transaction) -> Result<(), String> {
    let client = self.get_or_create_client_account(transaction.client_id);
    if let Some(amount) = transaction.amount {
      client.adjust_available_amount(amount);

      // If the transaction was successful, add it to the global transaction list.
      self
        .transactions
        .insert(transaction.id, transaction.clone());

      return Ok(());
    } else {
      return Err("Deposit failed due to missing amount.".to_owned());
    }
  }

  fn withdrawal(&mut self, transaction: &Transaction) -> Result<(), String> {
    let client = self.get_or_create_client_account(transaction.client_id);

    if let Some(amount) = transaction.amount {
      if client.get_available_amount() >= amount {
        client.adjust_available_amount(-amount);

        // If the transaction was successful, add it to the transaction list.
        self
          .transactions
          .insert(transaction.id, transaction.clone());

        return Ok(());
      } else {
        return Err("Withdrawal failed due to insufficient funds.".to_owned());
      }
    } else {
      return Err("Withdrawal failed due to missing amount.".to_owned());
    }
  }

  fn dispute(&mut self, transaction: &Transaction) -> Result<(), String> {
    // Check if the disputed transaction exists, has a valid amount, and is associated with the correct client.
    if !self.transactions.contains_key(&transaction.id) {
      return Err("Dispute failed; transaction does not exist.".to_owned());
    }
    let disputed_transaction = self.transactions.get(&transaction.id).unwrap().clone();
    if disputed_transaction.client_id != transaction.client_id {
      return Err("Dispute failed; transaction is not associated with client account.".to_owned());
    }
    if let None = disputed_transaction.amount {
      return Err("Dispute failed; transaction does not have amount.".to_owned());
    }

    // Reduce the client's available funds and increase their held funds by the disputed transaction's amount.
    let client = self.get_or_create_client_account(transaction.client_id);
    if let Some(amount) = disputed_transaction.amount {
      client.adjust_available_amount(-amount);
      client.adjust_held_amount(amount);
    }

    // Mark the transaction as disputed.
    self.transactions.entry(transaction.id).and_modify(|entry| {
      entry.is_disputed = true;
    });

    return Ok(());
  }

  fn resolve(&mut self, transaction: &Transaction) -> Result<(), String> {
    // Check if the disputed transaction exists, has a valid amount, and is actually under dispute.
    if !self.transactions.contains_key(&transaction.id) {
      return Err("Dispute resolution failed; transaction does not exist.".to_owned());
    }
    let disputed_transaction = self.transactions.get(&transaction.id).unwrap().clone();
    if disputed_transaction.client_id != transaction.client_id {
      return Err("Dispute failed; transaction is not associated with client account.".to_owned());
    }
    if let None = disputed_transaction.amount {
      return Err("Dispute resolution failed; transaction does not have amount.".to_owned());
    }
    if disputed_transaction.is_disputed == false {
      return Err("Dispute resolution failed; transaction is not being disputed.".to_owned());
    }

    // Reduce the client's held funds and increase their available funds by the disputed transaction's amount.
    let client = self.get_or_create_client_account(transaction.client_id);
    if let Some(amount) = disputed_transaction.amount {
      client.adjust_available_amount(amount);
      client.adjust_held_amount(-amount);
    }

    // Mark the transaction as no longer disputed.
    self.transactions.entry(transaction.id).and_modify(|entry| {
      entry.is_disputed = false;
    });

    return Ok(());
  }

  fn chargeback(&mut self, transaction: &Transaction) -> Result<(), String> {
    // Check if the disputed transaction exists and is actually under dispute.
    if !self.transactions.contains_key(&transaction.id) {
      return Err("Chargeback failed; transaction does not exist.".to_owned());
    }
    let disputed_transaction = self.transactions.get(&transaction.id).unwrap().clone();
    if disputed_transaction.client_id != transaction.client_id {
      return Err("Dispute failed; transaction is not associated with client account.".to_owned());
    }
    if disputed_transaction.is_disputed == false {
      return Err("Chargeback failed; transaction is not being disputed.".to_owned());
    }

    // Reduce the client's held funds by the disputed transaction's amount WITHOUT incrementing their available funds.
    let client = self.get_or_create_client_account(transaction.client_id);
    if let Some(amount) = disputed_transaction.amount {
      client.adjust_held_amount(-amount);
    }

    // Lock the client account.
    client.set_is_locked(true);

    // Mark the transaction as no longer disputed.
    self.transactions.entry(transaction.id).and_modify(|entry| {
      entry.is_disputed = false;
    });

    return Ok(());
  }
}
