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

  #define YYERROR_VERBOSE 1

  using namespace ruby_parser;
  using namespace std::string_literals;

  #define yyparse ruby_parser_typedruby24_yyparse

  extern "C" {
    int yyparse(parser::typedruby24& p);
  }
%}

%pure-parser

%lex-param { parser::typedruby24& p }
%parse-param { parser::typedruby24& p }

%union {
  ruby_parser::token *token;
  ruby_parser::delimited_node_list *delimited_list;
  ruby_parser::delimited_block *delimited_block;
  ruby_parser::node_with_token *with_token;
  ruby_parser::case_body *case_body;
  ruby_parser::foreign_ptr node;
  ruby_parser::node_list *list;
  ruby_parser::state_stack *state_stack;
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
static node_list *make_node_list() {
	return new node_list();
  }
  static node_list *make_node_list(foreign_ptr node) {
	node_list *list = new node_list();
	list->nodes.push_back(node);
	return list;
  }

static void concat_node_list(node_list *a, node_list *b) {
	a->nodes.insert(
	  a->nodes.end(),
	  std::make_move_iterator(b->nodes.begin()),
	  std::make_move_iterator(b->nodes.end())
	);
  }

  #define yyerror(p, msg) yyerror_(p, msg)

  static void yyerror_(parser::typedruby24& p, const char* msg) {
    p.diagnostic_(diagnostic_level::ERROR, std::string(msg), diagnostic::range(p.lexer_->last_token_s, p.lexer_->last_token_e));
  }

  static int yylex(YYSTYPE *lval, parser::typedruby24& p) {
    auto token = p.lexer_->advance();
    int token_type = static_cast<int>(token->type());
    assert(token_type >= 0);
    lval->token = token;
    return token_type;
  }
%}

