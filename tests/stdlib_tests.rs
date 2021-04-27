
extern crate rbs_parser;
use rbs_parser::{RBSParser, Rule};
use std::fs;
use pest::Parser;
#[test]
fn test_abbrev_0_abbrev() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//abbrev/0/abbrev.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_base64_0_base64() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//base64/0/base64.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_benchmark_0_benchmark() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//benchmark/0/benchmark.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_bigdecimal_0_big_decimal() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//bigdecimal/0/big_decimal.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_bigdecimal_math_0_big_math() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//bigdecimal-math/0/big_math.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_cgi_0_core() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//cgi/0/core.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_coverage_0_coverage() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//coverage/0/coverage.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_csv_0_csv() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//csv/0/csv.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_date_0_date() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//date/0/date.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_date_0_date_time() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//date/0/date_time.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_dbm_0_dbm() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//dbm/0/dbm.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_erb_0_erb() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//erb/0/erb.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_fiber_0_fiber() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//fiber/0/fiber.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_fileutils_0_fileutils() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//fileutils/0/fileutils.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_find_0_find() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//find/0/find.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_forwardable_0_forwardable() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//forwardable/0/forwardable.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_ipaddr_0_ipaddr() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//ipaddr/0/ipaddr.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_json_0_json() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//json/0/json.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_logger_0_formatter() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//logger/0/formatter.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_logger_0_log_device() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//logger/0/log_device.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_logger_0_logger() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//logger/0/logger.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_logger_0_period() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//logger/0/period.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_logger_0_severity() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//logger/0/severity.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_monitor_0_monitor() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//monitor/0/monitor.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_mutex_m_0_mutex_m() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//mutex_m/0/mutex_m.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_pathname_0_pathname() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//pathname/0/pathname.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_prettyprint_0_prettyprint() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//prettyprint/0/prettyprint.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_prime_0_integer_extension() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//prime/0/integer-extension.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_prime_0_prime() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//prime/0/prime.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_pstore_0_pstore() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//pstore/0/pstore.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_pty_0_pty() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//pty/0/pty.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_basic_specification() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/basic_specification.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_config_file() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/config_file.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_dependency_installer() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/dependency_installer.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_installer() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/installer.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_path_support() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/path_support.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_platform() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/platform.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_request_set() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/request_set.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_requirement() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/requirement.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_rubygems() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/rubygems.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_source_list() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/source_list.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_specification() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/specification.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_stream_ui() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/stream_ui.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_uninstaller() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/uninstaller.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_rubygems_0_version() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//rubygems/0/version.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_securerandom_0_securerandom() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//securerandom/0/securerandom.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_set_0_set() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//set/0/set.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_shellwords_0_shellwords() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//shellwords/0/shellwords.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_singleton_0_singleton() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//singleton/0/singleton.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_strscan_0_string_scanner() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//strscan/0/string_scanner.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_time_0_time() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//time/0/time.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_timeout_0_timeout() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//timeout/0/timeout.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_tmpdir_0_tmpdir() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//tmpdir/0/tmpdir.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_tsort_0_cyclic() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//tsort/0/cyclic.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_tsort_0_interfaces() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//tsort/0/interfaces.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_tsort_0_tsort() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//tsort/0/tsort.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_uri_0_common() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//uri/0/common.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_uri_0_file() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//uri/0/file.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_uri_0_generic() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//uri/0/generic.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_uri_0_http() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//uri/0/http.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_uri_0_https() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//uri/0/https.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_uri_0_ldap() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//uri/0/ldap.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_uri_0_ldaps() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//uri/0/ldaps.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_uri_0_rfc2396_parser() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//uri/0/rfc2396_parser.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_uri_0_rfc3986_parser() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//uri/0/rfc3986_parser.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_yaml_0_dbm() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//yaml/0/dbm.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_yaml_0_store() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//yaml/0/store.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test_zlib_0_zlib() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//zlib/0/zlib.rbs").expect("didnt work");
    let _pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
