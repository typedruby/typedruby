extern crate ruby_parser;

#[macro_use]
mod helpers;

use std::path::PathBuf;
use std::rc::Rc;
use ruby_parser::{DiagLevel, DiagClass};

const OPTIONS: ruby_parser::ParserOptions =
ruby_parser::ParserOptions {
  emit_file_vars_as_literals: false,
  emit_lambda: true,
  emit_procarg0: true,
  declare_env: &["foo", "bar", "baz"]
};

#[test]
fn diag_send_plain_cmd_ambiguous_prefix() {
	let code = "m +foo";
	assert_diag!(code, DiagLevel::Warning, DiagClass::AmbiguousPrefix, OPTIONS);
}

#[test]
fn diag_send_plain_cmd_ambiguous_prefix_1() {
	let code = "m -foo";
	assert_diag!(code, DiagLevel::Warning, DiagClass::AmbiguousPrefix, OPTIONS);
}

#[test]
fn diag_send_plain_cmd_ambiguous_prefix_2() {
	let code = "m &foo";
	assert_diag!(code, DiagLevel::Warning, DiagClass::AmbiguousPrefix, OPTIONS);
}

#[test]
fn diag_send_plain_cmd_ambiguous_prefix_3() {
	let code = "m *foo";
	assert_diag!(code, DiagLevel::Warning, DiagClass::AmbiguousPrefix, OPTIONS);
}

#[test]
fn diag_send_plain_cmd_ambiguous_prefix_4() {
	let code = "m **foo";
	assert_diag!(code, DiagLevel::Warning, DiagClass::AmbiguousPrefix, OPTIONS);
}

#[test]
fn diag_codepoint_too_large() {
	let code = "\"\\u{120 120000}\"";
	assert_diag!(code, DiagLevel::Error, DiagClass::UnicodePointTooLarge, OPTIONS);
}

#[test]
fn diag_defs_invalid() {
	let code = "def (1).foo; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::SingletonLiteral, OPTIONS);
}

#[test]
fn diag_defs_invalid_1() {
	let code = "def (\"foo\").foo; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::SingletonLiteral, OPTIONS);
}

#[test]
fn diag_defs_invalid_2() {
	let code = "def (\"foo#{bar}\").foo; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::SingletonLiteral, OPTIONS);
}

#[test]
fn diag_defs_invalid_3() {
	let code = "def (:foo).foo; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::SingletonLiteral, OPTIONS);
}

#[test]
fn diag_defs_invalid_4() {
	let code = "def (:\"foo#{bar}\").foo; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::SingletonLiteral, OPTIONS);
}

#[test]
fn diag_defs_invalid_5() {
	let code = "def ([]).foo; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::SingletonLiteral, OPTIONS);
}

#[test]
fn diag_defs_invalid_6() {
	let code = "def ({}).foo; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::SingletonLiteral, OPTIONS);
}

#[test]
fn diag_defs_invalid_7() {
	let code = "def (/foo/).foo; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::SingletonLiteral, OPTIONS);
}

#[test]
fn diag_asgn_keyword_invalid() {
	let code = "nil = foo";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidAssignment, OPTIONS);
}

#[test]
fn diag_asgn_keyword_invalid_1() {
	let code = "self = foo";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidAssignment, OPTIONS);
}

#[test]
fn diag_asgn_keyword_invalid_2() {
	let code = "true = foo";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidAssignment, OPTIONS);
}

#[test]
fn diag_asgn_keyword_invalid_3() {
	let code = "false = foo";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidAssignment, OPTIONS);
}

#[test]
fn diag_asgn_keyword_invalid_4() {
	let code = "__FILE__ = foo";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidAssignment, OPTIONS);
}

#[test]
fn diag_asgn_keyword_invalid_5() {
	let code = "__LINE__ = foo";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidAssignment, OPTIONS);
}

#[test]
fn diag_cpath_invalid() {
	let code = "module foo; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::ModuleNameConst, OPTIONS);
}

#[test]
fn diag_rescue_else_useless() {
	let code = "begin; 1; else; 2; end";
	assert_diag!(code, DiagLevel::Warning, DiagClass::UselessElse, OPTIONS);
}

#[test]
fn diag_arg_duplicate() {
	let code = "def foo(aa, aa); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DuplicateArgument, OPTIONS);
}

