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
%}

%pure-parser

%lex-param { parser::typedruby24& p }
%parse-param { parser::typedruby24& p }

%union {
  token_ptr* token;
  node_delimited_list_ptr* delimited_list;
  node_delimited_block_ptr* delimited_block;
  node_with_token_ptr* with_token;
  node_ptr* node;
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
  case_body
  cases
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
  static std::unique_ptr<T> take(std::unique_ptr<T>* raw_ptr) {
    auto ptr = std::move(*raw_ptr);
    delete raw_ptr;
    return ptr;
  }

  template<typename T>
  static std::unique_ptr<T>* put(std::unique_ptr<T> ptr) {
    return new std::unique_ptr<T>(ptr.release());
  }

  template<typename T>
  static std::unique_ptr<T>* put_copy(T obj) {
    return put(std::make_unique<T>(obj));
  }

  template<typename To, typename From>
  static std::unique_ptr<To> static_unique_cast(std::unique_ptr<From> from) {
    return std::unique_ptr<To> { static_cast<To*>(from.release()) };
  }

  static node_list_ptr make_node_list() {
    return std::make_unique<node_list>(std::vector<node_ptr>());
  }

  static node_list_ptr make_node_list(node_ptr&& node) {
    std::vector<node_ptr> vec;
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

    lval->token = put(std::move(token));

    return token_type;
  }
%}

