%{
  #include <ruby_parser/builder.hh>
  #include <ruby_parser/node.hh>
  #include <ruby_parser/token.hh>
  #include <ruby_parser/lexer.hh>

  using namespace ruby_parser;
%}

%pure-parser

%union {
  token* token;
  node* node;
  node_list* list;
  size_t size;
  bool boolean;
  std::stack<bool>* bool_stack;
}

%token <token>
      kCLASS kMODULE kDEF kUNDEF kBEGIN kRESCUE kENSURE kEND kIF kUNLESS
      kTHEN kELSIF kELSE kCASE kWHEN kWHILE kUNTIL kFOR kBREAK kNEXT
      kREDO kRETRY kIN kDO kDO_COND kDO_BLOCK kDO_LAMBDA kRETURN kYIELD kSUPER
      kSELF kNIL kTRUE kFALSE kAND kOR kNOT kIF_MOD kUNLESS_MOD kWHILE_MOD
      kUNTIL_MOD kRESCUE_MOD kALIAS kDEFINED klBEGIN klEND k__LINE__
      k__FILE__ k__ENCODING__ tIDENTIFIER tFID tGVAR tIVAR tCONSTANT
      tLABEL tCVAR tNTH_REF tBACK_REF tSTRING_CONTENT tINTEGER tFLOAT
      tUPLUS tUMINUS tUMINUS_NUM tPOW tCMP tEQ tEQQ tNEQ
      tGEQ tLEQ tANDOP tOROP tMATCH tNMATCH tDOT tDOT2 tDOT3 tAREF
      tASET tLSHFT tRSHFT tCOLON2 tCOLON3 tOP_ASGN tASSOC tLPAREN
      tLPAREN2 tRPAREN tLPAREN_ARG tLBRACK tLBRACK2 tRBRACK tLBRACE
      tLBRACE_ARG tSTAR tSTAR2 tAMPER tAMPER2 tTILDE tPERCENT tDIVIDE
      tDSTAR tPLUS tMINUS tLT tGT tPIPE tBANG tCARET tLCURLY tRCURLY
      tBACK_REF2 tSYMBEG tSTRING_BEG tXSTRING_BEG tREGEXP_BEG tREGEXP_OPT
      tWORDS_BEG tQWORDS_BEG tSYMBOLS_BEG tQSYMBOLS_BEG tSTRING_DBEG
      tSTRING_DVAR tSTRING_END tSTRING_DEND tSTRING tSYMBOL
      tNL tEH tCOLON tCOMMA tSPACE tSEMI tLAMBDA tLAMBEG tCHARACTER
      tRATIONAL tIMAGINARY tLABEL_END tANDDOT

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
  f_arg_asgn
  f_arg_item
  f_arglist
  f_args
  f_block_arg
  f_block_kw
  f_block_opt
  f_block_optarg
  f_kw
  f_kwarg
  f_larglist
  f_marg
  f_opt
  fitem
  for_var
  fsym
  if_tail
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
  opt_args_tail
  opt_block_param
  opt_else
  opt_ensure
  opt_f_block_arg
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
  superclass
  symbol
  symbols
  top_compstmt
  top_stmt
  tr_argsig
  tr_blockproto
  tr_cpath
  tr_gendeclargs
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
  brace_block
  brace_body
  bv_decls
  call_args
  case_body
  cases
  cmd_brace_block
  command_args
  do_block
  do_body
  exc_list
  exc_var
  f_arg
  f_block_kwarg
  f_kwrest
  f_marg_list
  f_margs
  f_optarg
  f_rest_arg
  lambda
  lambda_body
  list_none
  mlhs_basic
  mlhs_head
  mlhs_post
  mrhs
  opt_block_arg
  opt_block_args_tail
  opt_bv_decl
  opt_call_args
  opt_paren_args
  opt_rescue
  paren_args
  qsym_list
  qword_list
  regexp_contents
  stmts
  string
  string_contents
  symbol_list
  top_stmts
  tr_types
  undef_list
  word
  word_list
  xstring_contents

%type <token>
  blkarg_mark
  call_op
  cname
  dot_or_colon
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
  reswords
  restarg_mark
  rparen
  then
  do
  term

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
  std::unique_ptr<T> owned(T* ptr) {
    return new std::unique_ptr<T>(ptr);
  }
%}