%%
         program: top_compstmt
                    {
                      p.ast = $1;
                    }

    top_compstmt: top_stmts opt_terms
                    {
                      $$ = p.builder.compstmt($1);
                    }

       top_stmts: // nothing
                    {
                      $$ = make_node_list();
                    }
                | top_stmt
                    {
                      $$ = make_node_list($1);
                    }
                | top_stmts terms top_stmt
                    {
					  $1->nodes.push_back($3);
                      $$ = $1;
                    }
                | error top_stmt
                    {
                      $$ = make_node_list($2);
                    }

        top_stmt: stmt
                | klBEGIN tLCURLY top_compstmt tRCURLY
                    {
                      $$ = p.builder.preexe($1, $3, $4);
                    }

        bodystmt: compstmt opt_rescue opt_else opt_ensure
                    {
                      auto rescue_bodies = $2;
                      auto else_ = $3;
                      auto ensure = $4;

                      if (rescue_bodies->nodes.size() == 0 && else_ != nullptr) {
                        p.diagnostic_(diagnostic_level::WARNING, "else without rescue is useless", else_->token_);
                      }

                      $$ = p.builder.begin_body($1, rescue_bodies,
                            else_ ? else_->token_ : nullptr,
                            else_ ? else_->node_ : nullptr,
                            ensure ? ensure->token_ : nullptr,
                            ensure ? ensure->node_ : nullptr);
                    }

        compstmt: stmts opt_terms
                    {
                      $$ = p.builder.compstmt($1);
                    }

           stmts: // nothing
                    {
                      $$ = make_node_list();
                    }
                | stmt_or_begin
                    {
                      $$ = make_node_list($1);
                    }
                | stmts terms stmt_or_begin
                    {
                      $1->nodes.push_back($3);
                      $$ = $1;
                    }
                | error stmt
                    {
                      $$ = make_node_list($2);
                    }

   stmt_or_begin: stmt
                | klBEGIN tLCURLY top_compstmt tRCURLY
                    {
                      p.diagnostic_(diagnostic_level::ERROR, "BEGIN in method"s, $1);
                      YYERROR;
                    }

            stmt: kALIAS fitem
                    {
                      p.lexer_->set_state_expr_fname();
                    }
                    fitem
                    {
                      $$ = p.builder.alias($1, $2, $4);
                    }
                | kALIAS tGVAR tGVAR
                    {
                      $$ = p.builder.alias($1, p.builder.gvar($2), p.builder.gvar($3));
                    }
                | kALIAS tGVAR tBACK_REF
                    {
                      $$ = p.builder.alias($1, p.builder.gvar($2), p.builder.back_ref($3));
                    }
                | kALIAS tGVAR tNTH_REF
                    {
                      p.diagnostic_(diagnostic_level::ERROR, "cannot define an alias for a back-reference variable"s, $3);
                      YYERROR;
                    }
                | kUNDEF undef_list
                    {
                      $$ = p.builder.undef_method($1, $2);
                    }
                | stmt kIF_MOD expr_value
                    {
                      $$ = p.builder.condition_mod($1, nullptr, $3);
                    }
                | stmt kUNLESS_MOD expr_value
                    {
                      $$ = p.builder.condition_mod(nullptr, $1, $3);
                    }
                | stmt kWHILE_MOD expr_value
                    {
                      $$ = p.builder.loop_while_mod($1, $3);
                    }
                | stmt kUNTIL_MOD expr_value
                    {
                      $$ = p.builder.loop_until_mod($1, $3);
                    }
                | stmt kRESCUE_MOD stmt
                    {
                      auto rescue_body = make_node_list(
						p.builder.rescue_body($2, nullptr, nullptr, nullptr, nullptr, $3));
                      $$ = p.builder.begin_body($1, rescue_body, nullptr, nullptr, nullptr, nullptr);
                    }
                | klEND tLCURLY compstmt tRCURLY
                    {
                      $$ = p.builder.postexe($1, $3, $4);
                    }
                | command_asgn
                | mlhs tEQL command_call
                    {
                      $$ = p.builder.multi_assign($1, $3);
                    }
                | lhs tEQL mrhs
                    {
                      $$ = p.builder.assign($1, $2, p.builder.array(nullptr, $3, nullptr));
                    }
                | mlhs tEQL mrhs_arg
                    {
                      $$ = p.builder.multi_assign($1, $3);
                    }
                | kDEF tIVAR tCOLON tr_type
                    {
                      $$ = p.builder.tr_ivardecl($2, $4);
                    }
                | expr

    command_asgn: lhs tEQL command_rhs
                    {
                      $$ = p.builder.assign($1, $2, $3);
                    }
                | var_lhs tOP_ASGN command_rhs
                    {
                      $$ = p.builder.op_assign($1, $2, $3);
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN command_rhs
                    {
                      $$ = p.builder.op_assign(p.builder.index($1, $2, $3, $4), $5, $6);
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN command_rhs
                    {
                      $$ = p.builder.op_assign(p.builder.call_method($1, $2, $3, nullptr, nullptr, nullptr), $4, $5);
                    }
                | primary_value call_op tCONSTANT tOP_ASGN command_rhs
                    {
                      
                      
                      
                      
                      
                      $$ = p.builder.op_assign(p.builder.call_method($1, $2, $3, nullptr, nullptr, nullptr), $4, $5);
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN command_rhs
                    {
                      auto const_node = p.builder.const_op_assignable(p.builder.const_fetch($1, $2, $3));
                      $$ = p.builder.op_assign(const_node, $4, $5);
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN command_rhs
                    {
                      $$ = p.builder.op_assign(p.builder.call_method($1, $2, $3, nullptr, nullptr, nullptr), $4, $5);
                    }
                | backref tOP_ASGN command_rhs
                    {
					  // XXX: assign to $$?
                      $$ = p.builder.op_assign($1, $2, $3);
                    }

     command_rhs: command_call %prec tOP_ASGN
                | command_call kRESCUE_MOD stmt
                    {
                      auto rescue_body =
                        make_node_list(p.builder.rescue_body($2, nullptr, nullptr, nullptr, nullptr, $3));
                      $$ = p.builder.begin_body($1, rescue_body, nullptr, nullptr, nullptr, nullptr);
                    }
                | command_asgn

            expr: command_call
                | expr kAND expr
                    {
                      $$ = p.builder.logical_and($1, $2, $3);
                    }
                | expr kOR expr
                    {
                      $$ = p.builder.logical_or($1, $2, $3);
                    }
                | kNOT opt_nl expr
                    {
                      $$ = p.builder.not_op($1, nullptr, $3, nullptr);
                    }
                | tBANG command_call
                    {
                      $$ = p.builder.not_op($1, nullptr, $2, nullptr);
                    }
                | arg

      expr_value: expr

    command_call: command
                | block_command

   block_command: block_call
                | block_call dot_or_colon operation2 command_args
                    {
                      $$ = p.builder.call_method($1, $2, $3, nullptr, $4, nullptr);
                    }

 cmd_brace_block: tLBRACE_ARG brace_body tRCURLY
                    {
                      auto block = $2;
                      block->begin = $1;
                      block->end = $3;
                      $$ = block;
                    }

           fcall: operation

         command: fcall command_args %prec tLOWEST
                    {
                      $$ = p.builder.call_method(nullptr, nullptr, $1, nullptr, $2, nullptr);
                    }
                | fcall command_args cmd_brace_block
                    {
                      auto method_call = p.builder.call_method(nullptr, nullptr, $1, nullptr, $2, nullptr);
                      auto delimited_block = $3;
                      $$ = p.builder.block(method_call,
                                      delimited_block->begin,
                                      delimited_block->args,
                                      delimited_block->body,
                                      delimited_block->end);
                    }
                | primary_value call_op operation2 command_args %prec tLOWEST
                    {
                      $$ = p.builder.call_method($1, $2, $3, nullptr, $4, nullptr);
                    }
                | primary_value call_op operation2 command_args cmd_brace_block
                    {
                      auto method_call = p.builder.call_method($1, $2, $3, nullptr, $4, nullptr);
                      auto delimited_block = $5;
                      $$ = p.builder.block(method_call,
                                      delimited_block->begin,
                                      delimited_block->args,
                                      delimited_block->body,
                                      delimited_block->end);
                    }
                | primary_value tCOLON2 operation2 command_args %prec tLOWEST
                    {
                      $$ = p.builder.call_method($1, $2, $3, nullptr, $4, nullptr);
                    }
                | primary_value tCOLON2 operation2 command_args cmd_brace_block
                    {
                      auto method_call = p.builder.call_method($1, $2, $3, nullptr, $4, nullptr);
                      auto delimited_block = $5;
                      $$ = p.builder.block(method_call,
                                      delimited_block->begin,
                                      delimited_block->args,
                                      delimited_block->body,
                                      delimited_block->end);
                    }
                | kSUPER command_args
                    {
                      $$ = p.builder.keyword_super($1, nullptr, $2, nullptr);
                    }
                | kYIELD command_args
                    {
                      $$ = p.builder.keyword_yield($1, nullptr, $2, nullptr);
                    }
                | kRETURN call_args
                    {
                      $$ = p.builder.keyword_return($1, nullptr, $2, nullptr);
                    }
                | kBREAK call_args
                    {
                      $$ = p.builder.keyword_break($1, nullptr, $2, nullptr);
                    }
                | kNEXT call_args
                    {
                      $$ = p.builder.keyword_next($1, nullptr, $2, nullptr);
                    }

            mlhs: mlhs_basic
                    {
                      $$ = p.builder.multi_lhs(nullptr, $1, nullptr);
                    }
                | tLPAREN mlhs_inner rparen
                    {
                      $$ = p.builder.begin($1, $2, $3);
                    }

      mlhs_inner: mlhs_basic
                    {
                      $$ = p.builder.multi_lhs(nullptr, $1, nullptr);
                    }
                | tLPAREN mlhs_inner rparen
                    {
                      auto inner = make_node_list($2);
                      $$ = p.builder.multi_lhs($1, inner, $3);
                    }

      mlhs_basic: mlhs_head
                | mlhs_head mlhs_item
                    {
                      auto list = $1;
                      list->nodes.push_back($2);
                      $$ = list;
                    }
                | mlhs_head tSTAR mlhs_node
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.splat($2, $3));
                      $$ = list;
                    }
                | mlhs_head tSTAR mlhs_node tCOMMA mlhs_post
                    {
                      auto head = $1;
                      head->nodes.push_back(p.builder.splat($2, $3));
                      concat_node_list(head, $5);
                      $$ = head;
                    }
                | mlhs_head tSTAR
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.splat($2, nullptr));
                      $$ = list;
                    }
                | mlhs_head tSTAR tCOMMA mlhs_post
                    {
                      auto head = $1;
                      head->nodes.push_back(p.builder.splat($2, nullptr));
                      concat_node_list(head, $4);
                      $$ = head;
                    }
                | tSTAR mlhs_node
                    {
                      $$ = make_node_list(p.builder.splat($1, $2));
                    }
                | tSTAR mlhs_node tCOMMA mlhs_post
                    {
					  // XXX brackets?
                      auto items = make_node_list(p.builder.splat($1, $2));
                      concat_node_list(items, $4);
                      $$ = items;
                    }
                | tSTAR
                    {
                      $$ = make_node_list(p.builder.splat($1, nullptr));
                    }
                | tSTAR tCOMMA mlhs_post
                    {
                      auto items = make_node_list(p.builder.splat($1, nullptr));
                      concat_node_list(items, $3);
                      $$ = items;
                    }

       mlhs_item: mlhs_node
                | tLPAREN mlhs_inner rparen
                    {
                      $$ = p.builder.begin($1, $2, $3);
                    }

       mlhs_head: mlhs_item tCOMMA
                    {
                      $$ = make_node_list($1);
                    }
                | mlhs_head mlhs_item tCOMMA
                    {
                      auto list = $1;
                      list->nodes.push_back($2);
                      $$ = list;
                    }

       mlhs_post: mlhs_item
                    {
                      $$ = make_node_list($1);
                    }
                | mlhs_post tCOMMA mlhs_item
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }

       mlhs_node: user_variable
                    {
					// XXX: p pointer here
                      $$ = p.builder.assignable(&p, $1);
                    }
                | keyword_variable
                    {
                      $$ = p.builder.assignable(&p, $1);
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      $$ = p.builder.index_asgn($1, $2, $3, $4);
                    }
                | primary_value call_op tIDENTIFIER
                    {
                      $$ = p.builder.attr_asgn($1, $2, $3);
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                      $$ = p.builder.attr_asgn($1, $2, $3);
                    }
                | primary_value call_op tCONSTANT
                    {
                      $$ = p.builder.attr_asgn($1, $2, $3);
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = p.builder.assignable(&p, p.builder.const_fetch($1, $2, $3));
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = p.builder.assignable(&p, p.builder.const_global($1, $2));
                    }
                | backref
                    {
                      $$ = p.builder.assignable(&p, $1);
                    }

             lhs: user_variable
                    {
                      $$ = p.builder.assignable(&p, $1);
                    }
                | keyword_variable
                    {
                      $$ = p.builder.assignable(&p, $1);
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      
                      $$ = p.builder.index_asgn($1, $2, $3, $4);
                    }
                | primary_value call_op tIDENTIFIER
                    {
                      $$ = p.builder.attr_asgn($1, $2, $3);
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                      $$ = p.builder.attr_asgn($1, $2, $3);
                    }
                | primary_value call_op tCONSTANT
                    {
                      $$ = p.builder.attr_asgn($1, $2, $3);
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = p.builder.assignable(&p, p.builder.const_fetch($1, $2, $3));
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = p.builder.assignable(&p, p.builder.const_global($1, $2));
                    }
                | backref
                    {
                      $$ = p.builder.assignable(&p, $1);
                    }

           cname: tIDENTIFIER
                    {
                      p.diagnostic_(diagnostic_level::ERROR, "class or module name must be a constant literal"s, $1);
                      YYERROR;
                    }
                | tCONSTANT

           cpath: tCOLON3 cname
                    {
                      $$ = p.builder.const_global($1, $2);
                    }
                | cname
                    {
                      $$ = p.builder.const_($1);
                    }
                | primary_value tCOLON2 tLBRACK2 tr_gendeclargs rbracket
                    {
                      $$ = p.builder.tr_gendecl($1, $3, $4, $5);
                    }
                | primary_value tCOLON2 cname
                    {
                      $$ = p.builder.const_fetch($1, $2, $3);
                    }

           fname: tIDENTIFIER | tCONSTANT | tFID
                | op
                | reswords

            fsym: fname
                    {
                      $$ = p.builder.symbol($1);
                    }
                | symbol

           fitem: fsym
                | dsym

      undef_list: fitem
                    {
                      $$ = make_node_list($1);
                    }
                | undef_list tCOMMA
                    {
                      p.lexer_->set_state_expr_fname();
                    }
                    fitem
                    {
                      auto list = $1;
                      list->nodes.push_back($4);
                      $$ = list;
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
                      $$ = p.builder.assign($1, $2, $3);
                    }
                | var_lhs tOP_ASGN arg_rhs
                    {
                      $$ = p.builder.op_assign($1, $2, $3);
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs
                    {
                      $$ = p.builder.op_assign(p.builder.index($1, $2, $3, $4), $5, $6);
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN arg_rhs
                    {
                      $$ = p.builder.op_assign(p.builder.call_method($1, $2, $3, nullptr, nullptr, nullptr), $4, $5);
                    }
                | primary_value call_op tCONSTANT tOP_ASGN arg_rhs
                    {
                      $$ = p.builder.op_assign(p.builder.call_method($1, $2, $3, nullptr, nullptr, nullptr), $4, $5);
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN arg_rhs
                    {
                      $$ = p.builder.op_assign(p.builder.call_method($1, $2, $3, nullptr, nullptr, nullptr), $4, $5);
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN arg_rhs
                    {
                      auto const_ = p.builder.const_op_assignable(p.builder.const_fetch($1, $2, $3));
                      $$ = p.builder.op_assign(const_, $4, $5);
                    }
                | tCOLON3 tCONSTANT tOP_ASGN arg_rhs
                    {
                      auto const_ = p.builder.const_op_assignable(p.builder.const_global($1, $2));
                      $$ = p.builder.op_assign(const_, $3, $4);
                    }
                | backref tOP_ASGN arg_rhs
                    {
                      $$ = p.builder.op_assign($1, $2, $3);
                    }
                | arg tDOT2 arg
                    {
                      $$ = p.builder.range_inclusive($1, $2, $3);
                    }
                | arg tDOT3 arg
                    {
                      $$ = p.builder.range_exclusive($1, $2, $3);
                    }
                | arg tPLUS arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tMINUS arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tSTAR2 arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tDIVIDE arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tPERCENT arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tPOW arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | tUMINUS_NUM simple_numeric tPOW arg
                    {
                      $$ = p.builder.unary_op($1, p.builder.binary_op($2, $3, $4));
                    }
                | tUPLUS arg
                    {
                      $$ = p.builder.unary_op($1, $2);
                    }
                | tUMINUS arg
                    {
                      $$ = p.builder.unary_op($1, $2);
                    }
                | arg tPIPE arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tCARET arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tAMPER2 arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tCMP arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tGT arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tGEQ arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tLT arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tLEQ arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tEQ arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tEQQ arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tNEQ arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tMATCH arg
                    {
                      $$ = p.builder.match_op($1, $2, $3);
                    }
                | arg tNMATCH arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | tBANG arg
                    {
                      $$ = p.builder.not_op($1, nullptr, $2, nullptr);
                    }
                | tTILDE arg
                    {
                      $$ = p.builder.unary_op($1, $2);
                    }
                | arg tLSHFT arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tRSHFT arg
                    {
                      $$ = p.builder.binary_op($1, $2, $3);
                    }
                | arg tANDOP arg
                    {
                      $$ = p.builder.logical_and($1, $2, $3);
                    }
                | arg tOROP arg
                    {
                      $$ = p.builder.logical_or($1, $2, $3);
                    }
                | kDEFINED opt_nl arg
                    {
                      $$ = p.builder.keyword_defined($1, $3);
                    }
                | arg tEH arg opt_nl tCOLON arg
                    {
                      $$ = p.builder.ternary($1, $2, $3, $5, $6);
                    }
                | primary

       arg_value: arg

       aref_args: list_none
                | args trailer
                | args tCOMMA assocs trailer
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.associate(nullptr, $3, nullptr));
                      $$ = list;
                    }
                | assocs trailer
                    {
                      $$ = make_node_list(p.builder.associate(nullptr, $1, nullptr));
                    }

         arg_rhs: arg %prec tOP_ASGN
                | arg kRESCUE_MOD arg
                    {
                      auto rescue_body = make_node_list(p.builder.rescue_body($2, nullptr, nullptr, nullptr, nullptr, $3));
                      $$ = p.builder.begin_body($1, rescue_body, nullptr, nullptr, nullptr, nullptr);
                    }

      paren_args: tLPAREN2 opt_call_args rparen
                    {
                      $$ = new delimited_node_list($1, $2, $3);
                    }

  opt_paren_args: // nothing
                    {
                      $$ = new delimited_node_list(nullptr, make_node_list(), nullptr);
                    }
                | paren_args

   opt_call_args: // nothing
                    {
                      $$ = make_node_list();
                    }
                | call_args
                | args tCOMMA
                | args tCOMMA assocs tCOMMA
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.associate(nullptr, $3, nullptr));
                      $$ = list;
                    }
                | assocs tCOMMA
                    {
                      $$ = make_node_list(p.builder.associate(nullptr, $1, nullptr));
                    }

       call_args: command
                    {
                      $$ = make_node_list($1);
                    }
                | args opt_block_arg
                    {
                      auto args = $1;
                      concat_node_list(args, $2);
                      $$ = args;
                    }
                | assocs opt_block_arg
                    {
                      auto args = make_node_list(p.builder.associate(nullptr, $1, nullptr));
                      concat_node_list(args, $2);
                      $$ = args;
                    }
                | args tCOMMA assocs opt_block_arg
                    {
                      auto args = $1;
                      args->nodes.push_back(p.builder.associate(nullptr, $3, nullptr));
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                | block_arg
                    {
                      $$ = make_node_list($1);
                    }

    command_args:   {
						// XXX: allocation
                      $<state_stack>$ = new state_stack(p.lexer_->cmdarg);
                      p.lexer_->cmdarg.push(true);
                    }
                  call_args
                    {
                      p.lexer_->cmdarg = *$<state_stack>1;
                      $$ = $2;
                    }

       block_arg: tAMPER arg_value
                    {
                      $$ = p.builder.block_pass($1, $2);
                    }

   opt_block_arg: tCOMMA block_arg
                    {
                      $$ = make_node_list($2);
                    }
                | // nothing
                    {
                      $$ = make_node_list();
                    }

            args: arg_value
                    {
                      $$ = make_node_list($1);
                    }
                | tSTAR arg_value
                    {
                      $$ = make_node_list(p.builder.splat($1, $2));
                    }
                | args tCOMMA arg_value
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }
                | args tCOMMA tSTAR arg_value
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.splat($3, $4));
                      $$ = list;
                    }

        mrhs_arg: mrhs
                    {
                      $$ = p.builder.array(nullptr, $1, nullptr);
                    }
                | arg_value

            mrhs: args tCOMMA arg_value
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }
                | args tCOMMA tSTAR arg_value
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.splat($3, $4));
                      $$ = list;
                    }
                | tSTAR arg_value
                    {
                      $$ = make_node_list(p.builder.splat($1, $2));
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
                      $$ = p.builder.call_method(nullptr, nullptr, $1, nullptr, nullptr, nullptr);
                    }
                | kBEGIN
                    {
                      $<state_stack>$ = new state_stack(p.lexer_->cmdarg);
                      p.lexer_->cmdarg.clear();
                    }
                    bodystmt kEND
                    {
                      p.lexer_->cmdarg = *$<state_stack>2;
                      $$ = p.builder.begin_keyword($1, $3, $4);
                    }
                | tLPAREN_ARG
                    {
                      $<state_stack>$ = new state_stack(p.lexer_->cmdarg);
                      p.lexer_->cmdarg.clear();
                    }
                    stmt
                    {
                      p.lexer_->set_state_expr_endarg();
                    }
                    rparen
                    {
                      p.lexer_->cmdarg = *$<state_stack>2;
                      $$ = p.builder.begin($1, $3, $5);
                    }
                | tLPAREN_ARG
                    {
                      p.lexer_->set_state_expr_endarg();
                    }
                    opt_nl tRPAREN
                    {
                      $$ = p.builder.begin($1, nullptr, $4);
                    }
                | tLPAREN compstmt tRPAREN
                    {
                      $$ = p.builder.begin($1, $2, $3);
                    }
                | tLPAREN expr tCOLON tr_type tRPAREN
                    {
                      $$ = p.builder.tr_cast($1, $2, $3, $4, $5);
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = p.builder.const_fetch($1, $2, $3);
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = p.builder.const_global($1, $2);
                    }
                | tLBRACK aref_args tRBRACK
                    {
                      $$ = p.builder.array($1, $2, $3);
                    }
                | tLBRACE assoc_list tRCURLY
                    {
                      $$ = p.builder.associate($1, $2, $3);
                    }
                | kRETURN
                    {
                      $$ = p.builder.keyword_return($1, nullptr, nullptr, nullptr);
                    }
                | kYIELD tLPAREN2 call_args rparen
                    {
                      $$ = p.builder.keyword_yield($1, $2, $3, $4);
                    }
                | kYIELD tLPAREN2 rparen
                    {
                      $$ = p.builder.keyword_yield($1, $2, make_node_list(), $3);
                    }
                | kYIELD
                    {
                      $$ = p.builder.keyword_yield($1, nullptr, nullptr, nullptr);
                    }
                | kDEFINED opt_nl tLPAREN2 expr rparen
                    {
                      $$ = p.builder.keyword_defined($1, $4);
                    }
                | kNOT tLPAREN2 expr rparen
                    {
                      $$ = p.builder.not_op($1, $2, $3, $4);
                    }
                | kNOT tLPAREN2 rparen
                    {
                      $$ = p.builder.not_op($1, $2, nullptr, $3);
                    }
                | fcall brace_block
                    {
                      auto method_call = p.builder.call_method(nullptr, nullptr, $1, nullptr, nullptr, nullptr);
                      auto delimited_block = $2;

                      $$ = p.builder.block(method_call,
                        delimited_block->begin,
                        delimited_block->args,
                        delimited_block->body,
                        delimited_block->end);
                    }
                | method_call
                | method_call brace_block
                    {
                      auto delimited_block = $2;
                      $$ = p.builder.block($1,
                        delimited_block->begin,
                        delimited_block->args,
                        delimited_block->body,
                        delimited_block->end);
                    }
                | tLAMBDA lambda
                    {
                      auto lambda_call = p.builder.call_lambda($1);
                      auto lambda = $2;
                      $$ = p.builder.block(lambda_call,
                        lambda->begin,
                        lambda->args,
                        lambda->body,
                        lambda->end);
                    }
                | kIF expr_value then compstmt if_tail kEND
                    {
                      auto else_ = $5;
                      $$ = p.builder.condition($1, $2, $3, $4,
                        else_ ? else_->token_ : nullptr,
                        else_ ? else_->node_ : nullptr, $6);
                    }
                | kUNLESS expr_value then compstmt opt_else kEND
                    {
                      auto else_ = $5;
                      $$ = p.builder.condition($1, $2, $3,
                        else_ ? else_->node_ : nullptr,
                        else_ ? else_->token_ : nullptr, $4, $6);
                    }
                | kWHILE
                    {
                      p.lexer_->cond.push(true);
                    }
                    expr_value do
                    {
                      p.lexer_->cond.pop();
                    }
                    compstmt kEND
                    {
                      $$ = p.builder.loop_while($1, $3, $4, $6, $7);
                    }
                | kUNTIL
                    {
                      p.lexer_->cond.push(true);
                    }
                    expr_value do
                    {
                      p.lexer_->cond.pop();
                    }
                    compstmt kEND
                    {
                      $$ = p.builder.loop_until($1, $3, $4, $6, $7);
                    }
                | kCASE expr_value opt_terms case_body kEND
                    {
                      auto case_body = $4;
                      auto else_ = case_body->else_;
                      $$ = p.builder.case_($1, $2,
                        case_body->whens,
                        else_ ? else_->token_ : nullptr,
                        else_ ? else_->node_ : nullptr, $5);
                    }
                | kCASE            opt_terms case_body kEND
                    {
                      auto case_body = $3;
                      auto else_ = case_body->else_;
                      $$ = p.builder.case_($1, nullptr,
                        case_body->whens,
                        else_ ? else_->token_ : nullptr,
                        else_ ? else_->node_ : nullptr, $4);
                    }
                | kFOR for_var kIN
                    {
                      p.lexer_->cond.push(true);
                    }
                    expr_value do
                    {
                      p.lexer_->cond.pop();
                    }
                    compstmt kEND
                    {
                      $$ = p.builder.for_($1, $2, $3, $5, $6, $8, $9);
                    }
                | kCLASS cpath superclass
                    {
                      p.lexer_->extend_static();
                      $<state_stack>$ = new state_stack(p.lexer_->cmdarg);
                      p.lexer_->cmdarg.clear();
                    }
                    bodystmt kEND
                    {
                      auto class_tok = $1;
                      auto end_tok = $6;

                      if (p.def_level > 0) {
                        p.diagnostic_(diagnostic_level::ERROR, "class definition in method body"s, class_tok);
                        YYERROR;
                      }

                      auto superclass_ = $3;
                      auto lt_t       = superclass_ ? superclass_->token_ : nullptr;
                      auto superclass = superclass_ ? superclass_->node_ : nullptr;

                      $$ = p.builder.def_class(class_tok, $2, lt_t, superclass, $5, end_tok);
                      p.lexer_->cmdarg = *($<state_stack>4);
                      p.lexer_->unextend();
                    }
                | kCLASS tLSHFT expr term
                    {
                      $<size>$ = p.def_level;
                      p.def_level = 0;
                    }
                    {
                      p.lexer_->extend_static();
                      $<state_stack>$ = new state_stack(p.lexer_->cmdarg);
                      p.lexer_->cmdarg.clear();
                    }
                    bodystmt kEND
                    {
                      $$ = p.builder.def_sclass($1, $2, $3, $7, $8);
                      p.def_level = $<size>5;
                      p.lexer_->cmdarg = *($<state_stack>6);
                      p.lexer_->unextend();
                    }
                | kMODULE cpath
                    {
                      p.lexer_->extend_static();
                      $<state_stack>$ = new state_stack(p.lexer_->cmdarg);
                      p.lexer_->cmdarg.clear();
                    }
                    bodystmt kEND
                    {
                      auto module_tok = $1;
                      auto end_tok = $5;

                      if (p.def_level > 0) {
                        p.diagnostic_(diagnostic_level::ERROR, "module definition in method body"s, module_tok);
                        YYERROR;
                      }

                      $$ = p.builder.def_module(module_tok, $2, $4, end_tok);
                      p.lexer_->cmdarg = *($<state_stack>3);
                      p.lexer_->unextend();
                    }
                | kDEF fname
                    {
                      p.def_level++;
                      p.lexer_->extend_static();
                      $<state_stack>$ = new state_stack(p.lexer_->cmdarg);
                      p.lexer_->cmdarg.clear();
                    }
                    f_arglist bodystmt kEND
                    {
                      $$ = p.builder.def_method($1, $2, $4, $5, $6);
                      p.lexer_->cmdarg = *($<state_stack>3);
                      p.lexer_->unextend();
                      p.def_level--;
                    }
                | kDEF singleton dot_or_colon
                    {
                      p.lexer_->set_state_expr_fname();
                    }
                    fname
                    {
                      p.def_level++;
                      p.lexer_->extend_static();
                      $<state_stack>$ = new state_stack(p.lexer_->cmdarg);
                      p.lexer_->cmdarg.clear();
                    }
                    f_arglist bodystmt kEND
                    {
                      $$ = p.builder.def_singleton($1, $2, $3, $5, $7, $8, $9);
                      p.lexer_->cmdarg = *($<state_stack>6);
                      p.lexer_->unextend();
                      p.def_level--;
                    }
                | kBREAK
                    {
                      $$ = p.builder.keyword_break($1, nullptr, nullptr, nullptr);
                    }
                | kNEXT
                    {
                      $$ = p.builder.keyword_next($1, nullptr, nullptr, nullptr);
                    }
                | kREDO
                    {
                      $$ = p.builder.keyword_redo($1);
                    }
                | kRETRY
                    {
                      $$ = p.builder.keyword_retry($1);
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
                      auto elsif_t = $1;
                      auto else_ = $5;
                      $$ = new node_with_token(elsif_t,
						  p.builder.condition(
							  elsif_t, $2, $3, $4,
							  else_ ? else_->token_ : nullptr,
							  else_ ? else_->node_ : nullptr,
							  nullptr)
						);
                    }

        opt_else: none
                    {
                      $$ = nullptr;
                    }
                | kELSE compstmt
                    {
                      $$ = new node_with_token($1, $2);
                    }

         for_var: lhs
                | mlhs

          f_marg: f_norm_arg
                    {
                      $$ = p.builder.arg($1);
                    }
                | tLPAREN f_margs rparen
                    {
                      $$ = p.builder.multi_lhs($1, $2, $3);
                    }

     f_marg_list: f_marg
                    {
                      $$ = make_node_list($1);
                    }
                | f_marg_list tCOMMA f_marg
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }

         f_margs: f_marg_list
                | f_marg_list tCOMMA tSTAR f_norm_arg
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.restarg($3, $4));
                      $$ = list;
                    }
                | f_marg_list tCOMMA tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                      auto args = $1;
                      args->nodes.push_back(p.builder.restarg($3, $4));
                      concat_node_list(args, $6);
                      $$ = args;
                    }
                | f_marg_list tCOMMA tSTAR
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.restarg($3, nullptr));
                      $$ = list;
                    }
                | f_marg_list tCOMMA tSTAR            tCOMMA f_marg_list
                    {
                      auto args = $1;
                      args->nodes.push_back(p.builder.restarg($3, nullptr));
                      concat_node_list(args, $5);
                      $$ = args;
                    }
                |                    tSTAR f_norm_arg
                    {
                      $$ = make_node_list(p.builder.restarg($1, $2));
                    }
                |                    tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                      auto args = $4;
                      args->nodes.insert(args->nodes.begin(), p.builder.restarg($1, $2));
                      $$ = args;
                    }
                |                    tSTAR
                    {
                      $$ = make_node_list(p.builder.restarg($1, nullptr));
                    }
                |                    tSTAR tCOMMA f_marg_list
                    {
                      auto args = $3;
                      args->nodes.insert(args->nodes.begin(), p.builder.restarg($1, nullptr));
                      $$ = args;
                    }

 block_args_tail: f_block_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                | f_block_kwarg opt_f_block_arg
                    {
                      auto args = $1;
                      concat_node_list(args, $2);
                      $$ = args;
                    }
                | f_kwrest opt_f_block_arg
                    {
                      auto args = $1;
                      concat_node_list(args, $2);
                      $$ = args;
                    }
                | f_block_arg
                    {
                      $$ = $1;
                    }

