%{
  #include <ruby_parser/builder.hh>
  #include <ruby_parser/node.hh>
  #include <ruby_parser/token.hh>
  #include <ruby_parser/lexer.hh>
  #include <ruby_parser/parser.hh>
  #include <ruby_parser/state_stack.hh>
  #include <iterator>
  #include <iostream>
  #include <utility>
  #include <cstdlib>

  using namespace ruby_parser;

  #define yyparse ruby_parser_typedruby24_yyparse

  extern "C" {
    int yyparse(parser::typedruby24& p);
  }
%}

%pure-parser

%lex-param { parser::typedruby24& p }
%parse-param { parser::typedruby24& p }

%union {
  token_ptr* token;
  delimited_node_list_ptr* delimited_list;
  delimited_block_ptr* delimited_block;
  node_with_token_ptr* with_token;
  case_body_ptr* case_body;
  foreign_ptr* node;
  node_list_ptr* list;
  state_stack_ptr* state_stack;
  size_t size;
  bool boolean;
}

// mirrored in inc/ruby_parser/token.hh
// if any of these token values are changed here, the header must be updated
// as well.
%token <token>
  kCLASS              1001
  kMODULE             1002
  kDEF                1003
  kUNDEF              1004
  kBEGIN              1005
  kRESCUE             1006
  kENSURE             1007
  kEND                1008
  kIF                 1009
  kUNLESS             1010
  kTHEN               1011
  kELSIF              1012
  kELSE               1013
  kCASE               1014
  kWHEN               1015
  kWHILE              1016
  kUNTIL              1017
  kFOR                1018
  kBREAK              1019
  kNEXT               1020
  kREDO               1021
  kRETRY              1022
  kIN                 1023
  kDO                 1024
  kDO_COND            1025
  kDO_BLOCK           1026
  kDO_LAMBDA          1027
  kRETURN             1028
  kYIELD              1029
  kSUPER              1030
  kSELF               1031
  kNIL                1032
  kTRUE               1033
  kFALSE              1034
  kAND                1035
  kOR                 1036
  kNOT                1037
  kIF_MOD             1038
  kUNLESS_MOD         1039
  kWHILE_MOD          1040
  kUNTIL_MOD          1041
  kRESCUE_MOD         1042
  kALIAS              1043
  kDEFINED            1044
  klBEGIN             1045
  klEND               1046
  k__LINE__           1047
  k__FILE__           1048
  k__ENCODING__       1049
  tIDENTIFIER         1050
  tFID                1051
  tGVAR               1052
  tIVAR               1053
  tCONSTANT           1054
  tLABEL              1055
  tCVAR               1056
  tNTH_REF            1057
  tBACK_REF           1058
  tSTRING_CONTENT     1059
  tINTEGER            1060
  tFLOAT              1061
  tUPLUS              1062
  tUMINUS             1063
  tUMINUS_NUM         1064
  tPOW                1065
  tCMP                1066
  tEQ                 1067
  tEQQ                1068
  tNEQ                1069
  tEQL                1070
  tGEQ                1071
  tLEQ                1072
  tANDOP              1073
  tOROP               1074
  tMATCH              1075
  tNMATCH             1076
  tDOT                1077
  tDOT2               1078
  tDOT3               1079
  tAREF               1080
  tASET               1081
  tLSHFT              1082
  tRSHFT              1083
  tCOLON2             1084
  tCOLON3             1085
  tOP_ASGN            1086
  tASSOC              1087
  tLPAREN             1088
  tLPAREN2            1089
  tRPAREN             1090
  tLPAREN_ARG         1091
  tLBRACK             1092
  tLBRACK2            1093
  tRBRACK             1094
  tLBRACE             1095
  tLBRACE_ARG         1096
  tSTAR               1097
  tSTAR2              1098
  tAMPER              1099
  tAMPER2             1100
  tTILDE              1101
  tPERCENT            1102
  tDIVIDE             1103
  tDSTAR              1104
  tPLUS               1105
  tMINUS              1106
  tLT                 1107
  tGT                 1108
  tPIPE               1109
  tBANG               1110
  tCARET              1111
  tLCURLY             1112
  tRCURLY             1113
  tBACK_REF2          1114
  tSYMBEG             1115
  tSTRING_BEG         1116
  tXSTRING_BEG        1117
  tREGEXP_BEG         1118
  tREGEXP_OPT         1119
  tWORDS_BEG          1120
  tQWORDS_BEG         1121
  tSYMBOLS_BEG        1122
  tQSYMBOLS_BEG       1123
  tSTRING_DBEG        1124
  tSTRING_DVAR        1125
  tSTRING_END         1126
  tSTRING_DEND        1127
  tSTRING             1128
  tSYMBOL             1129
  tNL                 1130
  tEH                 1131
  tCOLON              1132
  tCOMMA              1133
  tSPACE              1134
  tSEMI               1135
  tLAMBDA             1136
  tLAMBEG             1137
  tCHARACTER          1138
  tRATIONAL           1139
  tIMAGINARY          1140
  tLABEL_END          1141
  tANDDOT             1142
  tRATIONAL_IMAGINARY 1143
  tFLOAT_IMAGINARY    1144

%type <node>
  arg
  arg_rhs
  arg_value
  assoc
  backref
  block_arg
  block_call
  block_command
  block_param_def
  bodystmt
  bvar
  command
  command_asgn
  command_call
  command_rhs
  compstmt
  cpath
  dsym
  expr
  expr_value
  f_arg_item
  f_arglist
  f_block_kw
  f_block_opt
  f_kw
  f_larglist
  f_marg
  f_opt
  fitem
  for_var
  fsym
  keyword_variable
  lhs
  literal
  method_call
  mlhs
  mlhs_inner
  mlhs_item
  mlhs_node
  mrhs_arg
  none
  numeric
  opt_block_param
  primary
  primary_value
  qsymbols
  qwords
  regexp
  simple_numeric
  singleton
  stmt
  stmt_or_begin
  string1
  string_content
  string_dvar
  strings
  symbol
  symbols
  top_compstmt
  top_stmt
  tr_argsig
  tr_blockproto
  tr_cpath
  tr_methodgenargs
  tr_returnsig
  tr_type
  tr_union_type
  user_variable
  var_lhs
  var_ref
  words
  xstring

%type <list>
  aref_args
  args
  args_tail
  assoc_list
  assocs
  block_args_tail
  block_param
  bv_decls
  call_args
  command_args
  exc_list
  f_arg
  f_args
  f_block_arg
  f_block_kwarg
  f_block_optarg
  f_kwarg
  f_kwrest
  f_marg_list
  f_margs
  f_optarg
  f_rest_arg
  list_none
  mlhs_basic
  mlhs_head
  mlhs_post
  mrhs
  opt_args_tail
  opt_block_arg
  opt_block_args_tail
  opt_bv_decl
  opt_call_args
  opt_f_block_arg
  opt_rescue
  qsym_list
  qword_list
  regexp_contents
  stmts
  string
  string_contents
  symbol_list
  top_stmts
  tr_gendeclargs
  tr_types
  undef_list
  word
  word_list
  xstring_contents

%type <token>
  blkarg_mark
  call_op
  cname
  do
  dot_or_colon
  f_arg_asgn
  f_bad_arg
  f_label
  f_norm_arg
  fcall
  fname
  kwrest_mark
  op
  operation
  operation2
  operation3
  rbracket
  restarg_mark
  reswords
  rparen
  term
  then

%type <delimited_list>
  opt_paren_args
  paren_args

%type <delimited_block>
  brace_block
  brace_body
  cmd_brace_block
  do_block
  do_body
  lambda
  lambda_body

%type <with_token>
  exc_var
  if_tail
  opt_else
  opt_ensure
  superclass

%type <case_body>
  case_body
  cases

%nonassoc tLOWEST
%nonassoc tLBRACE_ARG
%nonassoc kIF_MOD kUNLESS_MOD kWHILE_MOD kUNTIL_MOD
%left     kOR kAND
%right    kNOT
%nonassoc kDEFINED
%right    tEQL tOP_ASGN
%left     kRESCUE_MOD
%right    tEH tCOLON
%nonassoc tDOT2 tDOT3
%left     tOROP
%left     tANDOP
%nonassoc tCMP tEQ tEQQ tNEQ tMATCH tNMATCH
%left     tGT tGEQ tLT tLEQ
%left     tPIPE tCARET
%left     tAMPER2
%left     tLSHFT tRSHFT
%left     tPLUS tMINUS
%left     tSTAR2 tDIVIDE tPERCENT
%right    tUMINUS_NUM tUMINUS
%right    tPOW
%right    tBANG tTILDE tUPLUS

%{
  template<typename T>
  static T take(parser::base& p, T* raw_ptr) {
    if (!raw_ptr) {
      return nullptr;
    }

    auto iter = p.saved_pointers.find((void*)raw_ptr);

    if (iter == p.saved_pointers.end()) {
      fprintf(stderr, "tried to take dodgy pointer!\n");
      abort();
    }

    p.saved_pointers.erase(iter);

    auto ptr = std::move(*raw_ptr);
    delete raw_ptr;
    return ptr;
  }

  template<typename T>
  static T* put(parser::base& p, T ptr) {
    T* raw_ptr = new T(ptr.release());
    p.saved_pointers.insert((void*)raw_ptr);
    return raw_ptr;
  }

  template<typename T>
  static std::unique_ptr<T>* put_copy(parser::base& p, T obj) {
    return put(p, std::make_unique<T>(obj));
  }

  template<typename To, typename From>
  static std::unique_ptr<To> static_unique_cast(std::unique_ptr<From> from) {
    return std::unique_ptr<To> { static_cast<To*>(from.release()) };
  }

  static node_list_ptr make_node_list() {
    return std::make_unique<node_list>(std::vector<foreign_ptr>());
  }

  static node_list_ptr make_node_list(foreign_ptr&& node) {
    std::vector<foreign_ptr> vec;
    vec.push_back(std::move(node));
    return std::make_unique<node_list>(std::move(vec));
  }

  static void concat_node_list(node_list_ptr& a, node_list_ptr&& b) {
    a->nodes.insert(
      a->nodes.begin(),
      std::make_move_iterator(b->nodes.begin()),
      std::make_move_iterator(b->nodes.end())
    );
  }

  static int yyerror(parser::typedruby24& p, std::string message) {
    (void)p;
    std::cerr << message << std::endl;
    abort();
  }

  static int yylex(YYSTYPE *lval, parser::typedruby24& p) {
    auto token = p.lexer->advance();

    int token_type = static_cast<int>(token->type());

    if (token_type < 0) {
      // some sort of lex error!
      std::cerr << "lex error" << std::endl;
      abort();
    }

    lval->token = put(p, std::move(token));

    return token_type;
  }
%}