#[test]
fn diag_arg_duplicate_1() {
	let code = "def foo(aa, aa=1); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DuplicateArgument, OPTIONS);
}

#[test]
fn diag_arg_duplicate_2() {
	let code = "def foo(aa, *aa); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DuplicateArgument, OPTIONS);
}

#[test]
fn diag_arg_duplicate_3() {
	let code = "def foo(aa, &aa); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DuplicateArgument, OPTIONS);
}

#[test]
fn diag_arg_duplicate_4() {
	let code = "def foo(aa, (bb, aa)); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DuplicateArgument, OPTIONS);
}

#[test]
fn diag_arg_duplicate_5() {
	let code = "def foo(aa, *r, aa); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DuplicateArgument, OPTIONS);
}

#[test]
fn diag_arg_duplicate_6() {
	let code = "lambda do |aa; aa| end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DuplicateArgument, OPTIONS);
}

#[test]
fn diag_arg_duplicate_7() {
	let code = "def foo(aa, aa: 1); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DuplicateArgument, OPTIONS);
}

#[test]
fn diag_arg_duplicate_8() {
	let code = "def foo(aa, **aa); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DuplicateArgument, OPTIONS);
}

#[test]
fn diag_arg_duplicate_9() {
	let code = "def foo(aa, aa:); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DuplicateArgument, OPTIONS);
}

#[test]
fn diag_op_asgn_invalid() {
	let code = "$1 |= 1";
	assert_diag!(code, DiagLevel::Error, DiagClass::BackrefAssignment, OPTIONS);
}

#[test]
fn diag_op_asgn_invalid_1() {
	let code = "$+ |= 1";
	assert_diag!(code, DiagLevel::Error, DiagClass::BackrefAssignment, OPTIONS);
}

#[test]
fn diag_op_asgn_invalid_2() {
	let code = "$+ |= m foo";
	assert_diag!(code, DiagLevel::Error, DiagClass::BackrefAssignment, OPTIONS);
}

#[test]
fn diag_masgn_const_invalid() {
	let code = "def f; self::A, foo = foo; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DynamicConst, OPTIONS);
}

#[test]
fn diag_masgn_const_invalid_1() {
	let code = "def f; ::A, foo = foo; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DynamicConst, OPTIONS);
}

#[test]
fn diag_on_error() {
	let code = "def foo(bar baz); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::UnexpectedToken, OPTIONS);
}

#[test]
fn diag_module_invalid() {
	let code = "def a; module Foo; end; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::ModuleInDef, OPTIONS);
}

#[test]
fn diag_preexe_invalid() {
	let code = "def f; BEGIN{}; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::BeginInMethod, OPTIONS);
}

#[test]
fn diag_class_invalid() {
	let code = "def a; class Foo; end; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::ClassInDef, OPTIONS);
}

#[test]
fn diag_masgn_keyword_invalid() {
	let code = "nil, foo = bar";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidAssignment, OPTIONS);
}

#[test]
fn diag_log_asgn_invalid() {
	let code = "$1 &&= 1";
	assert_diag!(code, DiagLevel::Error, DiagClass::BackrefAssignment, OPTIONS);
}

#[test]
fn diag_log_asgn_invalid_1() {
	let code = "$+ ||= 1";
	assert_diag!(code, DiagLevel::Error, DiagClass::BackrefAssignment, OPTIONS);
}

#[test]
fn diag_ruby_bug_12686() {
	let code = "f(g rescue nil)";
	assert_diag!(code, DiagLevel::Error, DiagClass::UnexpectedToken, OPTIONS);
}

#[test]
fn diag_arg_duplicate_proc() {
	let code = "proc{|a,a|}";
	assert_diag!(code, DiagLevel::Error, DiagClass::DuplicateArgument, OPTIONS);
}

#[test]
fn diag_kwarg_invalid() {
	let code = "def foo(Abc: 1); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::ArgumentConst, OPTIONS);
}

#[test]
fn diag_kwarg_invalid_1() {
	let code = "def foo(Abc:); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::ArgumentConst, OPTIONS);
}

#[test]
fn diag_var_op_asgn_keyword_invalid() {
	let code = "nil += foo";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidAssignment, OPTIONS);
}

#[test]
fn diag_alias_nth_ref() {
	let code = "alias $a $1";
	assert_diag!(code, DiagLevel::Error, DiagClass::NthRefAlias, OPTIONS);
}