opt_block_args_tail:
                  tCOMMA block_args_tail
                    {
                      $$ = $2;
                    }
                | // nothing
                    {
                      $$ = make_node_list();
                    }

     block_param: f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg              opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $5);
                      concat_node_list(args, $6);
                      $$ = args;
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $5);
                      concat_node_list(args, $7);
                      concat_node_list(args, $8);
                      $$ = args;
                    }
                | f_arg tCOMMA f_block_optarg                                opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA                   f_arg opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $5);
                      concat_node_list(args, $6);
                      $$ = args;
                    }
                | f_arg tCOMMA                       f_rest_arg              opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                | f_arg tCOMMA
                | f_arg tCOMMA                       f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $5);
                      concat_node_list(args, $6);
                      $$ = args;
                    }
                | f_arg                                                      opt_block_args_tail
                    {
                      auto args = $1;
                      auto block_args_tail = $2;

                      if (block_args_tail->nodes.size() == 0 && args->nodes.size() == 1) {
                        $$ = make_node_list(p.builder.procarg0(args->nodes[0]));
                      } else {
                        concat_node_list(args, block_args_tail);
                        $$ = args;
                      }
                    }
                | f_block_optarg tCOMMA              f_rest_arg              opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                | f_block_optarg tCOMMA              f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $5);
                      concat_node_list(args, $6);
                      $$ = args;
                    }
                | f_block_optarg                                             opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $2);
                      $$ = args;
                    }
                | f_block_optarg tCOMMA                                f_arg opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                |                                    f_rest_arg              opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $2);
                      $$ = args;
                    }
                |                                    f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                |                                                                block_args_tail

 opt_block_param: // nothing
                    {
                      $$ = p.builder.args(nullptr, nullptr, nullptr, true);
                    }
                | block_param_def
                    {
                      p.lexer_->set_state_expr_value();
                    }
                  tr_returnsig
                    {
                      auto args = $1;
                      auto return_sig = $3;

                      if (return_sig) {
                        $$ = p.builder.prototype(nullptr, args, return_sig);
                      } else {
                        $$ = args;
                      }
                    }

 block_param_def: tPIPE opt_bv_decl tPIPE
                    {
                      $$ = p.builder.args($1, $2, $3, true);
                    }
                | tOROP
                    {
                      $$ = p.builder.args($1, nullptr, $1, true);
                    }
                | tPIPE block_param opt_bv_decl tPIPE
                    {
                      auto params = $2;
                      concat_node_list(params, $3);
                      $$ = p.builder.args($1, params, $4, true);
                    }

     opt_bv_decl: opt_nl
                    {
                      $$ = make_node_list();
                    }
                | opt_nl tSEMI bv_decls opt_nl
                    {
                      $$ = $3;
                    }

        bv_decls: bvar
                    {
                      $$ = make_node_list($1);
                    }
                | bv_decls tCOMMA bvar
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }

            bvar: tIDENTIFIER
                    {
                      auto ident = $1;
                      p.lexer_->declare(ident->string());
                      $$ = p.builder.shadowarg(ident);
                    }
                | f_bad_arg
                    {
                      $$ = nullptr;
                    }

          lambda:   {
                      p.lexer_->extend_dynamic();
                    }
                  f_larglist
                    {
                      $<state_stack>$ = new state_stack(p.lexer_->cmdarg);
                      p.lexer_->cmdarg.clear();
                    }
                  lambda_body
                    {
                      p.lexer_->cmdarg = *($<state_stack>3);
                      p.lexer_->cmdarg.lexpop();

                      auto delimited_block = $4;
                      delimited_block->args = $2;
                      $$ = delimited_block;
                      p.lexer_->unextend();
                    }

     f_larglist: tLPAREN2 f_args opt_bv_decl tRPAREN
                    {
                      auto args = $2;
                      concat_node_list(args, $3);
                      $$ = p.builder.args($1, args, $4, true);
                    }
                | f_args
                    {
                      $$ = p.builder.args(nullptr, $1, nullptr, true);
                    }

     lambda_body: tLAMBEG compstmt tRCURLY
                    {
                      $$ = new delimited_block($1, nullptr, $2, $3);
                    }
                | kDO_LAMBDA compstmt kEND
                    {
                      $$ = new delimited_block($1, nullptr, $2, $3);
                    }

        do_block: kDO_BLOCK do_body kEND
                    {
                      auto delimited_block = $2;
                      delimited_block->begin = $1;
                      delimited_block->end = $3;
                      $$ = delimited_block;
                    }

      block_call: command do_block
                    {
                      auto delimited_block = $2;
                      $$ = p.builder.block($1,
                          delimited_block->begin,
                          delimited_block->args,
                          delimited_block->body,
                          delimited_block->end
                        );
                    }
                | block_call dot_or_colon operation2 opt_paren_args
                    {
                      auto delimited = $4;
                      $$ = p.builder.call_method($1, $2, $3,
                                  delimited->begin,
                                  delimited->inner,
                                  delimited->end);
                    }
                | block_call dot_or_colon operation2 opt_paren_args brace_block
                    {
                      auto delimited = $4;
                      auto method_call = p.builder.call_method($1, $2, $3,
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                      auto block = $5;
                      $$ = p.builder.block(method_call,
                          block->begin,
                          block->args,
                          block->body,
                          block->end);
                    }
                | block_call dot_or_colon operation2 command_args do_block
                    {
                      auto method_call = p.builder.call_method($1, $2, $3, nullptr, $4, nullptr);
                      auto block = $5;
                      $$ = p.builder.block(method_call, block->begin, block->args, block->body, block->end);
                    }

     method_call: fcall paren_args
                    {
                      auto delimited = $2;
                      $$ = p.builder.call_method(nullptr, nullptr, $1,
                        delimited->begin,
                        delimited->inner,
                        delimited->end);
                    }
                | primary_value call_op operation2 opt_paren_args
                    {
                      auto delimited = $4;
                      $$ = p.builder.call_method($1, $2, $3,
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                    }
                | primary_value tCOLON2 operation2 paren_args
                    {
                      auto delimited = $4;
                      $$ = p.builder.call_method($1, $2, $3,
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                    }
                | primary_value tCOLON2 operation3
                    {
                      $$ = p.builder.call_method($1, $2, $3, nullptr, nullptr, nullptr);
                    }
                | primary_value call_op paren_args
                    {
                      auto delimited = $3;
                      $$ = p.builder.call_method($1, $2, nullptr,
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                    }
                | primary_value tCOLON2 paren_args
                    {
                      auto delimited = $3;
                      $$ = p.builder.call_method($1, $2, nullptr,
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                    }
                | kSUPER paren_args
                    {
                      auto delimited = $2;
                      $$ = p.builder.keyword_super($1,
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                    }
                | kSUPER
                    {
                      $$ = p.builder.keyword_zsuper($1);
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      $$ = p.builder.index($1, $2, $3, $4);
                    }

     brace_block: tLCURLY brace_body tRCURLY
                    {
                      auto block = $2;
                      block->begin = $1;
                      block->end = $3;
                      $$ = block;
                    }
                | kDO do_body kEND
                    {
                      auto block = $2;
                      block->begin = $1;
                      block->end = $3;
                      $$ = block;
                    }

      brace_body:   {
                      p.lexer_->extend_dynamic();
                    }
                    {
                      $<state_stack>$ = new state_stack(p.lexer_->cmdarg);
                      p.lexer_->cmdarg.clear();
                    }
                    opt_block_param compstmt
                    {
                      $$ = new delimited_block(nullptr, $3, $4, nullptr);

                      p.lexer_->unextend();
                      p.lexer_->cmdarg = *($<state_stack>2);
                      p.lexer_->cmdarg.pop();
                    }

         do_body:   {
                      p.lexer_->extend_dynamic();
                    }
                    {
                      $<state_stack>$ = new state_stack(p.lexer_->cmdarg);
                      p.lexer_->cmdarg.clear();
                    }
                    opt_block_param compstmt
                    {
                      $$ = new delimited_block(nullptr, $3, $4, nullptr);
                      p.lexer_->unextend();

                      p.lexer_->cmdarg = *($<state_stack>2);
                      p.lexer_->cmdarg.pop();
                    }

       case_body: kWHEN args then compstmt cases
                    {
                      auto cases = $5;
                      cases->whens->nodes.insert(cases->whens->nodes.begin(), p.builder.when($1, $2, $3, $4));
                      $$ = cases;
                    }

           cases: opt_else
                    {
                      $$ = new case_body($1);
                    }
                | case_body

      opt_rescue: kRESCUE exc_list exc_var then compstmt opt_rescue
                    {
                      auto exc_var = $3;
                      auto exc_list_ = $2;
                      auto exc_list = exc_list_
                        ? p.builder.array(nullptr, exc_list_, nullptr)
                        : nullptr;
                      auto rescues = $6;

                      rescues->nodes.insert(rescues->nodes.begin(),
                        p.builder.rescue_body($1,
                          exc_list,
                          exc_var ? exc_var->token_ : nullptr,
                          exc_var ? exc_var->node_ : nullptr,
                          $4, $5));

                      $$ = rescues;
                    }
                |
                    {
                      $$ = make_node_list();
                    }

        exc_list: arg_value
                    {
                      
                      $$ = make_node_list($1);
                    }
                | mrhs
                | list_none

         exc_var: tASSOC lhs
                    {
                      $$ = new node_with_token($1, $2);
                    }
                | // nothing
                    {
                      $$ = nullptr;
                    }

      opt_ensure: kENSURE compstmt
                    {
                      $$ = new node_with_token($1, $2);
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
                      $$ = p.builder.string_compose(nullptr, $1, nullptr);
                    }

          string: string1
                    {
                      $$ = make_node_list($1);
                    }
                | string string1
                    {
                      auto list = $1;
                      list->nodes.push_back($2);
                      $$ = list;
                    }

         string1: tSTRING_BEG string_contents tSTRING_END
                    {
                      auto str = p.builder.string_compose($1, $2, $3);
                      $$ = p.builder.dedent_string(str, p.lexer_->dedent_level() || 0);
                    }
                | tSTRING
                    {
                      auto str = p.builder.string($1);
                      $$ = p.builder.dedent_string(str, p.lexer_->dedent_level() || 0);
                    }
                | tCHARACTER
                    {
                      $$ = p.builder.character($1);
                    }

         xstring: tXSTRING_BEG xstring_contents tSTRING_END
                    {
                      auto xstr = p.builder.xstring_compose($1, $2, $3);
                      $$ = p.builder.dedent_string(xstr, p.lexer_->dedent_level() || 0);
                    }

          regexp: tREGEXP_BEG regexp_contents tSTRING_END tREGEXP_OPT
                    {
                      auto opts = p.builder.regexp_options($4);
                      $$ = p.builder.regexp_compose($1, $2, $3, opts);
                    }

           words: tWORDS_BEG word_list tSTRING_END
                    {
                      $$ = p.builder.words_compose($1, $2, $3);
                    }

       word_list: // nothing
                    {
                      $$ = make_node_list();
                    }
                | word_list word tSPACE
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.word($2));
                      $$ = list;
                    }

            word: string_content
                    {
                      $$ = make_node_list($1);
                    }
                | word string_content
                    {
                      auto list = $1;
                      list->nodes.push_back($2);
                      $$ = list;
                    }

         symbols: tSYMBOLS_BEG symbol_list tSTRING_END
                    {
                      $$ = p.builder.symbols_compose($1, $2, $3);
                    }

     symbol_list: // nothing
                    {
                      $$ = make_node_list();
                    }
                | symbol_list word tSPACE
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.word($2));
                      $$ = list;
                    }

          qwords: tQWORDS_BEG qword_list tSTRING_END
                    {
                      $$ = p.builder.words_compose($1, $2, $3);
                    }

        qsymbols: tQSYMBOLS_BEG qsym_list tSTRING_END
                    {
                      $$ = p.builder.symbols_compose($1, $2, $3);
                    }

      qword_list: // nothing
                    {
                      $$ = make_node_list();
                    }
                | qword_list tSTRING_CONTENT tSPACE
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.string_internal($2));
                      $$ = list;
                    }

       qsym_list: // nothing
                    {
                      $$ = make_node_list();
                    }
                | qsym_list tSTRING_CONTENT tSPACE
                    {
                      auto list = $1;
                      list->nodes.push_back(p.builder.symbol_internal($2));
                      $$ = list;
                    }

 string_contents: // nothing
                    {
                      $$ = make_node_list();
                    }
                | string_contents string_content
                    {
                      auto list = $1;
                      list->nodes.push_back($2);
                      $$ = list;
                    }

