#[derive(Clone, Debug)]
pub enum TransactionType {
  Deposit,
  Withdrawal,
  Dispute,
  Resolve,
  Chargeback,
}

impl TransactionType {
  pub fn get_from_str(input: &str) -> Option<TransactionType> {
    match input {
      "deposit" => Some(TransactionType::Deposit),
      "withdrawal" => Some(TransactionType::Withdrawal),
      "dispute" => Some(TransactionType::Dispute),
      "resolve" => Some(TransactionType::Resolve),
      "chargeback" => Some(TransactionType::Chargeback),
      _ => None,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Transaction {
  pub id: u32,
  pub client_id: u16,
  pub transaction_type: TransactionType,
  pub amount: Option<f32>,
  pub is_disputed: bool,
}