#[test]
fn diag_masgn_backref_invalid() {
	let code = "$1, = foo";
	assert_diag!(code, DiagLevel::Error, DiagClass::BackrefAssignment, OPTIONS);
}

#[test]
fn diag_casgn_invalid() {
	let code = "def f; Foo = 1; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DynamicConst, OPTIONS);
}

#[test]
fn diag_casgn_invalid_1() {
	let code = "def f; Foo::Bar = 1; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DynamicConst, OPTIONS);
}

#[test]
fn diag_casgn_invalid_2() {
	let code = "def f; ::Bar = 1; end";
	assert_diag!(code, DiagLevel::Error, DiagClass::DynamicConst, OPTIONS);
}

#[test]
fn diag_arg_invalid() {
	let code = "def foo(Abc); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::UnexpectedToken, OPTIONS);
}

#[test]
fn diag_arg_invalid_1() {
	let code = "def foo(@abc); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::ArgumentIvar, OPTIONS);
}

#[test]
fn diag_arg_invalid_2() {
	let code = "def foo($abc); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::ArgumentGvar, OPTIONS);
}

#[test]
fn diag_arg_invalid_3() {
	let code = "def foo(@@abc); end";
	assert_diag!(code, DiagLevel::Error, DiagClass::ArgumentCvar, OPTIONS);
}

#[test]
fn diag_unterminated_embedded_doc() {
	let code = "=begin\nfoo\nend";
	assert_diag!(code, DiagLevel::Fatal, DiagClass::EmbeddedDocument, OPTIONS);
}

#[test]
fn diag_unterminated_embedded_doc_1() {
	let code = "=begin\nfoo\nend\n";
	assert_diag!(code, DiagLevel::Fatal, DiagClass::EmbeddedDocument, OPTIONS);
}

#[test]
fn diag_send_plain_cmd_ambiguous_literal() {
	let code = "m /foo/";
	assert_diag!(code, DiagLevel::Warning, DiagClass::AmbiguousLiteral, OPTIONS);
}

#[test]
fn diag_yield_block() {
	let code = "yield foo do end";
	assert_diag!(code, DiagLevel::Error, DiagClass::BlockGivenToYield, OPTIONS);
}

#[test]
fn diag_yield_block_1() {
	let code = "yield(&foo)";
	assert_diag!(code, DiagLevel::Error, DiagClass::BlockGivenToYield, OPTIONS);
}

/*
#[test]
fn diag_bug_ascii_8bit_in_literal() {
	let code = "\".\\xc3.\"";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidEncoding, OPTIONS);
}

#[test]
fn diag_bug_ascii_8bit_in_literal_1() {
	let code = "%W\"x .\\xc3.\"";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidEncoding, OPTIONS);
}

#[test]
fn diag_bug_ascii_8bit_in_literal_2() {
	let code = ":\".\\xc3.\"";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidEncoding, OPTIONS);
}

#[test]
fn diag_bug_ascii_8bit_in_literal_3() {
	let code = "%I\"x .\\xc3.\"";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidEncoding, OPTIONS);
}

#[test]
fn diag_bug_ascii_8bit_in_literal_4() {
	let code = "?\\xc3";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidEncoding, OPTIONS);
}
*/

#[test]
fn diag_unknown_percent_str() {
	let code = "%k[foo]";
	assert_diag!(code, DiagLevel::Error, DiagClass::UnexpectedPercentStr, OPTIONS);
}

#[test]
fn diag_asgn_backref_invalid() {
	let code = "$1 = foo";
	assert_diag!(code, DiagLevel::Error, DiagClass::BackrefAssignment, OPTIONS);
}

#[test]
#[cfg(feature = "regex")]
fn diag_regex_error() {
	let code = "/?/";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidRegexp, OPTIONS);
}

#[test]
#[cfg(feature = "regex")]
fn diag_regex_error_1() {
	let code = "/#{\"\"}?/";
	assert_diag!(code, DiagLevel::Error, DiagClass::InvalidRegexp, OPTIONS);
}

#[test]
fn diag_send_block_blockarg() {
	let code = "fun(&bar) do end";
	assert_diag!(code, DiagLevel::Error, DiagClass::BlockAndBlockarg, OPTIONS);
}

