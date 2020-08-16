use std::fs;
use std::str::FromStr;
fn main() {
    let grammar = fs::read_to_string("rbs.pest").expect("rbs.pest not found");

    // Read some input from command-line
    let test = " module ChatApp
  VERSION: String

  class User
    attr_reader login: String
    attr_reader email: String

    def initialize: (login: String, email: String) -> void
  end
  end
  ";
}