%%
         program: top_compstmt

    top_compstmt: top_stmts opt_terms
                    {
                      $$ = builder::compstmt(owned($1)).release();
                    }

       top_stmts: // nothing
                    {
                      $$ = new node_list({});
                    }
                | top_stmt
                    {
                      $$ = new node_list({ owned($1) });
                    }
                | top_stmts terms top_stmt
                    {
                      $1->nodes.push_back(owned($3));
                      $$ = $1;
                    }
                | error top_stmt
                    {
                      $$ = new node_list({ owned($2) });
                    }

        top_stmt: stmt
                | klBEGIN tLCURLY top_compstmt tRCURLY
                    {
                      $$ = builder::preexe(owned($3)).release();
                    }

        bodystmt: compstmt opt_rescue opt_else opt_ensure
                    {
                      auto rescue_bodies = owned($2);
                      auto else_ = owned($3); // TODO needs to be a tuple of (else_t, else)
                      auto ensure = owned($4); // TODO needs to be a tuple of (ensure_t, else)

                      // rescue_bodies     = $2
                      // else_t,   else_   = $3
                      // ensure_t, ensure_ = $4

                      if (rescue_bodies->nodes.size() == 0 && else_ != nullptr) {
                        // TODO diagnostic :warning, :useless_else, nil, else_t
                      }

                      $$ = builder::begin_body(owned($1),
                            std::move(rescue_bodies),
                            /* TODO else_t, */ std::move(else_),
                            /* TODO ensure_t, */ std::move(ensure)).release();
                    }

        compstmt: stmts opt_terms
                    {
                      $$ = builder::compstmt(owned($1)).release();
                    }

           stmts: // nothing
                    {
                      $$ = new node_list({});
                    }
                | stmt_or_begin
                    {
                      $$ = new node_list({ owned($1) });
                    }
                | stmts terms stmt_or_begin
                    {
                      $1->nodes.push_back(owned($3));
                      $$ = $1;
                    }
                | error stmt
                    {
                      $$ = new node_list({ owned($2) });
                    }

   stmt_or_begin: stmt
                | klBEGIN tLCURLY top_compstmt tRCURLY
                    {
                      /* TODO diagnostic :error, :begin_in_method, nil, owned($1) */
                    }

            stmt: kALIAS fitem
                    {
                      // TODO lexer.set_state_expr_fname();
                    }
                    fitem
                    {
                      $$ = builder::alias(owned($2), owned($4)).release();
                    }
                | kALIAS tGVAR tGVAR
                    {
                      $$ = builder::alias(
                        builder::gvar($2->string()),
                        builder::gvar($3->string())).release();
                    }
                | kALIAS tGVAR tBACK_REF
                    {
                      $$ = builder::alias(
                        builder::gvar($2->string()),
                        builder::back_ref($3->string())).release();
                    }
                | kALIAS tGVAR tNTH_REF
                    {
                      // TODO diagnostic :error, :nth_ref_alias, nil, owned($3)
                    }
                | kUNDEF undef_list
                    {
                      $$ = builder::undef_method(owned($2)).release();
                    }
                | stmt kIF_MOD expr_value
                    {
                      $$ = builder::condition_mod(owned($1), nullptr, owned($3)).release();
                    }
                | stmt kUNLESS_MOD expr_value
                    {
                      $$ = builder::condition_mod(nullptr, owned($1), owned($3)).release();
                    }
                | stmt kWHILE_MOD expr_value
                    {
                      $$ = builder::loop_mod(node_type::WHILE, owned($1), owned($3)).release();
                    }
                | stmt kUNTIL_MOD expr_value
                    {
                      $$ = builder::loop_mod(node_type::UNTIL, owned($1), owned($3)).release();
                    }
                | stmt kRESCUE_MOD stmt
                    {
                      auto rescue_body = builder::rescue_body(nullptr, nullptr, owned($3));

                      $$ = builder::begin_body(
                        owned($1),
                        std::make_unique<node_list>(std::move(rescue_body)),
                        nullptr, nullptr).release();
                    }
                | klEND tLCURLY compstmt tRCURLY
                    {
                      $$ = builder::postexe(owned($3)).release();
                    }
                | command_asgn
                | mlhs tEQL command_call
                    {
                      $$ = builder::multi_assign(owned($1), owned($3)).release();
                    }
                | lhs tEQL mrhs
                    {
                      $$ = builder::assign(owned($1), builder::array(owned($3))).release();
                    }
                | mlhs tEQL mrhs_arg
                    {
                      $$ = builder::multi_assign(owned($1), owned($3)).release();
                    }
                | kDEF tIVAR tCOLON tr_type
                    {
                      $$ = builder::tr_ivardecl($2->string(), owned($4)).release();
                    }
                | expr

    command_asgn: lhs tEQL command_rhs
                    {
                      $$ = builder::assign(owned($1), owned($3)).release();
                    }
                | var_lhs tOP_ASGN command_rhs
                    {
                      $$ = builder::op_assign(owned($1), owned($2), owned($3));
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN command_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::index(
                                    owned($1), owned($2), owned($3), owned($4)),
                                  owned($5), owned($6))
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN command_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), owned($2), owned($3)),
                                  owned($4), owned($5))
                    }
                | primary_value call_op tCONSTANT tOP_ASGN command_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), owned($2), owned($3)),
                                  owned($4), owned($5))
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN command_rhs
                    {
                      const  = builder::const_op_assignable(
                                  builder::const_fetch(owned($1), owned($2), owned($3)))
                      $$ = builder::op_assign(const, owned($4), owned($5))
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN command_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), owned($2), owned($3)),
                                  owned($4), owned($5))
                    }
                | backref tOP_ASGN command_rhs
                    {
                      builder::op_assign(owned($1), owned($2), owned($3))
                    }

     command_rhs: command_call %prec tOP_ASGN
                | command_call kRESCUE_MOD stmt
                    {
                      rescue_body = builder::rescue_body(owned($2),
                                        nil, nil, nil,
                                        nil, owned($3))

                      $$ = builder::begin_body(owned($1), [ rescue_body ])
                    }
                | command_asgn

            expr: command_call
                | expr kAND expr
                    {
                      $$ = builder::logical_op(:and, owned($1), owned($2), owned($3))
                    }
                | expr kOR expr
                    {
                      $$ = builder::logical_op(:or, owned($1), owned($2), owned($3))
                    }
                | kNOT opt_nl expr
                    {
                      $$ = builder::not_op(owned($1), nil, owned($3), nil)
                    }
                | tBANG command_call
                    {
                      $$ = builder::not_op(owned($1), nil, owned($2), nil)
                    }
                | arg

      expr_value: expr

    command_call: command
                | block_command

   block_command: block_call
                | block_call dot_or_colon operation2 command_args
                    {
                      $$ = builder::call_method(owned($1), owned($2), owned($3),
                                  nil, owned($4), nil)
                    }

 cmd_brace_block: tLBRACE_ARG brace_body tRCURLY
                    {
                      $$ = [ owned($1), *owned($2), owned($3) ]
                    }

           fcall: operation

         command: fcall command_args %prec tLOWEST
                    {
                      $$ = builder::call_method(nil, nil, owned($1),
                                  nil, owned($2), nil)
                    }
                | fcall command_args cmd_brace_block
                    {
                      method_call = builder::call_method(nil, nil, owned($1),
                                        nil, owned($2), nil)

                      begin_t, args, body, end_t = $3
                      result      = builder::block(method_call,
                                      begin_t, args, body, end_t)
                    }
                | primary_value call_op operation2 command_args %prec tLOWEST
                    {
                      $$ = builder::call_method(owned($1), owned($2), owned($3),
                                  nil, owned($4), nil)
                    }
                | primary_value call_op operation2 command_args cmd_brace_block
                    {
                      method_call = builder::call_method(owned($1), owned($2), owned($3),
                                        nil, owned($4), nil)

                      begin_t, args, body, end_t = $5
                      result      = builder::block(method_call,
                                      begin_t, args, body, end_t)
                    }
                | primary_value tCOLON2 operation2 command_args %prec tLOWEST
                    {
                      $$ = builder::call_method(owned($1), owned($2), owned($3),
                                  nil, owned($4), nil)
                    }
                | primary_value tCOLON2 operation2 command_args cmd_brace_block
                    {
                      method_call = builder::call_method(owned($1), owned($2), owned($3),
                                        nil, owned($4), nil)

                      begin_t, args, body, end_t = $5
                      result      = builder::block(method_call,
                                      begin_t, args, body, end_t)
                    }
                | kSUPER command_args
                    {
                      $$ = builder::keyword_cmd(:super, owned($1),
                                  nil, owned($2), nil)
                    }
                | kYIELD command_args
                    {
                      $$ = builder::keyword_cmd(:yield, owned($1),
                                  nil, owned($2), nil)
                    }
                | kRETURN call_args
                    {
                      $$ = builder::keyword_cmd(:return, owned($1),
                                  nil, owned($2), nil)
                    }
                | kBREAK call_args
                    {
                      $$ = builder::keyword_cmd(:break, owned($1),
                                  nil, owned($2), nil)
                    }
                | kNEXT call_args
                    {
                      $$ = builder::keyword_cmd(:next, owned($1),
                                  nil, owned($2), nil)
                    }

            mlhs: mlhs_basic
                    {
                      $$ = builder::multi_lhs(nil, owned($1), nil)
                    }
                | tLPAREN mlhs_inner rparen
                    {
                      $$ = builder::begin(owned($1), owned($2), owned($3))
                    }

      mlhs_inner: mlhs_basic
                    {
                      $$ = builder::multi_lhs(nil, owned($1), nil)
                    }
                | tLPAREN mlhs_inner rparen
                    {
                      $$ = builder::multi_lhs(owned($1), owned($2), owned($3))
                    }

      mlhs_basic: mlhs_head
                | mlhs_head mlhs_item
                    {
                      $$ = $1.
                                  push(owned($2))
                    }
                | mlhs_head tSTAR mlhs_node
                    {
                      $$ = $1.
                                  push(builder::splat(owned($2), owned($3)))
                    }
                | mlhs_head tSTAR mlhs_node tCOMMA mlhs_post
                    {
                      $$ = $1.
                                  push(builder::splat(owned($2), owned($3))).
                                  concat(owned($5))
                    }
                | mlhs_head tSTAR
                    {
                      $$ = $1.
                                  push(builder::splat(owned($2)))
                    }
                | mlhs_head tSTAR tCOMMA mlhs_post
                    {
                      $$ = $1.
                                  push(builder::splat(owned($2))).
                                  concat(owned($4))
                    }
                | tSTAR mlhs_node
                    {
                      $$ = [ builder::splat(owned($1), owned($2)) ]
                    }
                | tSTAR mlhs_node tCOMMA mlhs_post
                    {
                      $$ = [ builder::splat(owned($1), owned($2)),
                                 *owned($4) ]
                    }
                | tSTAR
                    {
                      $$ = [ builder::splat(owned($1)) ]
                    }
                | tSTAR tCOMMA mlhs_post
                    {
                      $$ = [ builder::splat(owned($1)),
                                 *owned($3) ]
                    }

       mlhs_item: mlhs_node
                | tLPAREN mlhs_inner rparen
                    {
                      $$ = builder::begin(owned($1), owned($2), owned($3))
                    }

       mlhs_head: mlhs_item tCOMMA
                    {
                      $$ = [ owned($1) ]
                    }
                | mlhs_head mlhs_item tCOMMA
                    {
                      $$ = $1 << owned($2)
                    }

       mlhs_post: mlhs_item
                    {
                      $$ = [ owned($1) ]
                    }
                | mlhs_post tCOMMA mlhs_item
                    {
                      $$ = $1 << owned($3)
                    }

       mlhs_node: user_variable
                    {
                      $$ = builder::assignable(owned($1))
                    }
                | keyword_variable
                    {
                      $$ = builder::assignable(owned($1))
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      $$ = builder::index_asgn(owned($1), owned($2), owned($3), owned($4))
                    }
                | primary_value call_op tIDENTIFIER
                    {
                      $$ = builder::attr_asgn(owned($1), owned($2), owned($3))
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                      $$ = builder::attr_asgn(owned($1), owned($2), owned($3))
                    }
                | primary_value call_op tCONSTANT
                    {
                      $$ = builder::attr_asgn(owned($1), owned($2), owned($3))
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = builder::assignable(
                                  builder::const_fetch(owned($1), owned($2), owned($3)))
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = builder::assignable(
                                  builder::const_global(owned($1), owned($2)))
                    }
                | backref
                    {
                      $$ = builder::assignable(owned($1))
                    }

             lhs: user_variable
                    {
                      $$ = builder::assignable(owned($1))
                    }
                | keyword_variable
                    {
                      $$ = builder::assignable(owned($1))
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      $$ = builder::index_asgn(owned($1), owned($2), owned($3), owned($4))
                    }
                | primary_value call_op tIDENTIFIER
                    {
                      $$ = builder::attr_asgn(owned($1), owned($2), owned($3))
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                      $$ = builder::attr_asgn(owned($1), owned($2), owned($3))
                    }
                | primary_value call_op tCONSTANT
                    {
                      $$ = builder::attr_asgn(owned($1), owned($2), owned($3))
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = builder::assignable(
                                  builder::const_fetch(owned($1), owned($2), owned($3)))
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = builder::assignable(
                                  builder::const_global(owned($1), owned($2)))
                    }
                | backref
                    {
                      $$ = builder::assignable(owned($1))
                    }

           cname: tIDENTIFIER
                    {
                      diagnostic :error, :module_name_const, nil, owned($1)
                    }
                | tCONSTANT

           cpath: tCOLON3 cname
                    {
                      $$ = builder::const_global(owned($1), owned($2))
                    }
                | cname
                    {
                      $$ = builder::const(owned($1))
                    }
                | primary_value tCOLON2 tLBRACK2 tr_gendeclargs rbracket
                    {
                      $$ = builder::tr_gendecl(owned($1), owned($3), owned($4), owned($5))
                    }
                | primary_value tCOLON2 cname
                    {
                      $$ = builder::const_fetch(owned($1), owned($2), owned($3))
                    }

           fname: tIDENTIFIER | tCONSTANT | tFID
                | op
                | reswords

            fsym: fname
                    {
                      $$ = builder::symbol(owned($1))
                    }
                | symbol

           fitem: fsym
                | dsym

      undef_list: fitem
                    {
                      $$ = [ owned($1) ]
                    }
                | undef_list tCOMMA
                    {
                      @lexer.state = :expr_fname
                    }
                    fitem
                    {
                      $$ = $1 << owned($4)
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
                      $$ = builder::assign(owned($1), owned($3))
                    }
                | var_lhs tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(owned($1), owned($2), owned($3))
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::index(
                                    owned($1), owned($2), owned($3), owned($4)),
                                  owned($5), owned($6))
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), owned($2), owned($3)),
                                  owned($4), owned($5))
                    }
                | primary_value call_op tCONSTANT tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), owned($2), owned($3)),
                                  owned($4), owned($5))
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), owned($2), owned($3)),
                                  owned($4), owned($5))
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN arg_rhs
                    {
                      const  = builder::const_op_assignable(
                                  builder::const_fetch(owned($1), owned($2), owned($3)))
                      $$ = builder::op_assign(const, owned($4), owned($5))
                    }
                | tCOLON3 tCONSTANT tOP_ASGN arg_rhs
                    {
                      const  = builder::const_op_assignable(
                                  builder::const_global(owned($1), owned($2)))
                      $$ = builder::op_assign(const, owned($3), owned($4))
                    }
                | backref tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(owned($1), owned($2), owned($3))
                    }
                | arg tDOT2 arg
                    {
                      $$ = builder::range_inclusive(owned($1), owned($2), owned($3))
                    }
                | arg tDOT3 arg
                    {
                      $$ = builder::range_exclusive(owned($1), owned($2), owned($3))
                    }
                | arg tPLUS arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tMINUS arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tSTAR2 arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tDIVIDE arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tPERCENT arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tPOW arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | tUMINUS_NUM simple_numeric tPOW arg
                    {
                      $$ = builder::unary_op(owned($1),
                                  builder::binary_op(
                                    owned($2), owned($3), owned($4)))
                    }
                | tUPLUS arg
                    {
                      $$ = builder::unary_op(owned($1), owned($2))
                    }
                | tUMINUS arg
                    {
                      $$ = builder::unary_op(owned($1), owned($2))
                    }
                | arg tPIPE arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tCARET arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tAMPER2 arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tCMP arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tGT arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tGEQ arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tLT arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tLEQ arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tEQ arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tEQQ arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tNEQ arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tMATCH arg
                    {
                      $$ = builder::match_op(owned($1), owned($2), owned($3))
                    }
                | arg tNMATCH arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | tBANG arg
                    {
                      $$ = builder::not_op(owned($1), nil, owned($2), nil)
                    }
                | tTILDE arg
                    {
                      $$ = builder::unary_op(owned($1), owned($2))
                    }
                | arg tLSHFT arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tRSHFT arg
                    {
                      $$ = builder::binary_op(owned($1), owned($2), owned($3))
                    }
                | arg tANDOP arg
                    {
                      $$ = builder::logical_op(:and, owned($1), owned($2), owned($3))
                    }
                | arg tOROP arg
                    {
                      $$ = builder::logical_op(:or, owned($1), owned($2), owned($3))
                    }
                | kDEFINED opt_nl arg
                    {
                      $$ = builder::keyword_cmd(:defined?, owned($1), nil, [ owned($3) ], nil)
                    }
                | arg tEH arg opt_nl tCOLON arg
                    {
                      $$ = builder::ternary(owned($1), owned($2),
                                                owned($3), owned($5), owned($6))
                    }
                | primary

       arg_value: arg

       aref_args: list_none
                | args trailer
                | args tCOMMA assocs trailer
                    {
                      $$ = $1 << builder::associate(nil, owned($3), nil)
                    }
                | assocs trailer
                    {
                      $$ = [ builder::associate(nil, owned($1), nil) ]
                    }

         arg_rhs: arg %prec tOP_ASGN
                | arg kRESCUE_MOD arg
                    {
                      rescue_body = builder::rescue_body(owned($2),
                                        nil, nil, nil,
                                        nil, owned($3))

                      $$ = builder::begin_body(owned($1), [ rescue_body ])
                    }

      paren_args: tLPAREN2 opt_call_args rparen
                    {
                      $$ = val
                    }

  opt_paren_args: // nothing
                    {
                      $$ = [ nil, [], nil ]
                    }
                | paren_args

   opt_call_args: // nothing
                    {
                      $$ = []
                    }
                | call_args
                | args tCOMMA
                | args tCOMMA assocs tCOMMA
                    {
                      $$ = $1 << builder::associate(nil, owned($3), nil)
                    }
                | assocs tCOMMA
                    {
                      $$ = [ builder::associate(nil, owned($1), nil) ]
                    }

       call_args: command
                    {
                      $$ = [ owned($1) ]
                    }
                | args opt_block_arg
                    {
                      $$ = $1.concat(owned($2))
                    }
                | assocs opt_block_arg
                    {
                      $$ = [ builder::associate(nil, owned($1), nil) ]
                      result.concat(owned($2))
                    }
                | args tCOMMA assocs opt_block_arg
                    {
                      assocs = builder::associate(nil, owned($3), nil)
                      $$ = $1 << assocs
                      result.concat(owned($4))
                    }
                | block_arg
                    {
                      $$ =  [ owned($1) ]
                    }

    command_args:   {
                      $<bool_stack>$ = new std::stack<bool>(lexer.cmdarg)
                      @lexer.cmdarg.push(true)
                    }
                  call_args
                    {
                      std::stack<bool>* cmdarg = $<bool_stack>1;
                      @lexer.cmdarg = cmdarg;
                      delete cmdarg;

                      $$ = $2
                    }

       block_arg: tAMPER arg_value
                    {
                      $$ = builder::block_pass(owned($1), owned($2))
                    }

   opt_block_arg: tCOMMA block_arg
                    {
                      $$ = [ owned($2) ]
                    }
                | // nothing
                    {
                      $$ = []
                    }

            args: arg_value
                    {
                      $$ = [ owned($1) ]
                    }
                | tSTAR arg_value
                    {
                      $$ = [ builder::splat(owned($1), owned($2)) ]
                    }
                | args tCOMMA arg_value
                    {
                      $$ = $1 << owned($3)
                    }
                | args tCOMMA tSTAR arg_value
                    {
                      $$ = $1 << builder::splat(owned($3), owned($4))
                    }

        mrhs_arg: mrhs
                    {
                      $$ = builder::array(nil, owned($1), nil)
                    }
                | arg_value

            mrhs: args tCOMMA arg_value
                    {
                      $$ = $1 << owned($3)
                    }
                | args tCOMMA tSTAR arg_value
                    {
                      $$ = $1 << builder::splat(owned($3), owned($4))
                    }
                | tSTAR arg_value
                    {
                      $$ = [ builder::splat(owned($1), owned($2)) ]
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
                      $$ = builder::call_method(nil, nil, owned($1))
                    }
                | kBEGIN
                    {
                      $<bool_stack>$ = @lexer.cmdarg.dup
                      @lexer.cmdarg.clear
                    }
                    bodystmt kEND
                    {
                      @lexer.cmdarg = $<bool_stack>2

                      $$ = builder::begin_keyword(owned($1), owned($3), owned($4))
                    }
                | tLPAREN_ARG
                    {
                      $<bool_stack>$ = new std::stack<bool>(lexer.cmdarg);
                      @lexer.cmdarg.clear
                    }
                    stmt
                    {
                      @lexer.state = :expr_endarg
                    }
                    rparen
                    {
                      std::stack<bool>* cmdarg = $<bool_stack>2;
                      lexer.cmdarg = *cmdarg;
                      delete cmdarg;

                      $$ = builder::begin(owned($1), owned($3), owned($5))
                    }
                | tLPAREN_ARG
                    {
                      @lexer.state = :expr_endarg
                    }
                    opt_nl tRPAREN
                    {
                      $$ = builder::begin(owned($1), nil, owned($4))
                    }
                | tLPAREN compstmt tRPAREN
                    {
                      $$ = builder::begin(owned($1), owned($2), owned($3))
                    }
                | tLPAREN expr tCOLON tr_type tRPAREN
                    {
                      $$ = builder::tr_cast(owned($1), owned($2), owned($3), owned($4), owned($5))
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = builder::const_fetch(owned($1), owned($2), owned($3))
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = builder::const_global(owned($1), owned($2))
                    }
                | tLBRACK aref_args tRBRACK
                    {
                      $$ = builder::array(owned($1), owned($2), owned($3))
                    }
                | tLBRACE assoc_list tRCURLY
                    {
                      $$ = builder::associate(owned($1), owned($2), owned($3))
                    }
                | kRETURN
                    {
                      $$ = builder::keyword_cmd(:return, owned($1))
                    }
                | kYIELD tLPAREN2 call_args rparen
                    {
                      $$ = builder::keyword_cmd(:yield, owned($1), owned($2), owned($3), owned($4))
                    }
                | kYIELD tLPAREN2 rparen
                    {
                      $$ = builder::keyword_cmd(:yield, owned($1), owned($2), [], owned($3))
                    }
                | kYIELD
                    {
                      $$ = builder::keyword_cmd(:yield, owned($1))
                    }
                | kDEFINED opt_nl tLPAREN2 expr rparen
                    {
                      $$ = builder::keyword_cmd(:defined?, owned($1),
                                                    owned($3), [ owned($4) ], owned($5))
                    }
                | kNOT tLPAREN2 expr rparen
                    {
                      $$ = builder::not_op(owned($1), owned($2), owned($3), owned($4))
                    }
                | kNOT tLPAREN2 rparen
                    {
                      $$ = builder::not_op(owned($1), owned($2), nil, owned($3))
                    }
                | fcall brace_block
                    {
                      method_call = builder::call_method(nil, nil, owned($1))

                      begin_t, args, body, end_t = $2
                      result      = builder::block(method_call,
                                      begin_t, args, body, end_t)
                    }
                | method_call
                | method_call brace_block
                    {
                      begin_t, args, body, end_t = $2
                      result      = builder::block(owned($1),
                                      begin_t, args, body, end_t)
                    }
                | tLAMBDA lambda
                    {
                      lambda_call = builder::call_lambda(owned($1))

                      args, (begin_t, body, end_t) = $2
                      result      = builder::block(lambda_call,
                                      begin_t, args, body, end_t)
                    }
                | kIF expr_value then compstmt if_tail kEND
                    {
                      $$ = builder::condition(owned($1), owned($2), owned($3),
                                                  owned($4), owned($5), owned($6))
                    }
                | kUNLESS expr_value then compstmt opt_else kEND
                    {
                      $$ = builder::condition(owned($1), owned($2), owned($3), owned($5), owned($4), owned($6))
                    }
                | kWHILE
                    {
                      @lexer.cond.push(true)
                    }
                    expr_value do
                    {
                      @lexer.cond.pop
                    }
                    compstmt kEND
                    {
                      $$ = builder::loop(:while, owned($1), owned($3), owned($4),
                                             owned($6), owned($7))
                    }
                | kUNTIL
                    {
                      @lexer.cond.push(true)
                    }
                    expr_value do
                    {
                      @lexer.cond.pop
                    }
                    compstmt kEND
                    {
                      $$ = builder::loop(:until, owned($1), owned($3), owned($4),
                                             owned($6), owned($7))
                    }
                | kCASE expr_value opt_terms case_body kEND
                    {
                      *when_bodies, (else_t, else_body) = *owned($4)

                      $$ = builder::case(owned($1), owned($2),
                                             when_bodies, else_t, else_body,
                                             owned($5))
                    }
                | kCASE            opt_terms case_body kEND
                    {
                      *when_bodies, (else_t, else_body) = *owned($3)

                      $$ = builder::case(owned($1), nil,
                                             when_bodies, else_t, else_body,
                                             owned($4))
                    }
                | kFOR for_var kIN
                    {
                      @lexer.cond.push(true)
                    }
                    expr_value do
                    {
                      @lexer.cond.pop
                    }
                    compstmt kEND
                    {
                      $$ = builder::for(owned($1), owned($2),
                                            owned($3), owned($5),
                                            owned($6), owned($8), owned($9))
                    }
                | kCLASS cpath superclass
                    {
                      @static_env.extend_static
                      @lexer.push_cmdarg
                    }
                    bodystmt kEND
                    {
                      if in_def?
                        diagnostic :error, :class_in_def, nil, owned($1)
                      end

                      lt_t, superclass = $3
                      $$ = builder::def_class(owned($1), owned($2),
                                                  lt_t, superclass,
                                                  owned($5), owned($6))

                      @lexer.pop_cmdarg
                      @static_env.unextend
                    }
                | kCLASS tLSHFT expr term
                    {
                      $<size>$ = @def_level
                      @def_level = 0

                      @static_env.extend_static
                      @lexer.push_cmdarg
                    }
                    bodystmt kEND
                    {
                      $$ = builder::def_sclass(owned($1), owned($2), owned($3),
                                                   owned($6), owned($7))

                      @lexer.pop_cmdarg
                      @static_env.unextend

                      @def_level = $<size>5;
                    }
                | kMODULE cpath
                    {
                      @static_env.extend_static
                      @lexer.push_cmdarg
                    }
                    bodystmt kEND
                    {
                      if in_def?
                        diagnostic :error, :module_in_def, nil, owned($1)
                      end

                      $$ = builder::def_module(owned($1), owned($2),
                                                   owned($4), owned($5))

                      @lexer.pop_cmdarg
                      @static_env.unextend
                    }
                | kDEF fname
                    {
                      @def_level += 1
                      @static_env.extend_static
                      @lexer.push_cmdarg
                    }
                    f_arglist bodystmt kEND
                    {
                      $$ = builder::def_method(owned($1), owned($2),
                                  owned($4), owned($5), owned($6))

                      @lexer.pop_cmdarg
                      @static_env.unextend
                      @def_level -= 1
                    }
                | kDEF singleton dot_or_colon
                    {
                      @lexer.state = :expr_fname
                    }
                    fname
                    {
                      @def_level += 1
                      @static_env.extend_static
                      @lexer.push_cmdarg
                    }
                    f_arglist bodystmt kEND
                    {
                      $$ = builder::def_singleton(owned($1), owned($2), owned($3),
                                  owned($5), owned($7), owned($8), owned($9))

                      @lexer.pop_cmdarg
                      @static_env.unextend
                      @def_level -= 1
                    }
                | kBREAK
                    {
                      $$ = builder::keyword_cmd(:break, owned($1))
                    }
                | kNEXT
                    {
                      $$ = builder::keyword_cmd(:next, owned($1))
                    }
                | kREDO
                    {
                      $$ = builder::keyword_cmd(:redo, owned($1))
                    }
                | kRETRY
                    {
                      $$ = builder::keyword_cmd(:retry, owned($1))
                    }

   primary_value: primary

            then: term
                | kTHEN
                | term kTHEN
                    {
                      $$ = $2
                    }

              do: term
                | kDO_COND

         if_tail: opt_else
                | kELSIF expr_value then compstmt if_tail
                    {
                      $$ = builder::condition(owned($1), owned($2), owned($3),
                                              owned($4), owned($5), nullptr)
                    }

        opt_else: none
                | kELSE compstmt
                    {
                      $$ = $2
                    }

         for_var: lhs
                | mlhs

          f_marg: f_norm_arg
                    {
                      $$ = builder::arg(owned($1))
                    }
                | tLPAREN f_margs rparen
                    {
                      $$ = builder::multi_lhs(owned($1), owned($2), owned($3))
                    }

     f_marg_list: f_marg
                    {
                      $$ = [ owned($1) ]
                    }
                | f_marg_list tCOMMA f_marg
                    {
                      $$ = $1 << owned($3)
                    }

         f_margs: f_marg_list
                | f_marg_list tCOMMA tSTAR f_norm_arg
                    {
                      $$ = $1.
                                  push(builder::restarg(owned($3), owned($4)))
                    }
                | f_marg_list tCOMMA tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                      $$ = $1.
                                  push(builder::restarg(owned($3), owned($4))).
                                  concat(owned($6))
                    }
                | f_marg_list tCOMMA tSTAR
                    {
                      $$ = $1.
                                  push(builder::restarg(owned($3)))
                    }
                | f_marg_list tCOMMA tSTAR            tCOMMA f_marg_list
                    {
                      $$ = $1.
                                  push(builder::restarg(owned($3))).
                                  concat(owned($5))
                    }
                |                    tSTAR f_norm_arg
                    {
                      $$ = [ builder::restarg(owned($1), owned($2)) ]
                    }
                |                    tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                      $$ = [ builder::restarg(owned($1), owned($2)),
                                 *owned($4) ]
                    }
                |                    tSTAR
                    {
                      $$ = [ builder::restarg(owned($1)) ]
                    }
                |                    tSTAR tCOMMA f_marg_list
                    {
                      $$ = [ builder::restarg(owned($1)),
                                 *owned($3) ]
                    }

 block_args_tail: f_block_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                      $$ = $1.concat(owned($3)).concat(owned($4))
                    }
                | f_block_kwarg opt_f_block_arg
                    {
                      $$ = $1.concat(owned($2))
                    }
                | f_kwrest opt_f_block_arg
                    {
                      $$ = $1.concat(owned($2))
                    }
                | f_block_arg
                    {
                      $$ = [ owned($1) ]
                    }

