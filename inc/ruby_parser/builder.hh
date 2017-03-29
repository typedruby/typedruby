#ifndef RUBY_PARSER_BUILDER_HH
#define RUBY_PARSER_BUILDER_HH

#include <vector>
#include <memory>

#include "node.hh"
#include "token.hh"

namespace ruby_parser {
  namespace builder {
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
    node_ptr call_lambda(token_ptr lambda);
    node_ptr call_method(node_ptr receiver, token_ptr dot, token_ptr selector, token_ptr lparen = nullptr, node_list_ptr args = nullptr, token_ptr rparen = nullptr);
    node_ptr case_(token_ptr case_, node_ptr expr, node_list_ptr when_bodies, token_ptr else_tok, node_ptr else_body, token_ptr end);
    node_ptr compstmt(node_list_ptr node);
    node_ptr condition(token_ptr cond_tok, node_ptr cond, token_ptr then, node_ptr if_true, token_ptr else_, node_ptr if_false, token_ptr end);
    node_ptr condition_mod(node_ptr if_true, node_ptr if_false, node_ptr cond);
    node_ptr const_(token_ptr name);
    node_ptr const_fetch(node_ptr scope, token_ptr colon, node_ptr name);
    node_ptr const_fetch(node_ptr scope, token_ptr colon2, token_ptr name);
    node_ptr const_global(token_ptr colon, token_ptr name);
    node_ptr const_op_assignable(node_ptr node);
    node_ptr def_class(token_ptr class_, node_ptr name, token_ptr lt_, node_ptr superclass, node_ptr body, token_ptr end_);
    node_ptr def_method(token_ptr def, token_ptr name, node_ptr args, node_ptr body, token_ptr end);
    node_ptr def_module(token_ptr module, node_ptr name, node_ptr body, token_ptr end_);
    node_ptr def_sclass(token_ptr class_, token_ptr lshft_, node_ptr expr, node_ptr body, token_ptr end_);
    node_ptr def_singleton(token_ptr def, node_ptr definee, token_ptr dot, token_ptr name, node_ptr args, node_ptr body, token_ptr end);
    node_ptr for_(token_ptr for_, node_ptr iterator, token_ptr in, node_ptr iteratee, token_ptr do_, node_ptr body, token_ptr end);
    node_ptr gvar(token_ptr tok);
    node_ptr index(node_ptr receiver, token_ptr lbrack, node_list_ptr indexes, token_ptr rbrack);
    node_ptr index_asgn(node_ptr receiver, token_ptr lbrack, node_list_ptr indexes, token_ptr rbrack);
    node_ptr keyword_cmd(node_type type, token_ptr keyword, token_ptr lparen = nullptr, node_list_ptr args = nullptr, token_ptr rparen = nullptr);
    node_ptr logical_op(node_type type, node_ptr lhs, token_ptr op, node_ptr rhs);
    node_ptr loop(node_type type, token_ptr keyword, node_ptr cond, token_ptr do_, node_ptr body, token_ptr end);
    node_ptr loop_mod(node_type type, node_ptr body, node_ptr cond);
    node_ptr match_op(node_ptr receiver, token_ptr oper, node_ptr arg);
    node_ptr multi_assign(node_ptr mlhs, node_ptr rhs);
    node_ptr multi_lhs(token_ptr begin, node_list_ptr items, token_ptr end);
    node_ptr not_op(token_ptr not_t, token_ptr begin_t = nullptr, node_ptr receiver = nullptr, token_ptr end_t = nullptr);
    node_ptr op_assign(node_ptr lhs, token_ptr op, node_ptr rhs);
    node_ptr postexe(node_ptr body);
    node_ptr preexe(node_ptr node);
    node_ptr procarg0(node_ptr arg);
    node_ptr prototype(node_list_ptr genargs, node_ptr args, node_ptr return_type);
    node_ptr range_exclusive(node_ptr lhs, token_ptr oper, node_ptr rhs);
    node_ptr range_inclusive(node_ptr lhs, token_ptr oper, node_ptr rhs);
    node_ptr rescue_body(token_ptr rescue, node_list_ptr exc_list, token_ptr assoc, node_ptr exc_var, token_ptr then, node_ptr body);
    node_ptr restarg(token_ptr star, token_ptr name = nullptr);
    node_ptr shadowarg(token_ptr name);
    node_ptr splat(token_ptr star, node_ptr arg = nullptr);
    node_ptr symbol(token_ptr symbol);
    node_ptr ternary(node_ptr cond, token_ptr question, node_ptr if_true, token_ptr colon, node_ptr if_false);
    node_ptr tr_cast(token_ptr begin, node_ptr expr, token_ptr colon, node_ptr type, token_ptr end);
    node_ptr tr_gendecl(node_ptr cpath, token_ptr begin, node_list_ptr genargs, token_ptr end);
    node_ptr tr_ivardecl(token_ptr name, node_ptr type);
    node_ptr unary_op(token_ptr oper, node_ptr receiver);
    node_ptr undef_method(node_list_ptr name_list);
  };
};

#endif