%%
         program: top_compstmt
                    {
                      p.ast = take(p, $1);
                    }

    top_compstmt: top_stmts opt_terms
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.compstmt(_1.get()));
                    }

       top_stmts: // nothing
                    {
                      $$ = put(p, make_node_list());
                    }
                | top_stmt
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | top_stmts terms top_stmt
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }
                | error top_stmt
                    {
                      auto _2 = take(p, $2);
                      $$ = put(p, make_node_list(std::move(_2)));
                    }

        top_stmt: stmt
                | klBEGIN tLCURLY top_compstmt tRCURLY
                    {
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.preexe(std::move(_3)));
                    }

        bodystmt: compstmt opt_rescue opt_else opt_ensure
                    {
                      auto rescue_bodies = take(p, $2);
                      auto else_ = take(p, $3);

                      auto ensure = take(p, $4);

                      if (rescue_bodies->nodes.size() == 0 && else_ != nullptr) {
                        // TODO diagnostic :warning, :useless_else, nullptr, else_t
                      }

                      $$ = put(p, p.builder.begin_body(take(p, $1),
                            rescue_bodies.get(),
                            else_ ? else_->token_.get() : nullptr,
                            else_ ? std::move(else_->node_) : nullptr,
                            ensure ? ensure->token_.get() : nullptr,
                            ensure ? std::move(ensure->node_) : nullptr));
                    }

        compstmt: stmts opt_terms
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.compstmt(_1.get()));
                    }

           stmts: // nothing
                    {
                      $$ = put(p, make_node_list());
                    }
                | stmt_or_begin
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | stmts terms stmt_or_begin
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }
                | error stmt
                    {
                      auto _2 = take(p, $2);
                      $$ = put(p, make_node_list(std::move(_2)));
                    }

   stmt_or_begin: stmt
                | klBEGIN tLCURLY top_compstmt tRCURLY
                    {
                      auto _1 = take(p, $1);
                      /* TODO diagnostic :error, :begin_in_method, nullptr, std::move(_1) */
                    }

            stmt: kALIAS fitem
                    {
                      p.lexer->set_state_expr_fname();
                    }
                    fitem
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.alias(_1.get(), std::move(_2), std::move(_4)));
                    }
                | kALIAS tGVAR tGVAR
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.alias(_1.get(),
                        p.builder.gvar(_2.get()),
                        p.builder.gvar(_3.get())));
                    }
                | kALIAS tGVAR tBACK_REF
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.alias(_1.get(),
                        p.builder.gvar(_2.get()),
                        p.builder.back_ref(_3.get())));
                    }
                | kALIAS tGVAR tNTH_REF
                    {
                      // TODO diagnostic :error, :nth_ref_alias, nullptr, $3
                    }
                | kUNDEF undef_list
                    {
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.undef_method(_2.get()));
                    }
                | stmt kIF_MOD expr_value
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.condition_mod(std::move(_1), nullptr, std::move(_3)));
                    }
                | stmt kUNLESS_MOD expr_value
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.condition_mod(nullptr, std::move(_1), std::move(_3)));
                    }
                | stmt kWHILE_MOD expr_value
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.loop_mod(node_type::WHILE, std::move(_1), std::move(_3)));
                    }
                | stmt kUNTIL_MOD expr_value
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.loop_mod(node_type::UNTIL, std::move(_1), std::move(_3)));
                    }
                | stmt kRESCUE_MOD stmt
                    {
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _1 = take(p, $1);
                      auto rescue_body = make_node_list(p.builder.rescue_body(_2.get(), nullptr, nullptr, nullptr, nullptr, std::move(_3)));

                      $$ = put(p, p.builder.begin_body(
                        std::move(_1),
                        rescue_body.get(),
                        nullptr, nullptr, nullptr, nullptr));
                    }
                | klEND tLCURLY compstmt tRCURLY
                    {
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.postexe(std::move(_3)));
                    }
                | command_asgn
                | mlhs tEQL command_call
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.multi_assign(std::move(_1), std::move(_3)));
                    }
                | lhs tEQL mrhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.assign(std::move(_1), _2.get(), p.builder.array(nullptr, _3.get(), nullptr)));
                    }
                | mlhs tEQL mrhs_arg
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.multi_assign(std::move(_1), std::move(_3)));
                    }
                | kDEF tIVAR tCOLON tr_type
                    {
                      auto _2 = take(p, $2);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.tr_ivardecl(_2.get(), std::move(_4)));
                    }
                | expr

    command_asgn: lhs tEQL command_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.assign(std::move(_1), _2.get(), std::move(_3)));
                    }
                | var_lhs tOP_ASGN command_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.op_assign(std::move(_1), _2.get(), std::move(_3)));
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN command_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      $$ = put(p, p.builder.op_assign(
                                  p.builder.index(
                                    std::move(_1), _2.get(), _3.get(), _4.get()),
                                  _5.get(), std::move(_6)));
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN command_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      $$ = put(p, p.builder.op_assign(
                                  p.builder.call_method(
                                    std::move(_1), _2.get(), _3.get(), nullptr, nullptr, nullptr),
                                  _4.get(), std::move(_5)));
                    }
                | primary_value call_op tCONSTANT tOP_ASGN command_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      $$ = put(p, p.builder.op_assign(
                                  p.builder.call_method(
                                    std::move(_1), _2.get(), _3.get(), nullptr, nullptr, nullptr),
                                  _4.get(), std::move(_5)));
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN command_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      auto const_node = p.builder.const_op_assignable(
                                  p.builder.const_fetch(std::move(_1), _2.get(), _3.get()));
                      $$ = put(p, p.builder.op_assign(std::move(const_node), _4.get(), std::move(_5)));
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN command_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      $$ = put(p, p.builder.op_assign(
                                  p.builder.call_method(
                                    std::move(_1), _2.get(), _3.get(), nullptr, nullptr, nullptr),
                                  _4.get(), std::move(_5)));
                    }
                | backref tOP_ASGN command_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      p.builder.op_assign(std::move(_1), _2.get(), std::move(_3));
                    }

     command_rhs: command_call %prec tOP_ASGN
                | command_call kRESCUE_MOD stmt
                    {
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _1 = take(p, $1);
                      auto rescue_body =
                        make_node_list(
                          p.builder.rescue_body(_2.get(),
                                          nullptr, nullptr, nullptr,
                                          nullptr, std::move(_3)));

                      $$ = put(p, p.builder.begin_body(std::move(_1), rescue_body.get(), nullptr, nullptr, nullptr, nullptr));
                    }
                | command_asgn

            expr: command_call
                | expr kAND expr
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.logical_op(node_type::AND, std::move(_1), _2.get(), std::move(_3)));
                    }
                | expr kOR expr
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.logical_op(node_type::OR, std::move(_1), _2.get(), std::move(_3)));
                    }
                | kNOT opt_nl expr
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.not_op(_1.get(), nullptr, std::move(_3), nullptr));
                    }
                | tBANG command_call
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.not_op(_1.get(), nullptr, std::move(_2), nullptr));
                    }
                | arg

      expr_value: expr

    command_call: command
                | block_command

   block_command: block_call
                | block_call dot_or_colon operation2 command_args
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.call_method(std::move(_1), _2.get(), _3.get(),
                                  nullptr, _4.get(), nullptr));
                    }

 cmd_brace_block: tLBRACE_ARG brace_body tRCURLY
                    {
                      auto block = take(p, $2);
                      block->begin = take(p, $1);
                      block->end = take(p, $3);
                      $$ = put(p, std::move(block));
                    }

           fcall: operation

         command: fcall command_args %prec tLOWEST
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.call_method(nullptr, nullptr, _1.get(),
                                  nullptr, _2.get(), nullptr));
                    }
                | fcall command_args cmd_brace_block
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto method_call = p.builder.call_method(nullptr, nullptr, _1.get(),
                                                              nullptr, _2.get(), nullptr);

                      auto delimited_block = take(p, $3);

                      $$ = put(p, p.builder.block(std::move(method_call),
                                      delimited_block->begin.get(),
                                      std::move(delimited_block->args),
                                      std::move(delimited_block->body),
                                      delimited_block->end.get()));
                    }
                | primary_value call_op operation2 command_args %prec tLOWEST
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.call_method(std::move(_1), _2.get(), _3.get(),
                                  nullptr, _4.get(), nullptr));
                    }
                | primary_value call_op operation2 command_args cmd_brace_block
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto method_call = p.builder.call_method(std::move(_1), _2.get(), _3.get(),
                                        nullptr, _4.get(), nullptr);

                      auto delimited_block = take(p, $5);

                      $$ = put(p, p.builder.block(std::move(method_call),
                                      delimited_block->begin.get(),
                                      std::move(delimited_block->args),
                                      std::move(delimited_block->body),
                                      delimited_block->end.get()));
                    }
                | primary_value tCOLON2 operation2 command_args %prec tLOWEST
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.call_method(std::move(_1), _2.get(), _3.get(),
                                  nullptr, _4.get(), nullptr));
                    }
                | primary_value tCOLON2 operation2 command_args cmd_brace_block
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto method_call = p.builder.call_method(std::move(_1), _2.get(), _3.get(),
                                        nullptr, _4.get(), nullptr);

                      auto delimited_block = take(p, $5);

                      $$ = put(p, p.builder.block(std::move(method_call),
                                      delimited_block->begin.get(),
                                      std::move(delimited_block->args),
                                      std::move(delimited_block->body),
                                      delimited_block->end.get()));
                    }
                | kSUPER command_args
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.keyword_cmd(node_type::SUPER, _1.get(),
                                  nullptr, _2.get(), nullptr));
                    }
                | kYIELD command_args
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.keyword_cmd(node_type::YIELD, _1.get(),
                                  nullptr, _2.get(), nullptr));
                    }
                | kRETURN call_args
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.keyword_cmd(node_type::RETURN, _1.get(),
                                  nullptr, _2.get(), nullptr));
                    }
                | kBREAK call_args
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.keyword_cmd(node_type::BREAK, _1.get(),
                                  nullptr, _2.get(), nullptr));
                    }
                | kNEXT call_args
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.keyword_cmd(node_type::NEXT, _1.get(),
                                  nullptr, _2.get(), nullptr));
                    }

            mlhs: mlhs_basic
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.multi_lhs(nullptr, _1.get(), nullptr));
                    }
                | tLPAREN mlhs_inner rparen
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.begin(_1.get(), std::move(_2), _3.get()));
                    }

      mlhs_inner: mlhs_basic
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.multi_lhs(nullptr, _1.get(), nullptr));
                    }
                | tLPAREN mlhs_inner rparen
                    {
                      auto _2 = take(p, $2);
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      auto inner = make_node_list(std::move(_2));
                      $$ = put(p, p.builder.multi_lhs(_1.get(), inner.get(), _3.get()));
                    }

      mlhs_basic: mlhs_head
                | mlhs_head mlhs_item
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_2));
                      $$ = put(p, std::move(list));
                    }
                | mlhs_head tSTAR mlhs_node
                    {
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.splat(_2.get(), std::move(_3)));
                      $$ = put(p, std::move(list));
                    }
                | mlhs_head tSTAR mlhs_node tCOMMA mlhs_post
                    {
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto head = take(p, $1);

                      head->nodes.push_back(p.builder.splat(_2.get(), std::move(_3)));
                      concat_node_list(head, std::move(_5));

                      $$ = put(p, std::move(head));
                    }
                | mlhs_head tSTAR
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.splat(_2.get(), nullptr));
                      $$ = put(p, std::move(list));
                    }
                | mlhs_head tSTAR tCOMMA mlhs_post
                    {
                      auto _2 = take(p, $2);
                      auto _4 = take(p, $4);
                      auto head = take(p, $1);

                      head->nodes.push_back(p.builder.splat(_2.get(), nullptr));
                      concat_node_list(head, std::move(_4));

                      $$ = put(p, std::move(head));
                    }
                | tSTAR mlhs_node
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, make_node_list({ p.builder.splat(_1.get(), std::move(_2)) }));
                    }
                | tSTAR mlhs_node tCOMMA mlhs_post
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _4 = take(p, $4);
                      auto items = make_node_list({ p.builder.splat(_1.get(), std::move(_2)) });

                      concat_node_list(items, std::move(_4));

                      $$ = put(p, std::move(items));
                    }
                | tSTAR
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(p.builder.splat(_1.get(), nullptr)));
                    }
                | tSTAR tCOMMA mlhs_post
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      auto items = make_node_list(p.builder.splat(_1.get(), nullptr));

                      concat_node_list(items, std::move(_3));

                      $$ = put(p, std::move(items));
                    }

       mlhs_item: mlhs_node
                | tLPAREN mlhs_inner rparen
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.begin(_1.get(), std::move(_2), _3.get()));
                    }

       mlhs_head: mlhs_item tCOMMA
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | mlhs_head mlhs_item tCOMMA
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_2));
                      $$ = put(p, std::move(list));
                    }

       mlhs_post: mlhs_item
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | mlhs_post tCOMMA mlhs_item
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }

       mlhs_node: user_variable
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.assignable(std::move(_1)));
                    }
                | keyword_variable
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.assignable(std::move(_1)));
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.index_asgn(std::move(_1), _2.get(), _3.get(), _4.get()));
                    }
                | primary_value call_op tIDENTIFIER
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.attr_asgn(std::move(_1), _2.get(), _3.get()));
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.attr_asgn(std::move(_1), _2.get(), _3.get()));
                    }
                | primary_value call_op tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.attr_asgn(std::move(_1), _2.get(), _3.get()));
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.assignable(
                                  p.builder.const_fetch(std::move(_1), _2.get(), _3.get())));
                    }
                | tCOLON3 tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.assignable(
                                  p.builder.const_global(_1.get(), _2.get())));
                    }
                | backref
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.assignable(std::move(_1)));
                    }

             lhs: user_variable
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.assignable(std::move(_1)));
                    }
                | keyword_variable
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.assignable(std::move(_1)));
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.index_asgn(std::move(_1), _2.get(), _3.get(), _4.get()));
                    }
                | primary_value call_op tIDENTIFIER
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.attr_asgn(std::move(_1), _2.get(), _3.get()));
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.attr_asgn(std::move(_1), _2.get(), _3.get()));
                    }
                | primary_value call_op tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.attr_asgn(std::move(_1), _2.get(), _3.get()));
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.assignable(
                                  p.builder.const_fetch(std::move(_1), _2.get(), _3.get())));
                    }
                | tCOLON3 tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.assignable(
                                  p.builder.const_global(_1.get(), _2.get())));
                    }
                | backref
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.assignable(std::move(_1)));
                    }

           cname: tIDENTIFIER
                    {
                      auto _1 = take(p, $1);
                      // TODO diagnostic :error, :module_name_const, nullptr, std::move(_1)
                    }
                | tCONSTANT

           cpath: tCOLON3 cname
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.const_global(_1.get(), _2.get()));
                    }
                | cname
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.const_(_1.get()));
                    }
                | primary_value tCOLON2 tLBRACK2 tr_gendeclargs rbracket
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      $$ = put(p, p.builder.tr_gendecl(std::move(_1), _3.get(), _4.get(), _5.get()));
                    }
                | primary_value tCOLON2 cname
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.const_fetch(std::move(_1), _2.get(), _3.get()));
                    }

           fname: tIDENTIFIER | tCONSTANT | tFID
                | op
                | reswords

            fsym: fname
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.symbol(_1.get()));
                    }
                | symbol

           fitem: fsym
                | dsym

      undef_list: fitem
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | undef_list tCOMMA
                    {
                      p.lexer->set_state_expr_fname();
                    }
                    fitem
                    {
                      auto _4 = take(p, $4);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_4));
                      $$ = put(p, std::move(list));
                    }

              op:   tPIPE    | tCARET  | tAMPER2  | tCMP  | tEQ     | tEQQ
                |   tMATCH   | tNMATCH | tGT      | tGEQ  | tLT     | tLEQ
                |   tNEQ     | tLSHFT  | tRSHFT   | tPLUS | tMINUS  | tSTAR2
                |   tSTAR    | tDIVIDE | tPERCENT | tPOW  | tBANG   | tTILDE
                |   tUPLUS   | tUMINUS | tAREF    | tASET | tDSTAR  | tBACK_REF2

        reswords: k__LINE__ | k__FILE__ | k__ENCODING__ | klBEGIN | klEND
                | kALIAS    | kAND      | kBEGIN        | kBREAK  | kCASE
                | kCLASS    | kDEF      | kDEFINED      | kDO     | kELSE
                | kELSIF    | kEND      | kENSURE       | kFALSE  | kFOR
                | kIN       | kMODULE   | kNEXT         | kNIL    | kNOT
                | kOR       | kREDO     | kRESCUE       | kRETRY  | kRETURN
                | kSELF     | kSUPER    | kTHEN         | kTRUE   | kUNDEF
                | kWHEN     | kYIELD    | kIF           | kUNLESS | kWHILE
                | kUNTIL

             arg: lhs tEQL arg_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.assign(std::move(_1), _2.get(), std::move(_3)));
                    }
                | var_lhs tOP_ASGN arg_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.op_assign(std::move(_1), _2.get(), std::move(_3)));
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      $$ = put(p, p.builder.op_assign(
                                  p.builder.index(
                                    std::move(_1), _2.get(), _3.get(), _4.get()),
                                  _5.get(), std::move(_6)));
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN arg_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      $$ = put(p, p.builder.op_assign(
                                  p.builder.call_method(
                                    std::move(_1), _2.get(), _3.get(), nullptr, nullptr, nullptr),
                                  _4.get(), std::move(_5)));
                    }
                | primary_value call_op tCONSTANT tOP_ASGN arg_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      $$ = put(p, p.builder.op_assign(
                                  p.builder.call_method(
                                    std::move(_1), _2.get(), _3.get(), nullptr, nullptr, nullptr),
                                  _4.get(), std::move(_5)));
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN arg_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      $$ = put(p, p.builder.op_assign(
                                  p.builder.call_method(
                                    std::move(_1), _2.get(), _3.get(), nullptr, nullptr, nullptr),
                                  _4.get(), std::move(_5)));
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN arg_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      auto const_ = p.builder.const_op_assignable(
                                      p.builder.const_fetch(std::move(_1), _2.get(), _3.get()));

                      $$ = put(p, p.builder.op_assign(std::move(const_), _4.get(), std::move(_5)));
                    }
                | tCOLON3 tCONSTANT tOP_ASGN arg_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto const_ = p.builder.const_op_assignable(
                                  p.builder.const_global(_1.get(), _2.get()));

                      $$ = put(p, p.builder.op_assign(std::move(const_), _3.get(), std::move(_4)));
                    }
                | backref tOP_ASGN arg_rhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.op_assign(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tDOT2 arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.range_inclusive(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tDOT3 arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.range_exclusive(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tPLUS arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tMINUS arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tSTAR2 arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tDIVIDE arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tPERCENT arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tPOW arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | tUMINUS_NUM simple_numeric tPOW arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.unary_op(_1.get(),
                                  p.builder.binary_op(
                                    std::move(_2), _3.get(), std::move(_4))));
                    }
                | tUPLUS arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.unary_op(_1.get(), std::move(_2)));
                    }
                | tUMINUS arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.unary_op(_1.get(), std::move(_2)));
                    }
                | arg tPIPE arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tCARET arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tAMPER2 arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tCMP arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tGT arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tGEQ arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tLT arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tLEQ arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tEQ arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tEQQ arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tNEQ arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tMATCH arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.match_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tNMATCH arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | tBANG arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.not_op(_1.get(), nullptr, std::move(_2), nullptr));
                    }
                | tTILDE arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.unary_op(_1.get(), std::move(_2)));
                    }
                | arg tLSHFT arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tRSHFT arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.binary_op(std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tANDOP arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.logical_op(node_type::AND, std::move(_1), _2.get(), std::move(_3)));
                    }
                | arg tOROP arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.logical_op(node_type::OR, std::move(_1), _2.get(), std::move(_3)));
                    }
                | kDEFINED opt_nl arg
                    {
                      auto _3 = take(p, $3);
                      auto _1 = take(p, $1);
                      auto args = make_node_list(std::move(_3));

                      $$ = put(p, p.builder.keyword_cmd(node_type::DEFINED, _1.get(), nullptr, args.get(), nullptr));
                    }
                | arg tEH arg opt_nl tCOLON arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      $$ = put(p, p.builder.ternary(std::move(_1), _2.get(),
                                                std::move(_3), _5.get(), std::move(_6)));
                    }
                | primary

       arg_value: arg

       aref_args: list_none
                | args trailer
                | args tCOMMA assocs trailer
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.associate(nullptr, _3.get(), nullptr));
                      $$ = put(p, std::move(list));
                    }
                | assocs trailer
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list({ p.builder.associate(nullptr, _1.get(), nullptr) }));
                    }

         arg_rhs: arg %prec tOP_ASGN
                | arg kRESCUE_MOD arg
                    {
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _1 = take(p, $1);
                      auto rescue_body =
                        make_node_list(
                          p.builder.rescue_body(_2.get(),
                            nullptr, nullptr, nullptr,
                            nullptr, std::move(_3)));

                      $$ = put(p, p.builder.begin_body(std::move(_1), rescue_body.get(), nullptr, nullptr, nullptr, nullptr));
                    }

      paren_args: tLPAREN2 opt_call_args rparen
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, std::make_unique<delimited_node_list>(std::move(_1), std::move(_2), std::move(_3)));
                    }

  opt_paren_args: // nothing
                    {
                      $$ = put(p, std::make_unique<delimited_node_list>(nullptr, make_node_list(), nullptr));
                    }
                | paren_args

   opt_call_args: // nothing
                    {
                      $$ = put(p, make_node_list());
                    }
                | call_args
                | args tCOMMA
                | args tCOMMA assocs tCOMMA
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.associate(nullptr, _3.get(), nullptr));
                      $$ = put(p, std::move(list));
                    }
                | assocs tCOMMA
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list({
                          p.builder.associate(nullptr, _1.get(), nullptr) }));
                    }

       call_args: command
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | args opt_block_arg
                    {
                      auto _2 = take(p, $2);
                      auto args = take(p, $1);

                      concat_node_list(args, std::move(_2));

                      $$ = put(p, std::move(args));
                    }
                | assocs opt_block_arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto args = make_node_list({
                          p.builder.associate(nullptr, _1.get(), nullptr) });

                      concat_node_list(args, std::move(_2));

                      $$ = put(p, std::move(args));
                    }
                | args tCOMMA assocs opt_block_arg
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);

                      auto assocs = p.builder.associate(nullptr, _3.get(), nullptr);

                      args->nodes.push_back(std::move(assocs));

                      concat_node_list(args, std::move(_4));

                      $$ = put(p, std::move(args));
                    }
                | block_arg
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }

    command_args:   {
                      $<state_stack>$ = put_copy(p, p.lexer->cmdarg);
                      p.lexer->cmdarg.push(true);
                    }
                  call_args
                    {
                      p.lexer->cmdarg = *take(p, $<state_stack>1);

                      $$ = $2;
                    }

       block_arg: tAMPER arg_value
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.block_pass(_1.get(), std::move(_2)));
                    }

   opt_block_arg: tCOMMA block_arg
                    {
                      auto _2 = take(p, $2);
                      $$ = put(p, make_node_list(std::move(_2)));
                    }
                | // nothing
                    {
                      $$ = put(p, make_node_list());
                    }

            args: arg_value
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | tSTAR arg_value
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, make_node_list({
                          p.builder.splat(_1.get(), std::move(_2)) }));
                    }
                | args tCOMMA arg_value
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }
                | args tCOMMA tSTAR arg_value
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.splat(_3.get(), std::move(_4)));
                      $$ = put(p, std::move(list));
                    }

        mrhs_arg: mrhs
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.array(nullptr, _1.get(), nullptr));
                    }
                | arg_value

            mrhs: args tCOMMA arg_value
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }
                | args tCOMMA tSTAR arg_value
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.splat(_3.get(), std::move(_4)));
                      $$ = put(p, std::move(list));
                    }
                | tSTAR arg_value
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, make_node_list({
                          p.builder.splat(_1.get(), std::move(_2)) }));
                    }

         primary: literal
                | strings
                | xstring
                | regexp
                | words
                | qwords
                | symbols
                | qsymbols
                | var_ref
                | backref
                | tFID
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.call_method(nullptr, nullptr, _1.get(), nullptr, nullptr, nullptr));
                    }
                | kBEGIN
                    {
                      $<state_stack>$ = put_copy(p, p.lexer->cmdarg);
                      p.lexer->cmdarg.clear();
                    }
                    bodystmt kEND
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      p.lexer->cmdarg = *take(p, $<state_stack>2);

                      $$ = put(p, p.builder.begin_keyword(_1.get(), std::move(_3), _4.get()));
                    }
                | tLPAREN_ARG
                    {
                      $<state_stack>$ = put_copy(p, p.lexer->cmdarg);
                      p.lexer->cmdarg.clear();
                    }
                    stmt
                    {
                      p.lexer->set_state_expr_endarg();
                    }
                    rparen
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      p.lexer->cmdarg = *take(p, $<state_stack>2);

                      $$ = put(p, p.builder.begin(_1.get(), std::move(_3), _5.get()));
                    }
                | tLPAREN_ARG
                    {
                      p.lexer->set_state_expr_endarg();
                    }
                    opt_nl tRPAREN
                    {
                      auto _1 = take(p, $1);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.begin(_1.get(), nullptr, _4.get()));
                    }
                | tLPAREN compstmt tRPAREN
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.begin(_1.get(), std::move(_2), _3.get()));
                    }
                | tLPAREN expr tCOLON tr_type tRPAREN
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      $$ = put(p, p.builder.tr_cast(_1.get(), std::move(_2), _3.get(), std::move(_4), _5.get()));
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.const_fetch(std::move(_1), _2.get(), _3.get()));
                    }
                | tCOLON3 tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.const_global(_1.get(), _2.get()));
                    }
                | tLBRACK aref_args tRBRACK
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.array(_1.get(), _2.get(), _3.get()));
                    }
                | tLBRACE assoc_list tRCURLY
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.associate(_1.get(), _2.get(), _3.get()));
                    }
                | kRETURN
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.keyword_cmd(node_type::RETURN, _1.get(), nullptr, nullptr, nullptr));
                    }
                | kYIELD tLPAREN2 call_args rparen
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.keyword_cmd(node_type::YIELD, _1.get(), _2.get(), _3.get(), _4.get()));
                    }
                | kYIELD tLPAREN2 rparen
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto args = make_node_list();

                      $$ = put(p, p.builder.keyword_cmd(node_type::YIELD, _1.get(), _2.get(), args.get(), _3.get()));
                    }
                | kYIELD
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.keyword_cmd(node_type::YIELD, _1.get(), nullptr, nullptr, nullptr));
                    }
                | kDEFINED opt_nl tLPAREN2 expr rparen
                    {
                      auto _4 = take(p, $4);
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto args = make_node_list(std::move(_4));

                      $$ = put(p, p.builder.keyword_cmd(node_type::DEFINED, _1.get(),
                                                    _3.get(), args.get(), _5.get()));
                    }
                | kNOT tLPAREN2 expr rparen
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.not_op(_1.get(), _2.get(), std::move(_3), _4.get()));
                    }
                | kNOT tLPAREN2 rparen
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.not_op(_1.get(), _2.get(), nullptr, _3.get()));
                    }
                | fcall brace_block
                    {
                      auto _1 = take(p, $1);
                      auto method_call = p.builder.call_method(nullptr, nullptr, _1.get(), nullptr, nullptr, nullptr);

                      auto delimited_block = take(p, $2);

                      $$ = put(p, p.builder.block(std::move(method_call),
                        delimited_block->begin.get(),
                        std::move(delimited_block->args),
                        std::move(delimited_block->body),
                        delimited_block->end.get()));
                    }
                | method_call
                | method_call brace_block
                    {
                      auto _1 = take(p, $1);
                      auto delimited_block = take(p, $2);

                      $$ = put(p, p.builder.block(std::move(_1),
                        delimited_block->begin.get(),
                        std::move(delimited_block->args),
                        std::move(delimited_block->body),
                        delimited_block->end.get()));
                    }
                | tLAMBDA lambda
                    {
                      auto _1 = take(p, $1);
                      auto lambda_call = p.builder.call_lambda(_1.get());

                      auto lambda = take(p, $2);

                      $$ = put(p, p.builder.block(std::move(lambda_call),
                        lambda->begin.get(),
                        std::move(lambda->args),
                        std::move(lambda->body),
                        lambda->end.get()));
                    }
                | kIF expr_value then compstmt if_tail kEND
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _6 = take(p, $6);
                      auto else_ = take(p, $5);

                      $$ = put(p, p.builder.condition(
                        _1.get(), std::move(_2),
                        _3.get(), std::move(_4),
                        else_ ? else_->token_.get() : nullptr,
                        else_ ? std::move(else_->node_) : nullptr,
                        _6.get()));
                    }
                | kUNLESS expr_value then compstmt opt_else kEND
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _6 = take(p, $6);
                      auto else_ = take(p, $5);

                      $$ = put(p, p.builder.condition(
                        _1.get(), std::move(_2),
                        _3.get(),
                        else_ ? std::move(else_->node_) : nullptr,
                        else_ ? else_->token_.get() : nullptr,
                        std::move(_4),
                        _6.get()));
                    }
                | kWHILE
                    {
                      p.lexer->cond.push(true);
                    }
                    expr_value do
                    {
                      p.lexer->cond.pop();
                    }
                    compstmt kEND
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _6 = take(p, $6);
                      auto _7 = take(p, $7);
                      $$ = put(p, p.builder.loop(node_type::WHILE, _1.get(), std::move(_3), _4.get(),
                                             std::move(_6), _7.get()));
                    }
                | kUNTIL
                    {
                      p.lexer->cond.push(true);
                    }
                    expr_value do
                    {
                      p.lexer->cond.pop();
                    }
                    compstmt kEND
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _6 = take(p, $6);
                      auto _7 = take(p, $7);
                      $$ = put(p, p.builder.loop(node_type::UNTIL, _1.get(), std::move(_3), _4.get(),
                                             std::move(_6), _7.get()));
                    }
                | kCASE expr_value opt_terms case_body kEND
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _5 = take(p, $5);
                      auto case_body = take(p, $4);

                      auto else_ = std::move(case_body->else_);

                      $$ = put(p, p.builder.case_(_1.get(), std::move(_2),
                        case_body->whens.get(),
                        else_ ? else_->token_.get() : nullptr,
                        else_ ? std::move(else_->node_) : nullptr,
                        _5.get()));
                    }
                | kCASE            opt_terms case_body kEND
                    {
                      auto _1 = take(p, $1);
                      auto _4 = take(p, $4);
                      auto case_body = take(p, $3);

                      auto else_ = std::move(case_body->else_);

                      $$ = put(p, p.builder.case_(_1.get(), nullptr,
                        case_body->whens.get(),
                        else_ ? else_->token_.get() : nullptr,
                        else_ ? std::move(else_->node_) : nullptr,
                        _4.get()));
                    }
                | kFOR for_var kIN
                    {
                      p.lexer->cond.push(true);
                    }
                    expr_value do
                    {
                      p.lexer->cond.pop();
                    }
                    compstmt kEND
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      auto _8 = take(p, $8);
                      auto _9 = take(p, $9);
                      $$ = put(p, p.builder.for_(_1.get(), std::move(_2),
                                            _3.get(), std::move(_5),
                                            _6.get(), std::move(_8), _9.get()));
                    }
                | kCLASS cpath superclass
                    {
                      p.lexer->extend_static();
                      $<state_stack>$ = put_copy(p, p.lexer->cmdarg);
                    }
                    bodystmt kEND
                    {
                      auto _1 = take(p, $1);
                      auto _6 = take(p, $6);
                      if (p.def_level > 0) {
                        // TODO   diagnostic :error, :class_in_def, nullptr, std::move(_1)
                      }

                      auto superclass_ = take(p, $3);

                      auto lt_t       = superclass_ ? superclass_->token_.get() : nullptr;
                      auto superclass = superclass_ ? std::move(superclass_->node_) : nullptr;

                      $$ = put(p, p.builder.def_class(_1.get(), take(p, $2),
                                                  lt_t, std::move(superclass),
                                                  take(p, $5), _6.get()));

                      p.lexer->cmdarg = *take(p, $<state_stack>4);
                      p.lexer->unextend();
                    }
                | kCLASS tLSHFT expr term
                    {
                      $<size>$ = p.def_level;
                      p.def_level = 0;

                      p.lexer->extend_static();
                      $<state_stack>$ = put_copy(p, p.lexer->cmdarg);
                    }
                    bodystmt kEND
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _6 = take(p, $6);
                      auto _7 = take(p, $7);
                      $$ = put(p, p.builder.def_sclass(_1.get(), _2.get(), std::move(_3),
                                                   std::move(_6), _7.get()));

                      p.lexer->cmdarg = *take(p, $<state_stack>5);
                      p.lexer->unextend();

                      p.def_level = $<size>5;
                    }
                | kMODULE cpath
                    {
                      p.lexer->extend_static();
                      $<state_stack>$ = put_copy(p, p.lexer->cmdarg);
                    }
                    bodystmt kEND
                    {
                      auto _1 = take(p, $1);
                      auto _5 = take(p, $5);
                      if (p.def_level > 0) {
                        // TODO   diagnostic :error, :module_in_def, nullptr, std::move(_1)
                      }

                      $$ = put(p, p.builder.def_module(_1.get(), take(p, $2), take(p, $4), _5.get()));

                      p.lexer->cmdarg = *take(p, $<state_stack>3);
                      p.lexer->unextend();
                    }
                | kDEF fname
                    {
                      p.def_level++;
                      p.lexer->extend_static();
                      $<state_stack>$ = put_copy(p, p.lexer->cmdarg);
                    }
                    f_arglist bodystmt kEND
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      $$ = put(p, p.builder.def_method(_1.get(), _2.get(),
                                  std::move(_4), std::move(_5), _6.get()));

                      p.lexer->cmdarg = *take(p, $<state_stack>3);
                      p.lexer->unextend();
                      p.def_level--;
                    }
                | kDEF singleton dot_or_colon
                    {
                      p.lexer->set_state_expr_fname();
                    }
                    fname
                    {
                      p.def_level++;
                      p.lexer->extend_static();
                      $<state_stack>$ = put_copy(p, p.lexer->cmdarg);
                    }
                    f_arglist bodystmt kEND
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _7 = take(p, $7);
                      auto _8 = take(p, $8);
                      auto _9 = take(p, $9);
                      $$ = put(p, p.builder.def_singleton(_1.get(), std::move(_2), _3.get(),
                                  _5.get(), std::move(_7), std::move(_8), _9.get()));

                      p.lexer->cmdarg = *take(p, $<state_stack>6);
                      p.lexer->unextend();
                      p.def_level--;
                    }
                | kBREAK
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.keyword_cmd(node_type::BREAK, _1.get(), nullptr, nullptr, nullptr));
                    }
                | kNEXT
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.keyword_cmd(node_type::NEXT, _1.get(), nullptr, nullptr, nullptr));
                    }
                | kREDO
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.keyword_cmd(node_type::REDO, _1.get(), nullptr, nullptr, nullptr));
                    }
                | kRETRY
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.keyword_cmd(node_type::RETRY, _1.get(), nullptr, nullptr, nullptr));
                    }

   primary_value: primary

            then: term
                | kTHEN
                | term kTHEN
                    {
                      $$ = $2;
                    }

              do: term
                | kDO_COND

         if_tail: opt_else
                | kELSIF expr_value then compstmt if_tail
                    {
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto elsif_t = take(p, $1);

                      auto else_ = take(p, $5);

                      $$ = put(p, std::make_unique<node_with_token>(
                        std::move(elsif_t),
                        p.builder.condition(
                          elsif_t.get(), std::move(_2), _3.get(),
                          std::move(_4),
                          else_ ? else_->token_.get() : nullptr,
                          else_ ? std::move(else_->node_) : nullptr,
                          nullptr)));
                    }

        opt_else: none
                    {
                      $$ = nullptr;
                    }
                | kELSE compstmt
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, std::make_unique<node_with_token>(std::move(_1), std::move(_2)));
                    }

         for_var: lhs
                | mlhs

          f_marg: f_norm_arg
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.arg(_1.get()));
                    }
                | tLPAREN f_margs rparen
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.multi_lhs(_1.get(), _2.get(), _3.get()));
                    }

     f_marg_list: f_marg
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | f_marg_list tCOMMA f_marg
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }

         f_margs: f_marg_list
                | f_marg_list tCOMMA tSTAR f_norm_arg
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.restarg(_3.get(), _4.get()));
                      $$ = put(p, std::move(list));
                    }
                | f_marg_list tCOMMA tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _6 = take(p, $6);
                      auto args = take(p, $1);

                      args->nodes.push_back(p.builder.restarg(_3.get(), _4.get()));
                      concat_node_list(args, std::move(_6));

                      $$ = put(p, std::move(args));
                    }
                | f_marg_list tCOMMA tSTAR
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.restarg(_3.get(), nullptr));
                      $$ = put(p, std::move(list));
                    }
                | f_marg_list tCOMMA tSTAR            tCOMMA f_marg_list
                    {
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto args = take(p, $1);

                      args->nodes.push_back(p.builder.restarg(_3.get(), nullptr));
                      concat_node_list(args, std::move(_5));

                      $$ = put(p, std::move(args));
                    }
                |                    tSTAR f_norm_arg
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, make_node_list({
                          p.builder.restarg(_1.get(), _2.get()) }));
                    }
                |                    tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto args = take(p, $4);
                      args->nodes.insert(args->nodes.begin(), p.builder.restarg(_1.get(), _2.get()));
                      $$ = put(p, std::move(args));
                    }
                |                    tSTAR
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list({
                          p.builder.restarg(_1.get(), nullptr) }));
                    }
                |                    tSTAR tCOMMA f_marg_list
                    {
                      auto _1 = take(p, $1);
                      auto args = take(p, $3);
                      args->nodes.insert(args->nodes.begin(), p.builder.restarg(_1.get(), nullptr));
                      $$ = put(p, std::move(args));
                    }

 block_args_tail: f_block_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                      auto _3 = take(p, $3);
                      auto args = take(p, $1);

                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_3));

                      $$ = put(p, std::move(args));
                    }
                | f_block_kwarg opt_f_block_arg
                    {
                      auto _2 = take(p, $2);
                      auto args = take(p, $1);

                      concat_node_list(args, std::move(_2));

                      $$ = put(p, std::move(args));
                    }
                | f_kwrest opt_f_block_arg
                    {
                      auto _2 = take(p, $2);
                      auto args = take(p, $1);

                      concat_node_list(args, std::move(_2));

                      $$ = put(p, std::move(args));
                    }
                | f_block_arg
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, std::move(_1));
                    }