opt_block_args_tail:
                  tCOMMA block_args_tail
                    {
                      $$ = $2
                    }
                | // nothing
                    {
                      $$ = []
                    }

     block_param: f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg              opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($5)).
                                  concat(owned($6))
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($5)).
                                  concat(owned($7)).
                                  concat(owned($8))
                    }
                | f_arg tCOMMA f_block_optarg                                opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($4))
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA                   f_arg opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($5)).
                                  concat(owned($6))
                    }
                | f_arg tCOMMA                       f_rest_arg              opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($4))
                    }
                | f_arg tCOMMA
                | f_arg tCOMMA                       f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($5)).
                                  concat(owned($6))
                    }
                | f_arg                                                      opt_block_args_tail
                    {
                      if owned($2).empty? && owned($1).size == 1
                        $$ = [builder::procarg0(owned($1)[0])]
                      else
                        $$ = $1.concat(owned($2))
                      end
                    }
                | f_block_optarg tCOMMA              f_rest_arg              opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($4))
                    }
                | f_block_optarg tCOMMA              f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($5)).
                                  concat(owned($6))
                    }
                | f_block_optarg                                             opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($2))
                    }
                | f_block_optarg tCOMMA                                f_arg opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($4))
                    }
                |                                    f_rest_arg              opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($2))
                    }
                |                                    f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($4))
                    }
                |                                                                block_args_tail

 opt_block_param: // nothing
                    {
                      $$ = builder::args(nil, [], nil)
                    }
                | block_param_def
                    {
                      @lexer.state = :expr_value
                    }
                  tr_returnsig
                    {
                      $$ = $1

                      if owned($3)
                        $$ = builder::prototype(nil, result, owned($3))
                      end
                    }

 block_param_def: tPIPE opt_bv_decl tPIPE
                    {
                      $$ = builder::args(owned($1), owned($2), owned($3))
                    }
                | tOROP
                    {
                      $$ = builder::args(owned($1), [], owned($1))
                    }
                | tPIPE block_param opt_bv_decl tPIPE
                    {
                      $$ = builder::args(owned($1), owned($2).concat(owned($3)), owned($4))
                    }

     opt_bv_decl: opt_nl
                    {
                      $$ = []
                    }
                | opt_nl tSEMI bv_decls opt_nl
                    {
                      $$ = $3
                    }

        bv_decls: bvar
                    {
                      $$ = [ owned($1) ]
                    }
                | bv_decls tCOMMA bvar
                    {
                      $$ = $1 << owned($3)
                    }

            bvar: tIDENTIFIER
                    {
                      @static_env.declare owned($1)[0]
                      $$ = builder::shadowarg(owned($1))
                    }
                | f_bad_arg
                    {
                      $$ = nullptr;
                    }

          lambda:   {
                      @static_env.extend_dynamic
                    }
                  f_larglist
                    {
                      $<bool_stack>$ = new std::stack<bool>(lexer.cmdarg);
                      @lexer.cmdarg.clear
                    }
                  lambda_body
                    {
                      std::stack<bool>* cmdarg = $<bool_stack>3;
                      @lexer.cmdarg = *cmdarg;
                      delete cmdarg;
                      @lexer.cmdarg.lexpop

                      $$ = [ owned($2), owned($4) ]

                      @static_env.unextend
                    }

     f_larglist: tLPAREN2 f_args opt_bv_decl tRPAREN
                    {
                      $$ = builder::args(owned($1), owned($2).concat(owned($3)), owned($4))
                    }
                | f_args
                    {
                      $$ = builder::args(nil, owned($1), nil)
                    }

     lambda_body: tLAMBEG compstmt tRCURLY
                    {
                      $$ = [ owned($1), owned($2), owned($3) ]
                    }
                | kDO_LAMBDA compstmt kEND
                    {
                      $$ = [ owned($1), owned($2), owned($3) ]
                    }

        do_block: kDO_BLOCK do_body kEND
                    {
                      $$ = [ owned($1), *owned($2), owned($3) ]
                    }

      block_call: command do_block
                    {
                      begin_t, block_args, body, end_t = $2
                      result      = builder::block(owned($1),
                                      begin_t, block_args, body, end_t)
                    }
                | block_call dot_or_colon operation2 opt_paren_args
                    {
                      lparen_t, args, rparen_t = $4
                      $$ = builder::call_method(owned($1), owned($2), owned($3),
                                  lparen_t, args, rparen_t)
                    }
                | block_call dot_or_colon operation2 opt_paren_args brace_block
                    {
                      lparen_t, args, rparen_t = $4
                      method_call = builder::call_method(owned($1), owned($2), owned($3),
                                      lparen_t, args, rparen_t)

                      begin_t, args, body, end_t = $5
                      result      = builder::block(method_call,
                                      begin_t, args, body, end_t)
                    }
                | block_call dot_or_colon operation2 command_args do_block
                    {
                      method_call = builder::call_method(owned($1), owned($2), owned($3),
                                      nil, owned($4), nil)

                      begin_t, args, body, end_t = $5
                      result      = builder::block(method_call,
                                      begin_t, args, body, end_t)
                    }

     method_call: fcall paren_args
                    {
                      lparen_t, args, rparen_t = $2
                      $$ = builder::call_method(nil, nil, owned($1),
                                  lparen_t, args, rparen_t)
                    }
                | primary_value call_op operation2 opt_paren_args
                    {
                      lparen_t, args, rparen_t = $4
                      $$ = builder::call_method(owned($1), owned($2), owned($3),
                                  lparen_t, args, rparen_t)
                    }
                | primary_value tCOLON2 operation2 paren_args
                    {
                      lparen_t, args, rparen_t = $4
                      $$ = builder::call_method(owned($1), owned($2), owned($3),
                                  lparen_t, args, rparen_t)
                    }
                | primary_value tCOLON2 operation3
                    {
                      $$ = builder::call_method(owned($1), owned($2), owned($3))
                    }
                | primary_value call_op paren_args
                    {
                      lparen_t, args, rparen_t = $3
                      $$ = builder::call_method(owned($1), owned($2), nil,
                                  lparen_t, args, rparen_t)
                    }
                | primary_value tCOLON2 paren_args
                    {
                      lparen_t, args, rparen_t = $3
                      $$ = builder::call_method(owned($1), owned($2), nil,
                                  lparen_t, args, rparen_t)
                    }
                | kSUPER paren_args
                    {
                      lparen_t, args, rparen_t = $2
                      $$ = builder::keyword_cmd(:super, owned($1),
                                  lparen_t, args, rparen_t)
                    }
                | kSUPER
                    {
                      $$ = builder::keyword_cmd(:zsuper, owned($1))
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      $$ = builder::index(owned($1), owned($2), owned($3), owned($4))
                    }

     brace_block: tLCURLY brace_body tRCURLY
                    {
                      $$ = [ owned($1), *owned($2), owned($3) ]
                    }
                | kDO do_body kEND
                    {
                      $$ = [ owned($1), *owned($2), owned($3) ]
                    }

      brace_body:   {
                      @static_env.extend_dynamic
                    }
                    {
                      $<bool_stack>$ = new std::stack<bool>(lexer.cmdarg);
                      @lexer.cmdarg.clear
                    }
                    opt_block_param compstmt
                    {
                      $$ = [ owned($3), owned($4) ]

                      @static_env.unextend
                      std::stack<bool_stack>* cmdarg = $<bool_stack>2;
                      @lexer.cmdarg = *cmdarg;
                      delete cmdarg;
                      @lexer.cmdarg.pop
                    }

         do_body:   {
                      @static_env.extend_dynamic
                    }
                    {
                      $<bool_stack>$ = new std::stack<bool>(lexer.cmdarg);
                      @lexer.cmdarg.clear
                    }
                    opt_block_param compstmt
                    {
                      $$ = [ owned($3), owned($4) ]

                      @static_env.unextend

                      std::stack<bool>* cmdarg = $<bool_stack>2;
                      lexer.cmdarg = *cmdarg;
                      delete cmdarg;
                      @lexer.cmdarg.pop
                    }

       case_body: kWHEN args then compstmt cases
                    {
                      $$ = [ builder::when(owned($1), owned($2), owned($3), owned($4)),
                                 *owned($5) ]
                    }

           cases: opt_else
                    {
                      $$ = [ owned($1) ]
                    }
                | case_body

      opt_rescue: kRESCUE exc_list exc_var then compstmt opt_rescue
                    {
                      assoc_t, exc_var = $3

                      if owned($2)
                        exc_list = builder::array(nil, owned($2), nil)
                      end

                      $$ = [ builder::rescue_body(owned($1),
                                      exc_list, assoc_t, exc_var,
                                      owned($4), owned($5)),
                                 *owned($6) ]
                    }
                |
                    {
                      $$ = []
                    }

        exc_list: arg_value
                    {
                      $$ = [ owned($1) ]
                    }
                | mrhs
                | list_none

         exc_var: tASSOC lhs
                    {
                      $$ = [ owned($1), owned($2) ]
                    }
                | list_none

      opt_ensure: kENSURE compstmt
                    {
                      $$ = [ owned($1), owned($2) ]
                    }
                | none

         literal: numeric
                | symbol
                | dsym

         strings: string
                    {
                      $$ = builder::string_compose(nil, owned($1), nil)
                    }

          string: string1
                    {
                      $$ = [ owned($1) ]
                    }
                | string string1
                    {
                      $$ = $1 << owned($2)
                    }

         string1: tSTRING_BEG string_contents tSTRING_END
                    {
                      string = builder::string_compose(owned($1), owned($2), owned($3))
                      $$ = builder::dedent_string(string, @lexer.dedent_level)
                    }
                | tSTRING
                    {
                      string = builder::string(owned($1))
                      $$ = builder::dedent_string(string, @lexer.dedent_level)
                    }
                | tCHARACTER
                    {
                      $$ = builder::character(owned($1))
                    }

         xstring: tXSTRING_BEG xstring_contents tSTRING_END
                    {
                      string = builder::xstring_compose(owned($1), owned($2), owned($3))
                      $$ = builder::dedent_string(string, @lexer.dedent_level)
                    }

          regexp: tREGEXP_BEG regexp_contents tSTRING_END tREGEXP_OPT
                    {
                      opts   = builder::regexp_options(owned($4))
                      $$ = builder::regexp_compose(owned($1), owned($2), owned($3), opts)
                    }

           words: tWORDS_BEG word_list tSTRING_END
                    {
                      $$ = builder::words_compose(owned($1), owned($2), owned($3))
                    }

       word_list: // nothing
                    {
                      $$ = []
                    }
                | word_list word tSPACE
                    {
                      $$ = $1 << builder::word(owned($2))
                    }

            word: string_content
                    {
                      $$ = [ owned($1) ]
                    }
                | word string_content
                    {
                      $$ = $1 << owned($2)
                    }

         symbols: tSYMBOLS_BEG symbol_list tSTRING_END
                    {
                      $$ = builder::symbols_compose(owned($1), owned($2), owned($3))
                    }

     symbol_list: // nothing
                    {
                      $$ = []
                    }
                | symbol_list word tSPACE
                    {
                      $$ = $1 << builder::word(owned($2))
                    }

          qwords: tQWORDS_BEG qword_list tSTRING_END
                    {
                      $$ = builder::words_compose(owned($1), owned($2), owned($3))
                    }

        qsymbols: tQSYMBOLS_BEG qsym_list tSTRING_END
                    {
                      $$ = builder::symbols_compose(owned($1), owned($2), owned($3))
                    }

      qword_list: // nothing
                    {
                      $$ = []
                    }
                | qword_list tSTRING_CONTENT tSPACE
                    {
                      $$ = $1 << builder::string_internal(owned($2))
                    }

       qsym_list: // nothing
                    {
                      $$ = []
                    }
                | qsym_list tSTRING_CONTENT tSPACE
                    {
                      $$ = $1 << builder::symbol_internal(owned($2))
                    }

 string_contents: // nothing
                    {
                      $$ = []
                    }
                | string_contents string_content
                    {
                      $$ = $1 << owned($2)
                    }

