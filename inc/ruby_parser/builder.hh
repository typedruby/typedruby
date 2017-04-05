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
    foreign_ptr(*alias)(const token* alias, foreign_ptr to, foreign_ptr from);
    foreign_ptr(*arg)(const token* name);
    foreign_ptr(*args)(const token* begin, const node_list* args, const token* end, bool check_args);
    foreign_ptr(*array)(const token* begin, const node_list* elements, const token* end);
    foreign_ptr(*assign)(foreign_ptr lhs, const token* eql, foreign_ptr rhs);
    foreign_ptr(*assignable)(foreign_ptr node);
    foreign_ptr(*associate)(const token* begin, const node_list* pairs, const token* end);
    foreign_ptr(*attr_asgn)(foreign_ptr receiver, const token* dot, const token* selector);
    foreign_ptr(*back_ref)(const token* tok);
    foreign_ptr(*begin)(const token* begin, foreign_ptr body, const token* end);
    foreign_ptr(*begin_body)(foreign_ptr body, const node_list* rescue_bodies, const token* else_tok, foreign_ptr else_, const token* ensure_tok, foreign_ptr ensure);
    foreign_ptr(*begin_keyword)(const token* begin, foreign_ptr body, const token* end);
    foreign_ptr(*binary_op)(foreign_ptr receiver, const token* oper, foreign_ptr arg);
    foreign_ptr(*block)(foreign_ptr method_call, const token* begin, foreign_ptr args, foreign_ptr body, const token* end);
    foreign_ptr(*block_pass)(const token* amper, foreign_ptr arg);
    foreign_ptr(*blockarg)(const token* amper, const token* name);
    foreign_ptr(*call_lambda)(const token* lambda);
    foreign_ptr(*call_method)(foreign_ptr receiver, const token* dot, const token* selector, const token* lparen, const node_list* args, const token* rparen);
    foreign_ptr(*case_)(const token* case_, foreign_ptr expr, const node_list* when_bodies, const token* else_tok, foreign_ptr else_body, const token* end);
    foreign_ptr(*character)(const token* char_);
    foreign_ptr(*complex)(const token* tok);
    foreign_ptr(*compstmt)(const node_list* node);
    foreign_ptr(*condition)(const token* cond_tok, foreign_ptr cond, const token* then, foreign_ptr if_true, const token* else_, foreign_ptr if_false, const token* end);
    foreign_ptr(*condition_mod)(foreign_ptr if_true, foreign_ptr if_false, foreign_ptr cond);
    foreign_ptr(*const_)(const token* name);
    foreign_ptr(*const_fetch)(foreign_ptr scope, const token* colon, const token* name);
    foreign_ptr(*const_global)(const token* colon, const token* name);
    foreign_ptr(*const_op_assignable)(foreign_ptr node);
    foreign_ptr(*cvar)(const token* tok);
    foreign_ptr(*dedent_string)(foreign_ptr node, size_t dedent_level);
    foreign_ptr(*def_class)(const token* class_, foreign_ptr name, const token* lt_, foreign_ptr superclass, foreign_ptr body, const token* end_);
    foreign_ptr(*def_method)(const token* def, const token* name, foreign_ptr args, foreign_ptr body, const token* end);
    foreign_ptr(*def_module)(const token* module, foreign_ptr name, foreign_ptr body, const token* end_);
    foreign_ptr(*def_sclass)(const token* class_, const token* lshft_, foreign_ptr expr, foreign_ptr body, const token* end_);
    foreign_ptr(*def_singleton)(const token* def, foreign_ptr definee, const token* dot, const token* name, foreign_ptr args, foreign_ptr body, const token* end);
    foreign_ptr(*encoding_literal)(const token* tok);
    foreign_ptr(*false_)(const token* tok);
    foreign_ptr(*file_literal)(const token* tok);
    foreign_ptr(*float_)(const token* tok);
    foreign_ptr(*float_complex)(const token* tok);
    foreign_ptr(*for_)(const token* for_, foreign_ptr iterator, const token* in, foreign_ptr iteratee, const token* do_, foreign_ptr body, const token* end);
    foreign_ptr(*gvar)(const token* tok);
    foreign_ptr(*ident)(const token* tok);
    foreign_ptr(*index)(foreign_ptr receiver, const token* lbrack, const node_list* indexes, const token* rbrack);
    foreign_ptr(*index_asgn)(foreign_ptr receiver, const token* lbrack, const node_list* indexes, const token* rbrack);
    foreign_ptr(*integer)(const token* tok);
    foreign_ptr(*ivar)(const token* tok);
    foreign_ptr(*keyword_cmd)(node_type type, const token* keyword, const token* lparen, const node_list* args, const token* rparen);
    foreign_ptr(*kwarg)(const token* name);
    foreign_ptr(*kwoptarg)(const token* name, foreign_ptr value);
    foreign_ptr(*kwrestarg)(const token* dstar, const token* name);
    foreign_ptr(*kwsplat)(const token* dstar, foreign_ptr arg);
    foreign_ptr(*line_literal)(const token* tok);
    foreign_ptr(*logical_op)(node_type type, foreign_ptr lhs, const token* op, foreign_ptr rhs);
    foreign_ptr(*loop)(node_type type, const token* keyword, foreign_ptr cond, const token* do_, foreign_ptr body, const token* end);
    foreign_ptr(*loop_mod)(node_type type, foreign_ptr body, foreign_ptr cond);
    foreign_ptr(*match_op)(foreign_ptr receiver, const token* oper, foreign_ptr arg);
    foreign_ptr(*multi_assign)(foreign_ptr mlhs, foreign_ptr rhs);
    foreign_ptr(*multi_lhs)(const token* begin, const node_list* items, const token* end);
    foreign_ptr(*negate)(const token* uminus, foreign_ptr numeric);
    foreign_ptr(*nil)(const token* tok);
    foreign_ptr(*not_op)(const token* not_, const token* begin, foreign_ptr receiver, const token* end);
    foreign_ptr(*nth_ref)(const token* tok);
    foreign_ptr(*op_assign)(foreign_ptr lhs, const token* op, foreign_ptr rhs);
    foreign_ptr(*optarg)(const token* name, const token* eql, foreign_ptr value);
    foreign_ptr(*pair)(foreign_ptr key, const token* assoc, foreign_ptr value);
    foreign_ptr(*pair_keyword)(const token* key, foreign_ptr value);
    foreign_ptr(*pair_quoted)(const token* begin, const node_list* parts, const token* end, foreign_ptr value);
    foreign_ptr(*postexe)(foreign_ptr body);
    foreign_ptr(*preexe)(foreign_ptr node);
    foreign_ptr(*procarg0)(foreign_ptr arg);
    foreign_ptr(*prototype)(foreign_ptr genargs, foreign_ptr args, foreign_ptr return_type);
    foreign_ptr(*range_exclusive)(foreign_ptr lhs, const token* oper, foreign_ptr rhs);
    foreign_ptr(*range_inclusive)(foreign_ptr lhs, const token* oper, foreign_ptr rhs);
    foreign_ptr(*rational)(const token* tok);
    foreign_ptr(*rational_complex)(const token* tok);
    foreign_ptr(*regexp_compose)(const token* begin, const node_list* parts, const token* end, foreign_ptr options);
    foreign_ptr(*regexp_options)(const token* regopt);
    foreign_ptr(*rescue_body)(const token* rescue, foreign_ptr exc_list, const token* assoc, foreign_ptr exc_var, const token* then, foreign_ptr body);
    foreign_ptr(*restarg)(const token* star, const token* name);
    foreign_ptr(*self)(const token* tok);
    foreign_ptr(*shadowarg)(const token* name);
    foreign_ptr(*splat)(const token* star, foreign_ptr arg);
    foreign_ptr(*string)(const token* string_);
    foreign_ptr(*string_compose)(const token* begin, const node_list* parts, const token* end);
    foreign_ptr(*string_internal)(const token* string_);
    foreign_ptr(*symbol)(const token* symbol);
    foreign_ptr(*symbol_compose)(const token* begin, const node_list* parts, const token* end);
    foreign_ptr(*symbol_internal)(const token* symbol);
    foreign_ptr(*symbols_compose)(const token* begin, const node_list* parts, const token* end);
    foreign_ptr(*ternary)(foreign_ptr cond, const token* question, foreign_ptr if_true, const token* colon, foreign_ptr if_false);
    foreign_ptr(*tr_array)(const token* begin, foreign_ptr type, const token* end);
    foreign_ptr(*tr_cast)(const token* begin, foreign_ptr expr, const token* colon, foreign_ptr type, const token* end);
    foreign_ptr(*tr_cpath)(foreign_ptr cpath);
    foreign_ptr(*tr_genargs)(const token* begin, const node_list* genargs, const token* end);
    foreign_ptr(*tr_gendecl)(foreign_ptr cpath, const token* begin, const node_list* genargs, const token* end);
    foreign_ptr(*tr_gendeclarg)(const token* tok);
    foreign_ptr(*tr_geninst)(foreign_ptr cpath, const token* begin, const node_list* genargs, const token* end);
    foreign_ptr(*tr_hash)(const token* begin, foreign_ptr key_type, const token* assoc, foreign_ptr value_type, const token* end);
    foreign_ptr(*tr_ivardecl)(const token* name, foreign_ptr type);
    foreign_ptr(*tr_nil)(const token* nil);
    foreign_ptr(*tr_nillable)(const token* tilde, foreign_ptr type);
    foreign_ptr(*tr_or)(foreign_ptr a, foreign_ptr b);
    foreign_ptr(*tr_proc)(const token* begin, foreign_ptr args, const token* end);
    foreign_ptr(*tr_special)(const token* special);
    foreign_ptr(*tr_tuple)(const token* begin, const node_list* types, const token* end);
    foreign_ptr(*true_)(const token* tok);
    foreign_ptr(*typed_arg)(foreign_ptr type, foreign_ptr arg);
    foreign_ptr(*unary_op)(const token* oper, foreign_ptr receiver);
    foreign_ptr(*undef_method)(const node_list* name_list);
    foreign_ptr(*when)(const token* when, const node_list* patterns, const token* then, foreign_ptr body);
    foreign_ptr(*word)(const node_list* parts);
    foreign_ptr(*words_compose)(const token* begin, const node_list* parts, const token* end);
    foreign_ptr(*xstring_compose)(const token* begin, const node_list* parts, const token* end);
  };
};

#endif