opt_block_args_tail:
                  tCOMMA block_args_tail
                    {
                      $$ = $2;
                    }
                | // nothing
                    {
                      $$ = put(p, make_node_list());
                    }

     block_param: f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg              opt_block_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_5));
                      concat_node_list(args, std::move(_6));
                      $$ = put(p, std::move(args));
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _7 = take(p, $7);
                      auto _8 = take(p, $8);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_5));
                      concat_node_list(args, std::move(_7));
                      concat_node_list(args, std::move(_8));
                      $$ = put(p, std::move(args));
                    }
                | f_arg tCOMMA f_block_optarg                                opt_block_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_4));
                      $$ = put(p, std::move(args));
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA                   f_arg opt_block_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_5));
                      concat_node_list(args, std::move(_6));
                      $$ = put(p, std::move(args));
                    }
                | f_arg tCOMMA                       f_rest_arg              opt_block_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_4));
                      $$ = put(p, std::move(args));
                    }
                | f_arg tCOMMA
                | f_arg tCOMMA                       f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_5));
                      concat_node_list(args, std::move(_6));
                      $$ = put(p, std::move(args));
                    }
                | f_arg                                                      opt_block_args_tail
                    {
                      auto args = take(p, $1);
                      auto block_args_tail = take(p, $2);

                      if (block_args_tail->nodes.size() == 0 && args->nodes.size() == 1) {
                        $$ = put(p, make_node_list(p.builder.procarg0(std::move(args->nodes[0]))));
                      } else {
                        concat_node_list(args, std::move(block_args_tail));
                        $$ = put(p, std::move(args));
                      }
                    }
                | f_block_optarg tCOMMA              f_rest_arg              opt_block_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_4));
                      $$ = put(p, std::move(args));
                    }
                | f_block_optarg tCOMMA              f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_5));
                      concat_node_list(args, std::move(_6));
                      $$ = put(p, std::move(args));
                    }
                | f_block_optarg                                             opt_block_args_tail
                    {
                      auto _2 = take(p, $2);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_2));
                      $$ = put(p, std::move(args));
                    }
                | f_block_optarg tCOMMA                                f_arg opt_block_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_4));
                      $$ = put(p, std::move(args));
                    }
                |                                    f_rest_arg              opt_block_args_tail
                    {
                      auto _2 = take(p, $2);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_2));
                      $$ = put(p, std::move(args));
                    }
                |                                    f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_4));
                      $$ = put(p, std::move(args));
                    }
                |                                                                block_args_tail

 opt_block_param: // nothing
                    {
                      $$ = put(p, p.builder.args(nullptr, nullptr, nullptr, true));
                    }
                | block_param_def
                    {
                      p.lexer->set_state_expr_value();
                    }
                  tr_returnsig
                    {
                      auto args = take(p, $1);
                      auto return_sig = take(p, $3);

                      if (return_sig) {
                        $$ = put(p, p.builder.prototype(nullptr, std::move(args), std::move(return_sig)));
                      } else {
                        $$ = put(p, std::move(args));
                      }
                    }

 block_param_def: tPIPE opt_bv_decl tPIPE
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.args(_1.get(), _2.get(), _3.get(), true));
                    }
                | tOROP
                    {
                      auto tok = take(p, $1);
                      $$ = put(p, p.builder.args(tok.get(), nullptr, tok.get(), true));
                    }
                | tPIPE block_param opt_bv_decl tPIPE
                    {
                      auto _3 = take(p, $3);
                      auto _1 = take(p, $1);
                      auto _4 = take(p, $4);
                      auto params = take(p, $2);
                      concat_node_list(params, std::move(_3));
                      $$ = put(p, p.builder.args(_1.get(), params.get(), _4.get(), true));
                    }

     opt_bv_decl: opt_nl
                    {
                      $$ = put(p, make_node_list());
                    }
                | opt_nl tSEMI bv_decls opt_nl
                    {
                      $$ = $3;
                    }

        bv_decls: bvar
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | bv_decls tCOMMA bvar
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }

            bvar: tIDENTIFIER
                    {
                      auto ident = take(p, $1);
                      p.lexer->declare(ident->string());
                      $$ = put(p, p.builder.shadowarg(ident.get()));
                    }
                | f_bad_arg
                    {
                      $$ = nullptr;
                    }

          lambda:   {
                      p.lexer->extend_dynamic();
                    }
                  f_larglist
                    {
                      $<state_stack>$ = put_copy(p, p.lexer->cmdarg);
                      p.lexer->cmdarg.clear();
                    }
                  lambda_body
                    {
                      p.lexer->cmdarg = *take(p, $<state_stack>3);
                      p.lexer->cmdarg.lexpop();

                      auto delimited_block = take(p, $4);

                      delimited_block->args = take(p, $2);

                      $$ = put(p, std::move(delimited_block));

                      p.lexer->unextend();
                    }

     f_larglist: tLPAREN2 f_args opt_bv_decl tRPAREN
                    {
                      auto _3 = take(p, $3);
                      auto _1 = take(p, $1);
                      auto _4 = take(p, $4);
                      auto args = take(p, $2);
                      concat_node_list(args, std::move(_3));
                      $$ = put(p, p.builder.args(_1.get(), args.get(), _4.get(), true));
                    }
                | f_args
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.args(nullptr, _1.get(), nullptr, true));
                    }

     lambda_body: tLAMBEG compstmt tRCURLY
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, std::make_unique<delimited_block>(std::move(_1), nullptr, std::move(_2), std::move(_3)));
                    }
                | kDO_LAMBDA compstmt kEND
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, std::make_unique<delimited_block>(std::move(_1), nullptr, std::move(_2), std::move(_3)));
                    }

        do_block: kDO_BLOCK do_body kEND
                    {
                      auto delimited_block = take(p, $2);
                      delimited_block->begin = take(p, $1);
                      delimited_block->end = take(p, $3);
                      $$ = put(p, std::move(delimited_block));
                    }

      block_call: command do_block
                    {
                      auto _1 = take(p, $1);
                      auto delimited_block = take(p, $2);

                      $$ = put(p, p.builder.block(std::move(_1),
                          delimited_block->begin.get(),
                          std::move(delimited_block->args),
                          std::move(delimited_block->body),
                          delimited_block->end.get()
                        ));
                    }
                | block_call dot_or_colon operation2 opt_paren_args
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto delimited = take(p, $4);

                      $$ = put(p, p.builder.call_method(std::move(_1), _2.get(), _3.get(),
                                  delimited->begin.get(),
                                  delimited->inner.get(),
                                  delimited->end.get()));
                    }
                | block_call dot_or_colon operation2 opt_paren_args brace_block
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto delimited = take(p, $4);

                      auto method_call =
                        p.builder.call_method(std::move(_1), _2.get(), _3.get(),
                          delimited->begin.get(),
                          delimited->inner.get(),
                          delimited->end.get());

                      auto block = take(p, $5);

                      $$ = put(p,
                        p.builder.block(std::move(method_call),
                          block->begin.get(),
                          std::move(block->args),
                          std::move(block->body),
                          block->end.get()));
                    }
                | block_call dot_or_colon operation2 command_args do_block
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto method_call =
                        p.builder.call_method(std::move(_1), _2.get(), _3.get(),
                          nullptr, _4.get(), nullptr);

                      auto block = take(p, $5);

                      $$ = put(p,
                        p.builder.block(std::move(method_call),
                          block->begin.get(),
                          std::move(block->args),
                          std::move(block->body),
                          block->end.get()));
                    }

     method_call: fcall paren_args
                    {
                      auto _1 = take(p, $1);
                      auto delimited = take(p, $2);

                      $$ = put(p, p.builder.call_method(nullptr, nullptr, _1.get(),
                        delimited->begin.get(),
                        delimited->inner.get(),
                        delimited->end.get()));
                    }
                | primary_value call_op operation2 opt_paren_args
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto delimited = take(p, $4);

                      $$ = put(p,
                        p.builder.call_method(std::move(_1), _2.get(), _3.get(),
                          delimited->begin.get(),
                          delimited->inner.get(),
                          delimited->end.get()));
                    }
                | primary_value tCOLON2 operation2 paren_args
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto delimited = take(p, $4);

                      $$ = put(p,
                        p.builder.call_method(std::move(_1), _2.get(), _3.get(),
                          delimited->begin.get(),
                          delimited->inner.get(),
                          delimited->end.get()));
                    }
                | primary_value tCOLON2 operation3
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.call_method(std::move(_1), _2.get(), _3.get(), nullptr, nullptr, nullptr));
                    }
                | primary_value call_op paren_args
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto delimited = take(p, $3);

                      $$ = put(p,
                        p.builder.call_method(std::move(_1), _2.get(), nullptr,
                          delimited->begin.get(),
                          delimited->inner.get(),
                          delimited->end.get()));
                    }
                | primary_value tCOLON2 paren_args
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto delimited = take(p, $3);

                      $$ = put(p,
                        p.builder.call_method(std::move(_1), _2.get(), nullptr,
                          delimited->begin.get(),
                          delimited->inner.get(),
                          delimited->end.get()));
                    }
                | kSUPER paren_args
                    {
                      auto _1 = take(p, $1);
                      auto delimited = take(p, $2);

                      $$ = put(p,
                        p.builder.keyword_cmd(node_type::SUPER, _1.get(),
                          delimited->begin.get(),
                          delimited->inner.get(),
                          delimited->end.get()));
                    }
                | kSUPER
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.keyword_cmd(node_type::ZSUPER, _1.get(), nullptr, nullptr, nullptr));
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.index(std::move(_1), _2.get(), _3.get(), _4.get()));
                    }

     brace_block: tLCURLY brace_body tRCURLY
                    {
                      auto block = take(p, $2);

                      block->begin = take(p, $1);
                      block->end = take(p, $3);

                      $$ = put(p, std::move(block));
                    }
                | kDO do_body kEND
                    {
                      auto block = take(p, $2);

                      block->begin = take(p, $1);
                      block->end = take(p, $3);

                      $$ = put(p, std::move(block));
                    }

      brace_body:   {
                      p.lexer->extend_dynamic();
                    }
                    {
                      $<state_stack>$ = put_copy(p, p.lexer->cmdarg);
                      p.lexer->cmdarg.clear();
                    }
                    opt_block_param compstmt
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, std::make_unique<delimited_block>(nullptr, std::move(_3), std::move(_4), nullptr));

                      p.lexer->unextend();
                      p.lexer->cmdarg = *take(p, $<state_stack>2);
                      p.lexer->cmdarg.pop();
                    }

         do_body:   {
                      p.lexer->extend_dynamic();
                    }
                    {
                      $<state_stack>$ = put_copy(p, p.lexer->cmdarg);
                      p.lexer->cmdarg.clear();
                    }
                    opt_block_param compstmt
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, std::make_unique<delimited_block>(nullptr, std::move(_3), std::move(_4), nullptr));

                      p.lexer->unextend();

                      p.lexer->cmdarg = *take(p, $<state_stack>2);
                      p.lexer->cmdarg.pop();
                    }

       case_body: kWHEN args then compstmt cases
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto cases = take(p, $5);
                      cases->whens->nodes.insert(cases->whens->nodes.begin(),
                        p.builder.when(_1.get(), _2.get(), _3.get(), std::move(_4)));
                      $$ = put(p, std::move(cases));
                    }

           cases: opt_else
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, std::make_unique<case_body>(std::move(_1)));
                    }
                | case_body

      opt_rescue: kRESCUE exc_list exc_var then compstmt opt_rescue
                    {
                      auto _1 = take(p, $1);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      auto exc_var = take(p, $3);

                      auto exc_list_ = take(p, $2);

                      auto exc_list = exc_list_
                        ? p.builder.array(nullptr, exc_list_.get(), nullptr)
                        : nullptr;

                      auto rescues = take(p, $6);

                      rescues->nodes.insert(rescues->nodes.begin(),
                        p.builder.rescue_body(_1.get(),
                          std::move(exc_list),
                          exc_var ? exc_var->token_.get() : nullptr,
                          exc_var ? std::move(exc_var->node_) : nullptr,
                          _4.get(), std::move(_5)));

                      $$ = put(p, std::move(rescues));
                    }
                |
                    {
                      $$ = put(p, make_node_list());
                    }

        exc_list: arg_value
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | mrhs
                | list_none

         exc_var: tASSOC lhs
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, std::make_unique<node_with_token>(std::move(_1), std::move(_2)));
                    }
                | // nothing
                    {
                      $$ = nullptr;
                    }

      opt_ensure: kENSURE compstmt
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, std::make_unique<node_with_token>(std::move(_1), std::move(_2)));
                    }
                | // nothing
                    {
                      $$ = nullptr;
                    }

         literal: numeric
                | symbol
                | dsym

         strings: string
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.string_compose(nullptr, _1.get(), nullptr));
                    }

          string: string1
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | string string1
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_2));
                      $$ = put(p, std::move(list));
                    }

         string1: tSTRING_BEG string_contents tSTRING_END
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto str = p.builder.string_compose(_1.get(), _2.get(), _3.get());
                      $$ = put(p, p.builder.dedent_string(std::move(str), p.lexer->dedent_level() || 0));
                    }
                | tSTRING
                    {
                      auto _1 = take(p, $1);
                      auto str = p.builder.string(_1.get());
                      $$ = put(p, p.builder.dedent_string(std::move(str), p.lexer->dedent_level() || 0));
                    }
                | tCHARACTER
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.character(_1.get()));
                    }

         xstring: tXSTRING_BEG xstring_contents tSTRING_END
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto xstr = p.builder.xstring_compose(_1.get(), _2.get(), _3.get());
                      $$ = put(p, p.builder.dedent_string(std::move(xstr), p.lexer->dedent_level() || 0));
                    }

          regexp: tREGEXP_BEG regexp_contents tSTRING_END tREGEXP_OPT
                    {
                      auto _4 = take(p, $4);
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto opts = p.builder.regexp_options(_4.get());
                      $$ = put(p, p.builder.regexp_compose(_1.get(), _2.get(), _3.get(), std::move(opts)));
                    }

           words: tWORDS_BEG word_list tSTRING_END
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.words_compose(_1.get(), _2.get(), _3.get()));
                    }

       word_list: // nothing
                    {
                      $$ = put(p, make_node_list());
                    }
                | word_list word tSPACE
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.word(_2.get()));
                      $$ = put(p, std::move(list));
                    }

            word: string_content
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | word string_content
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_2));
                      $$ = put(p, std::move(list));
                    }

         symbols: tSYMBOLS_BEG symbol_list tSTRING_END
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.symbols_compose(_1.get(), _2.get(), _3.get()));
                    }

     symbol_list: // nothing
                    {
                      $$ = put(p, make_node_list());
                    }
                | symbol_list word tSPACE
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.word(_2.get()));
                      $$ = put(p, std::move(list));
                    }

          qwords: tQWORDS_BEG qword_list tSTRING_END
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.words_compose(_1.get(), _2.get(), _3.get()));
                    }

        qsymbols: tQSYMBOLS_BEG qsym_list tSTRING_END
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.symbols_compose(_1.get(), _2.get(), _3.get()));
                    }

      qword_list: // nothing
                    {
                      $$ = put(p, make_node_list());
                    }
                | qword_list tSTRING_CONTENT tSPACE
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.string_internal(_2.get()));
                      $$ = put(p, std::move(list));
                    }

       qsym_list: // nothing
                    {
                      $$ = put(p, make_node_list());
                    }
                | qsym_list tSTRING_CONTENT tSPACE
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.symbol_internal(_2.get()));
                      $$ = put(p, std::move(list));
                    }

 string_contents: // nothing
                    {
                      $$ = put(p, make_node_list());
                    }
                | string_contents string_content
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_2));
                      $$ = put(p, std::move(list));
                    }