xstring_contents: // nothing
                    {
                      $$ = []
                    }
                | xstring_contents string_content
                    {
                      $$ = $1 << owned($2)
                    }

regexp_contents: // nothing
                    {
                      $$ = []
                    }
                | regexp_contents string_content
                    {
                      $$ = $1 << owned($2)
                    }

  string_content: tSTRING_CONTENT
                    {
                      $$ = builder::string_internal(owned($1))
                    }
                | tSTRING_DVAR string_dvar
                    {
                      $$ = $2
                    }
                | tSTRING_DBEG
                    {
                      @lexer.cond.push(false)
                      @lexer.cmdarg.push(false)
                    }
                    compstmt tSTRING_DEND
                    {
                      @lexer.cond.lexpop
                      @lexer.cmdarg.lexpop

                      $$ = builder::begin(owned($1), owned($3), owned($4))
                    }

     string_dvar: tGVAR
                    {
                      $$ = builder::gvar(owned($1))
                    }
                | tIVAR
                    {
                      $$ = builder::ivar(owned($1))
                    }
                | tCVAR
                    {
                      $$ = builder::cvar(owned($1))
                    }
                | backref


          symbol: tSYMBOL
                    {
                      @lexer.state = :expr_endarg
                      $$ = builder::symbol(owned($1))
                    }

            dsym: tSYMBEG xstring_contents tSTRING_END
                    {
                      @lexer.state = :expr_endarg
                      $$ = builder::symbol_compose(owned($1), owned($2), owned($3))
                    }

         numeric: simple_numeric
                    {
                      $$ = $1
                    }
                | tUMINUS_NUM simple_numeric %prec tLOWEST
                    {
                      $$ = builder::negate(owned($1), owned($2))
                    }

  simple_numeric: tINTEGER
                    {
                      @lexer.state = :expr_endarg
                      $$ = builder::integer(owned($1))
                    }
                | tFLOAT
                    {
                      @lexer.state = :expr_endarg
                      $$ = builder::float(owned($1))
                    }
                | tRATIONAL
                    {
                      @lexer.state = :expr_endarg
                      $$ = builder::rational(owned($1))
                    }
                | tIMAGINARY
                    {
                      @lexer.state = :expr_endarg
                      $$ = builder::complex(owned($1))
                    }

   user_variable: tIDENTIFIER
                    {
                      $$ = builder::ident(owned($1))
                    }
                | tIVAR
                    {
                      $$ = builder::ivar(owned($1))
                    }
                | tGVAR
                    {
                      $$ = builder::gvar(owned($1))
                    }
                | tCONSTANT
                    {
                      $$ = builder::const(owned($1))
                    }
                | tCVAR
                    {
                      $$ = builder::cvar(owned($1))
                    }

