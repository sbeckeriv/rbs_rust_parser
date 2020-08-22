
extern crate rbs_parser;
use rbs_parser::{RBSParser, Rule};
use std::fs;
use pest::Parser;
#[test]
fn testclass_decl_large() {
    let string =  fs::read_to_string("./tests/rbs/class_decl-large.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::class_decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