%%
         program: top_compstmt

    top_compstmt: top_stmts opt_terms
                    {
                      $$ = put(builder::compstmt(take($1)));
                    }

       top_stmts: // nothing
                    {
                      $$ = put(make_node_list());
                    }
                | top_stmt
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | top_stmts terms top_stmt
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = $1;
                    }
                | error top_stmt
                    {
                      $$ = put(make_node_list(take($2)));
                    }

        top_stmt: stmt
                | klBEGIN tLCURLY top_compstmt tRCURLY
                    {
                      $$ = put(builder::preexe(take($3)));
                    }

        bodystmt: compstmt opt_rescue opt_else opt_ensure
                    {
                      auto rescue_bodies = take($2);
                      auto else_ = take($3);

                      auto ensure = take($4);

                      if (rescue_bodies->nodes.size() == 0 && else_ != nullptr) {
                        // TODO diagnostic :warning, :useless_else, nullptr, else_t
                      }

                      $$ = put(builder::begin_body(take($1),
                            std::move(rescue_bodies),
                            std::move(else_->token_), std::move(else_->node_),
                            std::move(ensure->token_), std::move(ensure->node_)));
                    }

        compstmt: stmts opt_terms
                    {
                      $$ = put(builder::compstmt(take($1)));
                    }

           stmts: // nothing
                    {
                      $$ = put(make_node_list());
                    }
                | stmt_or_begin
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | stmts terms stmt_or_begin
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }
                | error stmt
                    {
                      $$ = put(make_node_list(take($2)));
                    }

   stmt_or_begin: stmt
                | klBEGIN tLCURLY top_compstmt tRCURLY
                    {
                      /* TODO diagnostic :error, :begin_in_method, nullptr, take($1) */
                    }

            stmt: kALIAS fitem
                    {
                      p.lexer->set_state_expr_fname();
                    }
                    fitem
                    {
                      $$ = put(builder::alias(take($1), take($2), take($4)));
                    }
                | kALIAS tGVAR tGVAR
                    {
                      $$ = put(builder::alias(take($1),
                        builder::gvar(take($2)),
                        builder::gvar(take($3))));
                    }
                | kALIAS tGVAR tBACK_REF
                    {
                      $$ = put(builder::alias(take($1),
                        builder::gvar(take($2)),
                        builder::back_ref(take($3))));
                    }
                | kALIAS tGVAR tNTH_REF
                    {
                      // TODO diagnostic :error, :nth_ref_alias, nullptr, take($3)
                    }
                | kUNDEF undef_list
                    {
                      $$ = put(builder::undef_method(take($2)));
                    }
                | stmt kIF_MOD expr_value
                    {
                      $$ = put(builder::condition_mod(take($1), nullptr, take($3)));
                    }
                | stmt kUNLESS_MOD expr_value
                    {
                      $$ = put(builder::condition_mod(nullptr, take($1), take($3)));
                    }
                | stmt kWHILE_MOD expr_value
                    {
                      $$ = put(builder::loop_mod(node_type::WHILE, take($1), take($3)));
                    }
                | stmt kUNTIL_MOD expr_value
                    {
                      $$ = put(builder::loop_mod(node_type::UNTIL, take($1), take($3)));
                    }
                | stmt kRESCUE_MOD stmt
                    {
                      auto rescue_body = builder::rescue_body(take($2), nullptr, nullptr, nullptr, nullptr, take($3));

                      $$ = put(builder::begin_body(
                        take($1),
                        make_node_list(std::move(rescue_body)),
                        nullptr, nullptr));
                    }
                | klEND tLCURLY compstmt tRCURLY
                    {
                      $$ = put(builder::postexe(take($3)));
                    }
                | command_asgn
                | mlhs tEQL command_call
                    {
                      $$ = put(builder::multi_assign(take($1), take($3)));
                    }
                | lhs tEQL mrhs
                    {
                      $$ = put(builder::assign(take($1), take($2), builder::array(nullptr, take($3), nullptr)));
                    }
                | mlhs tEQL mrhs_arg
                    {
                      $$ = put(builder::multi_assign(take($1), take($3)));
                    }
                | kDEF tIVAR tCOLON tr_type
                    {
                      $$ = put(builder::tr_ivardecl(take($2), take($4)));
                    }
                | expr

    command_asgn: lhs tEQL command_rhs
                    {
                      $$ = put(builder::assign(take($1), take($2), take($3)));
                    }
                | var_lhs tOP_ASGN command_rhs
                    {
                      $$ = put(builder::op_assign(take($1), take($2), take($3)));
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN command_rhs
                    {
                      $$ = put(builder::op_assign(
                                  builder::index(
                                    take($1), take($2), take($3), take($4)),
                                  take($5), take($6)));
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN command_rhs
                    {
                      $$ = put(builder::op_assign(
                                  builder::call_method(
                                    take($1), take($2), take($3)),
                                  take($4), take($5)));
                    }
                | primary_value call_op tCONSTANT tOP_ASGN command_rhs
                    {
                      $$ = put(builder::op_assign(
                                  builder::call_method(
                                    take($1), take($2), take($3)),
                                  take($4), take($5)));
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN command_rhs
                    {
                      auto const_node = builder::const_op_assignable(
                                  builder::const_fetch(take($1), take($2), take($3)));
                      $$ = put(builder::op_assign(std::move(const_node), take($4), take($5)));
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN command_rhs
                    {
                      $$ = put(builder::op_assign(
                                  builder::call_method(
                                    take($1), take($2), take($3)),
                                  take($4), take($5)));
                    }
                | backref tOP_ASGN command_rhs
                    {
                      builder::op_assign(take($1), take($2), take($3));
                    }

     command_rhs: command_call %prec tOP_ASGN
                | command_call kRESCUE_MOD stmt
                    {
                      auto rescue_body =
                        builder::rescue_body(take($2),
                                        nullptr, nullptr, nullptr,
                                        nullptr, take($3));

                      auto rescue_bodies = make_node_list(std::move(rescue_body));

                      $$ = put(builder::begin_body(take($1), std::move(rescue_bodies)));
                    }
                | command_asgn

            expr: command_call
                | expr kAND expr
                    {
                      $$ = put(builder::logical_op(node_type::AND, take($1), take($2), take($3)));
                    }
                | expr kOR expr
                    {
                      $$ = put(builder::logical_op(node_type::OR, take($1), take($2), take($3)));
                    }
                | kNOT opt_nl expr
                    {
                      $$ = put(builder::not_op(take($1), nullptr, take($3), nullptr));
                    }
                | tBANG command_call
                    {
                      $$ = put(builder::not_op(take($1), nullptr, take($2), nullptr));
                    }
                | arg

      expr_value: expr

    command_call: command
                | block_command

   block_command: block_call
                | block_call dot_or_colon operation2 command_args
                    {
                      $$ = put(builder::call_method(take($1), take($2), take($3),
                                  nullptr, take($4), nullptr));
                    }

 cmd_brace_block: tLBRACE_ARG brace_body tRCURLY
                    {
                      auto block = take($2);
                      block->begin = take($1);
                      block->end = take($3);
                      $$ = put(std::move(block));
                    }

           fcall: operation

         command: fcall command_args %prec tLOWEST
                    {
                      $$ = put(builder::call_method(nullptr, nullptr, take($1),
                                  nullptr, take($2), nullptr));
                    }
                | fcall command_args cmd_brace_block
                    {
                      auto method_call = builder::call_method(nullptr, nullptr, take($1),
                                                              nullptr, take($2), nullptr);

                      auto delimited_block = take($3);

                      $$ = put(builder::block(std::move(method_call),
                                      std::move(delimited_block->begin),
                                      std::move(delimited_block->args),
                                      std::move(delimited_block->body),
                                      std::move(delimited_block->end)));
                    }
                | primary_value call_op operation2 command_args %prec tLOWEST
                    {
                      $$ = put(builder::call_method(take($1), take($2), take($3),
                                  nullptr, take($4), nullptr));
                    }
                | primary_value call_op operation2 command_args cmd_brace_block
                    {
                      auto method_call = builder::call_method(take($1), take($2), take($3),
                                        nullptr, take($4), nullptr);

                      auto delimited_block = take($5);

                      $$ = put(builder::block(std::move(method_call),
                                      std::move(delimited_block->begin),
                                      std::move(delimited_block->args),
                                      std::move(delimited_block->body),
                                      std::move(delimited_block->end)));
                    }
                | primary_value tCOLON2 operation2 command_args %prec tLOWEST
                    {
                      $$ = put(builder::call_method(take($1), take($2), take($3),
                                  nullptr, take($4), nullptr));
                    }
                | primary_value tCOLON2 operation2 command_args cmd_brace_block
                    {
                      auto method_call = builder::call_method(take($1), take($2), take($3),
                                        nullptr, take($4), nullptr);

                      auto delimited_block = take($5);

                      $$ = put(builder::block(std::move(method_call),
                                      std::move(delimited_block->begin),
                                      std::move(delimited_block->args),
                                      std::move(delimited_block->body),
                                      std::move(delimited_block->end)));
                    }
                | kSUPER command_args
                    {
                      $$ = put(builder::keyword_cmd(node_type::SUPER, take($1),
                                  nullptr, take($2), nullptr));
                    }
                | kYIELD command_args
                    {
                      $$ = put(builder::keyword_cmd(node_type::YIELD, take($1),
                                  nullptr, take($2), nullptr));
                    }
                | kRETURN call_args
                    {
                      $$ = put(builder::keyword_cmd(node_type::RETURN, take($1),
                                  nullptr, take($2), nullptr));
                    }
                | kBREAK call_args
                    {
                      $$ = put(builder::keyword_cmd(node_type::BREAK, take($1),
                                  nullptr, take($2), nullptr));
                    }
                | kNEXT call_args
                    {
                      $$ = put(builder::keyword_cmd(node_type::NEXT, take($1),
                                  nullptr, take($2), nullptr));
                    }

            mlhs: mlhs_basic
                    {
                      $$ = put(builder::multi_lhs(nullptr, take($1), nullptr));
                    }
                | tLPAREN mlhs_inner rparen
                    {
                      $$ = put(builder::begin(take($1), take($2), take($3)));
                    }

      mlhs_inner: mlhs_basic
                    {
                      $$ = put(builder::multi_lhs(nullptr, take($1), nullptr));
                    }
                | tLPAREN mlhs_inner rparen
                    {
                      auto inner = make_node_list(take($2));
                      $$ = put(builder::multi_lhs(take($1), std::move(inner), take($3)));
                    }

      mlhs_basic: mlhs_head
                | mlhs_head mlhs_item
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($2));
                      $$ = put(std::move(list));
                    }
                | mlhs_head tSTAR mlhs_node
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::splat(take($2), take($3)));
                      $$ = $1;
                    }
                | mlhs_head tSTAR mlhs_node tCOMMA mlhs_post
                    {
                      auto head = take($1);

                      head->nodes.push_back(builder::splat(take($2), take($3)));
                      concat_node_list(head, take($5));

                      $$ = put(std::move(head));
                    }
                | mlhs_head tSTAR
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::splat(take($2)));
                      $$ = put(std::move(list));
                    }
                | mlhs_head tSTAR tCOMMA mlhs_post
                    {
                      auto head = take($1);

                      head->nodes.push_back(builder::splat(take($2)));
                      concat_node_list(head, take($4));

                      $$ = put(std::move(head));
                    }
                | tSTAR mlhs_node
                    {
                      $$ = put(make_node_list({ builder::splat(take($1), take($2)) }));
                    }
                | tSTAR mlhs_node tCOMMA mlhs_post
                    {
                      auto items = make_node_list({ builder::splat(take($1), take($2)) });

                      concat_node_list(items, take($4));

                      $$ = put(std::move(items));
                    }
                | tSTAR
                    {
                      $$ = put(make_node_list(builder::splat(take($1))));
                    }
                | tSTAR tCOMMA mlhs_post
                    {
                      auto items = make_node_list(builder::splat(take($1)));

                      concat_node_list(items, take($3));

                      $$ = put(std::move(items));
                    }

       mlhs_item: mlhs_node
                | tLPAREN mlhs_inner rparen
                    {
                      $$ = put(builder::begin(take($1), take($2), take($3)));
                    }

       mlhs_head: mlhs_item tCOMMA
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | mlhs_head mlhs_item tCOMMA
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($2));
                      $$ = put(std::move(list));
                    }

       mlhs_post: mlhs_item
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | mlhs_post tCOMMA mlhs_item
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }

       mlhs_node: user_variable
                    {
                      $$ = put(builder::assignable(take($1)));
                    }
                | keyword_variable
                    {
                      $$ = put(builder::assignable(take($1)));
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      $$ = put(builder::index_asgn(take($1), take($2), take($3), take($4)));
                    }
                | primary_value call_op tIDENTIFIER
                    {
                      $$ = put(builder::attr_asgn(take($1), take($2), take($3)));
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                      $$ = put(builder::attr_asgn(take($1), take($2), take($3)));
                    }
                | primary_value call_op tCONSTANT
                    {
                      $$ = put(builder::attr_asgn(take($1), take($2), take($3)));
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = put(builder::assignable(
                                  builder::const_fetch(take($1), take($2), take($3))));
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = put(builder::assignable(
                                  builder::const_global(take($1), take($2))));
                    }
                | backref
                    {
                      $$ = put(builder::assignable(take($1)));
                    }

             lhs: user_variable
                    {
                      $$ = put(builder::assignable(take($1)));
                    }
                | keyword_variable
                    {
                      $$ = put(builder::assignable(take($1)));
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      $$ = put(builder::index_asgn(take($1), take($2), take($3), take($4)));
                    }
                | primary_value call_op tIDENTIFIER
                    {
                      $$ = put(builder::attr_asgn(take($1), take($2), take($3)));
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                      $$ = put(builder::attr_asgn(take($1), take($2), take($3)));
                    }
                | primary_value call_op tCONSTANT
                    {
                      $$ = put(builder::attr_asgn(take($1), take($2), take($3)));
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = put(builder::assignable(
                                  builder::const_fetch(take($1), take($2), take($3))));
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = put(builder::assignable(
                                  builder::const_global(take($1), take($2))));
                    }
                | backref
                    {
                      $$ = put(builder::assignable(take($1)));
                    }

           cname: tIDENTIFIER
                    {
                      // TODO diagnostic :error, :module_name_const, nullptr, take($1)
                    }
                | tCONSTANT

           cpath: tCOLON3 cname
                    {
                      $$ = put(builder::const_global(take($1), take($2)));
                    }
                | cname
                    {
                      $$ = put(builder::const_(take($1)));
                    }
                | primary_value tCOLON2 tLBRACK2 tr_gendeclargs rbracket
                    {
                      $$ = put(builder::tr_gendecl(take($1), take($3), take($4), take($5)));
                    }
                | primary_value tCOLON2 cname
                    {
                      $$ = put(builder::const_fetch(take($1), take($2), take($3)));
                    }

           fname: tIDENTIFIER | tCONSTANT | tFID
                | op
                | reswords

            fsym: fname
                    {
                      $$ = put(builder::symbol(take($1)));
                    }
                | symbol

           fitem: fsym
                | dsym

      undef_list: fitem
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | undef_list tCOMMA
                    {
                      p.lexer->set_state_expr_fname();
                    }
                    fitem
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($4));
                      $$ = put(std::move(list));
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
                      $$ = put(builder::assign(take($1), take($2), take($3)));
                    }
                | var_lhs tOP_ASGN arg_rhs
                    {
                      $$ = put(builder::op_assign(take($1), take($2), take($3)));
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs
                    {
                      $$ = put(builder::op_assign(
                                  builder::index(
                                    take($1), take($2), take($3), take($4)),
                                  take($5), take($6)));
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN arg_rhs
                    {
                      $$ = put(builder::op_assign(
                                  builder::call_method(
                                    take($1), take($2), take($3)),
                                  take($4), take($5)));
                    }
                | primary_value call_op tCONSTANT tOP_ASGN arg_rhs
                    {
                      $$ = put(builder::op_assign(
                                  builder::call_method(
                                    take($1), take($2), take($3)),
                                  take($4), take($5)));
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN arg_rhs
                    {
                      $$ = put(builder::op_assign(
                                  builder::call_method(
                                    take($1), take($2), take($3)),
                                  take($4), take($5)));
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN arg_rhs
                    {
                      auto const_ = builder::const_op_assignable(
                                      builder::const_fetch(take($1), take($2), take($3)));

                      $$ = put(builder::op_assign(std::move(const_), take($4), take($5)));
                    }
                | tCOLON3 tCONSTANT tOP_ASGN arg_rhs
                    {
                      auto const_ = builder::const_op_assignable(
                                  builder::const_global(take($1), take($2)));

                      $$ = put(builder::op_assign(std::move(const_), take($3), take($4)));
                    }
                | backref tOP_ASGN arg_rhs
                    {
                      $$ = put(builder::op_assign(take($1), take($2), take($3)));
                    }
                | arg tDOT2 arg
                    {
                      $$ = put(builder::range_inclusive(take($1), take($2), take($3)));
                    }
                | arg tDOT3 arg
                    {
                      $$ = put(builder::range_exclusive(take($1), take($2), take($3)));
                    }
                | arg tPLUS arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tMINUS arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tSTAR2 arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tDIVIDE arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tPERCENT arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tPOW arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | tUMINUS_NUM simple_numeric tPOW arg
                    {
                      $$ = put(builder::unary_op(take($1),
                                  builder::binary_op(
                                    take($2), take($3), take($4))));
                    }
                | tUPLUS arg
                    {
                      $$ = put(builder::unary_op(take($1), take($2)));
                    }
                | tUMINUS arg
                    {
                      $$ = put(builder::unary_op(take($1), take($2)));
                    }
                | arg tPIPE arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tCARET arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tAMPER2 arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tCMP arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tGT arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tGEQ arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tLT arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tLEQ arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tEQ arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tEQQ arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tNEQ arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tMATCH arg
                    {
                      $$ = put(builder::match_op(take($1), take($2), take($3)));
                    }
                | arg tNMATCH arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | tBANG arg
                    {
                      $$ = put(builder::not_op(take($1), nullptr, take($2), nullptr));
                    }
                | tTILDE arg
                    {
                      $$ = put(builder::unary_op(take($1), take($2)));
                    }
                | arg tLSHFT arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tRSHFT arg
                    {
                      $$ = put(builder::binary_op(take($1), take($2), take($3)));
                    }
                | arg tANDOP arg
                    {
                      $$ = put(builder::logical_op(node_type::AND, take($1), take($2), take($3)));
                    }
                | arg tOROP arg
                    {
                      $$ = put(builder::logical_op(node_type::OR, take($1), take($2), take($3)));
                    }
                | kDEFINED opt_nl arg
                    {
                      auto args = make_node_list(take($3));

                      $$ = put(builder::keyword_cmd(node_type::DEFINED, take($1), nullptr, std::move(args), nullptr));
                    }
                | arg tEH arg opt_nl tCOLON arg
                    {
                      $$ = put(builder::ternary(take($1), take($2),
                                                take($3), take($5), take($6)));
                    }
                | primary

       arg_value: arg

       aref_args: list_none
                | args trailer
                | args tCOMMA assocs trailer
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::associate(nullptr, take($3), nullptr));
                      $$ = put(std::move(list));
                    }
                | assocs trailer
                    {
                      $$ = put(make_node_list({ builder::associate(nullptr, take($1), nullptr) }));
                    }

         arg_rhs: arg %prec tOP_ASGN
                | arg kRESCUE_MOD arg
                    {
                      auto rescue_body = builder::rescue_body(take($2),
                                          nullptr, nullptr, nullptr,
                                          nullptr, take($3));

                      auto rescue_bodies = make_node_list(std::move(rescue_body));

                      $$ = put(builder::begin_body(take($1), std::move(rescue_bodies)));
                    }

      paren_args: tLPAREN2 opt_call_args rparen
                    {
                      $$ = put(std::make_unique<node_delimited_list>(take($1), take($2), take($3)));
                    }

  opt_paren_args: // nothing
                    {
                      $$ = put(std::make_unique<node_delimited_list>(nullptr, make_node_list(), nullptr));
                    }
                | paren_args

   opt_call_args: // nothing
                    {
                      $$ = put(make_node_list());
                    }
                | call_args
                | args tCOMMA
                | args tCOMMA assocs tCOMMA
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::associate(nullptr, take($3), nullptr));
                      $$ = put(std::move(list));
                    }
                | assocs tCOMMA
                    {
                      $$ = put(make_node_list({
                          builder::associate(nullptr, take($1), nullptr) }));
                    }

       call_args: command
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | args opt_block_arg
                    {
                      auto args = take($1);

                      concat_node_list(args, take($2));

                      $$ = put(std::move(args));
                    }
                | assocs opt_block_arg
                    {
                      auto args = make_node_list({
                          builder::associate(nullptr, take($1), nullptr) });

                      concat_node_list(args, take($2));

                      $$ = put(std::move(args));
                    }
                | args tCOMMA assocs opt_block_arg
                    {
                      auto args = take($1);

                      auto assocs = builder::associate(nullptr, take($3), nullptr);

                      args->nodes.push_back(std::move(assocs));

                      concat_node_list(args, take($4));

                      $$ = put(std::move(args));
                    }
                | block_arg
                    {
                      $$ = put(make_node_list(take($1)));
                    }

    command_args:   {
                      $<state_stack>$ = put_copy(p.lexer->cmdarg);
                      p.lexer->cmdarg.push(true);
                    }
                  call_args
                    {
                      p.lexer->cmdarg = *take($<state_stack>1);

                      $$ = $2;
                    }

       block_arg: tAMPER arg_value
                    {
                      $$ = put(builder::block_pass(take($1), take($2)));
                    }

   opt_block_arg: tCOMMA block_arg
                    {
                      $$ = put(make_node_list(take($2)));
                    }
                | // nothing
                    {
                      $$ = put(make_node_list());
                    }

            args: arg_value
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | tSTAR arg_value
                    {
                      $$ = put(make_node_list({
                          builder::splat(take($1), take($2)) }));
                    }
                | args tCOMMA arg_value
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }
                | args tCOMMA tSTAR arg_value
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::splat(take($3), take($4)));
                      $$ = $1;
                    }

        mrhs_arg: mrhs
                    {
                      $$ = put(builder::array(nullptr, take($1), nullptr));
                    }
                | arg_value

            mrhs: args tCOMMA arg_value
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }
                | args tCOMMA tSTAR arg_value
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::splat(take($3), take($4)));
                      $$ = $1;
                    }
                | tSTAR arg_value
                    {
                      $$ = put(make_node_list({
                          builder::splat(take($1), take($2)) }));
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
                      $$ = put(builder::call_method(nullptr, nullptr, take($1)));
                    }
                | kBEGIN
                    {
                      $<state_stack>$ = put_copy(p.lexer->cmdarg);
                      p.lexer->cmdarg.clear();
                    }
                    bodystmt kEND
                    {
                      p.lexer->cmdarg = *take($<state_stack>2);

                      $$ = put(builder::begin_keyword(take($1), take($3), take($4)));
                    }
                | tLPAREN_ARG
                    {
                      $<state_stack>$ = put_copy(p.lexer->cmdarg);
                      p.lexer->cmdarg.clear();
                    }
                    stmt
                    {
                      p.lexer->set_state_expr_endarg();
                    }
                    rparen
                    {
                      p.lexer->cmdarg = *take($<state_stack>2);

                      $$ = put(builder::begin(take($1), take($3), take($5)));
                    }
                | tLPAREN_ARG
                    {
                      p.lexer->set_state_expr_endarg();
                    }
                    opt_nl tRPAREN
                    {
                      $$ = put(builder::begin(take($1), nullptr, take($4)));
                    }
                | tLPAREN compstmt tRPAREN
                    {
                      $$ = put(builder::begin(take($1), take($2), take($3)));
                    }
                | tLPAREN expr tCOLON tr_type tRPAREN
                    {
                      $$ = put(builder::tr_cast(take($1), take($2), take($3), take($4), take($5)));
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = put(builder::const_fetch(take($1), take($2), take($3)));
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = put(builder::const_global(take($1), take($2)));
                    }
                | tLBRACK aref_args tRBRACK
                    {
                      $$ = put(builder::array(take($1), take($2), take($3)));
                    }
                | tLBRACE assoc_list tRCURLY
                    {
                      $$ = put(builder::associate(take($1), take($2), take($3)));
                    }
                | kRETURN
                    {
                      $$ = put(builder::keyword_cmd(node_type::RETURN, take($1)));
                    }
                | kYIELD tLPAREN2 call_args rparen
                    {
                      $$ = put(builder::keyword_cmd(node_type::YIELD, take($1), take($2), take($3), take($4)));
                    }
                | kYIELD tLPAREN2 rparen
                    {
                      auto args = make_node_list();

                      $$ = put(builder::keyword_cmd(node_type::YIELD, take($1), take($2), std::move(args), take($3)));
                    }
                | kYIELD
                    {
                      $$ = put(builder::keyword_cmd(node_type::YIELD, take($1)));
                    }
                | kDEFINED opt_nl tLPAREN2 expr rparen
                    {
                      auto args = make_node_list(take($4));

                      $$ = put(builder::keyword_cmd(node_type::DEFINED, take($1),
                                                    take($3), std::move(args), take($5)));
                    }
                | kNOT tLPAREN2 expr rparen
                    {
                      $$ = put(builder::not_op(take($1), take($2), take($3), take($4)));
                    }
                | kNOT tLPAREN2 rparen
                    {
                      $$ = put(builder::not_op(take($1), take($2), nullptr, take($3)));
                    }
                | fcall brace_block
                    {
                      auto method_call = builder::call_method(nullptr, nullptr, take($1));

                      auto delimited_block = take($2);

                      $$ = put(builder::block(std::move(method_call),
                        std::move(delimited_block->begin),
                        std::move(delimited_block->args),
                        std::move(delimited_block->body),
                        std::move(delimited_block->end)));
                    }
                | method_call
                | method_call brace_block
                    {
                      auto delimited_block = take($2);

                      $$ = put(builder::block(take($1),
                        std::move(delimited_block->begin),
                        std::move(delimited_block->args),
                        std::move(delimited_block->body),
                        std::move(delimited_block->end)));
                    }
                | tLAMBDA lambda
                    {
                      auto lambda_call = builder::call_lambda(take($1));

                      auto lambda = take($2);

                      $$ = put(builder::block(std::move(lambda_call),
                        std::move(lambda->begin),
                        std::move(lambda->args),
                        std::move(lambda->body),
                        std::move(lambda->end)));
                    }
                | kIF expr_value then compstmt if_tail kEND
                    {
                      auto else_ = take($5);

                      $$ = put(builder::condition(
                        take($1), take($2),
                        take($3), take($4),
                        std::move(else_->token_), std::move(else_->node_),
                        take($6)));
                    }
                | kUNLESS expr_value then compstmt opt_else kEND
                    {
                      auto else_ = take($5);

                      $$ = put(builder::condition(
                        take($1), take($2),
                        take($3), std::move(else_->node_),
                        std::move(else_->token_), take($4),
                        take($6)));
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
                      $$ = put(builder::loop(node_type::WHILE, take($1), take($3), take($4),
                                             take($6), take($7)));
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
                      $$ = put(builder::loop(node_type::UNTIL, take($1), take($3), take($4),
                                             take($6), take($7)));
                    }
                | kCASE expr_value opt_terms case_body kEND
                    {
                      auto case_body = take($4);

                      auto else_ = static_unique_cast<node_with_token>(std::move(case_body->nodes.back()));
                      case_body->nodes.pop_back();

                      $$ = put(builder::case_(take($1), take($2),
                        std::move(case_body),
                        std::move(else_->token_), std::move(else_->node_),
                        take($5)));
                    }
                | kCASE            opt_terms case_body kEND
                    {
                      auto case_body = take($3);

                      auto else_ = static_unique_cast<node_with_token>(std::move(case_body->nodes.back()));
                      case_body->nodes.pop_back();

                      $$ = put(builder::case_(take($1), nullptr,
                        std::move(case_body),
                        std::move(else_->token_), std::move(else_->node_),
                        take($4)));
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
                      $$ = put(builder::for_(take($1), take($2),
                                            take($3), take($5),
                                            take($6), take($8), take($9)));
                    }
                | kCLASS cpath superclass
                    {
                      p.lexer->extend_static();
                      $<state_stack>$ = put_copy(p.lexer->cmdarg);
                    }
                    bodystmt kEND
                    {
                      if (p.def_level > 0) {
                        // TODO   diagnostic :error, :class_in_def, nullptr, take($1)
                      }

                      auto superclass_ = take($3);

                      auto lt_t       = superclass_ ? std::move(superclass_->token_) : nullptr;
                      auto superclass = superclass_ ? std::move(superclass_->node_)  : nullptr;

                      $$ = put(builder::def_class(take($1), take($2),
                                                  std::move(lt_t), std::move(superclass),
                                                  take($5), take($6)));

                      p.lexer->cmdarg = *take($<state_stack>4);
                      p.lexer->unextend();
                    }
                | kCLASS tLSHFT expr term
                    {
                      $<size>$ = p.def_level;
                      p.def_level = 0;

                      p.lexer->extend_static();
                      $<state_stack>$ = put_copy(p.lexer->cmdarg);
                    }
                    bodystmt kEND
                    {
                      $$ = put(builder::def_sclass(take($1), take($2), take($3),
                                                   take($6), take($7)));

                      p.lexer->cmdarg = *take($<state_stack>5);
                      p.lexer->unextend();

                      p.def_level = $<size>5;
                    }
                | kMODULE cpath
                    {
                      p.lexer->extend_static();
                      $<state_stack>$ = put_copy(p.lexer->cmdarg);
                    }
                    bodystmt kEND
                    {
                      if (p.def_level > 0) {
                        // TODO   diagnostic :error, :module_in_def, nullptr, take($1)
                      }

                      $$ = put(builder::def_module(take($1), take($2), take($4), take($5)));

                      p.lexer->cmdarg = *take($<state_stack>3);
                      p.lexer->unextend();
                    }
                | kDEF fname
                    {
                      p.def_level++;
                      p.lexer->extend_static();
                      $<state_stack>$ = put_copy(p.lexer->cmdarg);
                    }
                    f_arglist bodystmt kEND
                    {
                      $$ = put(builder::def_method(take($1), take($2),
                                  take($4), take($5), take($6)));

                      p.lexer->cmdarg = *take($<state_stack>3);
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
                      $<state_stack>$ = put_copy(p.lexer->cmdarg);
                    }
                    f_arglist bodystmt kEND
                    {
                      $$ = put(builder::def_singleton(take($1), take($2), take($3),
                                  take($5), take($7), take($8), take($9)));

                      p.lexer->cmdarg = *take($<state_stack>6);
                      p.lexer->unextend();
                      p.def_level--;
                    }
                | kBREAK
                    {
                      $$ = put(builder::keyword_cmd(node_type::BREAK, take($1)));
                    }
                | kNEXT
                    {
                      $$ = put(builder::keyword_cmd(node_type::NEXT, take($1)));
                    }
                | kREDO
                    {
                      $$ = put(builder::keyword_cmd(node_type::REDO, take($1)));
                    }
                | kRETRY
                    {
                      $$ = put(builder::keyword_cmd(node_type::RETRY, take($1)));
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
                      auto elsif_t = take($1);

                      auto else_ = take($5);

                      $$ = put(std::make_unique<node_with_token>(
                        std::make_unique<token>(*elsif_t),
                        builder::condition(
                          std::make_unique<token>(*elsif_t), take($2), take($3),
                          take($4), std::move(else_->token_), std::move(else_->node_),
                          nullptr)));
                    }

        opt_else: none
                    {
                      $$ = nullptr;
                    }
                | kELSE compstmt
                    {
                      $$ = put(std::make_unique<node_with_token>(take($1), take($2)));
                    }

         for_var: lhs
                | mlhs

          f_marg: f_norm_arg
                    {
                      $$ = put(builder::arg(take($1)));
                    }
                | tLPAREN f_margs rparen
                    {
                      $$ = put(builder::multi_lhs(take($1), take($2), take($3)));
                    }

     f_marg_list: f_marg
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | f_marg_list tCOMMA f_marg
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }

         f_margs: f_marg_list
                | f_marg_list tCOMMA tSTAR f_norm_arg
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::restarg(take($3), take($4)));
                      $$ = $1;
                    }
                | f_marg_list tCOMMA tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                      auto args = take($1);

                      args->nodes.push_back(builder::restarg(take($3), take($4)));
                      concat_node_list(args, take($6));

                      $$ = put(std::move(args));
                    }
                | f_marg_list tCOMMA tSTAR
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::restarg(take($3)));
                      $$ = put(std::move(list));
                    }
                | f_marg_list tCOMMA tSTAR            tCOMMA f_marg_list
                    {
                      auto args = take($1);

                      args->nodes.push_back(builder::restarg(take($3)));
                      concat_node_list(args, take($5));

                      $$ = put(std::move(args));
                    }
                |                    tSTAR f_norm_arg
                    {
                      $$ = put(make_node_list({
                          builder::restarg(take($1), take($2)) }));
                    }
                |                    tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                      auto args = take($4);
                      args->nodes.insert(args->nodes.begin(), builder::restarg(take($1), take($2)));
                      $$ = put(std::move(args));
                    }
                |                    tSTAR
                    {
                      $$ = put(make_node_list({
                          builder::restarg(take($1)) }));
                    }
                |                    tSTAR tCOMMA f_marg_list
                    {
                      auto args = take($3);
                      args->nodes.insert(args->nodes.begin(), builder::restarg(take($1)));
                      $$ = put(std::move(args));
                    }

 block_args_tail: f_block_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                      auto args = take($1);

                      concat_node_list(args, take($3));
                      concat_node_list(args, take($3));

                      $$ = put(std::move(args));
                    }
                | f_block_kwarg opt_f_block_arg
                    {
                      auto args = take($1);

                      concat_node_list(args, take($2));

                      $$ = put(std::move(args));
                    }
                | f_kwrest opt_f_block_arg
                    {
                      auto args = take($1);

                      concat_node_list(args, take($2));

                      $$ = put(std::move(args));
                    }
                | f_block_arg
                    {
                      $$ = put(make_node_list(take($1)));
                    }