xstring_contents: // nothing
                    {
                      $$ = make_node_list();
                    }
                | xstring_contents string_content
                    {
                      auto list = $1;
                      list->nodes.push_back($2);
                      $$ = list;
                    }

regexp_contents: // nothing
                    {
                      $$ = make_node_list();
                    }
                | regexp_contents string_content
                    {
                      auto list = $1;
                      list->nodes.push_back($2);
                      $$ = list;
                    }

  string_content: tSTRING_CONTENT
                    {
                      $$ = p.builder.string_internal($1);
                    }
                | tSTRING_DVAR string_dvar
                    {
                      $$ = $2;
                    }
                | tSTRING_DBEG
                    {
                      p.lexer_->cond.push(false);
                      p.lexer_->cmdarg.push(false);
                    }
                    compstmt tSTRING_DEND
                    {
                      p.lexer_->cond.lexpop();
                      p.lexer_->cmdarg.lexpop();
                      $$ = p.builder.begin($1, $3, $4);
                    }

     string_dvar: tGVAR
                    {
                      $$ = p.builder.gvar($1);
                    }
                | tIVAR
                    {
                      $$ = p.builder.ivar($1);
                    }
                | tCVAR
                    {
                      $$ = p.builder.cvar($1);
                    }
                | backref


          symbol: tSYMBOL
                    {
                      p.lexer_->set_state_expr_endarg();
                      $$ = p.builder.symbol($1);
                    }

            dsym: tSYMBEG xstring_contents tSTRING_END
                    {
                      p.lexer_->set_state_expr_endarg();
                      $$ = p.builder.symbol_compose($1, $2, $3);
                    }

         numeric: simple_numeric
                    {
                      $$ = $1;
                    }
                | tUMINUS_NUM simple_numeric %prec tLOWEST
                    {
                      $$ = p.builder.negate($1, $2);
                    }

  simple_numeric: tINTEGER
                    {
                      p.lexer_->set_state_expr_endarg();
                      $$ = p.builder.integer($1);
                    }
                | tFLOAT
                    {
                      p.lexer_->set_state_expr_endarg();
                      $$ = p.builder.float_($1);
                    }
                | tRATIONAL
                    {
                      p.lexer_->set_state_expr_endarg();
                      $$ = p.builder.rational($1);
                    }
                | tIMAGINARY
                    {
                      p.lexer_->set_state_expr_endarg();
                      $$ = p.builder.complex($1);
                    }
                | tRATIONAL_IMAGINARY
                    {
                      p.lexer_->set_state_expr_endarg();
                      $$ = p.builder.rational_complex($1);
                    }
                | tFLOAT_IMAGINARY
                    {
                      p.lexer_->set_state_expr_endarg();
                      $$ = p.builder.float_complex($1);
                    }

   user_variable: tIDENTIFIER
                    {
                      $$ = p.builder.ident($1);
                    }
                | tIVAR
                    {
                      $$ = p.builder.ivar($1);
                    }
                | tGVAR
                    {
                      $$ = p.builder.gvar($1);
                    }
                | tCONSTANT
                    {
                      $$ = p.builder.const_($1);
                    }
                | tCVAR
                    {
                      $$ = p.builder.cvar($1);
                    }