keyword_variable: kNIL
                    {
                      $$ = builder::nil(owned($1))
                    }
                | kSELF
                    {
                      $$ = builder::self(owned($1))
                    }
                | kTRUE
                    {
                      $$ = builder::true(owned($1))
                    }
                | kFALSE
                    {
                      $$ = builder::false(owned($1))
                    }
                | k__FILE__
                    {
                      $$ = builder::__FILE__(owned($1))
                    }
                | k__LINE__
                    {
                      $$ = builder::__LINE__(owned($1))
                    }
                | k__ENCODING__
                    {
                      $$ = builder::__ENCODING__(owned($1))
                    }

         var_ref: user_variable
                    {
                      $$ = builder::accessible(owned($1))
                    }
                | keyword_variable
                    {
                      $$ = builder::accessible(owned($1))
                    }

         var_lhs: user_variable
                    {
                      $$ = builder::assignable(owned($1))
                    }
                | keyword_variable
                    {
                      $$ = builder::assignable(owned($1))
                    }

         backref: tNTH_REF
                    {
                      $$ = builder::nth_ref(owned($1))
                    }
                | tBACK_REF
                    {
                      $$ = builder::back_ref(owned($1))
                    }

      superclass: tLT
                    {
                      @lexer.state = :expr_value
                    }
                    expr_value term
                    {
                      $$ = [ owned($1), owned($3) ]
                    }
                | // nothing
                    {
                      $$ = nil
                    }