opt_block_args_tail:
                  tCOMMA block_args_tail
                    {
                      $$ = $2;
                    }
                | // nothing
                    {
                      $$ = put(make_node_list());
                    }

     block_param: f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg              opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($5));
                      concat_node_list(args, take($6));
                      $$ = put(std::move(args));
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($5));
                      concat_node_list(args, take($7));
                      concat_node_list(args, take($8));
                      $$ = put(std::move(args));
                    }
                | f_arg tCOMMA f_block_optarg                                opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($4));
                      $$ = put(std::move(args));
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA                   f_arg opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($5));
                      concat_node_list(args, take($6));
                      $$ = put(std::move(args));
                    }
                | f_arg tCOMMA                       f_rest_arg              opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($4));
                      $$ = put(std::move(args));
                    }
                | f_arg tCOMMA
                | f_arg tCOMMA                       f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($5));
                      concat_node_list(args, take($6));
                      $$ = put(std::move(args));
                    }
                | f_arg                                                      opt_block_args_tail
                    {
                      auto args = take($1);
                      auto block_args_tail = take($2);

                      if (block_args_tail->nodes.size() == 0 && args->nodes.size() == 1) {
                        $$ = put(make_node_list(builder::procarg0(std::move(args->nodes[0]))));
                      } else {
                        concat_node_list(args, std::move(block_args_tail));
                        $$ = put(std::move(args));
                      }
                    }
                | f_block_optarg tCOMMA              f_rest_arg              opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($4));
                      $$ = put(std::move(args));
                    }
                | f_block_optarg tCOMMA              f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($5));
                      concat_node_list(args, take($6));
                      $$ = put(std::move(args));
                    }
                | f_block_optarg                                             opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($2));
                      $$ = put(std::move(args));
                    }
                | f_block_optarg tCOMMA                                f_arg opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($4));
                      $$ = put(std::move(args));
                    }
                |                                    f_rest_arg              opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($2));
                      $$ = put(std::move(args));
                    }
                |                                    f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($4));
                      $$ = put(std::move(args));
                    }
                |                                                                block_args_tail

 opt_block_param: // nothing
                    {
                      $$ = put(builder::args(nullptr, make_node_list(), nullptr));
                    }
                | block_param_def
                    {
                      p.lexer->set_state_expr_value();
                    }
                  tr_returnsig
                    {
                      auto args = take($1);
                      auto return_sig = take($3);

                      if (return_sig) {
                        $$ = put(builder::prototype(nullptr, std::move(args), std::move(return_sig)));
                      } else {
                        $$ = put(std::move(args));
                      }
                    }

 block_param_def: tPIPE opt_bv_decl tPIPE
                    {
                      $$ = put(builder::args(take($1), take($2), take($3)));
                    }
                | tOROP
                    {
                      auto tok = take($1);
                      $$ = put(builder::args(std::make_unique<token>(*tok), make_node_list(), std::make_unique<token>(*tok)));
                    }
                | tPIPE block_param opt_bv_decl tPIPE
                    {
                      auto params = take($2);
                      concat_node_list(params, take($3));
                      $$ = put(builder::args(take($1), std::move(params), take($4)));
                    }

     opt_bv_decl: opt_nl
                    {
                      $$ = put(make_node_list());
                    }
                | opt_nl tSEMI bv_decls opt_nl
                    {
                      $$ = $3;
                    }

        bv_decls: bvar
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | bv_decls tCOMMA bvar
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }

            bvar: tIDENTIFIER
                    {
                      auto ident = take($1);
                      p.lexer->declare(ident->string());
                      $$ = put(builder::shadowarg(std::move(ident)));
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
                      $<state_stack>$ = put_copy(p.lexer->cmdarg);
                      p.lexer->cmdarg.clear();
                    }
                  lambda_body
                    {
                      p.lexer->cmdarg = *take($<state_stack>3);
                      p.lexer->cmdarg.lexpop();

                      auto delimited_block = take($4);

                      delimited_block->args = take($2);

                      $$ = put(std::move(delimited_block));

                      p.lexer->unextend();
                    }

     f_larglist: tLPAREN2 f_args opt_bv_decl tRPAREN
                    {
                      auto args = take($2);
                      concat_node_list(args, take($3));
                      $$ = put(builder::args(take($1), std::move(args), take($4)));
                    }
                | f_args
                    {
                      $$ = put(builder::args(nullptr, take($1), nullptr));
                    }

     lambda_body: tLAMBEG compstmt tRCURLY
                    {
                      $$ = put(std::make_unique<node_delimited_block>(take($1), nullptr, take($2), take($3)));
                    }
                | kDO_LAMBDA compstmt kEND
                    {
                      $$ = put(std::make_unique<node_delimited_block>(take($1), nullptr, take($2), take($3)));
                    }

        do_block: kDO_BLOCK do_body kEND
                    {
                      auto delimited_block = take($2);
                      delimited_block->begin = take($1);
                      delimited_block->end = take($3);
                      $$ = put(std::move(delimited_block));
                    }

      block_call: command do_block
                    {
                      auto delimited_block = take($2);

                      $$ = put(builder::block(take($1),
                          std::move(delimited_block->begin),
                          std::move(delimited_block->args),
                          std::move(delimited_block->body),
                          std::move(delimited_block->end)
                        ));
                    }
                | block_call dot_or_colon operation2 opt_paren_args
                    {
                      auto delimited = take($4);

                      $$ = put(builder::call_method(take($1), take($2), take($3),
                                  std::move(delimited->begin),
                                  std::move(delimited->inner),
                                  std::move(delimited->end)));
                    }
                | block_call dot_or_colon operation2 opt_paren_args brace_block
                    {
                      auto delimited = take($4);

                      auto method_call =
                        builder::call_method(take($1), take($2), take($3),
                          std::move(delimited->begin),
                          std::move(delimited->inner),
                          std::move(delimited->end));

                      auto block = take($5);

                      $$ = put(
                        builder::block(std::move(method_call),
                          std::move(block->begin),
                          std::move(block->args),
                          std::move(block->body),
                          std::move(block->end)));
                    }
                | block_call dot_or_colon operation2 command_args do_block
                    {
                      auto method_call =
                        builder::call_method(take($1), take($2), take($3),
                          nullptr, take($4), nullptr);

                      auto block = take($5);

                      $$ = put(
                        builder::block(std::move(method_call),
                          std::move(block->begin),
                          std::move(block->args),
                          std::move(block->body),
                          std::move(block->end)));
                    }

     method_call: fcall paren_args
                    {
                      auto delimited = take($2);

                      $$ = put(builder::call_method(nullptr, nullptr, take($1),
                        std::move(delimited->begin),
                        std::move(delimited->inner),
                        std::move(delimited->end)));
                    }
                | primary_value call_op operation2 opt_paren_args
                    {
                      auto delimited = take($4);

                      $$ = put(
                        builder::call_method(take($1), take($2), take($3),
                          std::move(delimited->begin),
                          std::move(delimited->inner),
                          std::move(delimited->end)));
                    }
                | primary_value tCOLON2 operation2 paren_args
                    {
                      auto delimited = take($4);

                      $$ = put(
                        builder::call_method(take($1), take($2), take($3),
                          std::move(delimited->begin),
                          std::move(delimited->inner),
                          std::move(delimited->end)));
                    }
                | primary_value tCOLON2 operation3
                    {
                      $$ = put(builder::call_method(take($1), take($2), take($3)));
                    }
                | primary_value call_op paren_args
                    {
                      auto delimited = take($3);

                      $$ = put(
                        builder::call_method(take($1), take($2), nullptr,
                          std::move(delimited->begin),
                          std::move(delimited->inner),
                          std::move(delimited->end)));
                    }
                | primary_value tCOLON2 paren_args
                    {
                      auto delimited = take($3);

                      $$ = put(
                        builder::call_method(take($1), take($2), nullptr,
                          std::move(delimited->begin),
                          std::move(delimited->inner),
                          std::move(delimited->end)));
                    }
                | kSUPER paren_args
                    {
                      auto delimited = take($2);

                      $$ = put(
                        builder::keyword_cmd(node_type::SUPER, take($1),
                          std::move(delimited->begin),
                          std::move(delimited->inner),
                          std::move(delimited->end)));
                    }
                | kSUPER
                    {
                      $$ = put(builder::keyword_cmd(node_type::ZSUPER, take($1)));
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      $$ = put(builder::index(take($1), take($2), take($3), take($4)));
                    }

     brace_block: tLCURLY brace_body tRCURLY
                    {
                      auto block = take($2);

                      block->begin = take($1);
                      block->end = take($3);

                      $$ = put(std::move(block));
                    }
                | kDO do_body kEND
                    {
                      auto block = take($2);

                      block->begin = take($1);
                      block->end = take($3);

                      $$ = put(std::move(block));
                    }

      brace_body:   {
                      p.lexer->extend_dynamic();
                    }
                    {
                      $<state_stack>$ = put_copy(p.lexer->cmdarg);
                      p.lexer->cmdarg.clear();
                    }
                    opt_block_param compstmt
                    {
                      $$ = put(std::make_unique<node_delimited_block>(nullptr, take($3), take($4), nullptr));

                      p.lexer->unextend();
                      p.lexer->cmdarg = *take($<state_stack>2);
                      p.lexer->cmdarg.pop();
                    }

         do_body:   {
                      p.lexer->extend_dynamic();
                    }
                    {
                      $<state_stack>$ = put_copy(p.lexer->cmdarg);
                      p.lexer->cmdarg.clear();
                    }
                    opt_block_param compstmt
                    {
                      $$ = put(std::make_unique<node_delimited_block>(nullptr, take($3), take($4), nullptr));

                      p.lexer->unextend();

                      p.lexer->cmdarg = *take($<state_stack>2);
                      p.lexer->cmdarg.pop();
                    }

       case_body: kWHEN args then compstmt cases
                    {
                      auto cases = take($5);
                      cases->nodes.insert(cases->nodes.begin(),
                        builder::when(take($1), take($2), take($3), take($4)));
                      $$ = put(std::move(cases));
                    }

           cases: opt_else
                    {
                      $$ = put(make_node_list(static_unique_cast<node>(take($1))));
                    }
                | case_body

      opt_rescue: kRESCUE exc_list exc_var then compstmt opt_rescue
                    {
                      auto exc_var = take($3);

                      auto exc_list_ = take($2);

                      auto exc_list = exc_list_
                        ? builder::array(nullptr, std::move(exc_list_), nullptr)
                        : nullptr;

                      auto rescues = take($6);

                      rescues->nodes.insert(rescues->nodes.begin(),
                        builder::rescue_body(take($1),
                          std::move(exc_list), std::move(exc_var->token_), std::move(exc_var->node_),
                          take($4), take($5)));

                      $$ = put(std::move(rescues));
                    }
                |
                    {
                      $$ = put(make_node_list());
                    }

        exc_list: arg_value
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | mrhs
                | list_none

         exc_var: tASSOC lhs
                    {
                      $$ = put(std::make_unique<node_with_token>(take($1), take($2)));
                    }
                | // nothing
                    {
                      $$ = nullptr;
                    }

      opt_ensure: kENSURE compstmt
                    {
                      $$ = put(std::make_unique<node_with_token>(take($1), take($2)));
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
                      $$ = put(builder::string_compose(nullptr, take($1), nullptr));
                    }

          string: string1
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | string string1
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($2));
                      $$ = put(std::move(list));
                    }

         string1: tSTRING_BEG string_contents tSTRING_END
                    {
                      auto str = builder::string_compose(take($1), take($2), take($3));
                      $$ = put(builder::dedent_string(std::move(str), 0 /* TODO @lexer.dedent_level */));
                    }
                | tSTRING
                    {
                      auto str = builder::string(take($1));
                      $$ = put(builder::dedent_string(std::move(str), 0 /* TODO @lexer.dedent_level */));
                    }
                | tCHARACTER
                    {
                      $$ = put(builder::character(take($1)));
                    }

         xstring: tXSTRING_BEG xstring_contents tSTRING_END
                    {
                      auto xstr = builder::xstring_compose(take($1), take($2), take($3));
                      $$ = put(builder::dedent_string(std::move(xstr), 0 /* TODO @lexer.dedent_level */));
                    }

          regexp: tREGEXP_BEG regexp_contents tSTRING_END tREGEXP_OPT
                    {
                      auto opts = builder::regexp_options(take($4));
                      $$ = put(builder::regexp_compose(take($1), take($2), take($3), std::move(opts)));
                    }

           words: tWORDS_BEG word_list tSTRING_END
                    {
                      $$ = put(builder::words_compose(take($1), take($2), take($3)));
                    }

       word_list: // nothing
                    {
                      $$ = put(make_node_list());
                    }
                | word_list word tSPACE
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::word(take($2)));
                      $$ = put(std::move(list));
                    }

            word: string_content
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | word string_content
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($2));
                      $$ = put(std::move(list));
                    }

         symbols: tSYMBOLS_BEG symbol_list tSTRING_END
                    {
                      $$ = put(builder::symbols_compose(take($1), take($2), take($3)));
                    }

     symbol_list: // nothing
                    {
                      $$ = put(make_node_list());
                    }
                | symbol_list word tSPACE
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::word(take($2)));
                      $$ = put(std::move(list));
                    }

          qwords: tQWORDS_BEG qword_list tSTRING_END
                    {
                      $$ = put(builder::words_compose(take($1), take($2), take($3)));
                    }

        qsymbols: tQSYMBOLS_BEG qsym_list tSTRING_END
                    {
                      $$ = put(builder::symbols_compose(take($1), take($2), take($3)));
                    }

      qword_list: // nothing
                    {
                      $$ = put(make_node_list());
                    }
                | qword_list tSTRING_CONTENT tSPACE
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::string_internal(take($2)));
                      $$ = put(std::move(list));
                    }

       qsym_list: // nothing
                    {
                      $$ = put(make_node_list());
                    }
                | qsym_list tSTRING_CONTENT tSPACE
                    {
                      auto list = take($1);
                      list->nodes.push_back(builder::symbol_internal(take($2)));
                      $$ = put(std::move(list));
                    }

 string_contents: // nothing
                    {
                      $$ = put(make_node_list());
                    }
                | string_contents string_content
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($2));
                      $$ = put(std::move(list));
                    }

