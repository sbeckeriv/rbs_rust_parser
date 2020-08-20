
extern crate rbs_parser;
use rbs_parser::{RBSParser, Rule};
use std::fs;
use pest::Parser;
#[test]
fn test__abbrev_abbrev() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//abbrev/abbrev.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__base64_base64() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//base64/base64.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__benchmark_benchmark() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//benchmark/benchmark.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_array() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/array.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_basic_object() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/basic_object.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_binding() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/binding.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_builtin() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/builtin.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_class() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/class.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_comparable() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/comparable.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_complex() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/complex.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_constants() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/constants.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_data() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/data.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_deprecated() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/deprecated.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_dir() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/dir.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_encoding() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/encoding.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_enumerable() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/enumerable.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_enumerator() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/enumerator.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_errno() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/errno.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_errors() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/errors.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_exception() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/exception.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_false_class() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/false_class.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_fiber() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/fiber.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_fiber_error() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/fiber_error.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_file() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/file.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_file_test() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/file_test.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_float() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/float.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_gc() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/gc.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_hash() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/hash.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_integer() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/integer.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_io() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/io.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_kernel() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/kernel.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_marshal() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/marshal.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_match_data() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/match_data.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_math() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/math.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_method() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/method.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_module() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/module.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_nil_class() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/nil_class.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_numeric() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/numeric.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_object() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/object.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_proc() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/proc.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_process() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/process.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_random() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/random.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_range() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/range.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_rational() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/rational.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_rb_config() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/rb_config.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_regexp() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/regexp.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_ruby_vm() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/ruby_vm.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_signal() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/signal.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_string() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/string.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_string_io() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/string_io.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_struct() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/struct.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_symbol() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/symbol.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_thread() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/thread.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_thread_group() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/thread_group.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_time() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/time.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_trace_point() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/trace_point.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_true_class() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/true_class.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_unbound_method() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/unbound_method.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__builtin_warning() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//builtin/warning.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__coverage_coverage() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//coverage/coverage.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__csv_csv() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//csv/csv.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__erb_erb() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//erb/erb.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__fiber_fiber() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//fiber/fiber.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__find_find() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//find/find.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__ipaddr_ipaddr() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//ipaddr/ipaddr.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__json_json() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//json/json.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__logger_formatter() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//logger/formatter.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__logger_log_device() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//logger/log_device.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__logger_logger() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//logger/logger.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__logger_period() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//logger/period.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__logger_severity() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//logger/severity.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__mutex_m_mutex_m() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//mutex_m/mutex_m.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__pathname_pathname() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//pathname/pathname.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__prime_integer_extension() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//prime/integer-extension.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__prime_prime() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//prime/prime.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__pty_pty() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//pty/pty.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__securerandom_securerandom() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//securerandom/securerandom.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__set_set() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//set/set.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__tmpdir_tmpdir() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//tmpdir/tmpdir.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
#[test]
fn test__zlib_zlib() {
    let string =  fs::read_to_string("./tests/stdlib/stdlib//zlib/zlib.rbs").expect("didnt work");
    let pairs = RBSParser::parse(Rule::decl, &string).unwrap_or_else(|e| panic!("error{}", e));
}
