extern crate rbs_parser;
use rbs_parser::{RBSParser, Rule};

use pest::Parser;

#[test]
fn test_files() {
    let string = "class Ref[A] < Object
  attr_reader value: A
  def initialize: (Int a, ?Int a, *Int, Int t, a: Int) -> String
end";
    let pairs = RBSParser::parse(Rule::class_decl, string).unwrap_or_else(|e| panic!("{}", e));
}