xstring_contents: // nothing
                    {
                      $$ = put(make_node_list());
                    }
                | xstring_contents string_content
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($2));
                      $$ = put(std::move(list));
                    }

regexp_contents: // nothing
                    {
                      $$ = put(make_node_list());
                    }
                | regexp_contents string_content
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($2));
                      $$ = put(std::move(list));
                    }

  string_content: tSTRING_CONTENT
                    {
                      $$ = put(builder::string_internal(take($1)));
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
                      p.lexer->cond.lexpop();
                      p.lexer->cmdarg.lexpop();

                      $$ = put(builder::begin(take($1), take($3), take($4)));
                    }

     string_dvar: tGVAR
                    {
                      $$ = put(builder::gvar(take($1)));
                    }
                | tIVAR
                    {
                      $$ = put(builder::ivar(take($1)));
                    }
                | tCVAR
                    {
                      $$ = put(builder::cvar(take($1)));
                    }
                | backref


          symbol: tSYMBOL
                    {
                      p.lexer->set_state_expr_endarg();
                      $$ = put(builder::symbol(take($1)));
                    }

            dsym: tSYMBEG xstring_contents tSTRING_END
                    {
                      p.lexer->set_state_expr_endarg();
                      $$ = put(builder::symbol_compose(take($1), take($2), take($3)));
                    }

         numeric: simple_numeric
                    {
                      $$ = $1;
                    }
                | tUMINUS_NUM simple_numeric %prec tLOWEST
                    {
                      $$ = put(builder::negate(take($1), take($2)));
                    }

  simple_numeric: tINTEGER
                    {
                      p.lexer->set_state_expr_endarg();
                      $$ = put(builder::integer(take($1)));
                    }
                | tFLOAT
                    {
                      p.lexer->set_state_expr_endarg();
                      $$ = put(builder::float_(take($1)));
                    }
                | tRATIONAL
                    {
                      p.lexer->set_state_expr_endarg();
                      $$ = put(builder::rational(take($1)));
                    }
                | tIMAGINARY
                    {
                      p.lexer->set_state_expr_endarg();
                      $$ = put(builder::complex(take($1)));
                    }
                | tRATIONAL_IMAGINARY
                    {
                      p.lexer->set_state_expr_endarg();
                      $$ = put(builder::rational_complex(take($1)));
                    }
                | tFLOAT_IMAGINARY
                    {
                      p.lexer->set_state_expr_endarg();
                      $$ = put(builder::float_complex(take($1)));
                    }

   user_variable: tIDENTIFIER
                    {
                      $$ = put(builder::ident(take($1)));
                    }
                | tIVAR
                    {
                      $$ = put(builder::ivar(take($1)));
                    }
                | tGVAR
                    {
                      $$ = put(builder::gvar(take($1)));
                    }
                | tCONSTANT
                    {
                      $$ = put(builder::const_(take($1)));
                    }
                | tCVAR
                    {
                      $$ = put(builder::cvar(take($1)));
                    }