keyword_variable: kNIL
                    {
                      $$ = p.builder.nil($1);
                    }
                | kSELF
                    {
                      $$ = p.builder.self($1);
                    }
                | kTRUE
                    {
                      $$ = p.builder.true_($1);
                    }
                | kFALSE
                    {
                      $$ = p.builder.false_($1);
                    }
                | k__FILE__
                    {
                      $$ = p.builder.file_literal($1);
                    }
                | k__LINE__
                    {
                      $$ = p.builder.line_literal($1);
                    }
                | k__ENCODING__
                    {
                      $$ = p.builder.encoding_literal($1);
                    }

         var_ref: user_variable
                    {
                      $$ = p.builder.accessible(&p, $1);
                    }
                | keyword_variable
                    {
                      $$ = p.builder.accessible(&p, $1);
                    }

         var_lhs: user_variable
                    {
                      $$ = p.builder.assignable(&p, $1);
                    }
                | keyword_variable
                    {
                      $$ = p.builder.assignable(&p, $1);
                    }

         backref: tNTH_REF
                    {
                      $$ = p.builder.nth_ref($1);
                    }
                | tBACK_REF
                    {
                      $$ = p.builder.back_ref($1);
                    }

      superclass: tLT
                    {
                      p.lexer_->set_state_expr_value();
                    }
                    expr_value term
                    {
                      $$ = new node_with_token($1, $3);
                    }
                | // nothing
                    {
                      $$ = nullptr;
                    }

