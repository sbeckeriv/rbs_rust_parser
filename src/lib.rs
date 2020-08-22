extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "../rbs.pest"]
pub struct RBSParser;