keyword_variable: kNIL
                    {
                      $$ = put(builder::nil(take($1)));
                    }
                | kSELF
                    {
                      $$ = put(builder::self(take($1)));
                    }
                | kTRUE
                    {
                      $$ = put(builder::true_(take($1)));
                    }
                | kFALSE
                    {
                      $$ = put(builder::false_(take($1)));
                    }
                | k__FILE__
                    {
                      $$ = put(builder::file_literal(take($1)));
                    }
                | k__LINE__
                    {
                      $$ = put(builder::line_literal(take($1)));
                    }
                | k__ENCODING__
                    {
                      $$ = put(builder::encoding_literal(take($1)));
                    }

         var_ref: user_variable
                    {
                      $$ = put(builder::accessible(take($1)));
                    }
                | keyword_variable
                    {
                      $$ = put(builder::accessible(take($1)));
                    }

         var_lhs: user_variable
                    {
                      $$ = put(builder::assignable(take($1)));
                    }
                | keyword_variable
                    {
                      $$ = put(builder::assignable(take($1)));
                    }

         backref: tNTH_REF
                    {
                      $$ = put(builder::nth_ref(take($1)));
                    }
                | tBACK_REF
                    {
                      $$ = put(builder::back_ref(take($1)));
                    }

      superclass: tLT
                    {
                      p.lexer->set_state_expr_value();
                    }
                    expr_value term
                    {
                      $$ = put(std::make_unique<node_with_token>(take($1), take($3)));
                    }
                | // nothing
                    {
                      $$ = nullptr;
                    }

