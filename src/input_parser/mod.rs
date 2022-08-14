use csv::{ReaderBuilder, Trim};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

pub struct InputParser {
  transactions: Vec<InputTransaction>,
}

#[derive(Debug, Deserialize)]
pub struct InputTransaction {
  #[serde(rename = "type")]
  pub transaction_type: String,
  pub client: u16,
  pub tx: u32,
  pub amount: Option<f32>,
}

impl InputParser {
  pub fn new() -> InputParser {
    InputParser {
      transactions: Vec::new(),
    }
  }

  pub fn get_transactions(&self) -> &Vec<InputTransaction> {
    &self.transactions
  }

  pub fn read_file_from_args(&mut self) -> Result<(), String> {
    // Get the file path from the command line argument list.
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
      return Err("No file path provided.".to_owned());
    }
    let file_path = &args[1];

    // Read the file into a string, then parse that string into a list of transactions.
    let csv_string = read_to_string(file_path)
      .expect(&format!("Error reading file at path {}", file_path).to_string());
    self
      .parse_input_csv(csv_string)
      .expect("Error parsing CSV.");

    return Ok(());
  }

  fn parse_input_csv(&mut self, data: String) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
      .trim(Trim::All)
      .from_reader(data.as_bytes());
    for result in rdr.deserialize() {
      let record: InputTransaction = result?;
      self.transactions.push(record);
    }
    Ok(())
  }
}
