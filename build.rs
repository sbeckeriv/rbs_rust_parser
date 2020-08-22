//https://blog.cyplo.dev/posts/2018/12/generate-rust-tests-from-data/
extern crate glob;
use glob::glob;
use std::env;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let destination = Path::new("tests/").join("tests.rs");

    eprintln!("{:?}", destination);
    let mut test_file = File::create(&destination).unwrap();

    // write test file header, put `use`, `const` etc there
    write_header(&mut test_file);

    let test_data_directories = read_dir("./tests/rbs/").unwrap();
    for directory in test_data_directories {
        dbg!(&directory);
        write_test(&mut test_file, &directory.unwrap());
    }

    let destination = Path::new("tests/").join("stdlib_tests.rs");

    let mut test_file = File::create(&destination).unwrap();

    // write test file header, put `use`, `const` etc there
    write_header(&mut test_file);

    for e in glob("./tests/stdlib/stdlib/**/*.rbs").expect("Failed to read glob pattern") {
        write_stdlib_test(&mut test_file, &e.unwrap());
    }
}

fn write_stdlib_test(test_file: &mut File, directory: &PathBuf) {
    let directory = directory.canonicalize().unwrap();
    let path = format!("{}", directory.display());
    let file_path = path.split("stdlib").collect::<Vec<_>>();
    let file_name = format!("{}", file_path.last().unwrap());
    let test_name = file_name.replace(".rbs", "");
    let split = test_name.split("-").collect::<Vec<_>>();
    let caller = split.first().unwrap();
    let test_name = test_name.replace("-", "_");
    let test_name = test_name.replace("/", "_");
    let test_name = test_name.replace("__", "_");
    let data = format!(
        "#[test]
fn test{}() {{
    let string =  fs::read_to_string(\"./tests/stdlib/stdlib/{}\").expect(\"didnt work\");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!(\"error{{}}\", e));
}}
",
        test_name, file_name
    );
    write!(test_file, "{}", data).unwrap();
}

fn write_test(test_file: &mut File, directory: &DirEntry) {
    let directory = directory.path().canonicalize().unwrap();
    let path = directory.display();
    let file_name = format!("{}", directory.file_name().unwrap().to_string_lossy());
    let test_name = file_name.replace(".rbs", "");
    let split = test_name.split("-").collect::<Vec<_>>();
    let caller = split.first().unwrap();
    let test_name = test_name.replace("-", "_");
    let test_name = test_name.replace("__", "_");
    let data = format!(
        "#[test]
fn test{}() {{
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
