#ifndef RUBY_PARSER_BUILDER_HH
#define RUBY_PARSER_BUILDER_HH

#include <vector>
#include <memory>

#include "node.hh"
#include "token.hh"
#include "optional_size.hh"
#include "parser.hh"

namespace ruby_parser {
  struct builder {
    foreign_ptr(*accessible)(foreign_ptr node);
    foreign_ptr(*alias)(token_ptr alias, foreign_ptr to, foreign_ptr from);
    foreign_ptr(*arg)(token_ptr name);
    foreign_ptr(*args)(token_ptr begin, node_list_ptr args, token_ptr end, bool check_args);
    foreign_ptr(*array)(token_ptr begin, node_list_ptr elements, token_ptr end);
    foreign_ptr(*assign)(foreign_ptr lhs, token_ptr eql, foreign_ptr rhs);
    foreign_ptr(*assignable)(foreign_ptr node);
    foreign_ptr(*associate)(token_ptr begin, node_list_ptr pairs, token_ptr end);
    foreign_ptr(*attr_asgn)(foreign_ptr receiver, token_ptr dot, token_ptr selector);
    foreign_ptr(*back_ref)(token_ptr tok);
    foreign_ptr(*begin)(token_ptr begin, foreign_ptr body, token_ptr end);
    foreign_ptr(*begin_body)(foreign_ptr body, node_list_ptr rescue_bodies, token_ptr else_tok, foreign_ptr else_, token_ptr ensure_tok, foreign_ptr ensure);
    foreign_ptr(*begin_keyword)(token_ptr begin, foreign_ptr body, token_ptr end);
    foreign_ptr(*binary_op)(foreign_ptr receiver, token_ptr oper, foreign_ptr arg);
    foreign_ptr(*block)(foreign_ptr method_call, token_ptr begin, foreign_ptr args, foreign_ptr body, token_ptr end);
    foreign_ptr(*block_pass)(token_ptr amper, foreign_ptr arg);
    foreign_ptr(*blockarg)(token_ptr amper, token_ptr name);
    foreign_ptr(*call_lambda)(token_ptr lambda);
    foreign_ptr(*call_method)(foreign_ptr receiver, token_ptr dot, token_ptr selector, token_ptr lparen, node_list_ptr args, token_ptr rparen);
    foreign_ptr(*case_)(token_ptr case_, foreign_ptr expr, node_list_ptr when_bodies, token_ptr else_tok, foreign_ptr else_body, token_ptr end);
    foreign_ptr(*character)(token_ptr char_);
    foreign_ptr(*complex)(token_ptr tok);
    foreign_ptr(*compstmt)(node_list_ptr node);
    foreign_ptr(*condition)(token_ptr cond_tok, foreign_ptr cond, token_ptr then, foreign_ptr if_true, token_ptr else_, foreign_ptr if_false, token_ptr end);
    foreign_ptr(*condition_mod)(foreign_ptr if_true, foreign_ptr if_false, foreign_ptr cond);
    foreign_ptr(*const_)(token_ptr name);
    foreign_ptr(*const_fetch)(foreign_ptr scope, token_ptr colon, token_ptr name);
    foreign_ptr(*const_global)(token_ptr colon, token_ptr name);
    foreign_ptr(*const_op_assignable)(foreign_ptr node);
    foreign_ptr(*cvar)(token_ptr tok);
    foreign_ptr(*dedent_string)(foreign_ptr node, optional_size dedent_level);
    foreign_ptr(*def_class)(token_ptr class_, foreign_ptr name, token_ptr lt_, foreign_ptr superclass, foreign_ptr body, token_ptr end_);
    foreign_ptr(*def_method)(token_ptr def, token_ptr name, foreign_ptr args, foreign_ptr body, token_ptr end);
    foreign_ptr(*def_module)(token_ptr module, foreign_ptr name, foreign_ptr body, token_ptr end_);
    foreign_ptr(*def_sclass)(token_ptr class_, token_ptr lshft_, foreign_ptr expr, foreign_ptr body, token_ptr end_);
    foreign_ptr(*def_singleton)(token_ptr def, foreign_ptr definee, token_ptr dot, token_ptr name, foreign_ptr args, foreign_ptr body, token_ptr end);
    foreign_ptr(*encoding_literal)(token_ptr tok);
    foreign_ptr(*false_)(token_ptr tok);
    foreign_ptr(*file_literal)(token_ptr tok);
    foreign_ptr(*float_)(token_ptr tok);
    foreign_ptr(*float_complex)(token_ptr tok);
    foreign_ptr(*for_)(token_ptr for_, foreign_ptr iterator, token_ptr in, foreign_ptr iteratee, token_ptr do_, foreign_ptr body, token_ptr end);
    foreign_ptr(*gvar)(token_ptr tok);
    foreign_ptr(*ident)(token_ptr tok);
    foreign_ptr(*index)(foreign_ptr receiver, token_ptr lbrack, node_list_ptr indexes, token_ptr rbrack);
    foreign_ptr(*index_asgn)(foreign_ptr receiver, token_ptr lbrack, node_list_ptr indexes, token_ptr rbrack);
    foreign_ptr(*integer)(token_ptr tok);
    foreign_ptr(*ivar)(token_ptr tok);
    foreign_ptr(*keyword_cmd)(node_type type, token_ptr keyword, token_ptr lparen, node_list_ptr args, token_ptr rparen);
    foreign_ptr(*kwarg)(token_ptr name);
    foreign_ptr(*kwoptarg)(token_ptr name, foreign_ptr value);
    foreign_ptr(*kwrestarg)(token_ptr dstar, token_ptr name);
    foreign_ptr(*kwsplat)(token_ptr dstar, foreign_ptr arg);
    foreign_ptr(*line_literal)(token_ptr tok);
    foreign_ptr(*logical_op)(node_type type, foreign_ptr lhs, token_ptr op, foreign_ptr rhs);
    foreign_ptr(*loop)(node_type type, token_ptr keyword, foreign_ptr cond, token_ptr do_, foreign_ptr body, token_ptr end);
    foreign_ptr(*loop_mod)(node_type type, foreign_ptr body, foreign_ptr cond);
    foreign_ptr(*match_op)(foreign_ptr receiver, token_ptr oper, foreign_ptr arg);
    foreign_ptr(*multi_assign)(foreign_ptr mlhs, foreign_ptr rhs);
    foreign_ptr(*multi_lhs)(token_ptr begin, node_list_ptr items, token_ptr end);
    foreign_ptr(*negate)(token_ptr uminus, foreign_ptr numeric);
    foreign_ptr(*nil)(token_ptr tok);
    foreign_ptr(*not_op)(token_ptr not_, token_ptr begin, foreign_ptr receiver, token_ptr end);
    foreign_ptr(*nth_ref)(token_ptr tok);
    foreign_ptr(*op_assign)(foreign_ptr lhs, token_ptr op, foreign_ptr rhs);
    foreign_ptr(*optarg)(token_ptr name, token_ptr eql, foreign_ptr value);
    foreign_ptr(*pair)(foreign_ptr key, token_ptr assoc, foreign_ptr value);
    foreign_ptr(*pair_keyword)(token_ptr key, foreign_ptr value);
    foreign_ptr(*pair_quoted)(token_ptr begin, node_list_ptr parts, token_ptr end, foreign_ptr value);
    foreign_ptr(*postexe)(foreign_ptr body);
    foreign_ptr(*preexe)(foreign_ptr node);
    foreign_ptr(*procarg0)(foreign_ptr arg);
    foreign_ptr(*prototype)(foreign_ptr genargs, foreign_ptr args, foreign_ptr return_type);
    foreign_ptr(*range_exclusive)(foreign_ptr lhs, token_ptr oper, foreign_ptr rhs);
    foreign_ptr(*range_inclusive)(foreign_ptr lhs, token_ptr oper, foreign_ptr rhs);
    foreign_ptr(*rational)(token_ptr tok);
    foreign_ptr(*rational_complex)(token_ptr tok);
    foreign_ptr(*regexp_compose)(token_ptr begin, node_list_ptr parts, token_ptr end, foreign_ptr options);
    foreign_ptr(*regexp_options)(token_ptr regopt);
    foreign_ptr(*rescue_body)(token_ptr rescue, foreign_ptr exc_list, token_ptr assoc, foreign_ptr exc_var, token_ptr then, foreign_ptr body);
    foreign_ptr(*restarg)(token_ptr star, token_ptr name);
    foreign_ptr(*self)(token_ptr tok);
    foreign_ptr(*shadowarg)(token_ptr name);
    foreign_ptr(*splat)(token_ptr star, foreign_ptr arg);
    foreign_ptr(*string)(token_ptr string_);
    foreign_ptr(*string_compose)(token_ptr begin, node_list_ptr parts, token_ptr end);
    foreign_ptr(*string_internal)(token_ptr string_);
    foreign_ptr(*symbol)(token_ptr symbol);
    foreign_ptr(*symbol_compose)(token_ptr begin, node_list_ptr parts, token_ptr end);
    foreign_ptr(*symbol_internal)(token_ptr symbol);
    foreign_ptr(*symbols_compose)(token_ptr begin, node_list_ptr parts, token_ptr end);
    foreign_ptr(*ternary)(foreign_ptr cond, token_ptr question, foreign_ptr if_true, token_ptr colon, foreign_ptr if_false);
    foreign_ptr(*tr_array)(token_ptr begin, foreign_ptr type, token_ptr end);
    foreign_ptr(*tr_cast)(token_ptr begin, foreign_ptr expr, token_ptr colon, foreign_ptr type, token_ptr end);
    foreign_ptr(*tr_cpath)(foreign_ptr cpath);
    foreign_ptr(*tr_genargs)(token_ptr begin, node_list_ptr genargs, token_ptr end);
    foreign_ptr(*tr_gendecl)(foreign_ptr cpath, token_ptr begin, node_list_ptr genargs, token_ptr end);
    foreign_ptr(*tr_gendeclarg)(token_ptr tok);
    foreign_ptr(*tr_geninst)(foreign_ptr cpath, token_ptr begin, node_list_ptr genargs, token_ptr end);
    foreign_ptr(*tr_hash)(token_ptr begin, foreign_ptr key_type, token_ptr assoc, foreign_ptr value_type, token_ptr end);
    foreign_ptr(*tr_ivardecl)(token_ptr name, foreign_ptr type);
    foreign_ptr(*tr_nil)(token_ptr nil);
    foreign_ptr(*tr_nillable)(token_ptr tilde, foreign_ptr type);
    foreign_ptr(*tr_or)(foreign_ptr a, foreign_ptr b);
    foreign_ptr(*tr_proc)(token_ptr begin, foreign_ptr args, token_ptr end);
    foreign_ptr(*tr_special)(token_ptr special);
    foreign_ptr(*tr_tuple)(token_ptr begin, node_list_ptr types, token_ptr end);
    foreign_ptr(*true_)(token_ptr tok);
    foreign_ptr(*typed_arg)(foreign_ptr type, foreign_ptr arg);
    foreign_ptr(*unary_op)(token_ptr oper, foreign_ptr receiver);
    foreign_ptr(*undef_method)(node_list_ptr name_list);
    foreign_ptr(*when)(token_ptr when, node_list_ptr patterns, token_ptr then, foreign_ptr body);
    foreign_ptr(*word)(node_list_ptr parts);
    foreign_ptr(*words_compose)(token_ptr begin, node_list_ptr parts, token_ptr end);
    foreign_ptr(*xstring_compose)(token_ptr begin, node_list_ptr parts, token_ptr end);
  };
};

#endif
