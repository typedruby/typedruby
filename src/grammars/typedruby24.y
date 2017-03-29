%{
  #include <ruby_parser/builder.hh>
  #include <ruby_parser/node.hh>
  #include <ruby_parser/token.hh>
  #include <ruby_parser/lexer.hh>
  #include <iterator>
  #include <utility>

  using namespace ruby_parser;
%}

%pure-parser

%union {
  token_ptr* token;
  node_delimited_ptr* delimited;
  node_delimited_block_ptr* delimited_block;
  node_with_token_ptr* with_token;
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
  f_block_arg
  f_block_kw
  f_block_opt
  f_kw
  f_kwarg
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
  opt_args_tail
  opt_block_param
  opt_ensure
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
  exc_var
  f_arg
  f_args
  f_block_kwarg
  f_block_optarg
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

%type <delimited>
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
  if_tail
  opt_else
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
  static std::unique_ptr<T> owned(T* ptr) {
    return new std::unique_ptr<T>(ptr);
  }

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

  template<typename To, typename From>
  static std::unique_ptr<To> static_unique_cast(std::unique_ptr<From>& from) {
    return std::unique_ptr<To> { from.release() };
  }

  static node_list_ptr make_node_list(std::initializer_list<node_ptr> args) {
    return std::make_unique<node_list>(std::vector<node_ptr>(args));
  }

  static void concat_node_list(node_list_ptr& a, node_list_ptr&& b) {
    a->nodes.insert(
      a->nodes.begin(),
      std::make_move_iterator(b->nodes.begin()),
      std::make_move_iterator(b->nodes.end())
    );
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
                      auto else_ = take($3);

                      auto ensure = owned($4); // TODO needs to be a tuple of (ensure_t, else)
                      token_ptr ensure_t = nullptr;

                      if (rescue_bodies->nodes.size() == 0 && else_ != nullptr) {
                        // TODO diagnostic :warning, :useless_else, nullptr, else_t
                      }

                      $$ = builder::begin_body(owned($1),
                            std::move(rescue_bodies),
                            std::move(else_->token_), std::move(else_->node_),
                            std::move(ensure_t), std::move(ensure)).release();
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
                      /* TODO diagnostic :error, :begin_in_method, nullptr, owned($1) */
                    }

            stmt: kALIAS fitem
                    {
                      // TODO lexer.set_state_expr_fname();
                    }
                    fitem
                    {
                      $$ = builder::alias(take($1), owned($2), owned($4)).release();
                    }
                | kALIAS tGVAR tGVAR
                    {
                      $$ = builder::alias(take($1),
                        builder::gvar(take($2)),
                        builder::gvar(take($3))).release();
                    }
                | kALIAS tGVAR tBACK_REF
                    {
                      $$ = builder::alias(take($1),
                        builder::gvar(take($2)),
                        builder::back_ref(take($3))).release();
                    }
                | kALIAS tGVAR tNTH_REF
                    {
                      // TODO diagnostic :error, :nth_ref_alias, nullptr, owned($3)
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
                      auto rescue_body = builder::rescue_body(take($2), nullptr, nullptr, nullptr, nullptr, owned($3));

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
                      $$ = builder::assign(owned($1), take($<token>2), builder::array(nullptr, owned($3), nullptr)).release();
                    }
                | mlhs tEQL mrhs_arg
                    {
                      $$ = builder::multi_assign(owned($1), owned($3)).release();
                    }
                | kDEF tIVAR tCOLON tr_type
                    {
                      $$ = builder::tr_ivardecl(take($2), owned($4)).release();
                    }
                | expr

    command_asgn: lhs tEQL command_rhs
                    {
                      $$ = builder::assign(owned($1), take($<token>2), owned($3)).release();
                    }
                | var_lhs tOP_ASGN command_rhs
                    {
                      $$ = builder::op_assign(owned($1), take($2), owned($3)).release();
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN command_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::index(
                                    owned($1), take($2), owned($3), take($4)),
                                  take($5), owned($6)).release();
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN command_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), take($2), take($3)),
                                  take($4), owned($5)).release();
                    }
                | primary_value call_op tCONSTANT tOP_ASGN command_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), take($2), take($3)),
                                  take($4), owned($5)).release();
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN command_rhs
                    {
                      auto const_node = builder::const_op_assignable(
                                  builder::const_fetch(owned($1), take($2), take($3)));
                      $$ = builder::op_assign(std::move(const_node), take($4), owned($5)).release();
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN command_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), take($2), take($3)),
                                  take($4), owned($5)).release();
                    }
                | backref tOP_ASGN command_rhs
                    {
                      builder::op_assign(owned($1), take($2), owned($3))
                    }

     command_rhs: command_call %prec tOP_ASGN
                | command_call kRESCUE_MOD stmt
                    {
                      auto rescue_body =
                        builder::rescue_body(take($2),
                                        nullptr, nullptr, nullptr,
                                        nullptr, owned($3));

                      auto rescue_bodies = make_node_list({ std::move(rescue_body) });

                      $$ = builder::begin_body(owned($1), std::move(rescue_bodies)).release();
                    }
                | command_asgn

            expr: command_call
                | expr kAND expr
                    {
                      $$ = builder::logical_op(node_type::AND, owned($1), take($2), owned($3)).release();
                    }
                | expr kOR expr
                    {
                      $$ = builder::logical_op(node_type::OR, owned($1), take($2), owned($3)).release();
                    }
                | kNOT opt_nl expr
                    {
                      $$ = builder::not_op(take($1), nullptr, owned($3), nullptr).release();
                    }
                | tBANG command_call
                    {
                      $$ = builder::not_op(take($1), nullptr, owned($2), nullptr).release();
                    }
                | arg

      expr_value: expr

    command_call: command
                | block_command

   block_command: block_call
                | block_call dot_or_colon operation2 command_args
                    {
                      $$ = builder::call_method(owned($1), take($2), take($3),
                                  nullptr, owned($4), nullptr).release();
                    }

 cmd_brace_block: tLBRACE_ARG brace_body tRCURLY
                    {
                      /* TODO $$ = [ owned($1), *owned($2), owned($3) ] */
                      $$ = nullptr;
                    }

           fcall: operation

         command: fcall command_args %prec tLOWEST
                    {
                      $$ = builder::call_method(nullptr, nullptr, take($1),
                                  nullptr, owned($2), nullptr).release();
                    }
                | fcall command_args cmd_brace_block
                    {
                      auto method_call = builder::call_method(nullptr, nullptr, take($1),
                                                              nullptr, owned($2), nullptr);

                      auto delimited_block = take($3);

                      $$ = builder::block(std::move(method_call),
                                      std::move(delimited_block->begin),
                                      std::move(delimited_block->args),
                                      std::move(delimited_block->body),
                                      std::move(delimited_block->end)).release();
                    }
                | primary_value call_op operation2 command_args %prec tLOWEST
                    {
                      $$ = builder::call_method(owned($1), take($2), take($3),
                                  nullptr, owned($4), nullptr).release();
                    }
                | primary_value call_op operation2 command_args cmd_brace_block
                    {
                      auto method_call = builder::call_method(owned($1), take($2), take($3),
                                        nullptr, owned($4), nullptr);

                      auto delimited_block = take($5);

                      $$ = builder::block(std::move(method_call),
                                      std::move(delimited_block->begin),
                                      std::move(delimited_block->args),
                                      std::move(delimited_block->body),
                                      std::move(delimited_block->end)).release();
                    }
                | primary_value tCOLON2 operation2 command_args %prec tLOWEST
                    {
                      $$ = builder::call_method(owned($1), take($2), take($3),
                                  nullptr, owned($4), nullptr).release();
                    }
                | primary_value tCOLON2 operation2 command_args cmd_brace_block
                    {
                      auto method_call = builder::call_method(owned($1), take($2), take($3),
                                        nullptr, owned($4), nullptr);

                      auto delimited_block = take($5);

                      $$ = builder::block(std::move(method_call),
                                      std::move(delimited_block->begin),
                                      std::move(delimited_block->args),
                                      std::move(delimited_block->body),
                                      std::move(delimited_block->end)).release();
                    }
                | kSUPER command_args
                    {
                      $$ = builder::keyword_cmd(node_type::SUPER, take($1),
                                  nullptr, owned($2), nullptr).release();
                    }
                | kYIELD command_args
                    {
                      $$ = builder::keyword_cmd(node_type::YIELD, take($1),
                                  nullptr, owned($2), nullptr).release();
                    }
                | kRETURN call_args
                    {
                      $$ = builder::keyword_cmd(node_type::RETURN, take($1),
                                  nullptr, owned($2), nullptr).release();
                    }
                | kBREAK call_args
                    {
                      $$ = builder::keyword_cmd(node_type::BREAK, take($1),
                                  nullptr, owned($2), nullptr).release();
                    }
                | kNEXT call_args
                    {
                      $$ = builder::keyword_cmd(node_type::NEXT, take($1),
                                  nullptr, owned($2), nullptr).release();
                    }

            mlhs: mlhs_basic
                    {
                      $$ = builder::multi_lhs(nullptr, owned($1), nullptr).release();
                    }
                | tLPAREN mlhs_inner rparen
                    {
                      $$ = builder::begin(take($1), owned($2), take($3)).release();
                    }

      mlhs_inner: mlhs_basic
                    {
                      $$ = builder::multi_lhs(nullptr, owned($1), nullptr).release();
                    }
                | tLPAREN mlhs_inner rparen
                    {
                      auto inner = make_node_list({ owned($2) });
                      $$ = builder::multi_lhs(take($1), std::move(inner), take($3)).release();
                    }

      mlhs_basic: mlhs_head
                | mlhs_head mlhs_item
                    {
                      $1->nodes.push_back(owned($2));
                      $$ = $1;
                    }
                | mlhs_head tSTAR mlhs_node
                    {
                      $1->nodes.push_back(builder::splat(take($2), owned($3)));
                      $$ = $1;
                    }
                | mlhs_head tSTAR mlhs_node tCOMMA mlhs_post
                    {
                      auto head = owned($1);

                      head->nodes.push_back(builder::splat(take($2), owned($3)));
                      concat_node_list(head, owned($5));

                      $$ = head.release();
                    }
                | mlhs_head tSTAR
                    {
                      $1->nodes.push_back(builder::splat(take($2)));
                      $$ = $1;
                    }
                | mlhs_head tSTAR tCOMMA mlhs_post
                    {
                      auto head = owned($1);

                      head->nodes.push_back(builder::splat(take($2)));
                      concat_node_list(head, owned($4));

                      $$ = head.release();
                    }
                | tSTAR mlhs_node
                    {
                      $$ = make_node_list({ builder::splat(take($1), owned($2)) }).release();
                    }
                | tSTAR mlhs_node tCOMMA mlhs_post
                    {
                      auto items = make_node_list({ builder::splat(take($1), owned($2)) });

                      concat_node_list(items, owned($4));

                      $$ = items.release();
                    }
                | tSTAR
                    {
                      $$ = make_node_list({ builder::splat(take($1)) }).release();
                    }
                | tSTAR tCOMMA mlhs_post
                    {
                      auto items = make_node_list({ builder::splat(take($1)) });

                      concat_node_list(items, owned($3));

                      $$ = items.release();
                    }

       mlhs_item: mlhs_node
                | tLPAREN mlhs_inner rparen
                    {
                      $$ = builder::begin(take($1), owned($2), take($3)).release();
                    }

       mlhs_head: mlhs_item tCOMMA
                    {
                      $$ = make_node_list({ owned($1) }).release();
                    }
                | mlhs_head mlhs_item tCOMMA
                    {
                      $1->nodes.push_back(owned($2));
                      $$ = $1;
                    }

       mlhs_post: mlhs_item
                    {
                      $$ = make_node_list({ owned($1) }).release();
                    }
                | mlhs_post tCOMMA mlhs_item
                    {
                      $1->nodes.push_back(owned($3));
                      $$ = $1;
                    }

       mlhs_node: user_variable
                    {
                      $$ = builder::assignable(owned($1)).release();
                    }
                | keyword_variable
                    {
                      $$ = builder::assignable(owned($1)).release();
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      $$ = builder::index_asgn(owned($1), take($2), owned($3), take($4)).release();
                    }
                | primary_value call_op tIDENTIFIER
                    {
                      $$ = builder::attr_asgn(owned($1), take($2), take($3)).release();
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                      $$ = builder::attr_asgn(owned($1), take($2), take($3)).release();
                    }
                | primary_value call_op tCONSTANT
                    {
                      $$ = builder::attr_asgn(owned($1), take($2), take($3)).release();
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = builder::assignable(
                                  builder::const_fetch(owned($1), take($2), take($3))).release();
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = builder::assignable(
                                  builder::const_global(take($1), take($2))).release();
                    }
                | backref
                    {
                      $$ = builder::assignable(owned($1)).release();
                    }

             lhs: user_variable
                    {
                      $$ = builder::assignable(owned($1)).release();
                    }
                | keyword_variable
                    {
                      $$ = builder::assignable(owned($1)).release();
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                      $$ = builder::index_asgn(owned($1), take($2), owned($3), take($4)).release();
                    }
                | primary_value call_op tIDENTIFIER
                    {
                      $$ = builder::attr_asgn(owned($1), take($2), take($3)).release();
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                      $$ = builder::attr_asgn(owned($1), take($2), take($3)).release();
                    }
                | primary_value call_op tCONSTANT
                    {
                      $$ = builder::attr_asgn(owned($1), take($2), take($3)).release();
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = builder::assignable(
                                  builder::const_fetch(owned($1), take($2), take($3))).release();
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = builder::assignable(
                                  builder::const_global(take($1), take($2))).release();
                    }
                | backref
                    {
                      $$ = builder::assignable(owned($1)).release();
                    }

           cname: tIDENTIFIER
                    {
                      // TODO diagnostic :error, :module_name_const, nullptr, owned($1)
                    }
                | tCONSTANT

           cpath: tCOLON3 cname
                    {
                      $$ = builder::const_global(take($1), take($2)).release();
                    }
                | cname
                    {
                      $$ = builder::const_(take($1)).release();
                    }
                | primary_value tCOLON2 tLBRACK2 tr_gendeclargs rbracket
                    {
                      $$ = builder::tr_gendecl(owned($1), take($3), owned($4), take($5)).release();
                    }
                | primary_value tCOLON2 cname
                    {
                      $$ = builder::const_fetch(owned($1), take($2), take($3)).release();
                    }

           fname: tIDENTIFIER | tCONSTANT | tFID
                | op
                | reswords

            fsym: fname
                    {
                      $$ = builder::symbol(take($1)).release();
                    }
                | symbol

           fitem: fsym
                | dsym

      undef_list: fitem
                    {
                      $$ = make_node_list({ owned($1) }).release();
                    }
                | undef_list tCOMMA
                    {
                      // TODO lexer.set_state_expr_fname();
                    }
                    fitem
                    {
                      $1->nodes.push_back(owned($4));
                      $$ = $1;
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
                      $$ = builder::assign(owned($1), take($<token>2), owned($3)).release();
                    }
                | var_lhs tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(owned($1), take($2), owned($3)).release();
                    }
                | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::index(
                                    owned($1), take($2), owned($3), take($4)),
                                  take($5), owned($6)).release();
                    }
                | primary_value call_op tIDENTIFIER tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), take($2), take($3)),
                                  take($4), owned($5)).release();
                    }
                | primary_value call_op tCONSTANT tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), take($2), take($3)),
                                  take($4), owned($5)).release();
                    }
                | primary_value tCOLON2 tIDENTIFIER tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(
                                  builder::call_method(
                                    owned($1), take($2), take($3)),
                                  take($4), owned($5)).release();
                    }
                | primary_value tCOLON2 tCONSTANT tOP_ASGN arg_rhs
                    {
                      auto const_ = builder::const_op_assignable(
                                      builder::const_fetch(owned($1), take($2), take($3)));

                      $$ = builder::op_assign(std::move(const_), take($4), owned($5)).release();
                    }
                | tCOLON3 tCONSTANT tOP_ASGN arg_rhs
                    {
                      auto const_ = builder::const_op_assignable(
                                  builder::const_global(take($1), take($2)));

                      $$ = builder::op_assign(std::move(const_), take($3), owned($4)).release();
                    }
                | backref tOP_ASGN arg_rhs
                    {
                      $$ = builder::op_assign(owned($1), take($2), owned($3)).release();
                    }
                | arg tDOT2 arg
                    {
                      $$ = builder::range_inclusive(owned($1), take($2), owned($3)).release();
                    }
                | arg tDOT3 arg
                    {
                      $$ = builder::range_exclusive(owned($1), take($2), owned($3)).release();
                    }
                | arg tPLUS arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tMINUS arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tSTAR2 arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tDIVIDE arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tPERCENT arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tPOW arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | tUMINUS_NUM simple_numeric tPOW arg
                    {
                      $$ = builder::unary_op(take($1),
                                  builder::binary_op(
                                    owned($2), take($3), owned($4))).release();
                    }
                | tUPLUS arg
                    {
                      $$ = builder::unary_op(take($1), owned($2)).release();
                    }
                | tUMINUS arg
                    {
                      $$ = builder::unary_op(take($1), owned($2)).release();
                    }
                | arg tPIPE arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tCARET arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tAMPER2 arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tCMP arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tGT arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tGEQ arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tLT arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tLEQ arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tEQ arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tEQQ arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tNEQ arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tMATCH arg
                    {
                      $$ = builder::match_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tNMATCH arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | tBANG arg
                    {
                      $$ = builder::not_op(take($1), nullptr, owned($2), nullptr).release();
                    }
                | tTILDE arg
                    {
                      $$ = builder::unary_op(take($1), owned($2)).release();
                    }
                | arg tLSHFT arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tRSHFT arg
                    {
                      $$ = builder::binary_op(owned($1), take($2), owned($3)).release();
                    }
                | arg tANDOP arg
                    {
                      $$ = builder::logical_op(node_type::AND, owned($1), take($2), owned($3)).release();
                    }
                | arg tOROP arg
                    {
                      $$ = builder::logical_op(node_type::OR, owned($1), take($2), owned($3)).release();
                    }
                | kDEFINED opt_nl arg
                    {
                      auto args = make_node_list({ owned($3) });

                      $$ = builder::keyword_cmd(node_type::DEFINED, take($1), nullptr, std::move(args), nullptr).release();
                    }
                | arg tEH arg opt_nl tCOLON arg
                    {
                      $$ = builder::ternary(owned($1), take($2),
                                                owned($3), take($5), owned($6)).release();
                    }
                | primary

       arg_value: arg

       aref_args: list_none
                | args trailer
                | args tCOMMA assocs trailer
                    {
                      $1->nodes.push_back(builder::associate(nullptr, owned($3), nullptr));
                      $$ = $1;
                    }
                | assocs trailer
                    {
                      $$ = make_node_list({ builder::associate(nullptr, owned($1), nullptr) }).release();
                    }

         arg_rhs: arg %prec tOP_ASGN
                | arg kRESCUE_MOD arg
                    {
                      auto rescue_body = builder::rescue_body(take($2),
                                          nullptr, nullptr, nullptr,
                                          nullptr, owned($3));

                      auto rescue_bodies = make_node_list({ std::move(rescue_body) });

                      $$ = builder::begin_body(owned($1), std::move(rescue_bodies)).release();
                    }

      paren_args: tLPAREN2 opt_call_args rparen
                    {
                      $$ = put(std::make_unique<node_delimited>(take($1), owned($2), take($3)));
                    }

  opt_paren_args: // nothing
                    {
                      auto args = make_node_list({});

                      $$ = put(std::make_unique<node_delimited>(nullptr, std::move(args), nullptr));
                    }
                | paren_args

   opt_call_args: // nothing
                    {
                      $$ = make_node_list({}).release();
                    }
                | call_args
                | args tCOMMA
                | args tCOMMA assocs tCOMMA
                    {
                      $1->nodes.push_back(builder::associate(nullptr, owned($3), nullptr));
                      $$ = $1;
                    }
                | assocs tCOMMA
                    {
                      $$ = make_node_list({
                          builder::associate(nullptr, owned($1), nullptr) }).release();
                    }

       call_args: command
                    {
                      $$ = make_node_list({ owned($1) }).release();
                    }
                | args opt_block_arg
                    {
                      auto args = owned($1);

                      concat_node_list(args, owned($2));

                      $$ = args.release();
                    }
                | assocs opt_block_arg
                    {
                      auto args = make_node_list({
                          builder::associate(nullptr, owned($1), nullptr) });

                      concat_node_list(args, owned($2));

                      $$ = args.release();
                    }
                | args tCOMMA assocs opt_block_arg
                    {
                      auto args = owned($1);

                      auto assocs = builder::associate(nullptr, owned($3), nullptr);

                      args->nodes.push_back(std::move(assocs));

                      concat_node_list(args, owned($4));

                      $$ = args.release();
                    }
                | block_arg
                    {
                      $$ = make_node_list({ owned($1) }).release();
                    }

    command_args:   {
                      $<bool_stack>$ = new std::stack<bool>(/* TODO lexer.cmdarg */);
                      // TODO lexer.cmdarg.push(true);
                    }
                  call_args
                    {
                      std::stack<bool>* cmdarg = $<bool_stack>1;
                      // TODO lexer.cmdarg = *cmdarg;
                      delete cmdarg;

                      $$ = $2
                    }

       block_arg: tAMPER arg_value
                    {
                      $$ = builder::block_pass(take($1), owned($2)).release();
                    }

   opt_block_arg: tCOMMA block_arg
                    {
                      $$ = make_node_list({ owned($2) }).release();
                    }
                | // nothing
                    {
                      $$ = make_node_list({}).release();
                    }

            args: arg_value
                    {
                      $$ = make_node_list({ owned($1) }).release();
                    }
                | tSTAR arg_value
                    {
                      $$ = make_node_list({
                          builder::splat(take($1), owned($2)) }).release();
                    }
                | args tCOMMA arg_value
                    {
                      $1->nodes.push_back(owned($3));
                      $$ = $1;
                    }
                | args tCOMMA tSTAR arg_value
                    {
                      $1->nodes.push_back(builder::splat(take($3), owned($4)));
                      $$ = $1;
                    }

        mrhs_arg: mrhs
                    {
                      $$ = builder::array(nullptr, owned($1), nullptr).release();
                    }
                | arg_value

            mrhs: args tCOMMA arg_value
                    {
                      $1->nodes.push_back(owned($3));
                      $$ = $1;
                    }
                | args tCOMMA tSTAR arg_value
                    {
                      $1->nodes.push_back(builder::splat(take($3), owned($4)));
                      $$ = $1;
                    }
                | tSTAR arg_value
                    {
                      $$ = make_node_list({
                          builder::splat(take($1), owned($2)) }).release();
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
                      $$ = builder::call_method(nullptr, nullptr, take($1)).release();
                    }
                | kBEGIN
                    {
                      $<bool_stack>$ = new std::stack<bool>(/* TODO lexer.cmdarg */);
                      // TODO lexer.cmdarg.clear
                    }
                    bodystmt kEND
                    {
                      auto cmdarg = $<bool_stack>2;
                      // TODO lexer.cmdarg = *cmdarg;
                      delete cmdarg;

                      $$ = builder::begin_keyword(take($1), owned($3), take($4)).release();
                    }
                | tLPAREN_ARG
                    {
                      $<bool_stack>$ = new std::stack<bool>(/* lexer.cmdarg */);
                      // TODO lexer.cmdarg.clear
                    }
                    stmt
                    {
                      // TODO lexer.set_state_expr_endarg();
                    }
                    rparen
                    {
                      auto cmdarg = $<bool_stack>2;
                      // TODO lexer.cmdarg = *cmdarg;
                      delete cmdarg;

                      $$ = builder::begin(take($1), owned($3), take($5)).release();
                    }
                | tLPAREN_ARG
                    {
                      // TODO lexer.set_state_expr_endarg();
                    }
                    opt_nl tRPAREN
                    {
                      $$ = builder::begin(take($1), nullptr, take($4)).release();
                    }
                | tLPAREN compstmt tRPAREN
                    {
                      $$ = builder::begin(take($1), owned($2), take($3)).release();
                    }
                | tLPAREN expr tCOLON tr_type tRPAREN
                    {
                      $$ = builder::tr_cast(take($1), owned($2), take($3), owned($4), take($5)).release();
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                      $$ = builder::const_fetch(owned($1), take($2), take($3)).release();
                    }
                | tCOLON3 tCONSTANT
                    {
                      $$ = builder::const_global(take($1), take($2)).release();
                    }
                | tLBRACK aref_args tRBRACK
                    {
                      $$ = builder::array(take($1), owned($2), take($3)).release();
                    }
                | tLBRACE assoc_list tRCURLY
                    {
                      $$ = builder::associate(take($1), owned($2), take($3)).release();
                    }
                | kRETURN
                    {
                      $$ = builder::keyword_cmd(node_type::RETURN, take($1)).release();
                    }
                | kYIELD tLPAREN2 call_args rparen
                    {
                      $$ = builder::keyword_cmd(node_type::YIELD, take($1), take($2), owned($3), take($4)).release();
                    }
                | kYIELD tLPAREN2 rparen
                    {
                      auto args = make_node_list({});

                      $$ = builder::keyword_cmd(node_type::YIELD, take($1), take($2), std::move(args), take($3)).release();
                    }
                | kYIELD
                    {
                      $$ = builder::keyword_cmd(node_type::YIELD, take($1)).release();
                    }
                | kDEFINED opt_nl tLPAREN2 expr rparen
                    {
                      auto args = make_node_list({ owned($4) });

                      $$ = builder::keyword_cmd(node_type::DEFINED, take($1),
                                                    take($3), std::move(args), take($5)).release();
                    }
                | kNOT tLPAREN2 expr rparen
                    {
                      $$ = builder::not_op(take($1), take($2), owned($3), take($4)).release();
                    }
                | kNOT tLPAREN2 rparen
                    {
                      $$ = builder::not_op(take($1), take($2), nullptr, take($3)).release();
                    }
                | fcall brace_block
                    {
                      auto method_call = builder::call_method(nullptr, nullptr, take($1));

                      auto delimited_block = take($2);

                      $$ = builder::block(std::move(method_call),
                        std::move(delimited_block->begin),
                        std::move(delimited_block->args),
                        std::move(delimited_block->body),
                        std::move(delimited_block->end)).release();
                    }
                | method_call
                | method_call brace_block
                    {
                      auto delimited_block = take($2);

                      $$ = builder::block(owned($1),
                        std::move(delimited_block->begin),
                        std::move(delimited_block->args),
                        std::move(delimited_block->body),
                        std::move(delimited_block->end)).release();
                    }
                | tLAMBDA lambda
                    {
                      auto lambda_call = builder::call_lambda(take($1));

                      auto lambda = take($2);

                      $$ = builder::block(std::move(lambda_call),
                        std::move(lambda->begin),
                        std::move(lambda->args),
                        std::move(lambda->body),
                        std::move(lambda->end)).release();
                    }
                | kIF expr_value then compstmt if_tail kEND
                    {
                      auto else_ = take($5);

                      $$ = builder::condition(
                        take($1), owned($2),
                        take($3), owned($4),
                        std::move(else_->token_), std::move(else_->node_),
                        take($6)).release();
                    }
                | kUNLESS expr_value then compstmt opt_else kEND
                    {
                      auto else_ = take($5);

                      $$ = builder::condition(
                        take($1), owned($2),
                        take($3), std::move(else_->node_),
                        std::move(else_->token_), owned($4),
                        take($6)).release();
                    }
                | kWHILE
                    {
                      // TODO lexer.cond.push(true)
                    }
                    expr_value do
                    {
                      // TODO lexer.cond.pop
                    }
                    compstmt kEND
                    {
                      $$ = builder::loop(node_type::WHILE, take($1), owned($3), take($4),
                                             owned($6), take($7)).release();
                    }
                | kUNTIL
                    {
                      // TODO lexer.cond.push(true)
                    }
                    expr_value do
                    {
                      // TODO lexer.cond.pop
                    }
                    compstmt kEND
                    {
                      $$ = builder::loop(node_type::UNTIL, take($1), owned($3), take($4),
                                             owned($6), take($7)).release();
                    }
                | kCASE expr_value opt_terms case_body kEND
                    {
                      auto case_body = owned($4);

                      auto else_ = static_unique_cast<node_with_token>(case_body->nodes.back());
                      case_body->nodes.pop_back();

                      $$ = builder::case_(take($1), owned($2),
                        std::move(case_body),
                        std::move(else_->token_), std::move(else_->node_),
                        take($5)).release();
                    }
                | kCASE            opt_terms case_body kEND
                    {
                      auto case_body = owned($3);

                      auto else_ = static_unique_cast<node_with_token>(case_body->nodes.back());
                      case_body->nodes.pop_back();

                      $$ = builder::case_(take($1), nullptr,
                        std::move(case_body),
                        std::move(else_->token_), std::move(else_->node_),
                        take($4)).release();
                    }
                | kFOR for_var kIN
                    {
                      // TODO lexer.cond.push(true)
                    }
                    expr_value do
                    {
                      // TODO lexer.cond.pop
                    }
                    compstmt kEND
                    {
                      $$ = builder::for_(take($1), owned($2),
                                            take($3), owned($5),
                                            take($6), owned($8), take($9)).release();
                    }
                | kCLASS cpath superclass
                    {
                      // TODO @static_env.extend_static
                      // TODO @lexer.push_cmdarg
                    }
                    bodystmt kEND
                    {
                      // TODO if in_def?
                      // TODO   diagnostic :error, :class_in_def, nullptr, owned($1)
                      // TODO end

                      auto superclass_ = take($3);

                      auto lt_t       = superclass_ ? std::move(superclass_->token_) : nullptr;
                      auto superclass = superclass_ ? std::move(superclass_->node_)  : nullptr;

                      $$ = builder::def_class(take($1), owned($2),
                                                  std::move(lt_t), std::move(superclass),
                                                  owned($5), take($6)).release();

                      // TODO @lexer.pop_cmdarg
                      // TODO @static_env.unextend
                    }
                | kCLASS tLSHFT expr term
                    {
                      // TODO $<size>$ = @def_level
                      // TODO @def_level = 0

                      // TODO @static_env.extend_static
                      // TODO @lexer.push_cmdarg
                    }
                    bodystmt kEND
                    {
                      $$ = builder::def_sclass(take($1), take($2), owned($3),
                                                   owned($6), take($7)).release();

                      // TODO @lexer.pop_cmdarg
                      // TODO @static_env.unextend

                      // TODO @def_level = $<size>5;
                    }
                | kMODULE cpath
                    {
                      // TODO @static_env.extend_static
                      // TODO @lexer.push_cmdarg
                    }
                    bodystmt kEND
                    {
                      // TODO if in_def?
                      // TODO   diagnostic :error, :module_in_def, nullptr, owned($1)
                      // TODO end

                      $$ = builder::def_module(take($1), owned($2), owned($4), take($5)).release();

                      // TODO @lexer.pop_cmdarg
                      // TODO @static_env.unextend
                    }
                | kDEF fname
                    {
                      // TODO @def_level += 1
                      // TODO @static_env.extend_static
                      // TODO @lexer.push_cmdarg
                    }
                    f_arglist bodystmt kEND
                    {
                      $$ = builder::def_method(take($1), take($2),
                                  owned($4), owned($5), take($6)).release();

                      // TODO @lexer.pop_cmdarg
                      // TODO @static_env.unextend
                      // TODO @def_level -= 1
                    }
                | kDEF singleton dot_or_colon
                    {
                      // TODO lexer.set_state_expr_fname();
                    }
                    fname
                    {
                      // TODO @def_level += 1
                      // TODO @static_env.extend_static
                      // TODO @lexer.push_cmdarg
                    }
                    f_arglist bodystmt kEND
                    {
                      $$ = builder::def_singleton(take($1), owned($2), take($3),
                                  take($5), owned($7), owned($8), take($9)).release();

                      // TODO @lexer.pop_cmdarg
                      // TODO @static_env.unextend
                      // TODO @def_level -= 1
                    }
                | kBREAK
                    {
                      $$ = builder::keyword_cmd(node_type::BREAK, take($1)).release();
                    }
                | kNEXT
                    {
                      $$ = builder::keyword_cmd(node_type::NEXT, take($1)).release();
                    }
                | kREDO
                    {
                      $$ = builder::keyword_cmd(node_type::REDO, take($1)).release();
                    }
                | kRETRY
                    {
                      $$ = builder::keyword_cmd(node_type::RETRY, take($1)).release();
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
                      auto elsif_t = take($1);

                      auto else_ = take($5);

                      $$ = put(std::make_unique<node_with_token>(
                        std::make_unique<token>(*elsif_t),
                        builder::condition(
                          std::make_unique<token>(*elsif_t), owned($2), take($3),
                          owned($4), std::move(else_->token_), std::move(else_->node_),
                          nullptr)));
                    }

        opt_else: none
                    {
                      $$ = nullptr;
                    }
                | kELSE compstmt
                    {
                      $$ = put(std::make_unique<node_with_token>(take($1), owned($2)));
                    }

         for_var: lhs
                | mlhs

          f_marg: f_norm_arg
                    {
                      $$ = builder::arg(take($1)).release();
                    }
                | tLPAREN f_margs rparen
                    {
                      $$ = builder::multi_lhs(take($1), owned($2), take($3)).release();
                    }

     f_marg_list: f_marg
                    {
                      $$ = make_node_list({ owned($1) }).release();
                    }
                | f_marg_list tCOMMA f_marg
                    {
                      $1->nodes.push_back(owned($3));
                      $$ = $1;
                    }

         f_margs: f_marg_list
                | f_marg_list tCOMMA tSTAR f_norm_arg
                    {
                      $1->nodes.push_back(builder::restarg(take($3), take($4)));
                      $$ = $1;
                    }
                | f_marg_list tCOMMA tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                      auto args = owned($1);

                      args->nodes.push_back(builder::restarg(take($3), take($4)));
                      concat_node_list(args, owned($6));

                      $$ = args.release();
                    }
                | f_marg_list tCOMMA tSTAR
                    {
                      $1->nodes.push_back(builder::restarg(take($3)));
                      $$ = $1;
                    }
                | f_marg_list tCOMMA tSTAR            tCOMMA f_marg_list
                    {
                      auto args = owned($1);

                      args->nodes.push_back(builder::restarg(take($3)));
                      concat_node_list(args, owned($5));

                      $$ = args.release();
                    }
                |                    tSTAR f_norm_arg
                    {
                      $$ = make_node_list({
                          builder::restarg(take($1), take($2)) }).release();
                    }
                |                    tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                      $4->nodes.insert($4->nodes.begin(), builder::restarg(take($1), take($2)));
                      $$ = $4;
                    }
                |                    tSTAR
                    {
                      $$ = make_node_list({
                          builder::restarg(take($1)) }).release();
                    }
                |                    tSTAR tCOMMA f_marg_list
                    {
                      $3->nodes.insert($3->nodes.begin(), builder::restarg(take($1)));
                      $$ = $3;
                    }

 block_args_tail: f_block_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                      auto args = owned($1);

                      concat_node_list(args, owned($3));
                      concat_node_list(args, owned($3));

                      $$ = args.release();
                    }
                | f_block_kwarg opt_f_block_arg
                    {
                      auto args = owned($1);

                      concat_node_list(args, owned($2));

                      $$ = args.release();
                    }
                | f_kwrest opt_f_block_arg
                    {
                      auto args = owned($1);

                      concat_node_list(args, owned($2));

                      $$ = args.release();
                    }
                | f_block_arg
                    {
                      $$ = make_node_list({ owned($1) }).release();
                    }

