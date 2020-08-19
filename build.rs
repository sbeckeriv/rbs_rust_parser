use std::env;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// build script's entry point
fn main() {
    let destination = Path::new("tests/").join("tests.rs");

    eprintln!("{:?}", destination);
    let mut test_file = File::create(&destination).unwrap();

    // write test file header, put `use`, `const` etc there
    write_header(&mut test_file);

    let test_data_directories = read_dir("./tests/rbs/").unwrap();
    dbg!(&test_data_directories);
    for directory in test_data_directories {
        dbg!(&directory);
        write_test(&mut test_file, &directory.unwrap());
    }
}

fn write_test(test_file: &mut File, directory: &DirEntry) {
    let directory = directory.path().canonicalize().unwrap();
    let path = directory.display();
    let file_name = format!("{}", directory.file_name().unwrap().to_string_lossy());
    let test_name = file_name.replace(".rbs", "");
    let split = test_name.split("-").collect::<Vec<_>>();
    let caller = split.first().unwrap();
    let test_name = test_name.replace("-", "_");
    let data = format!(
        "#[test]
fn test_{}() {{
    let string =  fs::read_to_string(\"./tests/rbs/{}\").expect(\"didnt work\");
    let pairs = RBSParser::parse(Rule::{}, &string).unwrap_or_else(|e| panic!(\"error{{}}\", e));
}}
",
        test_name, file_name, caller
    );
    write!(test_file, "{}", data).unwrap();
}

fn write_header(test_file: &mut File) {
    write!(
        test_file,
        r#"
extern crate rbs_parser;
use rbs_parser::{{RBSParser, Rule}};
use std::fs;
use pest::Parser;
"#
    )
    .unwrap();
}
