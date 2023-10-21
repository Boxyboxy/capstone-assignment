
mod location;
use location::*;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
mod transaction;
use transaction::Transaction;

fn main() {

  // a. create file variable by passing "./transactions.csv" into the File::open function, followed by calling the unwrap method
  
  let file = File::open("./transactions.csv").unwrap();
  // b. create reader variable by passing file variable into the BufReader::new function
  let reader = BufReader::new(file);
  // c. create mutable transactions variable of type Vec<Transaction> by calling Vec::new method
  let mut transactions: Vec<Transaction> = Vec::new();
  // d. create mutable skipped_lines variable of no explicit type simply calling Vec::new method
  let mut skipped_lines: Vec<_> = Vec::new();
  //   e. run a for loop destructured into arbitrary variables of (idx, line) using the reader variable which calls lines method followed by enumerate method
    // - if idx equals to 0, continue
    // - create line_str variable by using line to call the unwrap method
    // - create parsed_transaction variable by passing line_str into Transaction::fram_csv_line method
    // - match on parsed_transaction
    // - if matches on Ok variant, push value within Ok into transactions
    // - If matches on Err variant, push the tuple of (idx, value within Err, line_str) into skipped_lines
  for (idx, line) in reader.lines().enumerate(){
    if idx == 0 {
      continue;
    }
    let line_str = line.unwrap();
    let parsed_transaction = Transaction::from_csv_line(&line_str);
    match parsed_transaction {
      Ok(result) => transactions.push(result), 
      Err(error) => skipped_lines.push((idx, error, line_str))
    }
  }
  // f. run a for loop by using transactions to call the iter method
    // - print every item in transactions
  for tran in transactions {
    println!("{:?}", tran);
  }

  // g. do the same for skipped_lines
  for skip in skipped_lines {
    println!("{:?}", skip);
  }
  
}