xstring_contents: // nothing
                    {
                      $$ = put(p, make_node_list());
                    }
                | xstring_contents string_content
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_2));
                      $$ = put(p, std::move(list));
                    }

regexp_contents: // nothing
                    {
                      $$ = put(p, make_node_list());
                    }
                | regexp_contents string_content
                    {
                      auto _2 = take(p, $2);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_2));
                      $$ = put(p, std::move(list));
                    }

  string_content: tSTRING_CONTENT
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.string_internal(_1.get()));
                    }
                | tSTRING_DVAR string_dvar
                    {
                      $$ = $2;
                    }
                | tSTRING_DBEG
                    {
                      p.lexer->cond.push(false);
                      p.lexer->cmdarg.push(false);
                    }
                    compstmt tSTRING_DEND
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      p.lexer->cond.lexpop();
                      p.lexer->cmdarg.lexpop();

                      $$ = put(p, p.builder.begin(_1.get(), std::move(_3), _4.get()));
                    }

     string_dvar: tGVAR
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.gvar(_1.get()));
                    }
                | tIVAR
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.ivar(_1.get()));
                    }
                | tCVAR
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.cvar(_1.get()));
                    }
                | backref


          symbol: tSYMBOL
                    {
                      auto _1 = take(p, $1);
                      p.lexer->set_state_expr_endarg();
                      $$ = put(p, p.builder.symbol(_1.get()));
                    }

            dsym: tSYMBEG xstring_contents tSTRING_END
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      p.lexer->set_state_expr_endarg();
                      $$ = put(p, p.builder.symbol_compose(_1.get(), _2.get(), _3.get()));
                    }

         numeric: simple_numeric
                    {
                      $$ = $1;
                    }
                | tUMINUS_NUM simple_numeric %prec tLOWEST
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.negate(_1.get(), std::move(_2)));
                    }

  simple_numeric: tINTEGER
                    {
                      auto _1 = take(p, $1);
                      p.lexer->set_state_expr_endarg();
                      $$ = put(p, p.builder.integer(_1.get()));
                    }
                | tFLOAT
                    {
                      auto _1 = take(p, $1);
                      p.lexer->set_state_expr_endarg();
                      $$ = put(p, p.builder.float_(_1.get()));
                    }
                | tRATIONAL
                    {
                      auto _1 = take(p, $1);
                      p.lexer->set_state_expr_endarg();
                      $$ = put(p, p.builder.rational(_1.get()));
                    }
                | tIMAGINARY
                    {
                      auto _1 = take(p, $1);
                      p.lexer->set_state_expr_endarg();
                      $$ = put(p, p.builder.complex(_1.get()));
                    }
                | tRATIONAL_IMAGINARY
                    {
                      auto _1 = take(p, $1);
                      p.lexer->set_state_expr_endarg();
                      $$ = put(p, p.builder.rational_complex(_1.get()));
                    }
                | tFLOAT_IMAGINARY
                    {
                      auto _1 = take(p, $1);
                      p.lexer->set_state_expr_endarg();
                      $$ = put(p, p.builder.float_complex(_1.get()));
                    }

   user_variable: tIDENTIFIER
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.ident(_1.get()));
                    }
                | tIVAR
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.ivar(_1.get()));
                    }
                | tGVAR
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.gvar(_1.get()));
                    }
                | tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.const_(_1.get()));
                    }
                | tCVAR
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.cvar(_1.get()));
                    }

