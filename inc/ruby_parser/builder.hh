#ifndef RUBY_PARSER_BUILDER_HH
#define RUBY_PARSER_BUILDER_HH

#include <vector>
#include <memory>

#include "node.hh"
#include "token.hh"

namespace ruby_parser {
  namespace builder {
    node_ptr accessible(node_ptr node);
    node_ptr alias(token_ptr alias, node_ptr to, node_ptr from);
    node_ptr arg(token_ptr name);
    node_ptr args(token_ptr begin, node_list_ptr args, token_ptr end, bool check_args = true);
    node_ptr array(token_ptr begin, node_list_ptr elements, token_ptr end);
    node_ptr assign(node_ptr lhs, token_ptr eql, node_ptr rhs);
    node_ptr assignable(node_ptr node);
    node_ptr associate(token_ptr begin, node_list_ptr pairs, token_ptr end);
    node_ptr attr_asgn(node_ptr receiver, token_ptr dot, token_ptr selector);
    node_ptr back_ref(token_ptr tok);
    node_ptr begin(token_ptr begin, node_ptr body, token_ptr end);
    node_ptr begin_body(node_ptr body, node_list_ptr rescue_bodies = nullptr, token_ptr else_tok = nullptr, node_ptr else_ = nullptr, token_ptr ensure_tok = nullptr, node_ptr ensure = nullptr);
    node_ptr begin_keyword(token_ptr begin, node_ptr body, token_ptr end);
    node_ptr binary_op(node_ptr receiver, token_ptr oper, node_ptr arg);
    node_ptr block(node_ptr method_call, token_ptr begin, node_ptr args, node_ptr body, token_ptr end);
    node_ptr block_pass(token_ptr amper, node_ptr arg);
    node_ptr blockarg(token_ptr amper, token_ptr name = nullptr);
    node_ptr call_lambda(token_ptr lambda);
    node_ptr call_method(node_ptr receiver, token_ptr dot, token_ptr selector, token_ptr lparen = nullptr, node_list_ptr args = nullptr, token_ptr rparen = nullptr);
    node_ptr case_(token_ptr case_, node_ptr expr, node_list_ptr when_bodies, token_ptr else_tok, node_ptr else_body, token_ptr end);
    node_ptr character(token_ptr char_);
    node_ptr complex(token_ptr tok);
    node_ptr compstmt(node_list_ptr node);
    node_ptr condition(token_ptr cond_tok, node_ptr cond, token_ptr then, node_ptr if_true, token_ptr else_, node_ptr if_false, token_ptr end);
    node_ptr condition_mod(node_ptr if_true, node_ptr if_false, node_ptr cond);
    node_ptr const_(token_ptr name);
    node_ptr const_fetch(node_ptr scope, token_ptr colon, node_ptr name);
    node_ptr const_fetch(node_ptr scope, token_ptr colon2, token_ptr name);
    node_ptr const_global(token_ptr colon, token_ptr name);
    node_ptr const_op_assignable(node_ptr node);
    node_ptr cvar(token_ptr tok);
    node_ptr dedent_string(node_ptr node, size_t dedent_level);
    node_ptr def_class(token_ptr class_, node_ptr name, token_ptr lt_, node_ptr superclass, node_ptr body, token_ptr end_);
    node_ptr def_method(token_ptr def, token_ptr name, node_ptr args, node_ptr body, token_ptr end);
    node_ptr def_module(token_ptr module, node_ptr name, node_ptr body, token_ptr end_);
    node_ptr def_sclass(token_ptr class_, token_ptr lshft_, node_ptr expr, node_ptr body, token_ptr end_);
    node_ptr def_singleton(token_ptr def, node_ptr definee, token_ptr dot, token_ptr name, node_ptr args, node_ptr body, token_ptr end);
    node_ptr encoding_literal(token_ptr tok);
    node_ptr false_(token_ptr tok);
    node_ptr file_literal(token_ptr tok);
    node_ptr float_(token_ptr tok);
    node_ptr float_complex(token_ptr tok);
    node_ptr for_(token_ptr for_, node_ptr iterator, token_ptr in, node_ptr iteratee, token_ptr do_, node_ptr body, token_ptr end);
    node_ptr gvar(token_ptr tok);
    node_ptr ident(token_ptr tok);
    node_ptr index(node_ptr receiver, token_ptr lbrack, node_list_ptr indexes, token_ptr rbrack);
    node_ptr index_asgn(node_ptr receiver, token_ptr lbrack, node_list_ptr indexes, token_ptr rbrack);
    node_ptr integer(token_ptr tok);
    node_ptr ivar(token_ptr tok);
    node_ptr keyword_cmd(node_type type, token_ptr keyword, token_ptr lparen = nullptr, node_list_ptr args = nullptr, token_ptr rparen = nullptr);
    node_ptr kwarg(token_ptr name);
    node_ptr kwoptarg(token_ptr name, node_ptr value);
    node_ptr kwrestarg(token_ptr dstar, token_ptr name = nullptr);
    node_ptr kwsplat(token_ptr dstar, node_ptr arg);
    node_ptr line_literal(token_ptr tok);
    node_ptr logical_op(node_type type, node_ptr lhs, token_ptr op, node_ptr rhs);
    node_ptr loop(node_type type, token_ptr keyword, node_ptr cond, token_ptr do_, node_ptr body, token_ptr end);
    node_ptr loop_mod(node_type type, node_ptr body, node_ptr cond);
    node_ptr match_op(node_ptr receiver, token_ptr oper, node_ptr arg);
    node_ptr multi_assign(node_ptr mlhs, node_ptr rhs);
    node_ptr multi_lhs(token_ptr begin, node_list_ptr items, token_ptr end);
    node_ptr negate(token_ptr uminus, node_ptr numeric);
    node_ptr nil(token_ptr tok);
    node_ptr not_op(token_ptr not_, token_ptr begin = nullptr, node_ptr receiver = nullptr, token_ptr end = nullptr);
    node_ptr nth_ref(token_ptr tok);
    node_ptr op_assign(node_ptr lhs, token_ptr op, node_ptr rhs);
    node_ptr optarg(token_ptr name, token_ptr eql, node_ptr value);
    node_ptr pair(node_ptr key, token_ptr assoc, node_ptr value);
    node_ptr pair_keyword(token_ptr key, node_ptr value);
    node_ptr pair_quoted(token_ptr begin, node_list_ptr parts, token_ptr end, node_ptr value);
    node_ptr postexe(node_ptr body);
    node_ptr preexe(node_ptr node);
    node_ptr procarg0(node_ptr arg);
    node_ptr prototype(node_ptr genargs, node_ptr args, node_ptr return_type);
    node_ptr range_exclusive(node_ptr lhs, token_ptr oper, node_ptr rhs);
    node_ptr range_inclusive(node_ptr lhs, token_ptr oper, node_ptr rhs);
    node_ptr rational(token_ptr tok);
    node_ptr rational_complex(token_ptr tok);
    node_ptr regexp_compose(token_ptr begin, node_list_ptr parts, token_ptr end, node_ptr options);
    node_ptr regexp_options(token_ptr regopt);
    node_ptr rescue_body(token_ptr rescue, node_ptr exc_list, token_ptr assoc, node_ptr exc_var, token_ptr then, node_ptr body);
    node_ptr restarg(token_ptr star, token_ptr name = nullptr);
    node_ptr self(token_ptr tok);
    node_ptr shadowarg(token_ptr name);
    node_ptr splat(token_ptr star, node_ptr arg = nullptr);
    node_ptr string(token_ptr string_);
    node_ptr string_compose(token_ptr begin, node_list_ptr parts, token_ptr end);
    node_ptr string_internal(token_ptr string_);
    node_ptr symbol(token_ptr symbol);
    node_ptr symbol_compose(token_ptr begin, node_list_ptr parts, token_ptr end);
    node_ptr symbol_internal(token_ptr symbol);
    node_ptr symbols_compose(token_ptr begin, node_list_ptr parts, token_ptr end);
    node_ptr ternary(node_ptr cond, token_ptr question, node_ptr if_true, token_ptr colon, node_ptr if_false);
    node_ptr tr_array(token_ptr begin, node_ptr type, token_ptr end);
    node_ptr tr_cast(token_ptr begin, node_ptr expr, token_ptr colon, node_ptr type, token_ptr end);
    node_ptr tr_cpath(node_ptr cpath);
    node_ptr tr_genargs(token_ptr begin, node_list_ptr genargs, token_ptr end);
    node_ptr tr_gendecl(node_ptr cpath, token_ptr begin, node_list_ptr genargs, token_ptr end);
    node_ptr tr_gendeclarg(token_ptr tok);
    node_ptr tr_geninst(node_ptr cpath, token_ptr begin, node_ptr genargs, token_ptr end);
    node_ptr tr_hash(token_ptr begin, node_ptr key_type, token_ptr assoc, node_ptr value_type, token_ptr end);
    node_ptr tr_ivardecl(token_ptr name, node_ptr type);
    node_ptr tr_nil(token_ptr nil);
    node_ptr tr_nillable(token_ptr tilde, node_ptr type);
    node_ptr tr_or(node_ptr a, node_ptr b);
    node_ptr tr_proc(token_ptr begin, node_ptr args, token_ptr end);
    node_ptr tr_special(token_ptr special);
    node_ptr tr_tuple(token_ptr begin, node_list_ptr types, token_ptr end);
    node_ptr true_(token_ptr tok);
    node_ptr typed_arg(node_ptr type, node_ptr arg);
    node_ptr unary_op(token_ptr oper, node_ptr receiver);
    node_ptr undef_method(node_list_ptr name_list);
    node_ptr when(token_ptr when, node_list_ptr patterns, token_ptr then, node_ptr body);
    node_ptr word(node_list_ptr parts);
    node_ptr words_compose(token_ptr begin, node_list_ptr parts, token_ptr end);
    node_ptr xstring_compose(token_ptr begin, node_list_ptr parts, token_ptr end);
  };
};

#endif