opt_block_args_tail:
                  tCOMMA block_args_tail
                    {
                      $$ = $2
                    }
                | // nothing
                    {
                      $$ = make_node_list({}).release();
                    }

     block_param: f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg              opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($3));
                      concat_node_list(args, owned($5));
                      concat_node_list(args, owned($6));
                      $$ = args.release();
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($3));
                      concat_node_list(args, owned($5));
                      concat_node_list(args, owned($7));
                      concat_node_list(args, owned($8));
                      $$ = args.release();
                    }
                | f_arg tCOMMA f_block_optarg                                opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($3));
                      concat_node_list(args, owned($4));
                      $$ = args.release();
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA                   f_arg opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($3));
                      concat_node_list(args, owned($5));
                      concat_node_list(args, owned($6));
                      $$ = args.release();
                    }
                | f_arg tCOMMA                       f_rest_arg              opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($3));
                      concat_node_list(args, owned($4));
                      $$ = args.release();
                    }
                | f_arg tCOMMA
                | f_arg tCOMMA                       f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($3));
                      concat_node_list(args, owned($5));
                      concat_node_list(args, owned($6));
                      $$ = args.release();
                    }
                | f_arg                                                      opt_block_args_tail
                    {
                      auto args = owned($1);
                      auto block_args_tail = owned($2);

                      if (block_args_tail->nodes.size() == 0 && args->nodes.size() == 1) {
                        $$ = make_node_list({ builder::procarg0(std::move(args->nodes[0])) }).release();
                      } else {
                        concat_node_list(args, std::move(block_args_tail));
                        $$ = args.release();
                      }
                    }
                | f_block_optarg tCOMMA              f_rest_arg              opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($3));
                      concat_node_list(args, owned($4));
                      $$ = args.release();
                    }
                | f_block_optarg tCOMMA              f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($3));
                      concat_node_list(args, owned($5));
                      concat_node_list(args, owned($6));
                      $$ = args.release();
                    }
                | f_block_optarg                                             opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($2));
                      $$ = args.release();
                    }
                | f_block_optarg tCOMMA                                f_arg opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($3));
                      concat_node_list(args, owned($4));
                      $$ = args.release();
                    }
                |                                    f_rest_arg              opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($2));
                      $$ = args.release();
                    }
                |                                    f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                      auto args = owned($1);
                      concat_node_list(args, owned($3));
                      concat_node_list(args, owned($4));
                      $$ = args.release();
                    }
                |                                                                block_args_tail

 opt_block_param: // nothing
                    {
                      $$ = builder::args(nullptr, make_node_list({}), nullptr).release();
                    }
                | block_param_def
                    {
                      // TODO @lexer.state.set_state_expr_value();
                    }
                  tr_returnsig
                    {
                      auto args = owned($1);
                      auto return_sig = owned($3);

                      if (return_sig) {
                        $$ = builder::prototype(nullptr, std::move(args), std::move(return_sig)).release();
                      } else {
                        $$ = args.release();
                      }
                    }

 block_param_def: tPIPE opt_bv_decl tPIPE
                    {
                      $$ = builder::args(take($1), owned($2), take($3)).release();
                    }
                | tOROP
                    {
                      auto tok = take($1);
                      $$ = builder::args(std::make_unique<token>(*tok), make_node_list({}), std::make_unique<token>(*tok)).release();
                    }
                | tPIPE block_param opt_bv_decl tPIPE
                    {
                      auto params = owned($2);
                      concat_node_list(params, owned($3));
                      $$ = builder::args(take($1), std::move(params), take($4)).release();
                    }

     opt_bv_decl: opt_nl
                    {
                      $$ = make_node_list({}).release();
                    }
                | opt_nl tSEMI bv_decls opt_nl
                    {
                      $$ = $3;
                    }

        bv_decls: bvar
                    {
                      $$ = make_node_list({ owned($1) }).release();
                    }
                | bv_decls tCOMMA bvar
                    {
                      $1->nodes.push_back(owned($3));
                      $$ = $1;
                    }

            bvar: tIDENTIFIER
                    {
                      // TODO @static_env.declare owned($1)[0]
                      $$ = builder::shadowarg(take($1)).release();
                    }
                | f_bad_arg
                    {
                      $$ = nullptr;
                    }

          lambda:   {
                      // TODO @static_env.extend_dynamic
                    }
                  f_larglist
                    {
                      $<bool_stack>$ = new std::stack<bool>(/* TODO lexer.cmdarg */);
                      // TODO @lexer.cmdarg.clear
                    }
                  lambda_body
                    {
                      std::stack<bool>* cmdarg = $<bool_stack>3;
                      // TODO @lexer.cmdarg = *cmdarg;
                      delete cmdarg;
                      // TODO @lexer.cmdarg.lexpop

                      $$ = put(std::make_unique<node_delimited_block>(
                        nullptr, owned($2), owned($4), nullptr));

                      // TODO @static_env.unextend
                    }

     f_larglist: tLPAREN2 f_args opt_bv_decl tRPAREN
                    {
                      auto args = owned($2);
                      concat_node_list(args, owned($3));
                      $$ = builder::args(take($1), std::move(args), take($4)).release();
                    }
                | f_args
                    {
                      $$ = builder::args(nullptr, owned($1), nullptr).release();
                    }

     lambda_body: tLAMBEG compstmt tRCURLY
                    {
                      $$ = put(std::make_unique<node_delimited_block>(take($1), nullptr, owned($2), take($3)));
                    }
                | kDO_LAMBDA compstmt kEND
                    {
                      $$ = put(std::make_unique<node_delimited_block>(take($1), nullptr, owned($2), take($3)));
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

                      $$ = builder::block(owned($1),
                          std::move(delimited_block->begin),
                          std::move(delimited_block->args),
                          std::move(delimited_block->body),
                          std::move(delimited_block->end)
                        ).release();
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
                                      nullptr, owned($4), nullptr)

                      begin_t, args, body, end_t = $5
                      result      = builder::block(method_call,
                                      begin_t, args, body, end_t)
                    }

     method_call: fcall paren_args
                    {
                      lparen_t, args, rparen_t = $2
                      $$ = builder::call_method(nullptr, nullptr, owned($1),
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
                      $$ = builder::call_method(owned($1), owned($2), nullptr,
                                  lparen_t, args, rparen_t)
                    }
                | primary_value tCOLON2 paren_args
                    {
                      lparen_t, args, rparen_t = $3
                      $$ = builder::call_method(owned($1), owned($2), nullptr,
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
                        exc_list = builder::array(nullptr, owned($2), nullptr)
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
                      $$ = builder::string_compose(nullptr, owned($1), nullptr)
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
                      $$ = builder::nullptr(owned($1))
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
                      $$ = nullptr
                    }

tr_methodgenargs: tLBRACK2 tr_gendeclargs rbracket
                    {
                      $$ = builder::tr_genargs(owned($1), owned($2), owned($3))
                    }
                | // nothing
                    {
                      $$ = nullptr
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
                      $$ = builder::args(nullptr, owned($3), nullptr)

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
                      diagnostic :error, :argument_ivar, nullptr, owned($1)
                    }
                | tGVAR
                    {
                      diagnostic :error, :argument_gvar, nullptr, owned($1)
                    }
                | tCVAR
                    {
                      diagnostic :error, :argument_cvar, nullptr, owned($1)
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
                      $$ = builder::blockarg(owned($2), nullptr)

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
                          builder::prototype(nullptr, owned($2), owned($3))
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
                      $$ = nullptr
                    }

    tr_returnsig: tASSOC tr_type
                    {
                      $$ = $2
                    }
                |
                    {
                      $$ = nullptr
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
