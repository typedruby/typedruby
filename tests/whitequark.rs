extern crate ruby_parser;
extern crate difference;

#[macro_use]
mod helpers;

use std::path::PathBuf;
use std::rc::Rc;
use difference::{Changeset, Difference};

#[test]
fn test_character() {
	let code = r##"?a"##;
	let sexp = r##"
(str "a")
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_space_args_cmd() {
	let code = r##"fun (f bar)"##;
	let sexp = r##"
(send nil :fun
  (begin
    (send nil :f
      (lvar :bar))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs() {
	let code = r##"f{ |foo: 1, bar: 2, **baz, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (kwoptarg :foo
      (int 1))
    (kwoptarg :bar
      (int 2))
    (kwrestarg :baz)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_1() {
	let code = r##"f{ |foo: 1, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (kwoptarg :foo
      (int 1))
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_2() {
	let code = r##"f{ |**baz, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (kwrestarg :baz)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_plain() {
	let code = r##"foo.fun"##;
	let sexp = r##"
(send
  (lvar :foo) :fun)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_plain_1() {
	let code = r##"foo::fun"##;
	let sexp = r##"
(send
  (lvar :foo) :fun)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_plain_2() {
	let code = r##"foo::Fun()"##;
	let sexp = r##"
(send
  (lvar :foo) :Fun)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_cond_eflipflop() {
	let code = r##"if foo...bar; end"##;
	let sexp = r##"
(if
  (eflipflop
    (lvar :foo)
    (lvar :bar)) nil nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_words_interp() {
	let code = r##"%W[foo #{bar}]"##;
	let sexp = r##"
(array
  (str "foo")
  (dstr
    (begin
      (lvar :bar))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_words_interp_1() {
	let code = r##"%W[foo #{bar}foo#@baz]"##;
	let sexp = r##"
(array
  (str "foo")
  (dstr
    (begin
      (lvar :bar))
    (str "foo")
    (ivar :@baz)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_assocs() {
	let code = r##"[ 1 => 2 ]"##;
	let sexp = r##"
(array
  (hash
    (pair
      (int 1)
      (int 2))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_assocs_1() {
	let code = r##"[ 1, 2 => 3 ]"##;
	let sexp = r##"
(array
  (int 1)
  (hash
    (pair
      (int 2)
      (int 3))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rescue_else() {
	let code = r##"begin; meth; rescue; foo; else; bar; end"##;
	let sexp = r##"
(kwbegin
  (rescue
    (send nil :meth)
    (resbody nil nil
      (lvar :foo))
    (lvar :bar)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_symbols_empty() {
	let code = r##"%i[]"##;
	let sexp = r##"
(array)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_symbols_empty_1() {
	let code = r##"%I()"##;
	let sexp = r##"
(array)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_xstring_interp() {
	let code = r##"`foo#{bar}baz`"##;
	let sexp = r##"
(xstr
  (str "foo")
  (begin
    (lvar :bar))
  (str "baz"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_postexe() {
	let code = r##"END { 1 }"##;
	let sexp = r##"
(postexe
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_range_exclusive() {
	let code = r##"1...2"##;
	let sexp = r##"
(erange
  (int 1)
  (int 2))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_lambda_args_noparen() {
	let code = r##"-> a: 1 { }"##;
	let sexp = r##"
(block
  (lambda)
  (args
    (kwoptarg :a
      (int 1))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_lambda_args_noparen_1() {
	let code = r##"-> a: { }"##;
	let sexp = r##"
(block
  (lambda)
  (args
    (kwarg :a)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_op_asgn() {
	let code = r##"foo.a += 1"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :a) :+
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_op_asgn_1() {
	let code = r##"foo::a += 1"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :a) :+
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_op_asgn_2() {
	let code = r##"foo.A += 1"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :A) :+
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_args_assocs_comma() {
	let code = r##"foo[bar, :baz => 1,]"##;
	let sexp = r##"
(send
  (lvar :foo) :[]
  (lvar :bar)
  (hash
    (pair
      (sym :baz)
      (int 1))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_cvasgn() {
	let code = r##"@@var = 10"##;
	let sexp = r##"
(cvasgn :@@var
  (int 10))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_regexp_encoding() {
	let code = r##"/\xa8/n =~ """##;
	let sexp = r##"
(match-with-lvasgn
  (regexp
    (str "\\xa8")
    (regopt :n))
  (str ""))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_asgn_mrhs() {
	let code = r##"foo = bar, 1"##;
	let sexp = r##"
(lvasgn :foo
  (array
    (lvar :bar)
    (int 1)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_asgn_mrhs_1() {
	let code = r##"foo = *bar"##;
	let sexp = r##"
(lvasgn :foo
  (array
    (splat
      (lvar :bar))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_asgn_mrhs_2() {
	let code = r##"foo = baz, *bar"##;
	let sexp = r##"
(lvasgn :foo
  (array
    (lvar :baz)
    (splat
      (lvar :bar))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rescue_else_useless() {
	let code = r##"begin; else; 2; end"##;
	let sexp = r##"
(kwbegin
  (begin
    (int 2)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rescue_else_useless_1() {
	let code = r##"begin; 1; else; 2; end"##;
	let sexp = r##"
(kwbegin
  (int 1)
  (begin
    (int 2)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rescue_else_useless_2() {
	let code = r##"begin; 1; 2; else; 3; end"##;
	let sexp = r##"
(kwbegin
  (int 1)
  (int 2)
  (begin
    (int 3)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_sclass() {
	let code = r##"class << foo; nil; end"##;
	let sexp = r##"
(sclass
  (lvar :foo)
  (nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_space_args_hash_literal_then_block() {
	let code = r##"f 1, {1 => 2} {1}"##;
	let sexp = r##"
(block
  (send nil :f
    (int 1)
    (hash
      (pair
        (int 1)
        (int 2))))
  (args)
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_nth_ref() {
	let code = r##"$10"##;
	let sexp = r##"
(nth-ref 10)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_10279() {
	let code = r##"{a: if true then 42 end}"##;
	let sexp = r##"
(hash
  (pair
    (sym :a)
    (if
      (true)
      (int 42) nil)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_until_post() {
	let code = r##"begin meth end until foo"##;
	let sexp = r##"
(until-post
  (lvar :foo)
  (kwbegin
    (send nil :meth)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_symbols() {
	let code = r##"%i[foo bar]"##;
	let sexp = r##"
(array
  (sym :foo)
  (sym :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_regex_interp() {
	let code = r##"/foo#{bar}baz/"##;
	let sexp = r##"
(regexp
  (str "foo")
  (begin
    (lvar :bar))
  (str "baz")
  (regopt))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_space_args_arg_call() {
	let code = r##"fun (1).to_i"##;
	let sexp = r##"
(send nil :fun
  (send
    (begin
      (int 1)) :to_i))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_class_super_label() {
	let code = r##"class Foo < a:b; end"##;
	let sexp = r##"
(class
  (const nil :Foo)
  (send nil :a
    (sym :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_self() {
	let code = r##"fun"##;
	let sexp = r##"
(send nil :fun)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_self_1() {
	let code = r##"fun!"##;
	let sexp = r##"
(send nil :fun!)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_self_2() {
	let code = r##"fun(1)"##;
	let sexp = r##"
(send nil :fun
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_symbols_interp() {
	let code = r##"%I[foo #{bar}]"##;
	let sexp = r##"
(array
  (sym :foo)
  (dsym
    (begin
      (lvar :bar))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_symbols_interp_1() {
	let code = r##"%I[foo#{bar}]"##;
	let sexp = r##"
(array
  (dsym
    (str "foo")
    (begin
      (lvar :bar))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_cmdarg() {
	let code = r##"assert dogs"##;
	let sexp = r##"
(send nil :assert
  (send nil :dogs))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_cmdarg_1() {
	let code = r##"assert do: true"##;
	let sexp = r##"
(send nil :assert
  (hash
    (pair
      (sym :do)
      (true))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_cmdarg_2() {
	let code = r##"f x: -> do meth do end end"##;
	let sexp = r##"
(send nil :f
  (hash
    (pair
      (sym :x)
      (block
        (lambda)
        (args)
        (block
          (send nil :meth)
          (args) nil)))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_restarg_unnamed() {
	let code = r##"def f(*); end"##;
	let sexp = r##"
(def :f
  (args
    (restarg)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_while_post() {
	let code = r##"begin meth end while foo"##;
	let sexp = r##"
(while-post
  (lvar :foo)
  (kwbegin
    (send nil :meth)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_while_not_parens_do() {
	let code = r##"while not (true) do end"##;
	let sexp = r##"
(while
  (send
    (begin
      (true)) :!) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_block_chain_cmd() {
	let code = r##"meth 1 do end.fun bar"##;
	let sexp = r##"
(send
  (block
    (send nil :meth
      (int 1))
    (args) nil) :fun
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_block_chain_cmd_1() {
	let code = r##"meth 1 do end.fun(bar)"##;
	let sexp = r##"
(send
  (block
    (send nil :meth
      (int 1))
    (args) nil) :fun
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_block_chain_cmd_2() {
	let code = r##"meth 1 do end::fun bar"##;
	let sexp = r##"
(send
  (block
    (send nil :meth
      (int 1))
    (args) nil) :fun
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_block_chain_cmd_3() {
	let code = r##"meth 1 do end::fun(bar)"##;
	let sexp = r##"
(send
  (block
    (send nil :meth
      (int 1))
    (args) nil) :fun
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_block_chain_cmd_4() {
	let code = r##"meth 1 do end.fun bar do end"##;
	let sexp = r##"
(block
  (send
    (block
      (send nil :meth
        (int 1))
      (args) nil) :fun
    (lvar :bar))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_block_chain_cmd_5() {
	let code = r##"meth 1 do end.fun(bar) {}"##;
	let sexp = r##"
(block
  (send
    (block
      (send nil :meth
        (int 1))
      (args) nil) :fun
    (lvar :bar))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_block_chain_cmd_6() {
	let code = r##"meth 1 do end.fun {}"##;
	let sexp = r##"
(block
  (send
    (block
      (send nil :meth
        (int 1))
      (args) nil) :fun)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_preexe() {
	let code = r##"BEGIN { 1 }"##;
	let sexp = r##"
(preexe
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_casgn_toplevel() {
	let code = r##"::Foo = 10"##;
	let sexp = r##"
(casgn
  (cbase) :Foo
  (int 10))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_lambda() {
	let code = r##"->{ }"##;
	let sexp = r##"
(block
  (lambda)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_lambda_1() {
	let code = r##"-> * { }"##;
	let sexp = r##"
(block
  (lambda)
  (args
    (restarg)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_lambda_2() {
	let code = r##"-> do end"##;
	let sexp = r##"
(block
  (lambda)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_defs() {
	let code = r##"def self.foo; end"##;
	let sexp = r##"
(defs
  (self) :foo
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_defs_1() {
	let code = r##"def self::foo; end"##;
	let sexp = r##"
(defs
  (self) :foo
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_defs_2() {
	let code = r##"def (foo).foo; end"##;
	let sexp = r##"
(defs
  (lvar :foo) :foo
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_defs_3() {
	let code = r##"def String.foo; end"##;
	let sexp = r##"
(defs
  (const nil :String) :foo
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_defs_4() {
	let code = r##"def String::foo; end"##;
	let sexp = r##"
(defs
  (const nil :String) :foo
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rescue_mod_asgn() {
	let code = r##"foo = meth rescue bar"##;
	let sexp = r##"
(lvasgn :foo
  (rescue
    (send nil :meth)
    (resbody nil nil
      (lvar :bar)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_self() {
	let code = r##"self"##;
	let sexp = r##"
(self)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_while() {
	let code = r##"while foo do meth end"##;
	let sexp = r##"
(while
  (lvar :foo)
  (send nil :meth))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_while_1() {
	let code = r##"while foo; meth end"##;
	let sexp = r##"
(while
  (lvar :foo)
  (send nil :meth))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_empty_stmt() {
	let code = r##""##;
	let sexp = r##"

"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_back_ref() {
	let code = r##"$+"##;
	let sexp = r##"
(back-ref :$+)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_kwoptarg() {
	let code = r##"def f(foo: 1); end"##;
	let sexp = r##"
(def :f
  (args
    (kwoptarg :foo
      (int 1))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11989() {
	let code = r##"p <<~"E"
  x\n   y
E"##;
	let sexp = r##"
(send nil :p
  (str "x\n   y\n"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_lambda_leakage() {
	let code = r##"->(scope) {}; scope"##;
	let sexp = r##"
(begin
  (block
    (lambda)
    (args
      (arg :scope)) nil)
  (send nil :scope))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rational() {
	let code = r##"42r"##;
	let sexp = r##"
(rational (42/1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rational_1() {
	let code = r##"42.1r"##;
	let sexp = r##"
(rational (421/10))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_until() {
	let code = r##"until foo do meth end"##;
	let sexp = r##"
(until
  (lvar :foo)
  (send nil :meth))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_until_1() {
	let code = r##"until foo; meth end"##;
	let sexp = r##"
(until
  (lvar :foo)
  (send nil :meth))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_string_interp() {
	let code = r##""foo#{bar}baz""##;
	let sexp = r##"
(dstr
  (str "foo")
  (begin
    (lvar :bar))
  (str "baz"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_and() {
	let code = r##"foo and bar"##;
	let sexp = r##"
(and
  (lvar :foo)
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_and_1() {
	let code = r##"foo && bar"##;
	let sexp = r##"
(and
  (lvar :foo)
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rescue_ensure() {
	let code = r##"begin; meth; rescue; baz; ensure; bar; end"##;
	let sexp = r##"
(kwbegin
  (ensure
    (rescue
      (send nil :meth)
      (resbody nil nil
        (lvar :baz)) nil)
    (lvar :bar)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_space_args_arg_newline() {
	let code = r##"fun (1
)"##;
	let sexp = r##"
(send nil :fun
  (begin
    (int 1)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ensure() {
	let code = r##"begin; meth; ensure; bar; end"##;
	let sexp = r##"
(kwbegin
  (ensure
    (send nil :meth)
    (lvar :bar)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a() {
	let code = r##"a b{c d}, :e do end"##;
	let sexp = r##"
(block
  (send nil :a
    (block
      (send nil :b)
      (args)
      (send nil :c
        (send nil :d)))
    (sym :e))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_1() {
	let code = r##"a b{c(d)}, :e do end"##;
	let sexp = r##"
(block
  (send nil :a
    (block
      (send nil :b)
      (args)
      (send nil :c
        (send nil :d)))
    (sym :e))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_2() {
	let code = r##"a b(c d), :e do end"##;
	let sexp = r##"
(block
  (send nil :a
    (send nil :b
      (send nil :c
        (send nil :d)))
    (sym :e))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_3() {
	let code = r##"a b(c(d)), :e do end"##;
	let sexp = r##"
(block
  (send nil :a
    (send nil :b
      (send nil :c
        (send nil :d)))
    (sym :e))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_4() {
	let code = r##"a b{c d}, 1 do end"##;
	let sexp = r##"
(block
  (send nil :a
    (block
      (send nil :b)
      (args)
      (send nil :c
        (send nil :d)))
    (int 1))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_5() {
	let code = r##"a b{c(d)}, 1 do end"##;
	let sexp = r##"
(block
  (send nil :a
    (block
      (send nil :b)
      (args)
      (send nil :c
        (send nil :d)))
    (int 1))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_6() {
	let code = r##"a b(c d), 1 do end"##;
	let sexp = r##"
(block
  (send nil :a
    (send nil :b
      (send nil :c
        (send nil :d)))
    (int 1))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_7() {
	let code = r##"a b(c(d)), 1 do end"##;
	let sexp = r##"
(block
  (send nil :a
    (send nil :b
      (send nil :c
        (send nil :d)))
    (int 1))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_8() {
	let code = r##"a b{c d}, 1.0 do end"##;
	let sexp = r##"
(block
  (send nil :a
    (block
      (send nil :b)
      (args)
      (send nil :c
        (send nil :d)))
    (float 1.0))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_9() {
	let code = r##"a b{c(d)}, 1.0 do end"##;
	let sexp = r##"
(block
  (send nil :a
    (block
      (send nil :b)
      (args)
      (send nil :c
        (send nil :d)))
    (float 1.0))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_10() {
	let code = r##"a b(c d), 1.0 do end"##;
	let sexp = r##"
(block
  (send nil :a
    (send nil :b
      (send nil :c
        (send nil :d)))
    (float 1.0))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_11() {
	let code = r##"a b(c(d)), 1.0 do end"##;
	let sexp = r##"
(block
  (send nil :a
    (send nil :b
      (send nil :c
        (send nil :d)))
    (float 1.0))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_12() {
	let code = r##"a b{c d}, 1.0r do end"##;
	let sexp = r##"
(block
  (send nil :a
    (block
      (send nil :b)
      (args)
      (send nil :c
        (send nil :d)))
    (rational (1/1)))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_13() {
	let code = r##"a b{c(d)}, 1.0r do end"##;
	let sexp = r##"
(block
  (send nil :a
    (block
      (send nil :b)
      (args)
      (send nil :c
        (send nil :d)))
    (rational (1/1)))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_14() {
	let code = r##"a b(c d), 1.0r do end"##;
	let sexp = r##"
(block
  (send nil :a
    (send nil :b
      (send nil :c
        (send nil :d)))
    (rational (1/1)))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_15() {
	let code = r##"a b(c(d)), 1.0r do end"##;
	let sexp = r##"
(block
  (send nil :a
    (send nil :b
      (send nil :c
        (send nil :d)))
    (rational (1/1)))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_16() {
	let code = r##"a b{c d}, 1.0i do end"##;
	let sexp = r##"
(block
  (send nil :a
    (block
      (send nil :b)
      (args)
      (send nil :c
        (send nil :d)))
    (complex (0.0+1.0i)))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_17() {
	let code = r##"a b{c(d)}, 1.0i do end"##;
	let sexp = r##"
(block
  (send nil :a
    (block
      (send nil :b)
      (args)
      (send nil :c
        (send nil :d)))
    (complex (0.0+1.0i)))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_18() {
	let code = r##"a b(c d), 1.0i do end"##;
	let sexp = r##"
(block
  (send nil :a
    (send nil :b
      (send nil :c
        (send nil :d)))
    (complex (0.0+1.0i)))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_a_19() {
	let code = r##"a b(c(d)), 1.0i do end"##;
	let sexp = r##"
(block
  (send nil :a
    (send nil :b
      (send nil :c
        (send nil :d)))
    (complex (0.0+1.0i)))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_nil_expression() {
	let code = r##"()"##;
	let sexp = r##"
(begin)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_nil_expression_1() {
	let code = r##"begin end"##;
	let sexp = r##"
(kwbegin)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_attr_asgn_conditional() {
	let code = r##"a&.b = 1"##;
	let sexp = r##"
(csend
  (send nil :a) :b=
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_space_args_arg_block() {
	let code = r##"fun (1) {}"##;
	let sexp = r##"
(block
  (send nil :fun
    (begin
      (int 1)))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_space_args_arg_block_1() {
	let code = r##"foo.fun (1) {}"##;
	let sexp = r##"
(block
  (send
    (lvar :foo) :fun
    (begin
      (int 1)))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_space_args_arg_block_2() {
	let code = r##"foo::fun (1) {}"##;
	let sexp = r##"
(block
  (send
    (lvar :foo) :fun
    (begin
      (int 1)))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_self_block() {
	let code = r##"fun { }"##;
	let sexp = r##"
(block
  (send nil :fun)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_self_block_1() {
	let code = r##"fun() { }"##;
	let sexp = r##"
(block
  (send nil :fun)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_self_block_2() {
	let code = r##"fun(1) { }"##;
	let sexp = r##"
(block
  (send nil :fun
    (int 1))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_self_block_3() {
	let code = r##"fun do end"##;
	let sexp = r##"
(block
  (send nil :fun)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_encoding_() {
	let code = r##"__ENCODING__"##;
	let sexp = r##"
(const
  (const nil :Encoding) :UTF_8)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_args_assocs() {
	let code = r##"fun(foo, :foo => 1)"##;
	let sexp = r##"
(send nil :fun
  (lvar :foo)
  (hash
    (pair
      (sym :foo)
      (int 1))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_args_assocs_1() {
	let code = r##"fun(foo, :foo => 1, &baz)"##;
	let sexp = r##"
(send nil :fun
  (lvar :foo)
  (hash
    (pair
      (sym :foo)
      (int 1)))
  (block-pass
    (lvar :baz)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_float() {
	let code = r##"1.33"##;
	let sexp = r##"
(float 1.33)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_float_1() {
	let code = r##"-1.33"##;
	let sexp = r##"
(float -1.33)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_string_plain() {
	let code = r##"'foobar'"##;
	let sexp = r##"
(str "foobar")
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_string_plain_1() {
	let code = r##"%q(foobar)"##;
	let sexp = r##"
(str "foobar")
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_def() {
	let code = r##"def foo; end"##;
	let sexp = r##"
(def :foo
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_def_1() {
	let code = r##"def String; end"##;
	let sexp = r##"
(def :String
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_def_2() {
	let code = r##"def String=; end"##;
	let sexp = r##"
(def :String=
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_def_3() {
	let code = r##"def until; end"##;
	let sexp = r##"
(def :until
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_hash_hashrocket() {
	let code = r##"{ 1 => 2 }"##;
	let sexp = r##"
(hash
  (pair
    (int 1)
    (int 2)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_hash_hashrocket_1() {
	let code = r##"{ 1 => 2, :foo => "bar" }"##;
	let sexp = r##"
(hash
  (pair
    (int 1)
    (int 2))
  (pair
    (sym :foo)
    (str "bar")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_cmd_string_lookahead() {
	let code = r##"desc "foo" do end"##;
	let sexp = r##"
(block
  (send nil :desc
    (str "foo"))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_class_super() {
	let code = r##"class Foo < Bar; end"##;
	let sexp = r##"
(class
  (const nil :Foo)
  (const nil :Bar) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_nil() {
	let code = r##"nil"##;
	let sexp = r##"
(nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_casgn_scoped() {
	let code = r##"Bar::Foo = 10"##;
	let sexp = r##"
(casgn
  (const nil :Bar) :Foo
  (int 10))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_words_empty() {
	let code = r##"%w[]"##;
	let sexp = r##"
(array)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_words_empty_1() {
	let code = r##"%W()"##;
	let sexp = r##"
(array)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_parser_bug_198() {
	let code = r##"[/()\1/, ?#]"##;
	let sexp = r##"
(array
  (regexp
    (str "()\\1")
    (regopt))
  (str "#"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_unless_else() {
	let code = r##"unless foo then bar; else baz; end"##;
	let sexp = r##"
(if
  (lvar :foo)
  (lvar :baz)
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_unless_else_1() {
	let code = r##"unless foo; bar; else baz; end"##;
	let sexp = r##"
(if
  (lvar :foo)
  (lvar :baz)
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_kwrestarg_named() {
	let code = r##"def f(**foo); end"##;
	let sexp = r##"
(def :f
  (args
    (kwrestarg :foo)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_super() {
	let code = r##"super(foo)"##;
	let sexp = r##"
(super
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_super_1() {
	let code = r##"super foo"##;
	let sexp = r##"
(super
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_super_2() {
	let code = r##"super()"##;
	let sexp = r##"
(super)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_and_or_masgn() {
	let code = r##"foo && (a, b = bar)"##;
	let sexp = r##"
(and
  (lvar :foo)
  (begin
    (masgn
      (mlhs
        (lvasgn :a)
        (lvasgn :b))
      (lvar :bar))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_and_or_masgn_1() {
	let code = r##"foo || (a, b = bar)"##;
	let sexp = r##"
(or
  (lvar :foo)
  (begin
    (masgn
      (mlhs
        (lvasgn :a)
        (lvasgn :b))
      (lvar :bar))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_hash_kwsplat() {
	let code = r##"{ foo: 2, **bar }"##;
	let sexp = r##"
(hash
  (pair
    (sym :foo)
    (int 2))
  (kwsplat
    (lvar :bar)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_resbody_var() {
	let code = r##"begin; meth; rescue => ex; bar; end"##;
	let sexp = r##"
(kwbegin
  (rescue
    (send nil :meth)
    (resbody nil
      (lvasgn :ex)
      (lvar :bar)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_resbody_var_1() {
	let code = r##"begin; meth; rescue => @ex; bar; end"##;
	let sexp = r##"
(kwbegin
  (rescue
    (send nil :meth)
    (resbody nil
      (ivasgn :@ex)
      (lvar :bar)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_not_cmd() {
	let code = r##"not m foo"##;
	let sexp = r##"
(send
  (send nil :m
    (lvar :foo)) :!)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_cpath() {
	let code = r##"module ::Foo; end"##;
	let sexp = r##"
(module
  (const
    (cbase) :Foo) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_cpath_1() {
	let code = r##"module Bar::Foo; end"##;
	let sexp = r##"
(module
  (const
    (const nil :Bar) :Foo) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_if() {
	let code = r##"if foo then bar; end"##;
	let sexp = r##"
(if
  (lvar :foo)
  (lvar :bar) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_if_1() {
	let code = r##"if foo; bar; end"##;
	let sexp = r##"
(if
  (lvar :foo)
  (lvar :bar) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_when_splat() {
	let code = r##"case foo; when 1, *baz; bar; when *foo; end"##;
	let sexp = r##"
(case
  (lvar :foo)
  (when
    (int 1)
    (splat
      (lvar :baz))
    (lvar :bar))
  (when
    (splat
      (lvar :foo)) nil) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_interp_single() {
	let code = r##""#{1}""##;
	let sexp = r##"
(dstr
  (begin
    (int 1)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_interp_single_1() {
	let code = r##"%W"#{1}""##;
	let sexp = r##"
(array
  (dstr
    (begin
      (int 1))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_defined() {
	let code = r##"defined? foo"##;
	let sexp = r##"
(defined?
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_defined_1() {
	let code = r##"defined?(foo)"##;
	let sexp = r##"
(defined?
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_defined_2() {
	let code = r##"defined? @foo"##;
	let sexp = r##"
(defined?
  (ivar :@foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_case_expr_else() {
	let code = r##"case foo; when 'bar'; bar; else baz; end"##;
	let sexp = r##"
(case
  (lvar :foo)
  (when
    (str "bar")
    (lvar :bar))
  (lvar :baz))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_complex() {
	let code = r##"42i"##;
	let sexp = r##"
(complex (0+42i))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_complex_1() {
	let code = r##"42ri"##;
	let sexp = r##"
(complex (0+(42/1)*i))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_complex_2() {
	let code = r##"42.1i"##;
	let sexp = r##"
(complex (0+42.1i))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_complex_3() {
	let code = r##"42.1ri"##;
	let sexp = r##"
(complex (0+(421/10)*i))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_for_mlhs() {
	let code = r##"for a, b in foo; p a, b; end"##;
	let sexp = r##"
(for
  (mlhs
    (lvasgn :a)
    (lvasgn :b))
  (lvar :foo)
  (send nil :p
    (lvar :a)
    (lvar :b)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_non_lvar_injecting_match() {
	let code = r##"/#{1}(?<match>bar)/ =~ 'bar'"##;
	let sexp = r##"
(send
  (regexp
    (begin
      (int 1))
    (str "(?<match>bar)")
    (regopt)) :=~
  (str "bar"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_star() {
	let code = r##"fun(*bar)"##;
	let sexp = r##"
(send nil :fun
  (splat
    (lvar :bar)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_star_1() {
	let code = r##"fun(*bar, &baz)"##;
	let sexp = r##"
(send nil :fun
  (splat
    (lvar :bar))
  (block-pass
    (lvar :baz)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_kwarg() {
	let code = r##"def f(foo:); end"##;
	let sexp = r##"
(def :f
  (args
    (kwarg :foo)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args() {
	let code = r##"def f foo:
; end"##;
	let sexp = r##"
(def :f
  (args
    (kwarg :foo)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_1() {
	let code = r##"def f foo: -1
; end"##;
	let sexp = r##"
(def :f
  (args
    (kwoptarg :foo
      (int -1))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_true() {
	let code = r##"true"##;
	let sexp = r##"
(true)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_zsuper() {
	let code = r##"super"##;
	let sexp = r##"
(zsuper)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_range_inclusive() {
	let code = r##"1..2"##;
	let sexp = r##"
(irange
  (int 1)
  (int 2))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_gvasgn() {
	let code = r##"$var = 10"##;
	let sexp = r##"
(gvasgn :$var
  (int 10))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_resbody_list() {
	let code = r##"begin; meth; rescue Exception; bar; end"##;
	let sexp = r##"
(kwbegin
  (rescue
    (send nil :meth)
    (resbody
      (array
        (const nil :Exception)) nil
      (lvar :bar)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_undef() {
	let code = r##"undef foo, :bar, :"foo#{1}""##;
	let sexp = r##"
(undef
  (sym :foo)
  (sym :bar)
  (dsym
    (str "foo")
    (begin
      (int 1))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_cvar() {
	let code = r##"@@foo"##;
	let sexp = r##"
(cvar :@@foo)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_assocs() {
	let code = r##"fun(:foo => 1)"##;
	let sexp = r##"
(send nil :fun
  (hash
    (pair
      (sym :foo)
      (int 1))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_assocs_1() {
	let code = r##"fun(:foo => 1, &baz)"##;
	let sexp = r##"
(send nil :fun
  (hash
    (pair
      (sym :foo)
      (int 1)))
  (block-pass
    (lvar :baz)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_cond_begin() {
	let code = r##"if (bar); foo; end"##;
	let sexp = r##"
(if
  (begin
    (lvar :bar))
  (lvar :foo) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_index_cmd() {
	let code = r##"foo[m bar]"##;
	let sexp = r##"
(send
  (lvar :foo) :[]
  (send nil :m
    (lvar :bar)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_op_asgn_index_cmd() {
	let code = r##"foo[0, 1] += m foo"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :[]
    (int 0)
    (int 1)) :+
  (send nil :m
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_for() {
	let code = r##"for a in foo do p a; end"##;
	let sexp = r##"
(for
  (lvasgn :a)
  (lvar :foo)
  (send nil :p
    (lvar :a)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_for_1() {
	let code = r##"for a in foo; p a; end"##;
	let sexp = r##"
(for
  (lvasgn :a)
  (lvar :foo)
  (send nil :p
    (lvar :a)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_resbody_list_var() {
	let code = r##"begin; meth; rescue foo => ex; bar; end"##;
	let sexp = r##"
(kwbegin
  (rescue
    (send nil :meth)
    (resbody
      (array
        (lvar :foo))
      (lvasgn :ex)
      (lvar :bar)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_asgn_cmd() {
	let code = r##"foo = m foo"##;
	let sexp = r##"
(lvasgn :foo
  (send nil :m
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_asgn_cmd_1() {
	let code = r##"foo = bar = m foo"##;
	let sexp = r##"
(lvasgn :foo
  (lvasgn :bar
    (send nil :m
      (lvar :foo))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_case_cond() {
	let code = r##"case; when foo; 'foo'; end"##;
	let sexp = r##"
(case nil
  (when
    (lvar :foo)
    (str "foo")) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_symbol_interp() {
	let code = r##":"foo#{bar}baz""##;
	let sexp = r##"
(dsym
  (str "foo")
  (begin
    (lvar :bar))
  (str "baz"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_not() {
	let code = r##"not foo"##;
	let sexp = r##"
(send
  (lvar :foo) :!)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_not_1() {
	let code = r##"not(foo)"##;
	let sexp = r##"
(send
  (lvar :foo) :!)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_not_2() {
	let code = r##"not()"##;
	let sexp = r##"
(send
  (begin) :!)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_10653() {
	let code = r##"true ? 1.tap do |n| p n end : 0"##;
	let sexp = r##"
(if
  (true)
  (block
    (send
      (int 1) :tap)
    (args
      (procarg0 :n))
    (send nil :p
      (lvar :n)))
  (int 0))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_10653_1() {
	let code = r##"false ? raise {} : tap {}"##;
	let sexp = r##"
(if
  (false)
  (block
    (send nil :raise)
    (args) nil)
  (block
    (send nil :tap)
    (args) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_10653_2() {
	let code = r##"false ? raise do end : tap do end"##;
	let sexp = r##"
(if
  (false)
  (block
    (send nil :raise)
    (args) nil)
  (block
    (send nil :tap)
    (args) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_false() {
	let code = r##"false"##;
	let sexp = r##"
(false)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_index_asgn() {
	let code = r##"foo[1, 2] = 3"##;
	let sexp = r##"
(send
  (lvar :foo) :[]=
  (int 1)
  (int 2)
  (int 3))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_if_else() {
	let code = r##"if foo then bar; else baz; end"##;
	let sexp = r##"
(if
  (lvar :foo)
  (lvar :bar)
  (lvar :baz))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_if_else_1() {
	let code = r##"if foo; bar; else baz; end"##;
	let sexp = r##"
(if
  (lvar :foo)
  (lvar :bar)
  (lvar :baz))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_unless_mod() {
	let code = r##"bar unless foo"##;
	let sexp = r##"
(if
  (lvar :foo) nil
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_yield() {
	let code = r##"yield(foo)"##;
	let sexp = r##"
(yield
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_yield_1() {
	let code = r##"yield foo"##;
	let sexp = r##"
(yield
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_yield_2() {
	let code = r##"yield()"##;
	let sexp = r##"
(yield)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_yield_3() {
	let code = r##"yield"##;
	let sexp = r##"
(yield)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_alias_gvar() {
	let code = r##"alias $a $b"##;
	let sexp = r##"
(alias
  (gvar :$a)
  (gvar :$b))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_alias_gvar_1() {
	let code = r##"alias $a $+"##;
	let sexp = r##"
(alias
  (gvar :$a)
  (back-ref :$+))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12073() {
	let code = r##"a = 1; a b: 1"##;
	let sexp = r##"
(begin
  (lvasgn :a
    (int 1))
  (send nil :a
    (hash
      (pair
        (sym :b)
        (int 1)))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12073_1() {
	let code = r##"def foo raise; raise A::B, ''; end"##;
	let sexp = r##"
(def :foo
  (args
    (arg :raise))
  (send nil :raise
    (const
      (const nil :A) :B)
    (str "")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_op_asgn_cmd() {
	let code = r##"foo.a += m foo"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :a) :+
  (send nil :m
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_op_asgn_cmd_1() {
	let code = r##"foo::a += m foo"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :a) :+
  (send nil :m
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_op_asgn_cmd_2() {
	let code = r##"foo.A += m foo"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :A) :+
  (send nil :m
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_op_asgn_cmd_3() {
	let code = r##"foo::A += m foo"##;
	let sexp = r##"
(op-asgn
  (casgn
    (lvar :foo) :A) :+
  (send nil :m
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_until_mod() {
	let code = r##"meth until foo"##;
	let sexp = r##"
(until
  (lvar :foo)
  (send nil :meth))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_ascii_8bit_in_literal() {
	let code = r##"# coding:utf-8
         "\xD0\xBF\xD1\x80\xD0\xBE\xD0\xB2\xD0\xB5\xD1\x80\xD0\xBA\xD0\xB0""##;
	let sexp = r##"
(str "проверка")
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_redo() {
	let code = r##"redo"##;
	let sexp = r##"
(redo)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_lambda_args() {
	let code = r##"->(a) { }"##;
	let sexp = r##"
(block
  (lambda)
  (args
    (arg :a)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_lambda_args_1() {
	let code = r##"-> (a) { }"##;
	let sexp = r##"
(block
  (lambda)
  (args
    (arg :a)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_args_star() {
	let code = r##"fun(foo, *bar)"##;
	let sexp = r##"
(send nil :fun
  (lvar :foo)
  (splat
    (lvar :bar)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_args_star_1() {
	let code = r##"fun(foo, *bar, &baz)"##;
	let sexp = r##"
(send nil :fun
  (lvar :foo)
  (splat
    (lvar :bar))
  (block-pass
    (lvar :baz)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ternary() {
	let code = r##"foo ? 1 : 2"##;
	let sexp = r##"
(if
  (lvar :foo)
  (int 1)
  (int 2))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_xstring_plain() {
	let code = r##"`foobar`"##;
	let sexp = r##"
(xstr
  (str "foobar"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_when_multi() {
	let code = r##"case foo; when 'bar', 'baz'; bar; end"##;
	let sexp = r##"
(case
  (lvar :foo)
  (when
    (str "bar")
    (str "baz")
    (lvar :bar)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_nested() {
	let code = r##"a, (b, c) = foo"##;
	let sexp = r##"
(masgn
  (mlhs
    (lvasgn :a)
    (mlhs
      (lvasgn :b)
      (lvasgn :c)))
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_nested_1() {
	let code = r##"((b, )) = foo"##;
	let sexp = r##"
(masgn
  (mlhs
    (lvasgn :b))
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_splat() {
	let code = r##"@foo, @@bar = *foo"##;
	let sexp = r##"
(masgn
  (mlhs
    (ivasgn :@foo)
    (cvasgn :@@bar))
  (array
    (splat
      (lvar :foo))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_splat_1() {
	let code = r##"a, b = *foo, bar"##;
	let sexp = r##"
(masgn
  (mlhs
    (lvasgn :a)
    (lvasgn :b))
  (array
    (splat
      (lvar :foo))
    (lvar :bar)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_splat_2() {
	let code = r##"a, *b = bar"##;
	let sexp = r##"
(masgn
  (mlhs
    (lvasgn :a)
    (splat
      (lvasgn :b)))
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_splat_3() {
	let code = r##"a, *b, c = bar"##;
	let sexp = r##"
(masgn
  (mlhs
    (lvasgn :a)
    (splat
      (lvasgn :b))
    (lvasgn :c))
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_splat_4() {
	let code = r##"a, * = bar"##;
	let sexp = r##"
(masgn
  (mlhs
    (lvasgn :a)
    (splat))
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_splat_5() {
	let code = r##"a, *, c = bar"##;
	let sexp = r##"
(masgn
  (mlhs
    (lvasgn :a)
    (splat)
    (lvasgn :c))
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_splat_6() {
	let code = r##"*b = bar"##;
	let sexp = r##"
(masgn
  (mlhs
    (splat
      (lvasgn :b)))
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_splat_7() {
	let code = r##"*b, c = bar"##;
	let sexp = r##"
(masgn
  (mlhs
    (splat
      (lvasgn :b))
    (lvasgn :c))
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_splat_8() {
	let code = r##"* = bar"##;
	let sexp = r##"
(masgn
  (mlhs
    (splat))
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_splat_9() {
	let code = r##"*, c, d = bar"##;
	let sexp = r##"
(masgn
  (mlhs
    (splat)
    (lvasgn :c)
    (lvasgn :d))
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_heredoc_do() {
	let code = r##"f <<-TABLE do
TABLE
end"##;
	let sexp = r##"
(block
  (send nil :f
    (dstr))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_lvar_injecting_match() {
	let code = r##"/(?<match>bar)/ =~ 'bar'; match"##;
	let sexp = r##"
(begin
  (match-with-lvasgn
    (regexp
      (str "(?<match>bar)")
      (regopt))
    (str "bar"))
  (lvar :match))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rescue_else_ensure() {
	let code = r##"begin; meth; rescue; baz; else foo; ensure; bar end"##;
	let sexp = r##"
(kwbegin
  (ensure
    (rescue
      (send nil :meth)
      (resbody nil nil
        (lvar :baz))
      (lvar :foo))
    (lvar :bar)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_2() {
	let code = r##"def f a, o=1, *r, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (arg :a)
    (optarg :o
      (int 1))
    (restarg :r)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_3() {
	let code = r##"def f a, o=1, *r, p, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (arg :a)
    (optarg :o
      (int 1))
    (restarg :r)
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_4() {
	let code = r##"def f a, o=1, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (arg :a)
    (optarg :o
      (int 1))
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_5() {
	let code = r##"def f a, o=1, p, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (arg :a)
    (optarg :o
      (int 1))
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_6() {
	let code = r##"def f a, *r, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (arg :a)
    (restarg :r)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_7() {
	let code = r##"def f a, *r, p, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (arg :a)
    (restarg :r)
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_8() {
	let code = r##"def f a, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (arg :a)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_9() {
	let code = r##"def f o=1, *r, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (optarg :o
      (int 1))
    (restarg :r)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_10() {
	let code = r##"def f o=1, *r, p, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (optarg :o
      (int 1))
    (restarg :r)
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_11() {
	let code = r##"def f o=1, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (optarg :o
      (int 1))
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_12() {
	let code = r##"def f o=1, p, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (optarg :o
      (int 1))
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_13() {
	let code = r##"def f *r, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (restarg :r)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_14() {
	let code = r##"def f *r, p, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (restarg :r)
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_15() {
	let code = r##"def f &b; end"##;
	let sexp = r##"
(def :f
  (args
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_16() {
	let code = r##"def f ; end"##;
	let sexp = r##"
(def :f
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_or_asgn() {
	let code = r##"foo.a ||= 1"##;
	let sexp = r##"
(or-asgn
  (send
    (lvar :foo) :a)
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_or_asgn_1() {
	let code = r##"foo[0, 1] ||= 2"##;
	let sexp = r##"
(or-asgn
  (send
    (lvar :foo) :[]
    (int 0)
    (int 1))
  (int 2))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_alias() {
	let code = r##"alias :foo bar"##;
	let sexp = r##"
(alias
  (sym :foo)
  (sym :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_do_block_in_call_args() {
	let code = r##"bar def foo; self.each do end end"##;
	let sexp = r##"
(send nil :bar
  (def :foo
    (args)
    (block
      (send
        (self) :each)
      (args) nil)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11873_b() {
	let code = r##"p p{p(p);p p}, tap do end"##;
	let sexp = r##"
(block
  (send nil :p
    (block
      (send nil :p)
      (args)
      (begin
        (send nil :p
          (send nil :p))
        (send nil :p
          (send nil :p))))
    (send nil :tap))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc() {
	let code = r##"p <<~E
E"##;
	let sexp = r##"
(send nil :p
  (dstr))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_1() {
	let code = r##"p <<~E
  E"##;
	let sexp = r##"
(send nil :p
  (dstr))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_2() {
	let code = r##"p <<~E
  x
E"##;
	let sexp = r##"
(send nil :p
  (str "x\n"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_3() {
	let code = r##"p <<~E
  x
    y
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "x\n")
    (str "  y\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_4() {
	let code = r##"p <<~E
	x
    y
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "x\n")
    (str "y\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_5() {
	let code = r##"p <<~E
	x
        y
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "x\n")
    (str "y\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_6() {
	let code = r##"p <<~E
	x
        y
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "x\n")
    (str "y\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_7() {
	let code = r##"p <<~E
		x
	y
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "\tx\n")
    (str "y\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_8() {
	let code = r##"p <<~E
  x

y
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "  x\n")
    (str "\n")
    (str "y\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_9() {
	let code = r##"p <<~E
  x

  y
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "x\n")
    (str "  \n")
    (str "y\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_10() {
	let code = r##"p <<~E
    x
  \  y
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "  x\n")
    (str "  y\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_11() {
	let code = r##"p <<~E
    x
  \	y
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "  x\n")
    (str "\ty\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_12() {
	let code = r##"p <<~"E"
    x
  #{foo}
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "  x\n")
    (str "")
    (begin
      (lvar :foo))
    (str "\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_13() {
	let code = r##"p <<~`E`
    x
  #{foo}
E"##;
	let sexp = r##"
(send nil :p
  (xstr
    (str "  x\n")
    (str "")
    (begin
      (lvar :foo))
    (str "\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_dedenting_heredoc_14() {
	let code = r##"p <<~"E"
    x
  #{"  y"}
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "  x\n")
    (str "")
    (begin
      (str "  y"))
    (str "\n")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_call() {
	let code = r##"foo.(1)"##;
	let sexp = r##"
(send
  (lvar :foo) :call
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_call_1() {
	let code = r##"foo::(1)"##;
	let sexp = r##"
(send
  (lvar :foo) :call
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_unless() {
	let code = r##"unless foo then bar; end"##;
	let sexp = r##"
(if
  (lvar :foo) nil
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_unless_1() {
	let code = r##"unless foo; bar; end"##;
	let sexp = r##"
(if
  (lvar :foo) nil
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_lambda_legacy() {
	let code = r##"->{ }"##;
	let sexp = r##"
(block
  (send nil :lambda)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_casgn_unscoped() {
	let code = r##"Foo = 10"##;
	let sexp = r##"
(casgn nil :Foo
  (int 10))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_17() {
	let code = r##"def f (((a))); end"##;
	let sexp = r##"
(def :f
  (args
    (mlhs
      (mlhs
        (arg :a)))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_18() {
	let code = r##"def f ((a, a1)); end"##;
	let sexp = r##"
(def :f
  (args
    (mlhs
      (arg :a)
      (arg :a1))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_19() {
	let code = r##"def f ((a, *r)); end"##;
	let sexp = r##"
(def :f
  (args
    (mlhs
      (arg :a)
      (restarg :r))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_20() {
	let code = r##"def f ((a, *r, p)); end"##;
	let sexp = r##"
(def :f
  (args
    (mlhs
      (arg :a)
      (restarg :r)
      (arg :p))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_21() {
	let code = r##"def f ((a, *)); end"##;
	let sexp = r##"
(def :f
  (args
    (mlhs
      (arg :a)
      (restarg))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_22() {
	let code = r##"def f ((a, *, p)); end"##;
	let sexp = r##"
(def :f
  (args
    (mlhs
      (arg :a)
      (restarg)
      (arg :p))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_23() {
	let code = r##"def f ((*r)); end"##;
	let sexp = r##"
(def :f
  (args
    (mlhs
      (restarg :r))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_24() {
	let code = r##"def f ((*r, p)); end"##;
	let sexp = r##"
(def :f
  (args
    (mlhs
      (restarg :r)
      (arg :p))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_25() {
	let code = r##"def f ((*)); end"##;
	let sexp = r##"
(def :f
  (args
    (mlhs
      (restarg))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_26() {
	let code = r##"def f ((*, p)); end"##;
	let sexp = r##"
(def :f
  (args
    (mlhs
      (restarg)
      (arg :p))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_break() {
	let code = r##"break(foo)"##;
	let sexp = r##"
(break
  (begin
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_break_1() {
	let code = r##"break foo"##;
	let sexp = r##"
(break
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_break_2() {
	let code = r##"break()"##;
	let sexp = r##"
(break
  (begin))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_break_3() {
	let code = r##"break"##;
	let sexp = r##"
(break)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_var_op_asgn() {
	let code = r##"a += 1"##;
	let sexp = r##"
(op-asgn
  (lvasgn :a) :+
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_var_op_asgn_1() {
	let code = r##"@a |= 1"##;
	let sexp = r##"
(op-asgn
  (ivasgn :@a) :|
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_var_op_asgn_2() {
	let code = r##"@@var |= 10"##;
	let sexp = r##"
(op-asgn
  (cvasgn :@@var) :|
  (int 10))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_var_op_asgn_3() {
	let code = r##"def a; @@var |= 10; end"##;
	let sexp = r##"
(def :a
  (args)
  (op-asgn
    (cvasgn :@@var) :|
    (int 10)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_arg_duplicate_ignored() {
	let code = r##"def foo(_, _); end"##;
	let sexp = r##"
(def :foo
  (args
    (arg :_)
    (arg :_)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_arg_duplicate_ignored_1() {
	let code = r##"def foo(_a, _a); end"##;
	let sexp = r##"
(def :foo
  (args
    (arg :_a)
    (arg :_a)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_cond_match_current_line() {
	let code = r##"if /wat/; end"##;
	let sexp = r##"
(if
  (match-current-line
    (regexp
      (str "wat")
      (regopt))) nil nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_plain_cmd() {
	let code = r##"foo.fun bar"##;
	let sexp = r##"
(send
  (lvar :foo) :fun
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_plain_cmd_1() {
	let code = r##"foo::fun bar"##;
	let sexp = r##"
(send
  (lvar :foo) :fun
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_plain_cmd_2() {
	let code = r##"foo::Fun bar"##;
	let sexp = r##"
(send
  (lvar :foo) :Fun
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_int_line_() {
	let code = r##"__LINE__"##;
	let sexp = r##"
(int 1)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rescue_mod() {
	let code = r##"meth rescue bar"##;
	let sexp = r##"
(rescue
  (send nil :meth)
  (resbody nil nil
    (lvar :bar)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_cond_begin_masgn() {
	let code = r##"if (bar; a, b = foo); end"##;
	let sexp = r##"
(if
  (begin
    (lvar :bar)
    (masgn
      (mlhs
        (lvasgn :a)
        (lvasgn :b))
      (lvar :foo))) nil nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_retry() {
	let code = r##"retry"##;
	let sexp = r##"
(retry)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_optarg() {
	let code = r##"def f foo = 1; end"##;
	let sexp = r##"
(def :f
  (args
    (optarg :foo
      (int 1))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_optarg_1() {
	let code = r##"def f(foo=1, bar=2); end"##;
	let sexp = r##"
(def :f
  (args
    (optarg :foo
      (int 1))
    (optarg :bar
      (int 2))) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rescue_mod_op_assign() {
	let code = r##"foo += meth rescue bar"##;
	let sexp = r##"
(op-asgn
  (lvasgn :foo) :+
  (rescue
    (send nil :meth)
    (resbody nil nil
      (lvar :bar)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12669() {
	let code = r##"a = b = raise :x"##;
	let sexp = r##"
(lvasgn :a
  (lvasgn :b
    (send nil :raise
      (sym :x))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12669_1() {
	let code = r##"a += b = raise :x"##;
	let sexp = r##"
(op-asgn
  (lvasgn :a) :+
  (lvasgn :b
    (send nil :raise
      (sym :x))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12669_2() {
	let code = r##"a = b += raise :x"##;
	let sexp = r##"
(lvasgn :a
  (op-asgn
    (lvasgn :b) :+
    (send nil :raise
      (sym :x))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12669_3() {
	let code = r##"a += b += raise :x"##;
	let sexp = r##"
(op-asgn
  (lvasgn :a) :+
  (op-asgn
    (lvasgn :b) :+
    (send nil :raise
      (sym :x))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_var_and_asgn() {
	let code = r##"a &&= 1"##;
	let sexp = r##"
(and-asgn
  (lvasgn :a)
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_case_expr() {
	let code = r##"case foo; when 'bar'; bar; end"##;
	let sexp = r##"
(case
  (lvar :foo)
  (when
    (str "bar")
    (lvar :bar)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_case_cond_else() {
	let code = r##"case; when foo; 'foo'; else 'bar'; end"##;
	let sexp = r##"
(case nil
  (when
    (lvar :foo)
    (str "foo"))
  (str "bar"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_3() {
	let code = r##"f{ |a| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_lvasgn() {
	let code = r##"var = 10; var"##;
	let sexp = r##"
(begin
  (lvasgn :var
    (int 10))
  (lvar :var))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_cmd() {
	let code = r##"foo, bar = m foo"##;
	let sexp = r##"
(masgn
  (mlhs
    (lvasgn :foo)
    (lvasgn :bar))
  (send nil :m
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_op_asgn_conditional() {
	let code = r##"a&.b &&= 1"##;
	let sexp = r##"
(and-asgn
  (csend
    (send nil :a) :b)
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_class() {
	let code = r##"class Foo; end"##;
	let sexp = r##"
(class
  (const nil :Foo) nil nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_class_1() {
	let code = r##"class Foo end"##;
	let sexp = r##"
(class
  (const nil :Foo) nil nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_when_then() {
	let code = r##"case foo; when 'bar' then bar; end"##;
	let sexp = r##"
(case
  (lvar :foo)
  (when
    (str "bar")
    (lvar :bar)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_rescue_empty_else() {
	let code = r##"begin; rescue LoadError; else; end"##;
	let sexp = r##"
(kwbegin
  (rescue nil
    (resbody
      (array
        (const nil :LoadError)) nil nil) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_4() {
	let code = r##"f{  }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_5() {
	let code = r##"f{ | | }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_6() {
	let code = r##"f{ |;a| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (shadowarg :a)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_7() {
	let code = r##"f{ |;
a
| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (shadowarg :a)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_8() {
	let code = r##"f{ || }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_9() {
	let code = r##"f{ |a| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (procarg0 :a)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_10() {
	let code = r##"f{ |a, c| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (arg :c)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_11() {
	let code = r##"f{ |a,| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_12() {
	let code = r##"f{ |a, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_13() {
	let code = r##"f{ |a, *s, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (restarg :s)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_14() {
	let code = r##"f{ |a, *, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (restarg)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_15() {
	let code = r##"f{ |a, *s| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (restarg :s)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_16() {
	let code = r##"f{ |a, *| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (restarg)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_17() {
	let code = r##"f{ |*s, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (restarg :s)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_18() {
	let code = r##"f{ |*, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (restarg)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_19() {
	let code = r##"f{ |*s| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (restarg :s)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_20() {
	let code = r##"f{ |*| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (restarg)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_21() {
	let code = r##"f{ |&b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_22() {
	let code = r##"f{ |a, o=1, o1=2, *r, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (optarg :o
      (int 1))
    (optarg :o1
      (int 2))
    (restarg :r)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_23() {
	let code = r##"f{ |a, o=1, *r, p, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (optarg :o
      (int 1))
    (restarg :r)
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_24() {
	let code = r##"f{ |a, o=1, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (optarg :o
      (int 1))
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_25() {
	let code = r##"f{ |a, o=1, p, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (optarg :o
      (int 1))
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_26() {
	let code = r##"f{ |a, *r, p, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (restarg :r)
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_27() {
	let code = r##"f{ |o=1, *r, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (optarg :o
      (int 1))
    (restarg :r)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_28() {
	let code = r##"f{ |o=1, *r, p, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (optarg :o
      (int 1))
    (restarg :r)
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_29() {
	let code = r##"f{ |o=1, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (optarg :o
      (int 1))
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_30() {
	let code = r##"f{ |o=1, p, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (optarg :o
      (int 1))
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_31() {
	let code = r##"f{ |*r, p, &b| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (restarg :r)
    (arg :p)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_splat() {
	let code = r##"[1, *foo, 2]"##;
	let sexp = r##"
(array
  (int 1)
  (splat
    (lvar :foo))
  (int 2))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_splat_1() {
	let code = r##"[1, *foo]"##;
	let sexp = r##"
(array
  (int 1)
  (splat
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_splat_2() {
	let code = r##"[*foo]"##;
	let sexp = r##"
(array
  (splat
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_resbody_list_mrhs() {
	let code = r##"begin; meth; rescue Exception, foo; bar; end"##;
	let sexp = r##"
(kwbegin
  (rescue
    (send nil :meth)
    (resbody
      (array
        (const nil :Exception)
        (lvar :foo)) nil
      (lvar :bar)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_break_block() {
	let code = r##"break fun foo do end"##;
	let sexp = r##"
(break
  (block
    (send nil :fun
      (lvar :foo))
    (args) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bang_cmd() {
	let code = r##"!m foo"##;
	let sexp = r##"
(send
  (send nil :m
    (lvar :foo)) :!)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_gvar() {
	let code = r##"$foo"##;
	let sexp = r##"
(gvar :$foo)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_cond_iflipflop() {
	let code = r##"if foo..bar; end"##;
	let sexp = r##"
(if
  (iflipflop
    (lvar :foo)
    (lvar :bar)) nil nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_symbol_plain() {
	let code = r##":foo"##;
	let sexp = r##"
(sym :foo)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_symbol_plain_1() {
	let code = r##":'foo'"##;
	let sexp = r##"
(sym :foo)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11107() {
	let code = r##"p ->() do a() do end end"##;
	let sexp = r##"
(send nil :p
  (block
    (lambda)
    (args)
    (block
      (send nil :a)
      (args) nil)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_conditional() {
	let code = r##"a&.b"##;
	let sexp = r##"
(csend
  (send nil :a) :b)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_arg() {
	let code = r##"def f(foo); end"##;
	let sexp = r##"
(def :f
  (args
    (arg :foo)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_arg_1() {
	let code = r##"def f(foo, bar); end"##;
	let sexp = r##"
(def :f
  (args
    (arg :foo)
    (arg :bar)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_space_args_arg() {
	let code = r##"fun (1)"##;
	let sexp = r##"
(send nil :fun
  (begin
    (int 1)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_27() {
	let code = r##"def f (foo: 1, bar: 2, **baz, &b); end"##;
	let sexp = r##"
(def :f
  (args
    (kwoptarg :foo
      (int 1))
    (kwoptarg :bar
      (int 2))
    (kwrestarg :baz)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_28() {
	let code = r##"def f (foo: 1, &b); end"##;
	let sexp = r##"
(def :f
  (args
    (kwoptarg :foo
      (int 1))
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_29() {
	let code = r##"def f **baz, &b; end"##;
	let sexp = r##"
(def :f
  (args
    (kwrestarg :baz)
    (blockarg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_args_30() {
	let code = r##"def f *, **; end"##;
	let sexp = r##"
(def :f
  (args
    (restarg)
    (kwrestarg)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_space_args_block() {
	let code = r##"fun () {}"##;
	let sexp = r##"
(block
  (send nil :fun
    (begin))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_block_pass() {
	let code = r##"fun(&bar)"##;
	let sexp = r##"
(send nil :fun
  (block-pass
    (lvar :bar)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_blockarg() {
	let code = r##"def f(&block); end"##;
	let sexp = r##"
(def :f
  (args
    (blockarg :block)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_regex_verification() {
	let code = r##"/#)/x"##;
	let sexp = r##"
(regexp
  (str "#)")
  (regopt :x))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_restarg_named() {
	let code = r##"def f(*foo); end"##;
	let sexp = r##"
(def :f
  (args
    (restarg :foo)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_const_unscoped() {
	let code = r##"Foo"##;
	let sexp = r##"
(const nil :Foo)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_index() {
	let code = r##"foo[1, 2]"##;
	let sexp = r##"
(send
  (lvar :foo) :[]
  (int 1)
  (int 2))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_if_elsif() {
	let code = r##"if foo; bar; elsif baz; 1; else 2; end"##;
	let sexp = r##"
(if
  (lvar :foo)
  (lvar :bar)
  (if
    (lvar :baz)
    (int 1)
    (int 2)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_while_mod() {
	let code = r##"meth while foo"##;
	let sexp = r##"
(while
  (lvar :foo)
  (send nil :meth))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_args_comma() {
	let code = r##"foo[bar,]"##;
	let sexp = r##"
(send
  (lvar :foo) :[]
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ensure_empty() {
	let code = r##"begin ensure end"##;
	let sexp = r##"
(kwbegin
  (ensure nil nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_const_scoped() {
	let code = r##"Bar::Foo"##;
	let sexp = r##"
(const
  (const nil :Bar) :Foo)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_heredoc() {
	let code = r##"<<HERE
foo
bar
HERE"##;
	let sexp = r##"
(dstr
  (str "foo\n")
  (str "bar\n"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_heredoc_1() {
	let code = r##"<<'HERE'
foo
bar
HERE"##;
	let sexp = r##"
(dstr
  (str "foo\n")
  (str "bar\n"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_heredoc_2() {
	let code = r##"<<`HERE`
foo
bar
HERE"##;
	let sexp = r##"
(xstr
  (str "foo\n")
  (str "bar\n"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_if_mod() {
	let code = r##"bar if foo"##;
	let sexp = r##"
(if
  (lvar :foo)
  (lvar :bar) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_const() {
	let code = r##"self::A, foo = foo"##;
	let sexp = r##"
(masgn
  (mlhs
    (casgn
      (self) :A)
    (lvasgn :foo))
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_const_1() {
	let code = r##"::A, foo = foo"##;
	let sexp = r##"
(masgn
  (mlhs
    (casgn
      (cbase) :A)
    (lvasgn :foo))
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_32() {
	let code = r##"f{ |a, b,| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (arg :a)
    (arg :b)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bang() {
	let code = r##"!foo"##;
	let sexp = r##"
(send
  (lvar :foo) :!)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_attr() {
	let code = r##"self.a, self[1, 2] = foo"##;
	let sexp = r##"
(masgn
  (mlhs
    (send
      (self) :a=)
    (send
      (self) :[]=
      (int 1)
      (int 2)))
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_attr_1() {
	let code = r##"self::a, foo = foo"##;
	let sexp = r##"
(masgn
  (mlhs
    (send
      (self) :a=)
    (lvasgn :foo))
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_attr_2() {
	let code = r##"self.A, foo = foo"##;
	let sexp = r##"
(masgn
  (mlhs
    (send
      (self) :A=)
    (lvasgn :foo))
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_return() {
	let code = r##"return(foo)"##;
	let sexp = r##"
(return
  (begin
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_return_1() {
	let code = r##"return foo"##;
	let sexp = r##"
(return
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_return_2() {
	let code = r##"return()"##;
	let sexp = r##"
(return
  (begin))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_return_3() {
	let code = r##"return"##;
	let sexp = r##"
(return)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11990() {
	let code = r##"p <<~E "  y"
  x
E"##;
	let sexp = r##"
(send nil :p
  (dstr
    (str "x\n")
    (str "  y")))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_attr_asgn() {
	let code = r##"foo.a = 1"##;
	let sexp = r##"
(send
  (lvar :foo) :a=
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_attr_asgn_1() {
	let code = r##"foo::a = 1"##;
	let sexp = r##"
(send
  (lvar :foo) :a=
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_attr_asgn_2() {
	let code = r##"foo.A = 1"##;
	let sexp = r##"
(send
  (lvar :foo) :A=
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_attr_asgn_3() {
	let code = r##"foo::A = 1"##;
	let sexp = r##"
(casgn
  (lvar :foo) :A
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_lambda_args_shadow() {
	let code = r##"->(a; foo, bar) { }"##;
	let sexp = r##"
(block
  (lambda)
  (args
    (arg :a)
    (shadowarg :foo)
    (shadowarg :bar)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_rescue() {
	let code = r##"begin; meth; rescue; foo; end"##;
	let sexp = r##"
(kwbegin
  (rescue
    (send nil :meth)
    (resbody nil nil
      (lvar :foo)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_module() {
	let code = r##"module Foo; end"##;
	let sexp = r##"
(module
  (const nil :Foo) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_hash_label() {
	let code = r##"{ foo: 2 }"##;
	let sexp = r##"
(hash
  (pair
    (sym :foo)
    (int 2)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_plain() {
	let code = r##"[1, 2]"##;
	let sexp = r##"
(array
  (int 1)
  (int 2))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402() {
	let code = r##"foo = raise(bar) rescue nil"##;
	let sexp = r##"
(lvasgn :foo
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_1() {
	let code = r##"foo += raise(bar) rescue nil"##;
	let sexp = r##"
(op-asgn
  (lvasgn :foo) :+
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_2() {
	let code = r##"foo[0] += raise(bar) rescue nil"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :[]
    (int 0)) :+
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_3() {
	let code = r##"foo.m += raise(bar) rescue nil"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :m) :+
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_4() {
	let code = r##"foo::m += raise(bar) rescue nil"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :m) :+
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_5() {
	let code = r##"foo.C += raise(bar) rescue nil"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :C) :+
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_6() {
	let code = r##"foo::C ||= raise(bar) rescue nil"##;
	let sexp = r##"
(or-asgn
  (casgn
    (lvar :foo) :C)
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_7() {
	let code = r##"foo = raise bar rescue nil"##;
	let sexp = r##"
(lvasgn :foo
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_8() {
	let code = r##"foo += raise bar rescue nil"##;
	let sexp = r##"
(op-asgn
  (lvasgn :foo) :+
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_9() {
	let code = r##"foo[0] += raise bar rescue nil"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :[]
    (int 0)) :+
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_10() {
	let code = r##"foo.m += raise bar rescue nil"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :m) :+
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_11() {
	let code = r##"foo::m += raise bar rescue nil"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :m) :+
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_12() {
	let code = r##"foo.C += raise bar rescue nil"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :C) :+
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12402_13() {
	let code = r##"foo::C ||= raise bar rescue nil"##;
	let sexp = r##"
(or-asgn
  (casgn
    (lvar :foo) :C)
  (rescue
    (send nil :raise
      (lvar :bar))
    (resbody nil nil
      (nil)) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_def_no_paren_eql_begin() {
	let code = r##"def foo
=begin
=end
end"##;
	let sexp = r##"
(def :foo
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ivasgn() {
	let code = r##"@var = 10"##;
	let sexp = r##"
(ivasgn :@var
  (int 10))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_var_or_asgn() {
	let code = r##"a ||= 1"##;
	let sexp = r##"
(or-asgn
  (lvasgn :a)
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_op_asgn_index() {
	let code = r##"foo[0, 1] += 2"##;
	let sexp = r##"
(op-asgn
  (send
    (lvar :foo) :[]
    (int 0)
    (int 1)) :+
  (int 2))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_arg_label() {
	let code = r##"def foo() a:b end"##;
	let sexp = r##"
(def :foo
  (args)
  (send nil :a
    (sym :b)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_arg_label_1() {
	let code = r##"def foo
 a:b end"##;
	let sexp = r##"
(def :foo
  (args)
  (send nil :a
    (sym :b)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_arg_label_2() {
	let code = r##"f { || a:b }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args)
  (send nil :a
    (sym :b)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_var_op_asgn_cmd() {
	let code = r##"foo += m foo"##;
	let sexp = r##"
(op-asgn
  (lvasgn :foo) :+
  (send nil :m
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_or() {
	let code = r##"foo or bar"##;
	let sexp = r##"
(or
  (lvar :foo)
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_or_1() {
	let code = r##"foo || bar"##;
	let sexp = r##"
(or
  (lvar :foo)
  (lvar :bar))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_if_nl_then() {
	let code = r##"if foo
then bar end"##;
	let sexp = r##"
(if
  (lvar :foo)
  (lvar :bar) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_string_file_() {
	let code = r##"__FILE__"##;
	let sexp = r##"
(str "(assert_parses)")
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_kwbegin_compstmt() {
	let code = r##"begin foo!; bar! end"##;
	let sexp = r##"
(kwbegin
  (send nil :foo!)
  (send nil :bar!))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_11380() {
	let code = r##"p -> { :hello }, a: 1 do end"##;
	let sexp = r##"
(block
  (send nil :p
    (block
      (lambda)
      (args)
      (sym :hello))
    (hash
      (pair
        (sym :a)
        (int 1))))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_hash_label_end() {
	let code = r##"{ 'foo': 2 }"##;
	let sexp = r##"
(hash
  (pair
    (sym :foo)
    (int 2)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_hash_label_end_1() {
	let code = r##"{ 'foo': 2, 'bar': {}}"##;
	let sexp = r##"
(hash
  (pair
    (sym :foo)
    (int 2))
  (pair
    (sym :bar)
    (hash)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_hash_label_end_2() {
	let code = r##"f(a ? "a":1)"##;
	let sexp = r##"
(send nil :f
  (if
    (send nil :a)
    (str "a")
    (int 1)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_string_concat() {
	let code = r##""foo#@a" "bar""##;
	let sexp = r##"
(dstr
  (dstr
    (str "foo")
    (ivar :@a))
  (str "bar"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_hash_empty() {
	let code = r##"{ }"##;
	let sexp = r##"
(hash)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_int() {
	let code = r##"42"##;
	let sexp = r##"
(int 42)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_int_1() {
	let code = r##"-42"##;
	let sexp = r##"
(int -42)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_begin_cmdarg() {
	let code = r##"p begin 1.times do 1 end end"##;
	let sexp = r##"
(send nil :p
  (kwbegin
    (block
      (send
        (int 1) :times)
      (args)
      (int 1))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_lvar() {
	let code = r##"foo"##;
	let sexp = r##"
(lvar :foo)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_super_block() {
	let code = r##"super foo, bar do end"##;
	let sexp = r##"
(block
  (super
    (lvar :foo)
    (lvar :bar))
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_super_block_1() {
	let code = r##"super do end"##;
	let sexp = r##"
(block
  (zsuper)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_cmd() {
	let code = r##"fun(f bar)"##;
	let sexp = r##"
(send nil :fun
  (send nil :f
    (lvar :bar)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_regex_plain() {
	let code = r##"/source/im"##;
	let sexp = r##"
(regexp
  (str "source")
  (regopt :i :m))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_string_dvar() {
	let code = r##""#@a #@@a #$a""##;
	let sexp = r##"
(dstr
  (ivar :@a)
  (str " ")
  (cvar :@@a)
  (str " ")
  (gvar :$a))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_and_asgn() {
	let code = r##"foo.a &&= 1"##;
	let sexp = r##"
(and-asgn
  (send
    (lvar :foo) :a)
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_and_asgn_1() {
	let code = r##"foo[0, 1] &&= 2"##;
	let sexp = r##"
(and-asgn
  (send
    (lvar :foo) :[]
    (int 0)
    (int 1))
  (int 2))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_unary_op() {
	let code = r##"-foo"##;
	let sexp = r##"
(send
  (lvar :foo) :-@)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_unary_op_1() {
	let code = r##"+foo"##;
	let sexp = r##"
(send
  (lvar :foo) :+@)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_unary_op_2() {
	let code = r##"~foo"##;
	let sexp = r##"
(send
  (lvar :foo) :~)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_args_assocs_comma() {
	let code = r##"foo[:baz => 1,]"##;
	let sexp = r##"
(send
  (lvar :foo) :[]
  (hash
    (pair
      (sym :baz)
      (int 1))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_pow_precedence() {
	let code = r##"-2 ** 10"##;
	let sexp = r##"
(send
  (send
    (int 2) :**
    (int 10)) :-@)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_pow_precedence_1() {
	let code = r##"-2.0 ** 10"##;
	let sexp = r##"
(send
  (send
    (float 2.0) :**
    (int 10)) :-@)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn assert_parses_blockargs_33() {
	let code = r##"f{ |foo:| }"##;
	let sexp = r##"
(block
  (send nil :f)
  (args
    (kwarg :foo)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op() {
	let code = r##"foo + 1"##;
	let sexp = r##"
(send
  (lvar :foo) :+
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_1() {
	let code = r##"foo - 1"##;
	let sexp = r##"
(send
  (lvar :foo) :-
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_2() {
	let code = r##"foo * 1"##;
	let sexp = r##"
(send
  (lvar :foo) :*
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_3() {
	let code = r##"foo / 1"##;
	let sexp = r##"
(send
  (lvar :foo) :/
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_4() {
	let code = r##"foo % 1"##;
	let sexp = r##"
(send
  (lvar :foo) :%
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_5() {
	let code = r##"foo ** 1"##;
	let sexp = r##"
(send
  (lvar :foo) :**
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_6() {
	let code = r##"foo | 1"##;
	let sexp = r##"
(send
  (lvar :foo) :|
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_7() {
	let code = r##"foo ^ 1"##;
	let sexp = r##"
(send
  (lvar :foo) :^
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_8() {
	let code = r##"foo & 1"##;
	let sexp = r##"
(send
  (lvar :foo) :&
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_9() {
	let code = r##"foo <=> 1"##;
	let sexp = r##"
(send
  (lvar :foo) :<=>
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_10() {
	let code = r##"foo < 1"##;
	let sexp = r##"
(send
  (lvar :foo) :<
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_11() {
	let code = r##"foo <= 1"##;
	let sexp = r##"
(send
  (lvar :foo) :<=
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_12() {
	let code = r##"foo > 1"##;
	let sexp = r##"
(send
  (lvar :foo) :>
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_13() {
	let code = r##"foo >= 1"##;
	let sexp = r##"
(send
  (lvar :foo) :>=
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_14() {
	let code = r##"foo == 1"##;
	let sexp = r##"
(send
  (lvar :foo) :==
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_15() {
	let code = r##"foo != 1"##;
	let sexp = r##"
(send
  (lvar :foo) :!=
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_16() {
	let code = r##"foo === 1"##;
	let sexp = r##"
(send
  (lvar :foo) :===
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_17() {
	let code = r##"foo =~ 1"##;
	let sexp = r##"
(send
  (lvar :foo) :=~
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_18() {
	let code = r##"foo !~ 1"##;
	let sexp = r##"
(send
  (lvar :foo) :!~
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_19() {
	let code = r##"foo << 1"##;
	let sexp = r##"
(send
  (lvar :foo) :<<
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_binary_op_20() {
	let code = r##"foo >> 1"##;
	let sexp = r##"
(send
  (lvar :foo) :>>
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_send_block_conditional() {
	let code = r##"foo&.bar {}"##;
	let sexp = r##"
(block
  (csend
    (lvar :foo) :bar)
  (args) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_arg_scope() {
	let code = r##"def f(var = defined?(var)) var end"##;
	let sexp = r##"
(def :f
  (args
    (optarg :var
      (defined?
        (lvar :var))))
  (lvar :var))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_arg_scope_1() {
	let code = r##"def f(var: defined?(var)) var end"##;
	let sexp = r##"
(def :f
  (args
    (kwoptarg :var
      (defined?
        (lvar :var))))
  (lvar :var))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_arg_scope_2() {
	let code = r##"lambda{|;a|a}"##;
	let sexp = r##"
(block
  (send nil :lambda)
  (args
    (shadowarg :a))
  (lvar :a))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_const_toplevel() {
	let code = r##"::Foo"##;
	let sexp = r##"
(const
  (cbase) :Foo)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_if_masgn_24() {
	let code = r##"if (a, b = foo); end"##;
	let sexp = r##"
(if
  (begin nil) nil nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn() {
	let code = r##"foo, bar = 1, 2"##;
	let sexp = r##"
(masgn
  (mlhs
    (lvasgn :foo)
    (lvasgn :bar))
  (array
    (int 1)
    (int 2)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_1() {
	let code = r##"(foo, bar) = 1, 2"##;
	let sexp = r##"
(masgn
  (mlhs
    (lvasgn :foo)
    (lvasgn :bar))
  (array
    (int 1)
    (int 2)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_masgn_2() {
	let code = r##"foo, bar, baz = 1, 2"##;
	let sexp = r##"
(masgn
  (mlhs
    (lvasgn :foo)
    (lvasgn :bar)
    (lvasgn :baz))
  (array
    (int 1)
    (int 2)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_array_words() {
	let code = r##"%w[foo bar]"##;
	let sexp = r##"
(array
  (str "foo")
  (str "bar"))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_bug_do_block_in_cmdarg() {
	let code = r##"tap (proc do end)"##;
	let sexp = r##"
(send nil :tap
  (begin
    (block
      (send nil :proc)
      (args) nil)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_return_block() {
	let code = r##"return fun foo do end"##;
	let sexp = r##"
(return
  (block
    (send nil :fun
      (lvar :foo))
    (args) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_12686() {
	let code = r##"f (g rescue nil)"##;
	let sexp = r##"
(send nil :f
  (begin
    (rescue
      (send nil :g)
      (resbody nil nil
        (nil)) nil)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ivar() {
	let code = r##"@foo"##;
	let sexp = r##"
(ivar :@foo)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_next_block() {
	let code = r##"next fun foo do end"##;
	let sexp = r##"
(next
  (block
    (send nil :fun
      (lvar :foo))
    (args) nil))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_parser_bug_272() {
	let code = r##"a @b do |c|;end"##;
	let sexp = r##"
(block
  (send nil :a
    (ivar :@b))
  (args
    (procarg0 :c)) nil)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_next() {
	let code = r##"next(foo)"##;
	let sexp = r##"
(next
  (begin
    (lvar :foo)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_next_1() {
	let code = r##"next foo"##;
	let sexp = r##"
(next
  (lvar :foo))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_next_2() {
	let code = r##"next()"##;
	let sexp = r##"
(next
  (begin))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_next_3() {
	let code = r##"next"##;
	let sexp = r##"
(next)
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_9669() {
	let code = r##"def a b:
return
end"##;
	let sexp = r##"
(def :a
  (args
    (kwarg :b))
  (return))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ruby_bug_9669_1() {
	let code = r##"o = {
a:
1
}"##;
	let sexp = r##"
(lvasgn :o
  (hash
    (pair
      (sym :a)
      (int 1))))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_ternary_ambiguous_symbol() {
	let code = r##"t=1;(foo)?t:T"##;
	let sexp = r##"
(begin
  (lvasgn :t
    (int 1))
  (if
    (begin
      (lvar :foo))
    (lvar :t)
    (const nil :T)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_const_op_asgn() {
	let code = r##"A += 1"##;
	let sexp = r##"
(op-asgn
  (casgn nil :A) :+
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_const_op_asgn_1() {
	let code = r##"::A += 1"##;
	let sexp = r##"
(op-asgn
  (casgn
    (cbase) :A) :+
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_const_op_asgn_2() {
	let code = r##"B::A += 1"##;
	let sexp = r##"
(op-asgn
  (casgn
    (const nil :B) :A) :+
  (int 1))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_const_op_asgn_3() {
	let code = r##"def x; self::A ||= 1; end"##;
	let sexp = r##"
(def :x
  (args)
  (or-asgn
    (casgn
      (self) :A)
    (int 1)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_const_op_asgn_4() {
	let code = r##"def x; ::A ||= 1; end"##;
	let sexp = r##"
(def :x
  (args)
  (or-asgn
    (casgn
      (cbase) :A)
    (int 1)))
"##;
	assert_sexp!(code, sexp);
}

#[test]
fn test_kwrestarg_unnamed() {
	let code = r##"def f(**); end"##;
	let sexp = r##"
(def :f
  (args
    (kwrestarg)) nil)
"##;
	assert_sexp!(code, sexp);
}
