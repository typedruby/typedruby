#ifndef RUBY_PARSER_BUILDER_HH
#define RUBY_PARSER_BUILDER_HH

#include <vector>
#include <memory>

#include "node.hh"
#include "token.hh"

namespace ruby_parser {
  namespace builder {
    node_ptr alias(token_ptr alias, node_ptr to, node_ptr from);
    node_ptr array(token_ptr begin, node_list_ptr elements, token_ptr end);
    node_ptr assign(node_ptr lhs, token_ptr eql, node_ptr rhs);
    node_ptr back_ref(token_ptr tok);
    node_ptr begin_body(node_ptr body, node_list_ptr rescue_bodies = nullptr, token_ptr else_tok = nullptr, node_ptr else_ = nullptr, token_ptr ensure_tok = nullptr, node_ptr ensure = nullptr);
    node_ptr compstmt(node_list_ptr node);
    node_ptr condition_mod(node_ptr if_true, node_ptr if_false, node_ptr cond);
    node_ptr gvar(token_ptr tok);
    node_ptr loop_mod(node_type type, node_ptr body, node_ptr cond);
    node_ptr multi_assign(node_ptr mlhs, node_ptr rhs);
    node_ptr postexe(node_ptr body);
    node_ptr preexe(node_ptr node);
    node_ptr rescue_body(token_ptr rescue, node_list_ptr exc_list, token_ptr assoc, node_ptr exc_var, token_ptr then, node_ptr body);
    node_ptr tr_ivardecl(token_ptr name, node_ptr type);
    node_ptr undef_method(node_list_ptr name_list);
    node_ptr op_assign(node_ptr lhs, token_ptr op, node_ptr rhs);
    node_ptr index(node_ptr receiver, token_ptr lbrack, node_list_ptr indexes, token_ptr rbrack);
    node_ptr call_method(node_ptr receiver, token_ptr dot, token_ptr selector, token_ptr lparen = nullptr, node_list_ptr args = nullptr, token_ptr rparen = nullptr);
    node_ptr const_op_assignable(node_ptr node);
    node_ptr const_fetch(node_ptr scope, token_ptr colon2, token_ptr name);
    node_ptr logical_op(node_type type, node_ptr lhs, token_ptr op, node_ptr rhs);
    node_ptr not_op(token_ptr not_t, token_ptr begin_t = nullptr, node_ptr receiver = nullptr, token_ptr end_t = nullptr);
    node_ptr block(node_ptr method_call, token_ptr begin, node_ptr args, node_ptr body, token_ptr end);
    node_ptr keyword_cmd(node_type type, token_ptr keyword, token_ptr lparen = nullptr, node_list_ptr args = nullptr, token_ptr rparen = nullptr);
    node_ptr multi_lhs(token_ptr begin, node_list_ptr items, token_ptr end);
    node_ptr begin(token_ptr begin, node_ptr body, token_ptr end);
    node_ptr splat(token_ptr star, node_ptr arg = nullptr);
  };
};

#endif