tr_methodgenargs: tLBRACK2 tr_gendeclargs rbracket
                    {
                      $$ = builder::tr_genargs(owned($1), owned($2), owned($3))
                    }
                | // nothing
                    {
                      $$ = nil
                    }

       f_arglist: tr_methodgenargs tLPAREN2 f_args rparen
                    {
                      @lexer.state = :expr_value
                    }
                  tr_returnsig
                    {
                      $$ = builder::args(owned($2), owned($3), owned($4))

                      if owned($1) || owned($6)
                        $$ = builder::prototype(owned($1), result, owned($6))
                      end
                    }
                | tr_methodgenargs
                    {
                      $<boolean>$ = @lexer.in_kwarg
                      @lexer.in_kwarg = true
                    }
                  f_args tr_returnsig term
                    {
                      @lexer.in_kwarg = $<boolean>2;
                      $$ = builder::args(nil, owned($3), nil)

                      if owned($1) || owned($4)
                        $$ = builder::prototype(owned($1), result, owned($4))
                      end
                    }

       args_tail: f_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                      $$ = $1.concat(owned($3)).concat(owned($4))
                    }
                | f_kwarg opt_f_block_arg
                    {
                      $$ = $1.concat(owned($2))
                    }
                | f_kwrest opt_f_block_arg
                    {
                      $$ = $1.concat(owned($2))
                    }
                | f_block_arg
                    {
                      $$ = [ owned($1) ]
                    }

   opt_args_tail: tCOMMA args_tail
                    {
                      $$ = $2
                    }
                | // nothing
                    {
                      $$ = []
                    }

          f_args: f_arg tCOMMA f_optarg tCOMMA f_rest_arg              opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($5)).
                                  concat(owned($6))
                    }
                | f_arg tCOMMA f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($5)).
                                  concat(owned($7)).
                                  concat(owned($8))
                    }
                | f_arg tCOMMA f_optarg                                opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($4))
                    }
                | f_arg tCOMMA f_optarg tCOMMA                   f_arg opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($5)).
                                  concat(owned($6))
                    }
                | f_arg tCOMMA                 f_rest_arg              opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($4))
                    }
                | f_arg tCOMMA                 f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($5)).
                                  concat(owned($6))
                    }
                | f_arg                                                opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($2))
                    }
                |              f_optarg tCOMMA f_rest_arg              opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($4))
                    }
                |              f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($5)).
                                  concat(owned($6))
                    }
                |              f_optarg                                opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($2))
                    }
                |              f_optarg tCOMMA                   f_arg opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($4))
                    }
                |                              f_rest_arg              opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($2))
                    }
                |                              f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                      $$ = $1.
                                  concat(owned($3)).
                                  concat(owned($4))
                    }
                |                                                          args_tail
                    {
                      $$ = $1
                    }
                | // nothing
                    {
                      $$ = []
                    }

       f_bad_arg: tIVAR
                    {
                      diagnostic :error, :argument_ivar, nil, owned($1)
                    }
                | tGVAR
                    {
                      diagnostic :error, :argument_gvar, nil, owned($1)
                    }
                | tCVAR
                    {
                      diagnostic :error, :argument_cvar, nil, owned($1)
                    }

      f_norm_arg: f_bad_arg
                | tIDENTIFIER
                    {
                      @static_env.declare owned($1)[0]

                      $$ = $1
                    }

      f_arg_asgn: f_norm_arg
                    {
                      $$ = $1
                    }

      f_arg_item: tr_argsig f_arg_asgn
                    {
                      $$ = builder::arg(owned($2))

                      if owned($1)
                        $$ = builder::typed_arg(owned($1), result)
                      end
                    }
                | tLPAREN f_margs rparen
                    {
                      $$ = builder::multi_lhs(owned($1), owned($2), owned($3))
                    }

           f_arg: f_arg_item
                    {
                      $$ = [ owned($1) ]
                    }
                | f_arg tCOMMA f_arg_item
                    {
                      $$ = $1 << owned($3)
                    }

         f_label: tLABEL
                    {
                      check_kwarg_name(owned($1))

                      @static_env.declare owned($1)[0]

                      $$ = $1
                    }

            f_kw: tr_argsig f_label arg_value
                    {
                      $$ = builder::kwoptarg(owned($2), owned($3))

                      if owned($1)
                        $$ = builder::typed_arg(owned($1), result)
                      end
                    }
                | tr_argsig f_label
                    {
                      $$ = builder::kwarg(owned($2))

                      if owned($1)
                        $$ = builder::typed_arg(owned($1), result)
                      end
                    }

      f_block_kw: tr_argsig f_label primary_value
                    {
                      $$ = builder::kwoptarg(owned($2), owned($3))

                      if owned($1)
                        $$ = builder::typed_arg(owned($1), result)
                      end
                    }
                | tr_argsig f_label
                    {
                      $$ = builder::kwarg(owned($2))

                      if owned($1)
                        $$ = builder::typed_arg(owned($1), result)
                      end
                    }

   f_block_kwarg: f_block_kw
                    {
                      $$ = [ owned($1) ]
                    }
                | f_block_kwarg tCOMMA f_block_kw
                    {
                      $$ = $1 << owned($3)
                    }

         f_kwarg: f_kw
                    {
                      $$ = [ owned($1) ]
                    }
                | f_kwarg tCOMMA f_kw
                    {
                      $$ = $1 << owned($3)
                    }

     kwrest_mark: tPOW | tDSTAR

        f_kwrest: kwrest_mark tIDENTIFIER
                    {
                      @static_env.declare owned($2)[0]

                      $$ = [ builder::kwrestarg(owned($1), owned($2)) ]
                    }
                | kwrest_mark
                    {
                      $$ = [ builder::kwrestarg(owned($1)) ]
                    }

           f_opt: tr_argsig f_arg_asgn tEQL arg_value
                    {
                      $$ = builder::optarg(owned($2), owned($4))

                      if owned($1)
                        $$ = builder::typed_arg(owned($1), result)
                      end
                    }

     f_block_opt: tr_argsig f_arg_asgn tEQL primary_value
                    {
                      $$ = builder::optarg(owned($2), owned($4))

                      if owned($1)
                        $$ = builder::typed_arg(owned($1), result)
                      end
                    }

  f_block_optarg: f_block_opt
                    {
                      $$ = [ owned($1) ]
                    }
                | f_block_optarg tCOMMA f_block_opt
                    {
                      $$ = $1 << owned($3)
                    }

        f_optarg: f_opt
                    {
                      $$ = [ owned($1) ]
                    }
                | f_optarg tCOMMA f_opt
                    {
                      $$ = $1 << owned($3)
                    }

    restarg_mark: tSTAR2 | tSTAR

      f_rest_arg: tr_argsig restarg_mark tIDENTIFIER
                    {
                      token* ident = $3;

                      @static_env.declare(ident->string())

                      restarg = builder::restarg(owned($2), ident->string());

                      if owned($1)
                        restarg = builder::typed_arg(owned($1), restarg)
                      end

                      $$ = [ restarg ]
                    }
                | tr_argsig restarg_mark
                    {
                      restarg = builder::restarg(owned($2), nullptr)

                      if owned($1)
                        restarg = builder::typed_arg(owned($1), restarg)
                      end

                      $$ = [ restarg ]
                    }

     blkarg_mark: tAMPER2 | tAMPER

     f_block_arg: tr_argsig blkarg_mark tIDENTIFIER
                    {
                      token* ident = $3;

                      @static_env.declare(ident->string())

                      $$ = builder::blockarg(owned($2), ident->string());

                      if owned($1)
                        $$ = builder::typed_arg(owned($1), result)
                      end
                    }
                | tr_argsig blkarg_mark
                    {
                      $$ = builder::blockarg(owned($2), nil)

                      if owned($1)
                        $$ = builder::typed_arg(owned($1), result)
                      end
                    }

 opt_f_block_arg: tCOMMA f_block_arg
                    {
                      $$ = [ owned($2) ]
                    }
                |
                    {
                      $$ = []
                    }

       singleton: var_ref
                | tLPAREN2 expr rparen
                    {
                      $$ = $2
                    }

      assoc_list: // nothing
                    {
                      $$ = []
                    }
                | assocs trailer

          assocs: assoc
                    {
                      $$ = [ owned($1) ]
                    }
                | assocs tCOMMA assoc
                    {
                      $$ = $1 << owned($3)
                    }

           assoc: arg_value tASSOC arg_value
                    {
                      $$ = builder::pair(owned($1), owned($2), owned($3))
                    }
                | tLABEL arg_value
                    {
                      $$ = builder::pair_keyword(owned($1), owned($2))
                    }
                | tSTRING_BEG string_contents tLABEL_END arg_value
                    {
                      $$ = builder::pair_quoted(owned($1), owned($2), owned($3), owned($4))
                    }
                | tDSTAR arg_value
                    {
                      $$ = builder::kwsplat(owned($1), owned($2))
                    }

       operation: tIDENTIFIER | tCONSTANT | tFID
      operation2: tIDENTIFIER | tCONSTANT | tFID | op
      operation3: tIDENTIFIER | tFID | op
    dot_or_colon: call_op | tCOLON2
         call_op: tDOT
                    {
                      $$ = [:dot, owned($1)[1]]
                    }
                | tANDDOT
                    {
                      $$ = [:anddot, owned($1)[1]]
                    }
       opt_terms:  | terms
          opt_nl:  | tNL
          rparen: opt_nl tRPAREN
                    {
                      $$ = $2
                    }
        rbracket: opt_nl tRBRACK
                    {
                      $$ = $2
                    }
         trailer:  | tNL | tCOMMA

            term: tSEMI
                  {
                    yyerrok
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
                      $$ = builder::const_global(owned($1), owned($2))
                    }
                | tCONSTANT
                    {
                      $$ = builder::const(owned($1))
                    }
                | tr_cpath tCOLON2 tCONSTANT
                    {
                      $$ = builder::const_fetch(owned($1), owned($2), owned($3))
                    }

       tr_types: tr_types tCOMMA tr_type
                   {
                     $$ = $1 << owned($3)
                   }
               | tr_type
                   {
                     $$ = [owned($1)]
                   }

         tr_type: tr_cpath
                    {
                      $$ = builder::tr_cpath(owned($1))
                    }
                | tr_cpath tCOLON2 tLBRACK2 tr_types rbracket
                    {
                      $$ = builder::tr_geninst(owned($1), owned($3), owned($4), owned($5))
                    }
                | tLBRACK tr_type rbracket
                    {
                      $$ = builder::tr_array(owned($1), owned($2), owned($3))
                    }
                | tLBRACK tr_type tCOMMA tr_types rbracket
                    {
                      types = $4
                      types.unshift(owned($2))
                      $$ = builder::tr_tuple(owned($1), types, owned($5))
                    }
                | tLBRACE tr_type tASSOC tr_type tRCURLY
                    {
                      $$ = builder::tr_hash(owned($1), owned($2), owned($3), owned($4), owned($5))
                    }
                | tLBRACE tr_blockproto tr_returnsig tRCURLY
                    {
                      prototype =
                        if owned($3)
                          builder::prototype(nil, owned($2), owned($3))
                        else
                          owned($2)
                        end

                      $$ = builder::tr_proc(owned($1), prototype, owned($4))
                    }
                | tTILDE tr_type
                    {
                      $$ = builder::tr_nillable(owned($1), owned($2))
                    }
                | kNIL
                    {
                      $$ = builder::tr_nil(owned($1))
                    }
                | tSYMBOL
                    {
                      $$ =
                        case owned($1)[0]
                        when "self", "instance", "class", "any"
                          builder::tr_special(owned($1))
                        else
                          diagnostic :error, :bad_special_type, { value: owned($1)[0] }, owned($1)
                        end
                    }
                | tLPAREN tr_union_type rparen
                    {
                      $$ = $2
                    }

   tr_union_type: tr_union_type tPIPE tr_type
                    {
                      $$ = builder::tr_or(owned($1), owned($3))
                    }
                | tr_type

       tr_argsig: tr_type
                    {
                      $$ = $1
                      @lexer.state = :expr_beg
                    }
                |
                    {
                      $$ = nil
                    }

    tr_returnsig: tASSOC tr_type
                    {
                      $$ = $2
                    }
                |
                    {
                      $$ = nil
                    }

  tr_gendeclargs: tr_gendeclargs tCOMMA tCONSTANT
                    {
                      $$ = $1 << builder::tr_gendeclarg(owned($3))
                    }
                | tCONSTANT
                    {
                      $$ = [builder::tr_gendeclarg(owned($1))]
                    }

   tr_blockproto: { @static_env.extend_dynamic }
                  block_param_def
                    {
                      @static_env.unextend
                      $$ = $2
                    }

%%