keyword_variable: kNIL
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.nil(_1.get()));
                    }
                | kSELF
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.self(_1.get()));
                    }
                | kTRUE
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.true_(_1.get()));
                    }
                | kFALSE
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.false_(_1.get()));
                    }
                | k__FILE__
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.file_literal(_1.get()));
                    }
                | k__LINE__
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.line_literal(_1.get()));
                    }
                | k__ENCODING__
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.encoding_literal(_1.get()));
                    }

         var_ref: user_variable
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.accessible(std::move(_1)));
                    }
                | keyword_variable
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.accessible(std::move(_1)));
                    }

         var_lhs: user_variable
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.assignable(std::move(_1)));
                    }
                | keyword_variable
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.assignable(std::move(_1)));
                    }

         backref: tNTH_REF
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.nth_ref(_1.get()));
                    }
                | tBACK_REF
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.back_ref(_1.get()));
                    }

      superclass: tLT
                    {
                      p.lexer->set_state_expr_value();
                    }
                    expr_value term
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      $$ = put(p, std::make_unique<node_with_token>(std::move(_1), std::move(_3)));
                    }
                | // nothing
                    {
                      $$ = nullptr;
                    }

tr_methodgenargs: tLBRACK2 tr_gendeclargs rbracket
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.tr_genargs(_1.get(), _2.get(), _3.get()));
                    }
                | // nothing
                    {
                      $$ = nullptr;
                    }

       f_arglist: tr_methodgenargs tLPAREN2 f_args rparen
                    {
                      p.lexer->set_state_expr_value();
                    }
                  tr_returnsig
                    {
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto genargs = take(p, $1);
                      auto args = p.builder.args(_2.get(), _3.get(), _4.get(), true);
                      auto returnsig = take(p, $6);

                      if (genargs || returnsig) {
                        $$ = put(p, p.builder.prototype(
                          std::move(genargs),
                          std::move(args),
                          std::move(returnsig)));
                      } else {
                        $$ = put(p, std::move(args));
                      }
                    }
                | tr_methodgenargs
                    {
                      $<boolean>$ = p.lexer->in_kwarg;
                      p.lexer->in_kwarg = true;
                    }
                  f_args tr_returnsig term
                    {
                      auto _3 = take(p, $3);
                      p.lexer->in_kwarg = $<boolean>2;

                      auto genargs = take(p, $1);
                      auto args = p.builder.args(nullptr, _3.get(), nullptr, true);
                      auto returnsig = take(p, $4);

                      if (genargs || returnsig) {
                        $$ = put(p, p.builder.prototype(
                          std::move(genargs),
                          std::move(args),
                          std::move(returnsig)));
                      } else {
                        $$ = put(p, std::move(args));
                      }
                    }

       args_tail: f_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_4));
                      $$ = put(p, std::move(args));
                    }
                | f_kwarg opt_f_block_arg
                    {
                      auto _2 = take(p, $2);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_2));
                      $$ = put(p, std::move(args));
                    }
                | f_kwrest opt_f_block_arg
                    {
                      auto _2 = take(p, $2);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_2));
                      $$ = put(p, std::move(args));
                    }
                | f_block_arg
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, std::move(_1));
                    }

   opt_args_tail: tCOMMA args_tail
                    {
                      $$ = $2;
                    }
                | // nothing
                    {
                      $$ = put(p, make_node_list());
                    }

          f_args: f_arg tCOMMA f_optarg tCOMMA f_rest_arg              opt_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_5));
                      concat_node_list(args, std::move(_6));
                      $$ = put(p, std::move(args));
                    }
                | f_arg tCOMMA f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _7 = take(p, $7);
                      auto _8 = take(p, $8);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_5));
                      concat_node_list(args, std::move(_7));
                      concat_node_list(args, std::move(_8));
                      $$ = put(p, std::move(args));
                    }
                | f_arg tCOMMA f_optarg                                opt_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_4));
                      $$ = put(p, std::move(args));
                    }
                | f_arg tCOMMA f_optarg tCOMMA                   f_arg opt_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_5));
                      concat_node_list(args, std::move(_6));
                      $$ = put(p, std::move(args));
                    }
                | f_arg tCOMMA                 f_rest_arg              opt_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_4));
                      $$ = put(p, std::move(args));
                    }
                | f_arg tCOMMA                 f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_5));
                      concat_node_list(args, std::move(_6));
                      $$ = put(p, std::move(args));
                    }
                | f_arg                                                opt_args_tail
                    {
                      auto _2 = take(p, $2);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_2));
                      $$ = put(p, std::move(args));
                    }
                |              f_optarg tCOMMA f_rest_arg              opt_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_4));
                      $$ = put(p, std::move(args));
                    }
                |              f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _5 = take(p, $5);
                      auto _6 = take(p, $6);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_5));
                      concat_node_list(args, std::move(_6));
                      $$ = put(p, std::move(args));
                    }
                |              f_optarg                                opt_args_tail
                    {
                      auto _2 = take(p, $2);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_2));
                      $$ = put(p, std::move(args));
                    }
                |              f_optarg tCOMMA                   f_arg opt_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_4));
                      $$ = put(p, std::move(args));
                    }
                |                              f_rest_arg              opt_args_tail
                    {
                      auto _2 = take(p, $2);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_2));
                      $$ = put(p, std::move(args));
                    }
                |                              f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto args = take(p, $1);
                      concat_node_list(args, std::move(_3));
                      concat_node_list(args, std::move(_4));
                      $$ = put(p, std::move(args));
                    }
                |                                                          args_tail
                    {
                      $$ = $1;
                    }
                | // nothing
                    {
                      $$ = put(p, make_node_list());
                    }

       f_bad_arg: tIVAR
                    {
                      auto _1 = take(p, $1);
                      // TODO diagnostic :error, :argument_ivar, nullptr, std::move(_1)
                    }
                | tGVAR
                    {
                      auto _1 = take(p, $1);
                      // TODO diagnostic :error, :argument_gvar, nullptr, std::move(_1)
                    }
                | tCVAR
                    {
                      auto _1 = take(p, $1);
                      // TODO diagnostic :error, :argument_cvar, nullptr, std::move(_1)
                    }

      f_norm_arg: f_bad_arg
                | tIDENTIFIER
                    {
                      auto ident = take(p, $1);

                      p.lexer->declare(ident->string());

                      $$ = put(p, std::move(ident));
                    }

      f_arg_asgn: f_norm_arg
                    {
                      $$ = $1;
                    }

      f_arg_item: tr_argsig f_arg_asgn
                    {
                      auto _2 = take(p, $2);
                      auto argsig = take(p, $1);
                      auto arg = p.builder.arg(_2.get());

                      if (argsig) {
                        $$ = put(p, p.builder.typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(p, std::move(arg));
                      }
                    }
                | tLPAREN f_margs rparen
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.multi_lhs(_1.get(), _2.get(), _3.get()));
                    }

           f_arg: f_arg_item
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | f_arg tCOMMA f_arg_item
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }

         f_label: tLABEL
                    {
                      auto label = take(p, $1);

                      p.check_kwarg_name(label);

                      p.lexer->declare(label->string());

                      $$ = put(p, std::move(label));
                    }

            f_kw: tr_argsig f_label arg_value
                    {
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto argsig = take(p, $1);
                      auto arg = p.builder.kwoptarg(_2.get(), std::move(_3));

                      if (argsig) {
                        $$ = put(p, p.builder.typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(p, std::move(arg));
                      }
                    }
                | tr_argsig f_label
                    {
                      auto _2 = take(p, $2);
                      auto argsig = take(p, $1);
                      auto arg = p.builder.kwarg(_2.get());

                      if (argsig) {
                        $$ = put(p, p.builder.typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(p, std::move(arg));
                      }
                    }

      f_block_kw: tr_argsig f_label primary_value
                    {
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto argsig = take(p, $1);
                      auto arg = p.builder.kwoptarg(_2.get(), std::move(_3));

                      if (argsig) {
                        $$ = put(p, p.builder.typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(p, std::move(arg));
                      }
                    }
                | tr_argsig f_label
                    {
                      auto _2 = take(p, $2);
                      auto argsig = take(p, $1);
                      auto arg = p.builder.kwarg(_2.get());

                      if (argsig) {
                        $$ = put(p, p.builder.typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(p, std::move(arg));
                      }
                    }

   f_block_kwarg: f_block_kw
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | f_block_kwarg tCOMMA f_block_kw
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }

         f_kwarg: f_kw
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | f_kwarg tCOMMA f_kw
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }

     kwrest_mark: tPOW | tDSTAR

        f_kwrest: kwrest_mark tIDENTIFIER
                    {
                      auto _1 = take(p, $1);
                      auto ident = take(p, $2);

                      p.lexer->declare(ident->string());

                      $$ = put(p, make_node_list({ p.builder.kwrestarg(_1.get(), ident.get()) }));
                    }
                | kwrest_mark
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(p.builder.kwrestarg(_1.get(), nullptr)));
                    }

           f_opt: tr_argsig f_arg_asgn tEQL arg_value
                    {
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto argsig = take(p, $1);
                      auto arg = p.builder.optarg(_2.get(), _3.get(), std::move(_4));

                      if (argsig) {
                        $$ = put(p, p.builder.typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(p, std::move(arg));
                      }
                    }

     f_block_opt: tr_argsig f_arg_asgn tEQL primary_value
                    {
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto argsig = take(p, $1);
                      auto arg = p.builder.optarg(_2.get(), _3.get(), std::move(_4));

                      if (argsig) {
                        $$ = put(p, p.builder.typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(p, std::move(arg));
                      }
                    }

  f_block_optarg: f_block_opt
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | f_block_optarg tCOMMA f_block_opt
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }

        f_optarg: f_opt
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | f_optarg tCOMMA f_opt
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }

    restarg_mark: tSTAR2 | tSTAR

      f_rest_arg: tr_argsig restarg_mark tIDENTIFIER
                    {
                      auto _2 = take(p, $2);
                      auto argsig = take(p, $1);
                      auto ident = take(p, $3);

                      p.lexer->declare(ident->string());

                      auto restarg = p.builder.restarg(_2.get(), ident.get());

                      if (argsig) {
                        restarg = p.builder.typed_arg(std::move(argsig), std::move(restarg));
                      }

                      $$ = put(p, make_node_list(std::move(restarg)));
                    }
                | tr_argsig restarg_mark
                    {
                      auto _2 = take(p, $2);
                      auto argsig = take(p, $1);
                      auto restarg = p.builder.restarg(_2.get(), nullptr);

                      if (restarg) {
                        restarg = p.builder.typed_arg(std::move(argsig), std::move(restarg));
                      }

                      $$ = put(p, make_node_list(std::move(restarg)));
                    }

     blkarg_mark: tAMPER2 | tAMPER

     f_block_arg: tr_argsig blkarg_mark tIDENTIFIER
                    {
                      auto _2 = take(p, $2);
                      auto argsig = take(p, $1);
                      auto ident = take(p, $3);

                      p.lexer->declare(ident->string());

                      auto blockarg = p.builder.blockarg(_2.get(), ident.get());

                      if (blockarg) {
                        blockarg = p.builder.typed_arg(std::move(argsig), std::move(blockarg));
                      }

                      $$ = put(p, make_node_list(std::move(blockarg)));
                    }
                | tr_argsig blkarg_mark
                    {
                      auto _2 = take(p, $2);
                      auto argsig = take(p, $1);
                      auto blockarg = p.builder.blockarg(_2.get(), nullptr);

                      if (blockarg) {
                        blockarg = p.builder.typed_arg(std::move(argsig), std::move(blockarg));
                      }

                      $$ = put(p, make_node_list(std::move(blockarg)));
                    }

 opt_f_block_arg: tCOMMA f_block_arg
                    {
                      $$ = $2;
                    }
                |
                    {
                      $$ = put(p, make_node_list());
                    }

       singleton: var_ref
                | tLPAREN2 expr rparen
                    {
                      $$ = $2;
                    }

      assoc_list: // nothing
                    {
                      $$ = put(p, make_node_list());
                    }
                | assocs trailer

          assocs: assoc
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }
                | assocs tCOMMA assoc
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }

           assoc: arg_value tASSOC arg_value
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.pair(std::move(_1), _2.get(), std::move(_3)));
                    }
                | tLABEL arg_value
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.pair_keyword(_1.get(), std::move(_2)));
                    }
                | tSTRING_BEG string_contents tLABEL_END arg_value
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      $$ = put(p, p.builder.pair_quoted(_1.get(), _2.get(), _3.get(), std::move(_4)));
                    }
                | tDSTAR arg_value
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.kwsplat(_1.get(), std::move(_2)));
                    }

       operation: tIDENTIFIER | tCONSTANT | tFID
      operation2: tIDENTIFIER | tCONSTANT | tFID | op
      operation3: tIDENTIFIER | tFID | op
    dot_or_colon: call_op | tCOLON2
         call_op: tDOT
                    {
                      // what is this???
                      // $$ = put(p, [:dot, $1[1]]
                      $$ = $1;
                    }
                | tANDDOT
                    {
                      // what is this???
                      // $$ = [:anddot, $1[1]]
                      $$ = $1;
                    }
       opt_terms:  | terms
          opt_nl:  | tNL
          rparen: opt_nl tRPAREN
                    {
                      $$ = $2;
                    }
        rbracket: opt_nl tRBRACK
                    {
                      $$ = $2;
                    }
         trailer:  | tNL | tCOMMA

            term: tSEMI
                  {
                    yyerrok;
                  }
                | tNL

           terms: term
                | terms tSEMI

            none: // nothing
                  {
                    $$ = nullptr;
                  }

       list_none: // nothing
                  {
                    $$ = nullptr;
                  }

        tr_cpath: tCOLON3 tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.const_global(_1.get(), _2.get()));
                    }
                | tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.const_(_1.get()));
                    }
                | tr_cpath tCOLON2 tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.const_fetch(std::move(_1), _2.get(), _3.get()));
                    }

       tr_types: tr_types tCOMMA tr_type
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(std::move(_3));
                      $$ = put(p, std::move(list));
                    }
               | tr_type
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(std::move(_1)));
                    }

         tr_type: tr_cpath
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.tr_cpath(std::move(_1)));
                    }
                | tr_cpath tCOLON2 tLBRACK2 tr_types rbracket
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      $$ = put(p, p.builder.tr_geninst(std::move(_1), _3.get(), _4.get(), _5.get()));
                    }
                | tLBRACK tr_type rbracket
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.tr_array(_1.get(), std::move(_2), _3.get()));
                    }
                | tLBRACK tr_type tCOMMA tr_types rbracket
                    {
                      auto _2 = take(p, $2);
                      auto _1 = take(p, $1);
                      auto _5 = take(p, $5);
                      auto types = take(p, $4);

                      types->nodes.insert(types->nodes.begin(), std::move(_2));

                      $$ = put(p, p.builder.tr_tuple(_1.get(), types.get(), _5.get()));
                    }
                | tLBRACE tr_type tASSOC tr_type tRCURLY
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      auto _3 = take(p, $3);
                      auto _4 = take(p, $4);
                      auto _5 = take(p, $5);
                      $$ = put(p, p.builder.tr_hash(_1.get(), std::move(_2), _3.get(), std::move(_4), _5.get()));
                    }
                | tLBRACE tr_blockproto tr_returnsig tRCURLY
                    {
                      auto _1 = take(p, $1);
                      auto _4 = take(p, $4);
                      auto blockproto = take(p, $2);
                      auto returnsig = take(p, $3);

                      auto prototype = returnsig
                        ? p.builder.prototype(nullptr, std::move(blockproto), std::move(returnsig))
                        : std::move(blockproto);

                      $$ = put(p, p.builder.tr_proc(_1.get(), std::move(prototype), _4.get()));
                    }
                | tTILDE tr_type
                    {
                      auto _1 = take(p, $1);
                      auto _2 = take(p, $2);
                      $$ = put(p, p.builder.tr_nillable(_1.get(), std::move(_2)));
                    }
                | kNIL
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.tr_nil(_1.get()));
                    }
                | tSYMBOL
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, p.builder.tr_special(_1.get()));
                      // diagnostic :error, :bad_special_type, { value: std::move(_1)[0] }, std::move(_1)
                    }
                | tLPAREN tr_union_type rparen
                    {
                      $$ = $2;
                    }

   tr_union_type: tr_union_type tPIPE tr_type
                    {
                      auto _1 = take(p, $1);
                      auto _3 = take(p, $3);
                      $$ = put(p, p.builder.tr_or(std::move(_1), std::move(_3)));
                    }
                | tr_type

       tr_argsig: tr_type
                    {
                      $$ = $1;
                      p.lexer->set_state_expr_beg();
                    }
                |
                    {
                      $$ = nullptr;
                    }

    tr_returnsig: tASSOC tr_type
                    {
                      $$ = $2;
                    }
                |
                    {
                      $$ = nullptr;
                    }

  tr_gendeclargs: tr_gendeclargs tCOMMA tCONSTANT
                    {
                      auto _3 = take(p, $3);
                      auto list = take(p, $1);
                      list->nodes.push_back(p.builder.tr_gendeclarg(_3.get()));
                      $$ = put(p, std::move(list));
                    }
                | tCONSTANT
                    {
                      auto _1 = take(p, $1);
                      $$ = put(p, make_node_list(p.builder.tr_gendeclarg(_1.get())));
                    }

   tr_blockproto: { p.lexer->extend_dynamic(); }
                  block_param_def
                    {
                      p.lexer->unextend();
                      $$ = $2;
                    }

%%