tr_methodgenargs: tLBRACK2 tr_gendeclargs rbracket
                    {
                      $$ = p.builder.tr_genargs($1, $2, $3);
                    }
                | // nothing
                    {
                      $$ = nullptr;
                    }

       f_arglist: tr_methodgenargs tLPAREN2 f_args rparen
                    {
                      p.lexer_->set_state_expr_value();
                    }
                  tr_returnsig
                    {
                      auto genargs = $1;
                      auto args = p.builder.args($2, $3, $4, true);
                      auto returnsig = $6;

                      if (genargs || returnsig) {
                        $$ = p.builder.prototype(genargs, args, returnsig);
                      } else {
                        $$ = args;
                      }
                    }
                | tr_methodgenargs
                    {
                      $<boolean>$ = p.lexer_->in_kwarg;
                      p.lexer_->in_kwarg = true;
                    }
                  f_args tr_returnsig term
                    {
                      p.lexer_->in_kwarg = $<boolean>2;

                      auto genargs = $1;
                      auto args = p.builder.args(nullptr, $3, nullptr, true);
                      auto returnsig = $4;

                      if (genargs || returnsig) {
                        $$ = p.builder.prototype(genargs, args, returnsig);
                      } else {
                        $$ = args;
                      }
                    }

       args_tail: f_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                | f_kwarg opt_f_block_arg
                    {
                      auto args = $1;
                      concat_node_list(args, $2);
                      $$ = args;
                    }
                | f_kwrest opt_f_block_arg
                    {
                      auto args = $1;
                      concat_node_list(args, $2);
                      $$ = args;
                    }
                | f_block_arg
                    {
                      $$ = $1;
                    }

   opt_args_tail: tCOMMA args_tail
                    {
                      $$ = $2;
                    }
                | // nothing
                    {
                      $$ = make_node_list();
                    }

          f_args: f_arg tCOMMA f_optarg tCOMMA f_rest_arg              opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $5);
                      concat_node_list(args, $6);
                      $$ = args;
                    }
                | f_arg tCOMMA f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $5);
                      concat_node_list(args, $7);
                      concat_node_list(args, $8);
                      $$ = args;
                    }
                | f_arg tCOMMA f_optarg                                opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                | f_arg tCOMMA f_optarg tCOMMA                   f_arg opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $5);
                      concat_node_list(args, $6);
                      $$ = args;
                    }
                | f_arg tCOMMA                 f_rest_arg              opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                | f_arg tCOMMA                 f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $5);
                      concat_node_list(args, $6);
                      $$ = args;
                    }
                | f_arg                                                opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $2);
                      $$ = args;
                    }
                |              f_optarg tCOMMA f_rest_arg              opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                |              f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $5);
                      concat_node_list(args, $6);
                      $$ = args;
                    }
                |              f_optarg                                opt_args_tail
                    {
                      
                      auto args = $1;
                      concat_node_list(args, $2);
                      $$ = args;
                    }
                |              f_optarg tCOMMA                   f_arg opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                |                              f_rest_arg              opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $2);
                      $$ = args;
                    }
                |                              f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      auto args = $1;
                      concat_node_list(args, $3);
                      concat_node_list(args, $4);
                      $$ = args;
                    }
                |                                                          args_tail
                    {
                      $$ = $1;
                    }
                | // nothing
                    {
                      $$ = make_node_list();
                    }

       f_bad_arg: tIVAR
                    {
                      p.diagnostic_(diagnostic_level::ERROR, "formal argument cannot be an instance variable"s, $1);
                      YYERROR;
                    }
                | tGVAR
                    {
                      p.diagnostic_(diagnostic_level::ERROR, "formal argument cannot be a global variable"s, $1);
                      YYERROR;
                    }
                | tCVAR
                    {
                      p.diagnostic_(diagnostic_level::ERROR, "formal argument cannot be a class variable"s, $1);
                      YYERROR;
                    }

      f_norm_arg: f_bad_arg
                | tIDENTIFIER
                    {
                      auto ident = $1;
                      p.lexer_->declare(ident->string());
                      $$ = ident;
                    }

      f_arg_asgn: f_norm_arg
                    {
                      $$ = $1;
                    }

      f_arg_item: tr_argsig f_arg_asgn
                    {
                      auto argsig = $1;
                      auto arg = p.builder.arg($2);

                      if (argsig) {
                        $$ = p.builder.typed_arg(argsig, arg);
                      } else {
                        $$ = arg;
                      }
                    }
                | tLPAREN f_margs rparen
                    {
                      $$ = p.builder.multi_lhs($1, $2, $3);
                    }

           f_arg: f_arg_item
                    {
                      $$ = make_node_list($1);
                    }
                | f_arg tCOMMA f_arg_item
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }

         f_label: tLABEL
                    {
                      auto label = $1;
                      p.check_kwarg_name(label);
                      p.lexer_->declare(label->string());
                      $$ = label;
                    }

            f_kw: tr_argsig f_label arg_value
                    {
                      auto argsig = $1;
                      auto arg = p.builder.kwoptarg($2, $3);
                      if (argsig) {
                        $$ = p.builder.typed_arg(argsig, arg);
                      } else {
                        $$ = arg;
                      }
                    }
                | tr_argsig f_label
                    {
                      auto argsig = $1;
                      auto arg = p.builder.kwarg($2);
                      if (argsig) {
                        $$ = p.builder.typed_arg(argsig, arg);
                      } else {
                        $$ = arg;
                      }
                    }

      f_block_kw: tr_argsig f_label primary_value
                    {
                      auto argsig = $1;
                      auto arg = p.builder.kwoptarg($2, $3);

                      if (argsig) {
                        $$ = p.builder.typed_arg(argsig, arg);
                      } else {
                        $$ = arg;
                      }
                    }
                | tr_argsig f_label
                    {
                      auto argsig = $1;
                      auto arg = p.builder.kwarg($2);

                      if (argsig) {
                        $$ = p.builder.typed_arg(argsig, arg);
                      } else {
                        $$ = arg;
                      }
                    }

   f_block_kwarg: f_block_kw
                    {
                      $$ = make_node_list($1);
                    }
                | f_block_kwarg tCOMMA f_block_kw
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }

         f_kwarg: f_kw
                    {
                      $$ = make_node_list($1);
                    }
                | f_kwarg tCOMMA f_kw
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }

     kwrest_mark: tPOW | tDSTAR

        f_kwrest: kwrest_mark tIDENTIFIER
                    {
                      auto ident = $2;
                      p.lexer_->declare(ident->string());
                      $$ = make_node_list(p.builder.kwrestarg($1, ident));
                    }
                | kwrest_mark
                    {
                      $$ = make_node_list(p.builder.kwrestarg($1, nullptr));
                    }

           f_opt: tr_argsig f_arg_asgn tEQL arg_value
                    {
                      auto argsig = $1;
                      auto arg = p.builder.optarg($2, $3, $4);
                      if (argsig) {
                        $$ = p.builder.typed_arg(argsig, arg);
                      } else {
                        $$ = arg;
                      }
                    }

     f_block_opt: tr_argsig f_arg_asgn tEQL primary_value
                    {
                      auto argsig = $1;
                      auto arg = p.builder.optarg($2, $3, $4);
                      if (argsig) {
                        $$ = p.builder.typed_arg(argsig, arg);
                      } else {
                        $$ = arg;
                      }
                    }

  f_block_optarg: f_block_opt
                    {
                      $$ = make_node_list($1);
                    }
                | f_block_optarg tCOMMA f_block_opt
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }

        f_optarg: f_opt
                    {
                      $$ = make_node_list($1);
                    }
                | f_optarg tCOMMA f_opt
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }

    restarg_mark: tSTAR2 | tSTAR

      f_rest_arg: tr_argsig restarg_mark tIDENTIFIER
                    {
                      auto argsig = $1;
                      auto ident = $3;

                      p.lexer_->declare(ident->string());

                      auto restarg = p.builder.restarg($2, ident);

                      if (argsig) {
                        restarg = p.builder.typed_arg(argsig, restarg);
                      }

                      $$ = make_node_list(restarg);
                    }
                | tr_argsig restarg_mark
                    {
                      auto argsig = $1;
                      auto restarg = p.builder.restarg($2, nullptr);

                      if (argsig) {
                        restarg = p.builder.typed_arg(argsig, restarg);
                      }

                      $$ = make_node_list(restarg);
                    }

     blkarg_mark: tAMPER2 | tAMPER

     f_block_arg: tr_argsig blkarg_mark tIDENTIFIER
                    {
                      auto argsig = $1;
                      auto ident = $3;

                      p.lexer_->declare(ident->string());

                      auto blockarg = p.builder.blockarg($2, ident);

                      if (argsig) {
                        blockarg = p.builder.typed_arg(argsig, blockarg);
                      }

                      $$ = make_node_list(blockarg);
                    }
                | tr_argsig blkarg_mark
                    {
                      auto argsig = $1;
                      auto blockarg = p.builder.blockarg($2, nullptr);

                      if (argsig) {
                        blockarg = p.builder.typed_arg(argsig, blockarg);
                      }

                      $$ = make_node_list(blockarg);
                    }

 opt_f_block_arg: tCOMMA f_block_arg
                    {
                      $$ = $2;
                    }
                |
                    {
                      $$ = make_node_list();
                    }

       singleton: var_ref
                | tLPAREN2 expr rparen
                    {
                      $$ = $2;
                    }

      assoc_list: // nothing
                    {
                      $$ = make_node_list();
                    }
                | assocs trailer

          assocs: assoc
                    {
                      $$ = make_node_list($1);
                    }
                | assocs tCOMMA assoc
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }

           assoc: arg_value tASSOC arg_value
                    {
                      $$ = p.builder.pair($1, $2, $3);
                    }
                | tLABEL arg_value
                    {
                      $$ = p.builder.pair_keyword($1, $2);
                    }
                | tSTRING_BEG string_contents tLABEL_END arg_value
                    {
                      $$ = p.builder.pair_quoted($1, $2, $3, $4);
                    }
                | tDSTAR arg_value
                    {
                      $$ = p.builder.kwsplat($1, $2);
                    }

       operation: tIDENTIFIER | tCONSTANT | tFID
      operation2: tIDENTIFIER | tCONSTANT | tFID | op
      operation3: tIDENTIFIER | tFID | op
    dot_or_colon: call_op | tCOLON2
         call_op: tDOT
                    {
                      // XXX what is this???
                      // $$ = put(p, [:dot, $1[1]]
                      $$ = $1;
                    }
                | tANDDOT
                    {
                      // XXX what is this???
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
                      $$ = p.builder.const_global($1, $2);
                    }
                | tCONSTANT
                    {
                      $$ = p.builder.const_($1);
                    }
                | tr_cpath tCOLON2 tCONSTANT
                    {
                      $$ = p.builder.const_fetch($1, $2, $3);
                    }

       tr_types: tr_types tCOMMA tr_type
                    {
                      auto list = $1;
                      list->nodes.push_back($3);
                      $$ = list;
                    }
               | tr_type
                    {
                      $$ = make_node_list($1);
                    }

         tr_type: tr_cpath
                    {
                      $$ = p.builder.tr_cpath($1);
                    }
                | tr_cpath tCOLON2 tLBRACK2 tr_types rbracket
                    {
                      $$ = p.builder.tr_geninst($1, $3, $4, $5);
                    }
                | tLBRACK tr_type rbracket
                    {
                      $$ = p.builder.tr_array($1, $2, $3);
                    }
                | tLBRACK tr_type tCOMMA tr_types rbracket
                    {
                      auto types = $4;
                      types->nodes.insert(types->nodes.begin(), $2);
                      $$ = p.builder.tr_tuple($1, types, $5);
                    }
                | tLBRACE tr_type tASSOC tr_type tRCURLY
                    {
                      $$ = p.builder.tr_hash($1, $2, $3, $4, $5);
                    }
                | tLBRACE tr_blockproto tr_returnsig tRCURLY
                    {
                      auto blockproto = $2;
                      auto returnsig = $3;

                      auto prototype = returnsig
                        ? p.builder.prototype(nullptr, blockproto, returnsig)
                        : blockproto;

                      $$ = p.builder.tr_proc($1, prototype, $4);
                    }
                | tTILDE tr_type
                    {
                      $$ = p.builder.tr_nillable($1, $2);
                    }
                | kNIL
                    {
                      $$ = p.builder.tr_nil($1);
                    }
                | tSYMBOL
                    {
                      if ($1->string() == "any") {
                        $$ = p.builder.tr_any($1);
                      } else if ($1->string() == "class") {
                        $$ = p.builder.tr_class($1);
                      } else if ($1->string() == "instance") {
                        $$ = p.builder.tr_instance($1);
                      } else if ($1->string() == "self") {
                        $$ = p.builder.tr_self($1);
                      } else {
                        p.diagnostic_(diagnostic_level::ERROR, "bad type: " + $1->string(), $1);
                        YYERROR;
                      }
                    }
                | tLPAREN tr_union_type rparen
                    {
                      $$ = $2;
                    }

   tr_union_type: tr_union_type tPIPE tr_type
                    {
                      $$ = p.builder.tr_or($1, $3);
                    }
                | tr_type

       tr_argsig: tr_type
                    {
                      $$ = $1;
                      p.lexer_->set_state_expr_beg();
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
                      auto list = $1;
                      list->nodes.push_back(p.builder.tr_gendeclarg($3));
                      $$ = list;
                    }
                | tCONSTANT
                    {
                      $$ = make_node_list(p.builder.tr_gendeclarg($1));
                    }

   tr_blockproto: { p.lexer_->extend_dynamic(); }
                  block_param_def
                    {
                      p.lexer_->unextend();
                      $$ = $2;
                    }

%%