tr_methodgenargs: tLBRACK2 tr_gendeclargs rbracket
                    {
                      $$ = put(builder::tr_genargs(take($1), take($2), take($3)));
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
                      auto genargs = take($1);
                      auto args = builder::args(take($2), take($3), take($4));
                      auto returnsig = take($6);

                      if (genargs || returnsig) {
                        $$ = put(builder::prototype(
                          std::move(genargs),
                          std::move(args),
                          std::move(returnsig)));
                      } else {
                        $$ = put(std::move(args));
                      }
                    }
                | tr_methodgenargs
                    {
                      $<boolean>$ = p.lexer->in_kwarg;
                      p.lexer->in_kwarg = true;
                    }
                  f_args tr_returnsig term
                    {
                      p.lexer->in_kwarg = $<boolean>2;

                      auto genargs = take($1);
                      auto args = builder::args(nullptr, take($3), nullptr);
                      auto returnsig = take($4);

                      if (genargs || returnsig) {
                        $$ = put(builder::prototype(
                          std::move(genargs),
                          std::move(args),
                          std::move(returnsig)));
                      } else {
                        $$ = put(std::move(args));
                      }
                    }

       args_tail: f_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($4));
                      $$ = put(std::move(args));
                    }
                | f_kwarg opt_f_block_arg
                    {
                      auto args = take($1);
                      concat_node_list(args, take($2));
                      $$ = put(std::move(args));
                    }
                | f_kwrest opt_f_block_arg
                    {
                      auto args = take($1);
                      concat_node_list(args, take($2));
                      $$ = put(std::move(args));
                    }
                | f_block_arg
                    {
                      $$ = put(make_node_list(take($1)));
                    }

   opt_args_tail: tCOMMA args_tail
                    {
                      $$ = $2;
                    }
                | // nothing
                    {
                      $$ = put(make_node_list());
                    }

          f_args: f_arg tCOMMA f_optarg tCOMMA f_rest_arg              opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($5));
                      concat_node_list(args, take($6));
                      $$ = put(std::move(args));
                    }
                | f_arg tCOMMA f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($5));
                      concat_node_list(args, take($7));
                      concat_node_list(args, take($8));
                      $$ = put(std::move(args));
                    }
                | f_arg tCOMMA f_optarg                                opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($4));
                      $$ = put(std::move(args));
                    }
                | f_arg tCOMMA f_optarg tCOMMA                   f_arg opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($5));
                      concat_node_list(args, take($6));
                      $$ = put(std::move(args));
                    }
                | f_arg tCOMMA                 f_rest_arg              opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($4));
                      $$ = put(std::move(args));
                    }
                | f_arg tCOMMA                 f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($5));
                      concat_node_list(args, take($6));
                      $$ = put(std::move(args));
                    }
                | f_arg                                                opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($2));
                      $$ = put(std::move(args));
                    }
                |              f_optarg tCOMMA f_rest_arg              opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($4));
                      $$ = put(std::move(args));
                    }
                |              f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($5));
                      concat_node_list(args, take($6));
                      $$ = put(std::move(args));
                    }
                |              f_optarg                                opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($2));
                      $$ = put(std::move(args));
                    }
                |              f_optarg tCOMMA                   f_arg opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($4));
                      $$ = put(std::move(args));
                    }
                |                              f_rest_arg              opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($2));
                      $$ = put(std::move(args));
                    }
                |                              f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto args = take($1);
                      concat_node_list(args, take($3));
                      concat_node_list(args, take($4));
                      $$ = put(std::move(args));
                    }
                |                                                          args_tail
                    {
                      $$ = $1;
                    }
                | // nothing
                    {
                      $$ = put(make_node_list());
                    }

       f_bad_arg: tIVAR
                    {
                      // TODO diagnostic :error, :argument_ivar, nullptr, take($1)
                    }
                | tGVAR
                    {
                      // TODO diagnostic :error, :argument_gvar, nullptr, take($1)
                    }
                | tCVAR
                    {
                      // TODO diagnostic :error, :argument_cvar, nullptr, take($1)
                    }

      f_norm_arg: f_bad_arg
                | tIDENTIFIER
                    {
                      auto ident = take($1);

                      p.lexer->declare(ident->string());

                      $$ = put(std::move(ident));
                    }

      f_arg_asgn: f_norm_arg
                    {
                      $$ = $1;
                    }

      f_arg_item: tr_argsig f_arg_asgn
                    {
                      auto argsig = take($1);
                      auto arg = builder::arg(take($2));

                      if (argsig) {
                        $$ = put(builder::typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(std::move(arg));
                      }
                    }
                | tLPAREN f_margs rparen
                    {
                      $$ = put(builder::multi_lhs(take($1), take($2), take($3)));
                    }

           f_arg: f_arg_item
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | f_arg tCOMMA f_arg_item
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }

         f_label: tLABEL
                    {
                      auto label = take($1);

                      p.check_kwarg_name(label);

                      p.lexer->declare(label->string());

                      $$ = put(std::move(label));
                    }

            f_kw: tr_argsig f_label arg_value
                    {
                      auto argsig = take($1);
                      auto arg = builder::kwoptarg(take($2), take($3));

                      if (argsig) {
                        $$ = put(builder::typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(std::move(arg));
                      }
                    }
                | tr_argsig f_label
                    {
                      auto argsig = take($1);
                      auto arg = builder::kwarg(take($2));

                      if (argsig) {
                        $$ = put(builder::typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(std::move(arg));
                      }
                    }

      f_block_kw: tr_argsig f_label primary_value
                    {
                      auto argsig = take($1);
                      auto arg = builder::kwoptarg(take($2), take($3));

                      if (argsig) {
                        $$ = put(builder::typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(std::move(arg));
                      }
                    }
                | tr_argsig f_label
                    {
                      auto argsig = take($1);
                      auto arg = builder::kwarg(take($2));

                      if (argsig) {
                        $$ = put(builder::typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(std::move(arg));
                      }
                    }

   f_block_kwarg: f_block_kw
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | f_block_kwarg tCOMMA f_block_kw
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }

         f_kwarg: f_kw
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | f_kwarg tCOMMA f_kw
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }

     kwrest_mark: tPOW | tDSTAR

        f_kwrest: kwrest_mark tIDENTIFIER
                    {
                      auto ident = take($2);

                      p.lexer->declare(ident->string());

                      $$ = put(make_node_list({ builder::kwrestarg(take($1), std::move(ident)) }));
                    }
                | kwrest_mark
                    {
                      $$ = put(make_node_list(builder::kwrestarg(take($1))));
                    }

           f_opt: tr_argsig f_arg_asgn tEQL arg_value
                    {
                      auto argsig = take($1);
                      auto arg = builder::optarg(take($2), take($3), take($4));

                      if (argsig) {
                        $$ = put(builder::typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(std::move(arg));
                      }
                    }

     f_block_opt: tr_argsig f_arg_asgn tEQL primary_value
                    {
                      auto argsig = take($1);
                      auto arg = builder::optarg(take($2), take($3), take($4));

                      if (argsig) {
                        $$ = put(builder::typed_arg(std::move(argsig), std::move(arg)));
                      } else {
                        $$ = put(std::move(arg));
                      }
                    }

  f_block_optarg: f_block_opt
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | f_block_optarg tCOMMA f_block_opt
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }

        f_optarg: f_opt
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | f_optarg tCOMMA f_opt
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }

    restarg_mark: tSTAR2 | tSTAR

      f_rest_arg: tr_argsig restarg_mark tIDENTIFIER
                    {
                      auto argsig = take($1);
                      auto ident = take($3);

                      p.lexer->declare(ident->string());

                      auto restarg = builder::restarg(take($2), std::move(ident));

                      if (argsig) {
                        restarg = builder::typed_arg(std::move(argsig), std::move(restarg));
                      }

                      $$ = put(make_node_list(std::move(restarg)));
                    }
                | tr_argsig restarg_mark
                    {
                      auto argsig = take($1);
                      auto restarg = builder::restarg(take($2), nullptr);

                      if (restarg) {
                        restarg = builder::typed_arg(std::move(argsig), std::move(restarg));
                      }

                      $$ = put(make_node_list(std::move(restarg)));
                    }

     blkarg_mark: tAMPER2 | tAMPER

     f_block_arg: tr_argsig blkarg_mark tIDENTIFIER
                    {
                      auto argsig = take($1);
                      auto ident = take($3);

                      p.lexer->declare(ident->string());

                      auto blockarg = builder::blockarg(take($2), std::move(ident));

                      if (blockarg) {
                        blockarg = builder::typed_arg(std::move(argsig), std::move(blockarg));
                      }

                      $$ = put(make_node_list(std::move(blockarg)));
                    }
                | tr_argsig blkarg_mark
                    {
                      auto argsig = take($1);
                      auto blockarg = builder::blockarg(take($2), nullptr);

                      if (blockarg) {
                        blockarg = builder::typed_arg(std::move(argsig), std::move(blockarg));
                      }

                      $$ = put(make_node_list(std::move(blockarg)));
                    }

 opt_f_block_arg: tCOMMA f_block_arg
                    {
                      $$ = put(make_node_list(take($2)));
                    }
                |
                    {
                      $$ = put(make_node_list());
                    }

       singleton: var_ref
                | tLPAREN2 expr rparen
                    {
                      $$ = $2;
                    }

      assoc_list: // nothing
                    {
                      $$ = put(make_node_list());
                    }
                | assocs trailer

          assocs: assoc
                    {
                      $$ = put(make_node_list(take($1)));
                    }
                | assocs tCOMMA assoc
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }

           assoc: arg_value tASSOC arg_value
                    {
                      $$ = put(builder::pair(take($1), take($2), take($3)));
                    }
                | tLABEL arg_value
                    {
                      $$ = put(builder::pair_keyword(take($1), take($2)));
                    }
                | tSTRING_BEG string_contents tLABEL_END arg_value
                    {
                      $$ = put(builder::pair_quoted(take($1), take($2), take($3), take($4)));
                    }
                | tDSTAR arg_value
                    {
                      $$ = put(builder::kwsplat(take($1), take($2)));
                    }

       operation: tIDENTIFIER | tCONSTANT | tFID
      operation2: tIDENTIFIER | tCONSTANT | tFID | op
      operation3: tIDENTIFIER | tFID | op
    dot_or_colon: call_op | tCOLON2
         call_op: tDOT
                    {
                      // what is this???
                      // $$ = put([:dot, take($1)[1]]
                      $$ = $1;
                    }
                | tANDDOT
                    {
                      // what is this???
                      // $$ = [:anddot, take($1)[1]]
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
                      $$ = put(builder::const_global(take($1), take($2)));
                    }
                | tCONSTANT
                    {
                      $$ = put(builder::const_(take($1)));
                    }
                | tr_cpath tCOLON2 tCONSTANT
                    {
                      $$ = put(builder::const_fetch(take($1), take($2), take($3)));
                    }

       tr_types: tr_types tCOMMA tr_type
                    {
                      auto list = take($1);
                      list->nodes.push_back(take($3));
                      $$ = put(std::move(list));
                    }
               | tr_type
                    {
                      $$ = put(make_node_list(take($1)));
                    }

         tr_type: tr_cpath
                    {
                      $$ = put(builder::tr_cpath(take($1)));
                    }
                | tr_cpath tCOLON2 tLBRACK2 tr_types rbracket
                    {
                      $$ = put(builder::tr_geninst(take($1), take($3), take($4), take($5)));
                    }
                | tLBRACK tr_type rbracket
                    {
                      $$ = put(builder::tr_array(take($1), take($2), take($3)));
                    }
                | tLBRACK tr_type tCOMMA tr_types rbracket
                    {
                      auto types = take($4);

                      types->nodes.insert(types->nodes.begin(), take($2));

                      $$ = put(builder::tr_tuple(take($1), std::move(types), take($5)));
                    }
                | tLBRACE tr_type tASSOC tr_type tRCURLY
                    {
                      $$ = put(builder::tr_hash(take($1), take($2), take($3), take($4), take($5)));
                    }
                | tLBRACE tr_blockproto tr_returnsig tRCURLY
                    {
                      auto blockproto = take($2);
                      auto returnsig = take($3);

                      auto prototype = returnsig
                        ? builder::prototype(nullptr, std::move(blockproto), std::move(returnsig))
                        : std::move(blockproto);

                      $$ = put(builder::tr_proc(take($1), std::move(prototype), take($4)));
                    }
                | tTILDE tr_type
                    {
                      $$ = put(builder::tr_nillable(take($1), take($2)));
                    }
                | kNIL
                    {
                      $$ = put(builder::tr_nil(take($1)));
                    }
                | tSYMBOL
                    {
                      $$ = put(builder::tr_special(take($1)));
                      // diagnostic :error, :bad_special_type, { value: take($1)[0] }, take($1)
                    }
                | tLPAREN tr_union_type rparen
                    {
                      $$ = $2;
                    }

   tr_union_type: tr_union_type tPIPE tr_type
                    {
                      $$ = put(builder::tr_or(take($1), take($3)));
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
                      auto list = take($1);
                      list->nodes.push_back(builder::tr_gendeclarg(take($3)));
                      $$ = $1;
                    }
                | tCONSTANT
                    {
                      $$ = put(make_node_list(builder::tr_gendeclarg(take($1))));
                    }

   tr_blockproto: { p.lexer->extend_dynamic(); }
                  block_param_def
                    {
                      p.lexer->unextend();
                      $$ = $2;
                    }

%%
