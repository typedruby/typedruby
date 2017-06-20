// A Bison parser, made by GNU Bison 3.0.4.

// Skeleton implementation for Bison LALR(1) parsers in C++

// Copyright (C) 2002-2015 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// As a special exception, you may create a larger work that contains
// part or all of the Bison parser skeleton and distribute that work
// under terms of your choice, so long as that work isn't itself a
// parser generator using the skeleton or a modified version thereof
// as a parser skeleton.  Alternatively, if you modify or redistribute
// the parser skeleton itself, you may (at your option) remove this
// special exception, which will cause the skeleton and the resulting
// Bison output files to be licensed under the GNU General Public
// License without this special exception.

// This special exception was added by the Free Software Foundation in
// version 2.2 of Bison.

// Take the name prefix into account.
#define yylex   typedruby24lex

// First part of user declarations.

#line 39 "cc/grammars/typedruby24.cc" // lalr1.cc:404

# ifndef YY_NULLPTR
#  if defined __cplusplus && 201103L <= __cplusplus
#   define YY_NULLPTR nullptr
#  else
#   define YY_NULLPTR 0
#  endif
# endif

#include "typedruby24.hh"

// User implementation prologue.

#line 53 "cc/grammars/typedruby24.cc" // lalr1.cc:412
// Unqualified %code blocks.
#line 395 "cc/grammars/typedruby24.ypp" // lalr1.cc:413

namespace ruby_parser {
namespace bison {
namespace typedruby24 {

#define DIAGCHECK() do { \
	if (driver.pending_error) { \
		driver.pending_error = false; \
		YYERROR; \
	} \
} while(false);

void parser::error(const std::string &msg) {
	driver.diagnostics.emplace_back(
		dlevel::ERROR, dclass::UnexpectedToken,
		diagnostic::range(driver.lex.last_token_s, driver.lex.last_token_e),
		msg);
}

int yylex(parser::semantic_type *lval, ruby_parser::typedruby24 &driver) {
	auto token = driver.lex.advance();
	int token_type = static_cast<int>(token->type());
	assert(token_type >= 0);
	lval->token = token;
	return token_type;
}

}}} // namespace

#line 85 "cc/grammars/typedruby24.cc" // lalr1.cc:413


#ifndef YY_
# if defined YYENABLE_NLS && YYENABLE_NLS
#  if ENABLE_NLS
#   include <libintl.h> // FIXME: INFRINGES ON USER NAME SPACE.
#   define YY_(msgid) dgettext ("bison-runtime", msgid)
#  endif
# endif
# ifndef YY_
#  define YY_(msgid) msgid
# endif
#endif



// Suppress unused-variable warnings by "using" E.
#define YYUSE(E) ((void) (E))

// Enable debugging if requested.
#if TYPEDRUBY24DEBUG

// A pseudo ostream that takes yydebug_ into account.
# define YYCDEBUG if (yydebug_) (*yycdebug_)

# define YY_SYMBOL_PRINT(Title, Symbol)         \
  do {                                          \
    if (yydebug_)                               \
    {                                           \
      *yycdebug_ << Title << ' ';               \
      yy_print_ (*yycdebug_, Symbol);           \
      *yycdebug_ << std::endl;                  \
    }                                           \
  } while (false)

# define YY_REDUCE_PRINT(Rule)          \
  do {                                  \
    if (yydebug_)                       \
      yy_reduce_print_ (Rule);          \
  } while (false)

# define YY_STACK_PRINT()               \
  do {                                  \
    if (yydebug_)                       \
      yystack_print_ ();                \
  } while (false)

#else // !TYPEDRUBY24DEBUG

# define YYCDEBUG if (false) std::cerr
# define YY_SYMBOL_PRINT(Title, Symbol)  YYUSE(Symbol)
# define YY_REDUCE_PRINT(Rule)           static_cast<void>(0)
# define YY_STACK_PRINT()                static_cast<void>(0)

#endif // !TYPEDRUBY24DEBUG

#define yyerrok         (yyerrstatus_ = 0)
#define yyclearin       (yyla.clear ())

#define YYACCEPT        goto yyacceptlab
#define YYABORT         goto yyabortlab
#define YYERROR         goto yyerrorlab
#define YYRECOVERING()  (!!yyerrstatus_)

#line 26 "cc/grammars/typedruby24.ypp" // lalr1.cc:479
namespace ruby_parser { namespace bison { namespace typedruby24 {
#line 152 "cc/grammars/typedruby24.cc" // lalr1.cc:479

  /// Build a parser object.
  parser::parser (ruby_parser::typedruby24& driver_yyarg, ruby_parser::self_ptr self_yyarg)
    :
#if TYPEDRUBY24DEBUG
      yydebug_ (false),
      yycdebug_ (&std::cerr),
#endif
      driver (driver_yyarg),
      self (self_yyarg)
  {}

  parser::~parser ()
  {}


  /*---------------.
  | Symbol types.  |
  `---------------*/

  inline
  parser::syntax_error::syntax_error (const std::string& m)
    : std::runtime_error (m)
  {}

  // basic_symbol.
  template <typename Base>
  inline
  parser::basic_symbol<Base>::basic_symbol ()
    : value ()
  {}

  template <typename Base>
  inline
  parser::basic_symbol<Base>::basic_symbol (const basic_symbol& other)
    : Base (other)
    , value ()
  {
    value = other.value;
  }


  template <typename Base>
  inline
  parser::basic_symbol<Base>::basic_symbol (typename Base::kind_type t, const semantic_type& v)
    : Base (t)
    , value (v)
  {}


  /// Constructor for valueless symbols.
  template <typename Base>
  inline
  parser::basic_symbol<Base>::basic_symbol (typename Base::kind_type t)
    : Base (t)
    , value ()
  {}

  template <typename Base>
  inline
  parser::basic_symbol<Base>::~basic_symbol ()
  {
    clear ();
  }

  template <typename Base>
  inline
  void
  parser::basic_symbol<Base>::clear ()
  {
    Base::clear ();
  }

  template <typename Base>
  inline
  bool
  parser::basic_symbol<Base>::empty () const
  {
    return Base::type_get () == empty_symbol;
  }

  template <typename Base>
  inline
  void
  parser::basic_symbol<Base>::move (basic_symbol& s)
  {
    super_type::move(s);
    value = s.value;
  }

  // by_type.
  inline
  parser::by_type::by_type ()
    : type (empty_symbol)
  {}

  inline
  parser::by_type::by_type (const by_type& other)
    : type (other.type)
  {}

  inline
  parser::by_type::by_type (token_type t)
    : type (yytranslate_ (t))
  {}

  inline
  void
  parser::by_type::clear ()
  {
    type = empty_symbol;
  }

  inline
  void
  parser::by_type::move (by_type& that)
  {
    type = that.type;
    that.clear ();
  }

  inline
  int
  parser::by_type::type_get () const
  {
    return type;
  }


  // by_state.
  inline
  parser::by_state::by_state ()
    : state (empty_state)
  {}

  inline
  parser::by_state::by_state (const by_state& other)
    : state (other.state)
  {}

  inline
  void
  parser::by_state::clear ()
  {
    state = empty_state;
  }

  inline
  void
  parser::by_state::move (by_state& that)
  {
    state = that.state;
    that.clear ();
  }

  inline
  parser::by_state::by_state (state_type s)
    : state (s)
  {}

  inline
  parser::symbol_number_type
  parser::by_state::type_get () const
  {
    if (state == empty_state)
      return empty_symbol;
    else
      return yystos_[state];
  }

  inline
  parser::stack_symbol_type::stack_symbol_type ()
  {}


  inline
  parser::stack_symbol_type::stack_symbol_type (state_type s, symbol_type& that)
    : super_type (s)
  {
    value = that.value;
    // that is emptied.
    that.type = empty_symbol;
  }

  inline
  parser::stack_symbol_type&
  parser::stack_symbol_type::operator= (const stack_symbol_type& that)
  {
    state = that.state;
    value = that.value;
    return *this;
  }


  template <typename Base>
  inline
  void
  parser::yy_destroy_ (const char* yymsg, basic_symbol<Base>& yysym) const
  {
    if (yymsg)
      YY_SYMBOL_PRINT (yymsg, yysym);

    // User destructor.
    YYUSE (yysym.type_get ());
  }

#if TYPEDRUBY24DEBUG
  template <typename Base>
  void
  parser::yy_print_ (std::ostream& yyo,
                                     const basic_symbol<Base>& yysym) const
  {
    std::ostream& yyoutput = yyo;
    YYUSE (yyoutput);
    symbol_number_type yytype = yysym.type_get ();
    // Avoid a (spurious) G++ 4.8 warning about "array subscript is
    // below array bounds".
    if (yysym.empty ())
      std::abort ();
    yyo << (yytype < yyntokens_ ? "token" : "nterm")
        << ' ' << yytname_[yytype] << " (";
    YYUSE (yytype);
    yyo << ')';
  }
#endif

  inline
  void
  parser::yypush_ (const char* m, state_type s, symbol_type& sym)
  {
    stack_symbol_type t (s, sym);
    yypush_ (m, t);
  }

  inline
  void
  parser::yypush_ (const char* m, stack_symbol_type& s)
  {
    if (m)
      YY_SYMBOL_PRINT (m, s);
    yystack_.push (s);
  }

  inline
  void
  parser::yypop_ (unsigned int n)
  {
    yystack_.pop (n);
  }

#if TYPEDRUBY24DEBUG
  std::ostream&
  parser::debug_stream () const
  {
    return *yycdebug_;
  }

  void
  parser::set_debug_stream (std::ostream& o)
  {
    yycdebug_ = &o;
  }


  parser::debug_level_type
  parser::debug_level () const
  {
    return yydebug_;
  }

  void
  parser::set_debug_level (debug_level_type l)
  {
    yydebug_ = l;
  }
#endif // TYPEDRUBY24DEBUG

  inline parser::state_type
  parser::yy_lr_goto_state_ (state_type yystate, int yysym)
  {
    int yyr = yypgoto_[yysym - yyntokens_] + yystate;
    if (0 <= yyr && yyr <= yylast_ && yycheck_[yyr] == yystate)
      return yytable_[yyr];
    else
      return yydefgoto_[yysym - yyntokens_];
  }

  inline bool
  parser::yy_pact_value_is_default_ (int yyvalue)
  {
    return yyvalue == yypact_ninf_;
  }

  inline bool
  parser::yy_table_value_is_error_ (int yyvalue)
  {
    return yyvalue == yytable_ninf_;
  }

  int
  parser::parse ()
  {
    // State.
    int yyn;
    /// Length of the RHS of the rule being reduced.
    int yylen = 0;

    // Error handling.
    int yynerrs_ = 0;
    int yyerrstatus_ = 0;

    /// The lookahead symbol.
    symbol_type yyla;

    /// The return value of parse ().
    int yyresult;

    // FIXME: This shoud be completely indented.  It is not yet to
    // avoid gratuitous conflicts when merging into the master branch.
    try
      {
    YYCDEBUG << "Starting parse" << std::endl;


    /* Initialize the stack.  The initial state will be set in
       yynewstate, since the latter expects the semantical and the
       location values to have been already stored, initialize these
       stacks with a primary value.  */
    yystack_.clear ();
    yypush_ (YY_NULLPTR, 0, yyla);

    // A new symbol was pushed on the stack.
  yynewstate:
    YYCDEBUG << "Entering state " << yystack_[0].state << std::endl;

    // Accept?
    if (yystack_[0].state == yyfinal_)
      goto yyacceptlab;

    goto yybackup;

    // Backup.
  yybackup:

    // Try to take a decision without lookahead.
    yyn = yypact_[yystack_[0].state];
    if (yy_pact_value_is_default_ (yyn))
      goto yydefault;

    // Read a lookahead token.
    if (yyla.empty ())
      {
        YYCDEBUG << "Reading a token: ";
        try
          {
            yyla.type = yytranslate_ (yylex (&yyla.value, driver));
          }
        catch (const syntax_error& yyexc)
          {
            error (yyexc);
            goto yyerrlab1;
          }
      }
    YY_SYMBOL_PRINT ("Next token is", yyla);

    /* If the proper action on seeing token YYLA.TYPE is to reduce or
       to detect an error, take that action.  */
    yyn += yyla.type_get ();
    if (yyn < 0 || yylast_ < yyn || yycheck_[yyn] != yyla.type_get ())
      goto yydefault;

    // Reduce or error.
    yyn = yytable_[yyn];
    if (yyn <= 0)
      {
        if (yy_table_value_is_error_ (yyn))
          goto yyerrlab;
        yyn = -yyn;
        goto yyreduce;
      }

    // Count tokens shifted since error; after three, turn off error status.
    if (yyerrstatus_)
      --yyerrstatus_;

    // Shift the lookahead token.
    yypush_ ("Shifting", yyn, yyla);
    goto yynewstate;

  /*-----------------------------------------------------------.
  | yydefault -- do the default action for the current state.  |
  `-----------------------------------------------------------*/
  yydefault:
    yyn = yydefact_[yystack_[0].state];
    if (yyn == 0)
      goto yyerrlab;
    goto yyreduce;

  /*-----------------------------.
  | yyreduce -- Do a reduction.  |
  `-----------------------------*/
  yyreduce:
    yylen = yyr2_[yyn];
    {
      stack_symbol_type yylhs;
      yylhs.state = yy_lr_goto_state_(yystack_[yylen].state, yyr1_[yyn]);
      /* If YYLEN is nonzero, implement the default value of the
         action: '$$ = $1'.  Otherwise, use the top of the stack.

         Otherwise, the following line sets YYLHS.VALUE to garbage.
         This behavior is undocumented and Bison users should not rely
         upon it.  */
      if (yylen)
        yylhs.value = yystack_[yylen - 1].value;
      else
        yylhs.value = yystack_[0].value;


      // Perform the reduction.
      YY_REDUCE_PRINT (yyn);
      try
        {
          switch (yyn)
            {
  case 2:
#line 427 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.ast = (yystack_[0].value.node);
                    }
#line 582 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 3:
#line 432 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.compstmt(self, (yystack_[1].value.list));
                    }
#line 590 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 4:
#line 437 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 598 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 5:
#line 441 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 606 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 6:
#line 445 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yystack_[2].value.list)->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = (yystack_[2].value.list);
                    }
#line 615 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 7:
#line 450 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 623 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 9:
#line 456 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.preexe(self, (yystack_[3].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 631 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 10:
#line 461 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &rescue_bodies = (yystack_[2].value.list);
                      auto &else_ = (yystack_[1].value.with_token);
                      auto &ensure = (yystack_[0].value.with_token);

                      if (rescue_bodies->size() == 0 && else_ != nullptr) {
                        driver.diagnostics.emplace_back(
			  dlevel::WARNING, dclass::UselessElse, else_->tok);
                      }

                      (yylhs.value.node) = driver.build.begin_body(self, (yystack_[3].value.node), rescue_bodies,
						else_ ? else_->tok : nullptr,
						else_ ? else_->nod : nullptr,
						ensure ? ensure->tok : nullptr,
						ensure ? ensure->nod : nullptr);
                    }
#line 652 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 11:
#line 479 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.compstmt(self, (yystack_[1].value.list));
                    }
#line 660 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 12:
#line 484 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 668 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 13:
#line 488 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 676 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 14:
#line 492 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yystack_[2].value.list)->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = (yystack_[2].value.list);
                    }
#line 685 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 15:
#line 497 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 693 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 17:
#line 503 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.diagnostics.emplace_back(dlevel::ERROR,
			dclass::BeginInMethod, (yystack_[3].value.token));
                      YYERROR;
                    }
#line 703 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 18:
#line 510 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_fname();
                    }
#line 711 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 19:
#line 514 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.alias(self, (yystack_[3].value.token), (yystack_[2].value.node), (yystack_[0].value.node));
                    }
#line 719 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 20:
#line 518 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.alias(self, (yystack_[2].value.token), driver.build.gvar(self, (yystack_[1].value.token)), driver.build.gvar(self, (yystack_[0].value.token)));
                    }
#line 727 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 21:
#line 522 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.alias(self, (yystack_[2].value.token), driver.build.gvar(self, (yystack_[1].value.token)), driver.build.back_ref(self, (yystack_[0].value.token)));
                    }
#line 735 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 22:
#line 526 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.diagnostics.emplace_back(dlevel::ERROR, dclass::NthRefAlias, (yystack_[0].value.token));
                      YYERROR;
                    }
#line 744 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 23:
#line 531 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.undef_method(self, (yystack_[1].value.token), (yystack_[0].value.list));
                    }
#line 752 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 24:
#line 535 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.condition_mod(self, (yystack_[2].value.node), nullptr, (yystack_[0].value.node));
                    }
#line 760 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 25:
#line 539 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.condition_mod(self, nullptr, (yystack_[2].value.node), (yystack_[0].value.node));
                    }
#line 768 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 26:
#line 543 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.loop_while_mod(self, (yystack_[2].value.node), (yystack_[0].value.node));
                    }
#line 776 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 27:
#line 547 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.loop_until_mod(self, (yystack_[2].value.node), (yystack_[0].value.node));
                    }
#line 784 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 28:
#line 551 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      ruby_parser::node_list rescue_body(
						driver.build.rescue_body(self, (yystack_[1].value.token), nullptr, nullptr, nullptr, nullptr, (yystack_[0].value.node)));
                      (yylhs.value.node) = driver.build.begin_body(self, (yystack_[2].value.node), &rescue_body, nullptr, nullptr, nullptr, nullptr);
                    }
#line 794 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 29:
#line 557 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.postexe(self, (yystack_[3].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 802 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 31:
#line 562 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.multi_assign(self, (yystack_[2].value.node), (yystack_[0].value.node));
                    }
#line 810 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 32:
#line 566 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assign(self, (yystack_[2].value.node), (yystack_[1].value.token), driver.build.array(self, nullptr, (yystack_[0].value.list), nullptr));
                    }
#line 818 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 33:
#line 570 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.multi_assign(self, (yystack_[2].value.node), (yystack_[0].value.node));
                    }
#line 826 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 34:
#line 574 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_ivardecl(self, (yystack_[2].value.token), (yystack_[0].value.node));
                    }
#line 834 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 36:
#line 580 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assign(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 842 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 37:
#line 584 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 851 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 38:
#line 589 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, driver.build.index(self, (yystack_[5].value.node), (yystack_[4].value.token), (yystack_[3].value.list), (yystack_[2].value.token)), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 860 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 39:
#line 594 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, driver.build.call_method(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token), nullptr, nullptr, nullptr), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 869 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 40:
#line 599 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, driver.build.call_method(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token), nullptr, nullptr, nullptr), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 878 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 41:
#line 604 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto const_node = driver.build.const_op_assignable(self, driver.build.const_fetch(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token)));
                      (yylhs.value.node) = driver.build.op_assign(self, const_node, (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 888 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 42:
#line 610 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, driver.build.call_method(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token), nullptr, nullptr, nullptr), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 897 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 43:
#line 615 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 906 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 45:
#line 622 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      node_list rescue_body(
			driver.build.rescue_body(self, (yystack_[1].value.token), nullptr, nullptr, nullptr, nullptr, (yystack_[0].value.node)));
                      (yylhs.value.node) = driver.build.begin_body(self, (yystack_[2].value.node), &rescue_body, nullptr, nullptr, nullptr, nullptr);
                    }
#line 916 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 48:
#line 631 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.logical_and(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 924 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 49:
#line 635 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.logical_or(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 932 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 50:
#line 639 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.not_op(self, (yystack_[2].value.token), nullptr, (yystack_[0].value.node), nullptr);
                    }
#line 940 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 51:
#line 643 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.not_op(self, (yystack_[1].value.token), nullptr, (yystack_[0].value.node), nullptr);
                    }
#line 948 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 57:
#line 655 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.call_method(self, (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.token), nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 956 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 58:
#line 660 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &block = (yystack_[1].value.delimited_block);
                      block->begin = (yystack_[2].value.token);
                      block->end = (yystack_[0].value.token);
                      (yylhs.value.delimited_block) = block;
                    }
#line 967 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 60:
#line 670 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.call_method(self, nullptr, nullptr, (yystack_[1].value.token), nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 975 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 61:
#line 674 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto method_call = driver.build.call_method(self, nullptr, nullptr, (yystack_[2].value.token), nullptr, (yystack_[1].value.list), nullptr);
                      auto &delimited_block = (yystack_[0].value.delimited_block);
                      (yylhs.value.node) = driver.build.block(self, method_call,
                                      delimited_block->begin,
                                      delimited_block->args,
                                      delimited_block->body,
                                      delimited_block->end);
                      DIAGCHECK();
                    }
#line 990 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 62:
#line 685 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.call_method(self, (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.token), nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 998 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 63:
#line 689 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto method_call = driver.build.call_method(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token), nullptr, (yystack_[1].value.list), nullptr);
                      auto &delimited_block = (yystack_[0].value.delimited_block);
                      (yylhs.value.node) = driver.build.block(self, method_call,
                                      delimited_block->begin,
                                      delimited_block->args,
                                      delimited_block->body,
                                      delimited_block->end);
                      DIAGCHECK();
                    }
#line 1013 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 64:
#line 700 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.call_method(self, (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.token), nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 1021 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 65:
#line 704 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto method_call = driver.build.call_method(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token), nullptr, (yystack_[1].value.list), nullptr);
                      auto &delimited_block = (yystack_[0].value.delimited_block);
                      (yylhs.value.node) = driver.build.block(self, method_call,
                                      delimited_block->begin,
                                      delimited_block->args,
                                      delimited_block->body,
                                      delimited_block->end);
                      DIAGCHECK();
                    }
#line 1036 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 66:
#line 715 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_super(self, (yystack_[1].value.token), nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 1044 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 67:
#line 719 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_yield(self, (yystack_[1].value.token), nullptr, (yystack_[0].value.list), nullptr);
                      DIAGCHECK();
                    }
#line 1053 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 68:
#line 724 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_return(self, (yystack_[1].value.token), nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 1061 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 69:
#line 728 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_break(self, (yystack_[1].value.token), nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 1069 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 70:
#line 732 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_next(self, (yystack_[1].value.token), nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 1077 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 71:
#line 737 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.multi_lhs(self, nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 1085 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 72:
#line 741 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.begin(self, (yystack_[2].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 1093 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 73:
#line 746 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.multi_lhs(self, nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 1101 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 74:
#line 750 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.multi_lhs1(self, (yystack_[2].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 1109 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 76:
#line 756 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[1].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 1119 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 77:
#line 762 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back(driver.build.splat(self, (yystack_[1].value.token), (yystack_[0].value.node)));
                      (yylhs.value.list) = list;
                    }
#line 1129 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 78:
#line 768 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &head = (yystack_[4].value.list);
                      head->push_back(driver.build.splat(self, (yystack_[3].value.token), (yystack_[2].value.node)));
                      head->concat((yystack_[0].value.list));
                      (yylhs.value.list) = head;
                    }
#line 1140 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 79:
#line 775 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[1].value.list);
                      list->push_back(driver.build.splat(self, (yystack_[0].value.token), nullptr));
                      (yylhs.value.list) = list;
                    }
#line 1150 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 80:
#line 781 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &head = (yystack_[3].value.list);
                      head->push_back(driver.build.splat(self, (yystack_[2].value.token), nullptr));
                      head->concat((yystack_[0].value.list));
                      (yylhs.value.list) = head;
                    }
#line 1161 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 81:
#line 788 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list(driver.build.splat(self, (yystack_[1].value.token), (yystack_[0].value.node)));
                    }
#line 1169 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 82:
#line 792 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      node_list *items = driver.alloc.node_list(driver.build.splat(self, (yystack_[3].value.token), (yystack_[2].value.node)));
                      items->concat((yystack_[0].value.list));
                      (yylhs.value.list) = items;
                    }
#line 1179 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 83:
#line 798 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list(driver.build.splat(self, (yystack_[0].value.token), nullptr));
                    }
#line 1187 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 84:
#line 802 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      node_list *items = driver.alloc.node_list(driver.build.splat(self, (yystack_[2].value.token), nullptr));
                      items->concat((yystack_[0].value.list));
                      (yylhs.value.list) = items;
                    }
#line 1197 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 86:
#line 810 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.begin(self, (yystack_[2].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 1205 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 87:
#line 815 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[1].value.node));
                    }
#line 1213 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 88:
#line 819 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[1].value.node));
                      (yylhs.value.list) = list;
                    }
#line 1223 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 89:
#line 826 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 1231 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 90:
#line 830 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 1241 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 91:
#line 837 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1250 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 92:
#line 842 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1259 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 93:
#line 847 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.index_asgn(self, (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 1267 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 94:
#line 851 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.attr_asgn(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 1275 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 95:
#line 855 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.attr_asgn(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 1283 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 96:
#line 859 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.attr_asgn(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 1291 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 97:
#line 863 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, driver.build.const_fetch(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token)));
                      DIAGCHECK();
                    }
#line 1300 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 98:
#line 868 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, driver.build.const_global(self, (yystack_[1].value.token), (yystack_[0].value.token)));
                      DIAGCHECK();
                    }
#line 1309 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 99:
#line 873 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1318 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 100:
#line 879 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1327 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 101:
#line 884 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1336 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 102:
#line 889 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.index_asgn(self, (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 1344 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 103:
#line 893 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.attr_asgn(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 1352 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 104:
#line 897 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.attr_asgn(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 1360 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 105:
#line 901 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.attr_asgn(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 1368 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 106:
#line 905 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, driver.build.const_fetch(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token)));
                      DIAGCHECK();
                    }
#line 1377 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 107:
#line 910 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, driver.build.const_global(self, (yystack_[1].value.token), (yystack_[0].value.token)));
                      DIAGCHECK();
                    }
#line 1386 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 108:
#line 915 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1395 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 109:
#line 921 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.diagnostics.emplace_back(dlevel::ERROR, dclass::ModuleNameConst, (yystack_[0].value.token));
                      YYERROR;
                    }
#line 1404 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 111:
#line 928 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.const_global(self, (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 1412 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 112:
#line 932 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.const_(self, (yystack_[0].value.token));
                    }
#line 1420 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 113:
#line 936 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_gendecl(self, (yystack_[4].value.node), (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 1428 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 114:
#line 940 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.const_fetch(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 1436 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 120:
#line 949 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.symbol(self, (yystack_[0].value.token));
                    }
#line 1444 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 124:
#line 958 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 1452 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 125:
#line 962 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_fname();
                    }
#line 1460 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 126:
#line 966 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[3].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 1470 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 198:
#line 989 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assign(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1478 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 199:
#line 993 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1487 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 200:
#line 998 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, driver.build.index(self, (yystack_[5].value.node), (yystack_[4].value.token), (yystack_[3].value.list), (yystack_[2].value.token)), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1496 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 201:
#line 1003 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, driver.build.call_method(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token), nullptr, nullptr, nullptr), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1505 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 202:
#line 1008 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, driver.build.call_method(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token), nullptr, nullptr, nullptr), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1514 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 203:
#line 1013 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, driver.build.call_method(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token), nullptr, nullptr, nullptr), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1523 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 204:
#line 1018 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto const_ = driver.build.const_op_assignable(self, driver.build.const_fetch(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token)));
                      (yylhs.value.node) = driver.build.op_assign(self, const_, (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1533 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 205:
#line 1024 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto const_ = driver.build.const_op_assignable(self, driver.build.const_global(self, (yystack_[3].value.token), (yystack_[2].value.token)));
                      (yylhs.value.node) = driver.build.op_assign(self, const_, (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1543 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 206:
#line 1030 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.op_assign(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1552 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 207:
#line 1035 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.range_inclusive(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1560 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 208:
#line 1039 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.range_exclusive(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1568 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 209:
#line 1043 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1576 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 210:
#line 1047 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1584 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 211:
#line 1051 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1592 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 212:
#line 1055 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1600 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 213:
#line 1059 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1608 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 214:
#line 1063 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1616 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 215:
#line 1067 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.unary_op(self, (yystack_[3].value.token), driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node)));
                    }
#line 1624 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 216:
#line 1071 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.unary_op(self, (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1632 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 217:
#line 1075 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.unary_op(self, (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1640 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 218:
#line 1079 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1648 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 219:
#line 1083 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1656 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 220:
#line 1087 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1664 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 221:
#line 1091 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1672 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 222:
#line 1095 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1680 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 223:
#line 1099 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1688 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 224:
#line 1103 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1696 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 225:
#line 1107 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1704 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 226:
#line 1111 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1712 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 227:
#line 1115 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1720 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 228:
#line 1119 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1728 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 229:
#line 1123 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.match_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 1737 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 230:
#line 1128 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1745 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 231:
#line 1132 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.not_op(self, (yystack_[1].value.token), nullptr, (yystack_[0].value.node), nullptr);
                    }
#line 1753 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 232:
#line 1136 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.unary_op(self, (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1761 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 233:
#line 1140 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1769 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 234:
#line 1144 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.binary_op(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1777 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 235:
#line 1148 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.logical_and(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1785 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 236:
#line 1152 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.logical_or(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1793 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 237:
#line 1156 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_defined(self, (yystack_[2].value.token), (yystack_[0].value.node));
                    }
#line 1801 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 238:
#line 1160 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.ternary(self, (yystack_[5].value.node), (yystack_[4].value.token), (yystack_[3].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1809 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 243:
#line 1170 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[3].value.list);
                      list->push_back(driver.build.associate(self, nullptr, (yystack_[1].value.list), nullptr));
                      (yylhs.value.list) = list;
                    }
#line 1819 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 244:
#line 1176 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list(driver.build.associate(self, nullptr, (yystack_[1].value.list), nullptr));
                    }
#line 1827 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 246:
#line 1182 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      node_list rescue_body(driver.build.rescue_body(self, (yystack_[1].value.token), nullptr, nullptr, nullptr, nullptr, (yystack_[0].value.node)));
                      (yylhs.value.node) = driver.build.begin_body(self, (yystack_[2].value.node), &rescue_body, nullptr, nullptr, nullptr, nullptr);
                    }
#line 1836 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 247:
#line 1188 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.delimited_list) = driver.alloc.delimited_node_list((yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 1844 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 248:
#line 1193 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.delimited_list) = driver.alloc.delimited_node_list(nullptr, driver.alloc.node_list(), nullptr);
                    }
#line 1852 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 250:
#line 1199 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 1860 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 253:
#line 1205 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[3].value.list);
                      list->push_back(driver.build.associate(self, nullptr, (yystack_[1].value.list), nullptr));
                      (yylhs.value.list) = list;
                    }
#line 1870 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 254:
#line 1211 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list(driver.build.associate(self, nullptr, (yystack_[1].value.list), nullptr));
                    }
#line 1878 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 255:
#line 1216 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 1886 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 256:
#line 1220 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[1].value.list);
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 1896 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 257:
#line 1226 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      node_list *args = driver.alloc.node_list(driver.build.associate(self, nullptr, (yystack_[1].value.list), nullptr));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 1906 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 258:
#line 1232 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->push_back(driver.build.associate(self, nullptr, (yystack_[1].value.list), nullptr));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 1917 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 259:
#line 1239 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 1925 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 260:
#line 1243 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.stack) = driver.copy_stack();
                      driver.lex.cmdarg.push(true);
                    }
#line 1934 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 261:
#line 1248 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.replace_stack((yystack_[1].value.stack));
                      (yylhs.value.list) = (yystack_[0].value.list);
                    }
#line 1943 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 262:
#line 1254 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.block_pass(self, (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 1951 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 263:
#line 1259 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 1959 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 264:
#line 1263 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 1967 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 265:
#line 1268 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 1975 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 266:
#line 1272 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list(driver.build.splat(self, (yystack_[1].value.token), (yystack_[0].value.node)));
                    }
#line 1983 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 267:
#line 1276 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 1993 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 268:
#line 1282 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[3].value.list);
                      list->push_back(driver.build.splat(self, (yystack_[1].value.token), (yystack_[0].value.node)));
                      (yylhs.value.list) = list;
                    }
#line 2003 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 269:
#line 1289 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.array(self, nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 2011 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 271:
#line 1295 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 2021 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 272:
#line 1301 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[3].value.list);
                      list->push_back(driver.build.splat(self, (yystack_[1].value.token), (yystack_[0].value.node)));
                      (yylhs.value.list) = list;
                    }
#line 2031 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 273:
#line 1307 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list(driver.build.splat(self, (yystack_[1].value.token), (yystack_[0].value.node)));
                    }
#line 2039 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 284:
#line 1322 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.call_method(self, nullptr, nullptr, (yystack_[0].value.token), nullptr, nullptr, nullptr);
                    }
#line 2047 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 285:
#line 1326 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.stack) = driver.copy_stack();
                      driver.lex.cmdarg.clear();
                    }
#line 2056 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 286:
#line 1331 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.replace_stack((yystack_[2].value.stack));
                      (yylhs.value.node) = driver.build.begin_keyword(self, (yystack_[3].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 2065 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 287:
#line 1336 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.stack) = driver.copy_stack();
                      driver.lex.cmdarg.clear();
                    }
#line 2074 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 288:
#line 1341 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_endarg();
                    }
#line 2082 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 289:
#line 1345 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.replace_stack((yystack_[3].value.stack));
                      (yylhs.value.node) = driver.build.begin(self, (yystack_[4].value.token), (yystack_[2].value.node), (yystack_[0].value.token));
                    }
#line 2091 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 290:
#line 1350 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_endarg();
                    }
#line 2099 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 291:
#line 1354 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.begin(self, (yystack_[3].value.token), nullptr, (yystack_[0].value.token));
                    }
#line 2107 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 292:
#line 1358 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.begin(self, (yystack_[2].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 2115 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 293:
#line 1362 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_cast(self, (yystack_[4].value.token), (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 2123 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 294:
#line 1366 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.const_fetch(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 2131 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 295:
#line 1370 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.const_global(self, (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 2139 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 296:
#line 1374 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.array(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 2147 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 297:
#line 1378 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.associate(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 2155 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 298:
#line 1382 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_return(self, (yystack_[0].value.token), nullptr, nullptr, nullptr);
                    }
#line 2163 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 299:
#line 1386 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_yield(self, (yystack_[3].value.token), (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                      DIAGCHECK();
                    }
#line 2172 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 300:
#line 1391 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      node_list tmp;
                      (yylhs.value.node) = driver.build.keyword_yield(self, (yystack_[2].value.token), (yystack_[1].value.token), &tmp, (yystack_[0].value.token));
                      DIAGCHECK();
                    }
#line 2182 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 301:
#line 1397 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_yield(self, (yystack_[0].value.token), nullptr, nullptr, nullptr);
                      DIAGCHECK();
                    }
#line 2191 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 302:
#line 1402 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_defined(self, (yystack_[4].value.token), (yystack_[1].value.node));
                    }
#line 2199 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 303:
#line 1406 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.not_op(self, (yystack_[3].value.token), (yystack_[2].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 2207 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 304:
#line 1410 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.not_op(self, (yystack_[2].value.token), (yystack_[1].value.token), nullptr, (yystack_[0].value.token));
                    }
#line 2215 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 305:
#line 1414 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto method_call = driver.build.call_method(self, nullptr, nullptr, (yystack_[1].value.token), nullptr, nullptr, nullptr);
                      auto &delimited_block = (yystack_[0].value.delimited_block);

                      (yylhs.value.node) = driver.build.block(self, method_call,
                        delimited_block->begin,
                        delimited_block->args,
                        delimited_block->body,
                        delimited_block->end);
                      DIAGCHECK();
                    }
#line 2231 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 307:
#line 1427 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &delimited_block = (yystack_[0].value.delimited_block);
                      (yylhs.value.node) = driver.build.block(self, (yystack_[1].value.node),
                        delimited_block->begin,
                        delimited_block->args,
                        delimited_block->body,
                        delimited_block->end);
                      DIAGCHECK();
                    }
#line 2245 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 308:
#line 1437 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto lambda_call = driver.build.call_lambda(self, (yystack_[1].value.token));
                      auto &lambda = (yystack_[0].value.delimited_block);
                      (yylhs.value.node) = driver.build.block(self, lambda_call,
                        lambda->begin,
                        lambda->args,
                        lambda->body,
                        lambda->end);
                      DIAGCHECK();
                    }
#line 2260 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 309:
#line 1448 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &else_ = (yystack_[1].value.with_token);
                      (yylhs.value.node) = driver.build.condition(self, (yystack_[5].value.token), (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.node),
			else_ ? else_->tok : nullptr,
			else_ ? else_->nod : nullptr, (yystack_[0].value.token));
                    }
#line 2271 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 310:
#line 1455 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &else_ = (yystack_[1].value.with_token);
                      (yylhs.value.node) = driver.build.condition(self, (yystack_[5].value.token), (yystack_[4].value.node), (yystack_[3].value.token),
                        else_ ? else_->nod : nullptr,
			else_ ? else_->tok : nullptr, (yystack_[2].value.node), (yystack_[0].value.token));
                    }
#line 2282 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 311:
#line 1462 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.cond.push(true);
                    }
#line 2290 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 312:
#line 1466 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.cond.pop();
                    }
#line 2298 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 313:
#line 1470 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.loop_while(self, (yystack_[6].value.token), (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 2306 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 314:
#line 1474 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.cond.push(true);
                    }
#line 2314 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 315:
#line 1478 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.cond.pop();
                    }
#line 2322 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 316:
#line 1482 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.loop_until(self, (yystack_[6].value.token), (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 2330 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 317:
#line 1486 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &case_body = (yystack_[1].value.case_body);
                      auto &else_ = case_body->els;
                      (yylhs.value.node) = driver.build.case_(self, (yystack_[4].value.token), (yystack_[3].value.node),
                        &case_body->whens,
                        else_ ? else_->tok : nullptr,
			else_ ? else_->nod : nullptr, (yystack_[0].value.token));
                    }
#line 2343 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 318:
#line 1495 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &case_body = (yystack_[1].value.case_body);
                      auto &else_ = case_body->els;
                      (yylhs.value.node) = driver.build.case_(self, (yystack_[3].value.token), nullptr,
                        &case_body->whens,
                        else_ ? else_->tok : nullptr,
			else_ ? else_->nod : nullptr, (yystack_[0].value.token));
                    }
#line 2356 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 319:
#line 1504 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.cond.push(true);
                    }
#line 2364 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 320:
#line 1508 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.cond.pop();
                    }
#line 2372 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 321:
#line 1512 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.for_(self, (yystack_[8].value.token), (yystack_[7].value.node), (yystack_[6].value.token), (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 2380 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 322:
#line 1516 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.extend_static();
                      (yylhs.value.stack) = driver.copy_stack();
                      driver.lex.cmdarg.clear();
                    }
#line 2390 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 323:
#line 1522 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto class_tok = (yystack_[5].value.token);
                      auto end_tok = (yystack_[0].value.token);

                      if (driver.def_level > 0) {
                        driver.diagnostics.emplace_back(dlevel::ERROR, dclass::ClassInDef, class_tok);
                        YYERROR;
                      }

                      auto &superclass_ = (yystack_[3].value.with_token);
                      auto lt_t       = superclass_ ? superclass_->tok : nullptr;
                      auto superclass = superclass_ ? superclass_->nod : nullptr;

                      (yylhs.value.node) = driver.build.def_class(self, class_tok, (yystack_[4].value.node), lt_t, superclass, (yystack_[1].value.node), end_tok);
                      driver.replace_stack((yystack_[2].value.stack));
                      driver.lex.unextend();
                    }
#line 2412 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 324:
#line 1540 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.size) = driver.def_level;
                      driver.def_level = 0;
                    }
#line 2421 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 325:
#line 1544 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.extend_static();
                      (yylhs.value.stack) = driver.copy_stack();
                      driver.lex.cmdarg.clear();
                    }
#line 2431 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 326:
#line 1550 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.def_sclass(self, (yystack_[7].value.token), (yystack_[6].value.token), (yystack_[5].value.node), (yystack_[1].value.node), (yystack_[0].value.token));
                      driver.def_level = (yystack_[3].value.size);
                      driver.replace_stack((yystack_[2].value.stack));
                      driver.lex.unextend();
                    }
#line 2442 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 327:
#line 1557 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.extend_static();
                      (yylhs.value.stack) = driver.copy_stack();
                      driver.lex.cmdarg.clear();
                    }
#line 2452 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 328:
#line 1563 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto module_tok = (yystack_[4].value.token);
                      auto end_tok = (yystack_[0].value.token);

                      if (driver.def_level > 0) {
                        driver.diagnostics.emplace_back(dlevel::ERROR, dclass::ModuleInDef, module_tok);
                        YYERROR;
                      }

                      (yylhs.value.node) = driver.build.def_module(self, module_tok, (yystack_[3].value.node), (yystack_[1].value.node), end_tok);
                      driver.replace_stack((yystack_[2].value.stack));
                      driver.lex.unextend();
                    }
#line 2470 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 329:
#line 1577 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.def_level++;
                      driver.lex.extend_static();
                      (yylhs.value.stack) = driver.copy_stack();
                      driver.lex.cmdarg.clear();
                    }
#line 2481 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 330:
#line 1584 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.def_method(self, (yystack_[5].value.token), (yystack_[4].value.token), (yystack_[2].value.node), (yystack_[1].value.node), (yystack_[0].value.token));
                      driver.replace_stack((yystack_[3].value.stack));
                      driver.lex.unextend();
                      driver.def_level--;
                    }
#line 2492 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 331:
#line 1591 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_fname();
                    }
#line 2500 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 332:
#line 1595 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.def_level++;
                      driver.lex.extend_static();
                      (yylhs.value.stack) = driver.copy_stack();
                      driver.lex.cmdarg.clear();
                    }
#line 2511 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 333:
#line 1602 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.def_singleton(self, (yystack_[8].value.token), (yystack_[7].value.node), (yystack_[6].value.token), (yystack_[4].value.token), (yystack_[2].value.node), (yystack_[1].value.node), (yystack_[0].value.token));
                      DIAGCHECK();

                      driver.replace_stack((yystack_[3].value.stack));
                      driver.lex.unextend();
                      driver.def_level--;
                    }
#line 2524 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 334:
#line 1611 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_break(self, (yystack_[0].value.token), nullptr, nullptr, nullptr);
                    }
#line 2532 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 335:
#line 1615 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_next(self, (yystack_[0].value.token), nullptr, nullptr, nullptr);
                    }
#line 2540 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 336:
#line 1619 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_redo(self, (yystack_[0].value.token));
                    }
#line 2548 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 337:
#line 1623 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_retry(self, (yystack_[0].value.token));
                    }
#line 2556 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 341:
#line 1632 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.token) = (yystack_[0].value.token);
                    }
#line 2564 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 345:
#line 1641 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto elsif_t = (yystack_[4].value.token);
                      auto &else_ = (yystack_[0].value.with_token);
                      (yylhs.value.with_token) = driver.alloc.node_with_token(elsif_t,
                        driver.build.condition(self,
                          elsif_t, (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.node),
                          else_ ? else_->tok : nullptr,
                          else_ ? else_->nod : nullptr, nullptr)
                      );
                    }
#line 2579 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 346:
#line 1653 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.with_token) = nullptr;
                    }
#line 2587 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 347:
#line 1657 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.with_token) = driver.alloc.node_with_token((yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 2595 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 350:
#line 1665 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.arg(self, (yystack_[0].value.token));
                    }
#line 2603 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 351:
#line 1669 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.multi_lhs(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 2611 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 352:
#line 1674 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 2619 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 353:
#line 1678 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 2629 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 355:
#line 1686 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[3].value.list);
                      list->push_back(driver.build.restarg(self, (yystack_[1].value.token), (yystack_[0].value.token)));
                      (yylhs.value.list) = list;
                    }
#line 2639 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 356:
#line 1692 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[5].value.list);
                      args->push_back(driver.build.restarg(self, (yystack_[3].value.token), (yystack_[2].value.token)));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2650 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 357:
#line 1699 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back(driver.build.restarg(self, (yystack_[0].value.token), nullptr));
                      (yylhs.value.list) = list;
                    }
#line 2660 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 358:
#line 1705 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[4].value.list);
                      args->push_back(driver.build.restarg(self, (yystack_[2].value.token), nullptr));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2671 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 359:
#line 1712 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list(driver.build.restarg(self, (yystack_[1].value.token), (yystack_[0].value.token)));
                    }
#line 2679 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 360:
#line 1716 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[0].value.list);
                      args->push_front(driver.build.restarg(self, (yystack_[3].value.token), (yystack_[2].value.token)));
                      (yylhs.value.list) = args;
                    }
#line 2689 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 361:
#line 1722 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list(driver.build.restarg(self, (yystack_[0].value.token), nullptr));
                    }
#line 2697 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 362:
#line 1726 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[0].value.list);
                      args->push_front(driver.build.restarg(self, (yystack_[2].value.token), nullptr));
                      (yylhs.value.list) = args;
                    }
#line 2707 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 363:
#line 1733 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2718 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 364:
#line 1740 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[1].value.list);
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2728 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 365:
#line 1746 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[1].value.list);
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2738 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 366:
#line 1752 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = (yystack_[0].value.list);
                    }
#line 2746 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 367:
#line 1758 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = (yystack_[0].value.list);
                    }
#line 2754 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 368:
#line 1762 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 2762 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 369:
#line 1767 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[5].value.list);
                      args->concat((yystack_[3].value.list));
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2774 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 370:
#line 1775 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[7].value.list);
                      args->concat((yystack_[5].value.list));
                      args->concat((yystack_[3].value.list));
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2787 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 371:
#line 1784 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2798 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 372:
#line 1791 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[5].value.list);
                      args->concat((yystack_[3].value.list));
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2810 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 373:
#line 1799 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2821 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 375:
#line 1807 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[5].value.list);
                      args->concat((yystack_[3].value.list));
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2833 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 376:
#line 1815 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[1].value.list);
                      auto &block_args_tail = (yystack_[0].value.list);

                      if (block_args_tail->size() == 0 && args->size() == 1) {
                        (yylhs.value.list) = driver.alloc.node_list(driver.build.procarg0(self, args->at(0)));
                      } else {
                        args->concat(block_args_tail);
                        (yylhs.value.list) = args;
                      }
                    }
#line 2849 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 377:
#line 1827 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2860 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 378:
#line 1834 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[5].value.list);
                      args->concat((yystack_[3].value.list));
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2872 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 379:
#line 1842 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[1].value.list);
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2882 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 380:
#line 1848 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2893 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 381:
#line 1855 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[1].value.list);
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2903 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 382:
#line 1861 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 2914 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 384:
#line 1870 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.args(self, nullptr, nullptr, nullptr, true);
                      DIAGCHECK();
                    }
#line 2923 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 385:
#line 1875 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_value();
                    }
#line 2931 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 386:
#line 1879 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto args = (yystack_[2].value.node);
                      auto return_sig = (yystack_[0].value.node);

                      if (return_sig) {
                        (yylhs.value.node) = driver.build.prototype(self, nullptr, args, return_sig);
                      } else {
                        (yylhs.value.node) = args;
                      }
                    }
#line 2946 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 387:
#line 1891 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.args(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token), true);
                      DIAGCHECK();
                    }
#line 2955 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 388:
#line 1896 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.args(self, (yystack_[0].value.token), nullptr, (yystack_[0].value.token), true);
                      DIAGCHECK();
                    }
#line 2964 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 389:
#line 1901 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &params = (yystack_[2].value.list);
                      params->concat((yystack_[1].value.list));
                      (yylhs.value.node) = driver.build.args(self, (yystack_[3].value.token), params, (yystack_[0].value.token), true);
                      DIAGCHECK();
                    }
#line 2975 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 390:
#line 1909 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 2983 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 391:
#line 1913 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = (yystack_[1].value.list);
                    }
#line 2991 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 392:
#line 1918 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 2999 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 393:
#line 1922 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 3009 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 394:
#line 1929 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto ident = (yystack_[0].value.token);
                      driver.lex.declare(ident->string());
                      (yylhs.value.node) = driver.build.shadowarg(self, ident);
                    }
#line 3019 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 395:
#line 1935 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.diagnostics.emplace_back(dlevel::ERROR, dclass::ArgumentConst, (yystack_[0].value.token));
                      YYERROR;
                    }
#line 3028 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 396:
#line 1940 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.extend_dynamic();
                    }
#line 3036 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 397:
#line 1944 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.stack) = driver.copy_stack();
                      driver.lex.cmdarg.clear();
                    }
#line 3045 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 398:
#line 1949 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.replace_stack((yystack_[1].value.stack));
                      driver.lex.cmdarg.lexpop();

                      auto &delimited_block = (yystack_[0].value.delimited_block);
                      delimited_block->args = (yystack_[2].value.node);
                      (yylhs.value.delimited_block) = delimited_block;
                      driver.lex.unextend();
                    }
#line 3059 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 399:
#line 1960 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[2].value.list);
                      args->concat((yystack_[1].value.list));
                      (yylhs.value.node) = driver.build.args(self, (yystack_[3].value.token), args, (yystack_[0].value.token), true);
                      DIAGCHECK();
                    }
#line 3070 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 400:
#line 1967 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.args(self, nullptr, (yystack_[0].value.list), nullptr, true);
                      DIAGCHECK();
                    }
#line 3079 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 401:
#line 1973 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.delimited_block) = driver.alloc.delimited_block((yystack_[2].value.token), nullptr, (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 3087 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 402:
#line 1977 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.delimited_block) = driver.alloc.delimited_block((yystack_[2].value.token), nullptr, (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 3095 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 403:
#line 1982 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &delimited_block = (yystack_[1].value.delimited_block);
                      delimited_block->begin = (yystack_[2].value.token);
                      delimited_block->end = (yystack_[0].value.token);
                      (yylhs.value.delimited_block) = delimited_block;
                    }
#line 3106 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 404:
#line 1990 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &delimited_block = (yystack_[0].value.delimited_block);
                      (yylhs.value.node) = driver.build.block(self, (yystack_[1].value.node),
                          delimited_block->begin,
                          delimited_block->args,
                          delimited_block->body,
                          delimited_block->end
                        );
                      DIAGCHECK();
                    }
#line 3121 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 405:
#line 2001 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &delimited = (yystack_[0].value.delimited_list);
                      (yylhs.value.node) = driver.build.call_method(self, (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.token),
                                  delimited->begin,
                                  delimited->inner,
                                  delimited->end);
                    }
#line 3133 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 406:
#line 2009 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &delimited = (yystack_[1].value.delimited_list);
                      auto method_call = driver.build.call_method(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token),
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                      auto &block = (yystack_[0].value.delimited_block);
                      (yylhs.value.node) = driver.build.block(self, method_call,
                          block->begin,
                          block->args,
                          block->body,
                          block->end);
                      DIAGCHECK();
                    }
#line 3152 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 407:
#line 2024 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto method_call = driver.build.call_method(self, (yystack_[4].value.node), (yystack_[3].value.token), (yystack_[2].value.token), nullptr, (yystack_[1].value.list), nullptr);
                      auto &block = (yystack_[0].value.delimited_block);
                      (yylhs.value.node) = driver.build.block(self, method_call, block->begin, block->args, block->body, block->end);
                      DIAGCHECK();
                    }
#line 3163 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 408:
#line 2032 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &delimited = (yystack_[0].value.delimited_list);
                      (yylhs.value.node) = driver.build.call_method(self, nullptr, nullptr, (yystack_[1].value.token),
                        delimited->begin,
                        delimited->inner,
                        delimited->end);
                    }
#line 3175 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 409:
#line 2040 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &delimited = (yystack_[0].value.delimited_list);
                      (yylhs.value.node) = driver.build.call_method(self, (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.token),
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                    }
#line 3187 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 410:
#line 2048 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &delimited = (yystack_[0].value.delimited_list);
                      (yylhs.value.node) = driver.build.call_method(self, (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.token),
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                    }
#line 3199 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 411:
#line 2056 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.call_method(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token), nullptr, nullptr, nullptr);
                    }
#line 3207 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 412:
#line 2060 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &delimited = (yystack_[0].value.delimited_list);
                      (yylhs.value.node) = driver.build.call_method(self, (yystack_[2].value.node), (yystack_[1].value.token), nullptr,
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                    }
#line 3219 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 413:
#line 2068 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &delimited = (yystack_[0].value.delimited_list);
                      (yylhs.value.node) = driver.build.call_method(self, (yystack_[2].value.node), (yystack_[1].value.token), nullptr,
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                    }
#line 3231 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 414:
#line 2076 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &delimited = (yystack_[0].value.delimited_list);
                      (yylhs.value.node) = driver.build.keyword_super(self, (yystack_[1].value.token),
                          delimited->begin,
                          delimited->inner,
                          delimited->end);
                    }
#line 3243 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 415:
#line 2084 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.keyword_zsuper(self, (yystack_[0].value.token));
                    }
#line 3251 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 416:
#line 2088 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.index(self, (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 3259 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 417:
#line 2093 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &block = (yystack_[1].value.delimited_block);
                      block->begin = (yystack_[2].value.token);
                      block->end = (yystack_[0].value.token);
                      (yylhs.value.delimited_block) = block;
                    }
#line 3270 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 418:
#line 2100 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &block = (yystack_[1].value.delimited_block);
                      block->begin = (yystack_[2].value.token);
                      block->end = (yystack_[0].value.token);
                      (yylhs.value.delimited_block) = block;
                    }
#line 3281 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 419:
#line 2107 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.extend_dynamic();
                    }
#line 3289 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 420:
#line 2110 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.stack) = driver.copy_stack();
                      driver.lex.cmdarg.clear();
                    }
#line 3298 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 421:
#line 2115 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.delimited_block) = driver.alloc.delimited_block(nullptr, (yystack_[1].value.node), (yystack_[0].value.node), nullptr);

                      driver.lex.unextend();
                      driver.replace_stack((yystack_[2].value.stack));
                      driver.lex.cmdarg.pop();
                    }
#line 3310 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 422:
#line 2123 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.extend_dynamic();
                    }
#line 3318 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 423:
#line 2126 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.stack) = driver.copy_stack();
                      driver.lex.cmdarg.clear();
                    }
#line 3327 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 424:
#line 2131 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.delimited_block) = driver.alloc.delimited_block(nullptr, (yystack_[1].value.node), (yystack_[0].value.node), nullptr);
                      driver.lex.unextend();

                      driver.replace_stack((yystack_[2].value.stack));
                      driver.lex.cmdarg.pop();
                    }
#line 3339 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 425:
#line 2140 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &cases = (yystack_[0].value.case_body);
                      cases->whens.push_front(driver.build.when(self, (yystack_[4].value.token), (yystack_[3].value.list), (yystack_[2].value.token), (yystack_[1].value.node)));
                      (yylhs.value.case_body) = cases;
                    }
#line 3349 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 426:
#line 2147 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.case_body) = driver.alloc.case_body((yystack_[0].value.with_token));
                    }
#line 3357 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 428:
#line 2153 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &exc_var = (yystack_[3].value.with_token);
                      auto &exc_list_ = (yystack_[4].value.list);
                      auto exc_list = exc_list_
                        ? driver.build.array(self, nullptr, exc_list_, nullptr)
                        : nullptr;
                      auto &rescues = (yystack_[0].value.list);

                      rescues->push_front(driver.build.rescue_body(self, (yystack_[5].value.token),
                          exc_list,
			  exc_var ? exc_var->tok : nullptr,
			  exc_var ? exc_var->nod : nullptr,
			  (yystack_[2].value.token), (yystack_[1].value.node)));

                      (yylhs.value.list) = rescues;
                    }
#line 3378 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 429:
#line 2170 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 3386 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 430:
#line 2175 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 3394 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 433:
#line 2182 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.with_token) = driver.alloc.node_with_token((yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 3402 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 434:
#line 2186 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.with_token) = nullptr;
                    }
#line 3410 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 435:
#line 2191 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.with_token) = driver.alloc.node_with_token((yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 3418 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 436:
#line 2195 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.with_token) = nullptr;
                    }
#line 3426 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 440:
#line 2204 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.string_compose(self, nullptr, (yystack_[0].value.list), nullptr);
                    }
#line 3434 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 441:
#line 2209 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 3442 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 442:
#line 2213 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[1].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 3452 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 443:
#line 2220 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto str = driver.build.string_compose(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                      (yylhs.value.node) = driver.build.dedent_string(self, str, driver.lex.dedent_level().value_or(0));
                    }
#line 3461 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 444:
#line 2225 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto str = driver.build.string(self, (yystack_[0].value.token));
                      (yylhs.value.node) = driver.build.dedent_string(self, str, driver.lex.dedent_level().value_or(0));
                    }
#line 3470 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 445:
#line 2230 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.character(self, (yystack_[0].value.token));
                    }
#line 3478 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 446:
#line 2235 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto xstr = driver.build.xstring_compose(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                      (yylhs.value.node) = driver.build.dedent_string(self, xstr, driver.lex.dedent_level().value_or(0));
                    }
#line 3487 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 447:
#line 2241 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto opts = driver.build.regexp_options(self, (yystack_[0].value.token));
                      (yylhs.value.node) = driver.build.regexp_compose(self, (yystack_[3].value.token), (yystack_[2].value.list), (yystack_[1].value.token), opts);
                      DIAGCHECK();
                    }
#line 3497 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 448:
#line 2248 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.words_compose(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 3505 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 449:
#line 2253 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 3513 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 450:
#line 2257 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back(driver.build.word(self, (yystack_[1].value.list)));
                      (yylhs.value.list) = list;
                    }
#line 3523 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 451:
#line 2264 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 3531 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 452:
#line 2268 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[1].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 3541 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 453:
#line 2275 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.symbols_compose(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 3549 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 454:
#line 2280 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 3557 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 455:
#line 2284 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back(driver.build.word(self, (yystack_[1].value.list)));
                      (yylhs.value.list) = list;
                    }
#line 3567 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 456:
#line 2291 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.words_compose(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 3575 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 457:
#line 2296 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.symbols_compose(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 3583 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 458:
#line 2301 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 3591 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 459:
#line 2305 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back(driver.build.string_internal(self, (yystack_[1].value.token)));
                      (yylhs.value.list) = list;
                    }
#line 3601 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 460:
#line 2312 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 3609 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 461:
#line 2316 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back(driver.build.symbol_internal(self, (yystack_[1].value.token)));
                      (yylhs.value.list) = list;
                    }
#line 3619 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 462:
#line 2323 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 3627 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 463:
#line 2327 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[1].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 3637 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 464:
#line 2334 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 3645 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 465:
#line 2338 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[1].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 3655 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 466:
#line 2345 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 3663 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 467:
#line 2349 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[1].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 3673 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 468:
#line 2356 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.string_internal(self, (yystack_[0].value.token));
                    }
#line 3681 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 469:
#line 2360 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = (yystack_[0].value.node);
                    }
#line 3689 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 470:
#line 2364 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.cond.push(false);
                      driver.lex.cmdarg.push(false);
                    }
#line 3698 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 471:
#line 2369 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.cond.lexpop();
                      driver.lex.cmdarg.lexpop();
                      (yylhs.value.node) = driver.build.begin(self, (yystack_[3].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 3708 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 472:
#line 2376 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.gvar(self, (yystack_[0].value.token));
                    }
#line 3716 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 473:
#line 2380 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.ivar(self, (yystack_[0].value.token));
                    }
#line 3724 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 474:
#line 2384 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.cvar(self, (yystack_[0].value.token));
                    }
#line 3732 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 476:
#line 2391 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_endarg();
                      (yylhs.value.node) = driver.build.symbol(self, (yystack_[0].value.token));
                    }
#line 3741 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 477:
#line 2397 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_endarg();
                      (yylhs.value.node) = driver.build.symbol_compose(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 3750 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 478:
#line 2403 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = (yystack_[0].value.node);
                    }
#line 3758 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 479:
#line 2407 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.negate(self, (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 3766 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 480:
#line 2412 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_endarg();
                      (yylhs.value.node) = driver.build.integer(self, (yystack_[0].value.token));
                    }
#line 3775 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 481:
#line 2417 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_endarg();
                      (yylhs.value.node) = driver.build.float_(self, (yystack_[0].value.token));
                    }
#line 3784 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 482:
#line 2422 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_endarg();
                      (yylhs.value.node) = driver.build.rational(self, (yystack_[0].value.token));
                    }
#line 3793 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 483:
#line 2427 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_endarg();
                      (yylhs.value.node) = driver.build.complex(self, (yystack_[0].value.token));
                    }
#line 3802 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 484:
#line 2432 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_endarg();
                      (yylhs.value.node) = driver.build.rational_complex(self, (yystack_[0].value.token));
                    }
#line 3811 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 485:
#line 2437 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_endarg();
                      (yylhs.value.node) = driver.build.float_complex(self, (yystack_[0].value.token));
                    }
#line 3820 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 486:
#line 2443 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.ident(self, (yystack_[0].value.token));
                    }
#line 3828 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 487:
#line 2447 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.ivar(self, (yystack_[0].value.token));
                    }
#line 3836 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 488:
#line 2451 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.gvar(self, (yystack_[0].value.token));
                    }
#line 3844 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 489:
#line 2455 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.const_(self, (yystack_[0].value.token));
                    }
#line 3852 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 490:
#line 2459 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.cvar(self, (yystack_[0].value.token));
                    }
#line 3860 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 491:
#line 2464 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.nil(self, (yystack_[0].value.token));
                    }
#line 3868 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 492:
#line 2468 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.self_(self, (yystack_[0].value.token));
                    }
#line 3876 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 493:
#line 2472 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.true_(self, (yystack_[0].value.token));
                    }
#line 3884 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 494:
#line 2476 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.false_(self, (yystack_[0].value.token));
                    }
#line 3892 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 495:
#line 2480 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.file_literal(self, (yystack_[0].value.token));
                    }
#line 3900 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 496:
#line 2484 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.line_literal(self, (yystack_[0].value.token));
                    }
#line 3908 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 497:
#line 2488 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.encoding_literal(self, (yystack_[0].value.token));
                    }
#line 3916 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 498:
#line 2493 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.accessible(self, (yystack_[0].value.node));
                    }
#line 3924 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 499:
#line 2497 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.accessible(self, (yystack_[0].value.node));
                    }
#line 3932 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 500:
#line 2502 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 3941 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 501:
#line 2507 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.assignable(self, (yystack_[0].value.node));
                      DIAGCHECK();
                    }
#line 3950 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 502:
#line 2513 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.nth_ref(self, (yystack_[0].value.token));
                    }
#line 3958 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 503:
#line 2517 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.back_ref(self, (yystack_[0].value.token));
                    }
#line 3966 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 504:
#line 2522 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_value();
                    }
#line 3974 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 505:
#line 2526 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.with_token) = driver.alloc.node_with_token((yystack_[3].value.token), (yystack_[1].value.node));
                    }
#line 3982 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 506:
#line 2530 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.with_token) = nullptr;
                    }
#line 3990 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 507:
#line 2535 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_genargs(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 3998 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 508:
#line 2539 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = nullptr;
                    }
#line 4006 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 509:
#line 2544 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.set_state_expr_value();
                    }
#line 4014 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 510:
#line 2548 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto genargs = (yystack_[5].value.node);
                      auto returnsig = (yystack_[0].value.node);
                      auto args = driver.build.args(self, (yystack_[4].value.token), (yystack_[3].value.list), (yystack_[2].value.token), true);
                      DIAGCHECK();

                      if (genargs || returnsig) {
                        (yylhs.value.node) = driver.build.prototype(self, genargs, args, returnsig);
                      } else {
                        (yylhs.value.node) = args;
                      }
                    }
#line 4031 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 511:
#line 2561 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.boolean) = driver.lex.in_kwarg;
                      driver.lex.in_kwarg = true;
                    }
#line 4040 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 512:
#line 2566 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.in_kwarg = (yystack_[3].value.boolean);

                      auto genargs = (yystack_[4].value.node);
                      auto returnsig = (yystack_[1].value.node);
                      auto args = driver.build.args(self, nullptr, (yystack_[2].value.list), nullptr, true);
                      DIAGCHECK();

                      if (genargs || returnsig) {
                        (yylhs.value.node) = driver.build.prototype(self, genargs, args, returnsig);
                      } else {
                        (yylhs.value.node) = args;
                      }
                    }
#line 4059 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 513:
#line 2582 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4070 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 514:
#line 2589 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[1].value.list);
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4080 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 515:
#line 2595 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[1].value.list);
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4090 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 516:
#line 2601 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = (yystack_[0].value.list);
                    }
#line 4098 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 517:
#line 2606 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = (yystack_[0].value.list);
                    }
#line 4106 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 518:
#line 2610 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 4114 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 519:
#line 2615 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[5].value.list);
                      args->concat((yystack_[3].value.list));
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4126 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 520:
#line 2623 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[7].value.list);
                      args->concat((yystack_[5].value.list));
                      args->concat((yystack_[3].value.list));
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4139 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 521:
#line 2632 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4150 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 522:
#line 2639 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[5].value.list);
                      args->concat((yystack_[3].value.list));
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4162 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 523:
#line 2647 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4173 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 524:
#line 2654 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[5].value.list);
                      args->concat((yystack_[3].value.list));
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4185 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 525:
#line 2662 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[1].value.list);
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4195 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 526:
#line 2668 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4206 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 527:
#line 2675 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[5].value.list);
                      args->concat((yystack_[3].value.list));
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4218 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 528:
#line 2683 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {

                      auto &args = (yystack_[1].value.list);
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4229 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 529:
#line 2690 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4240 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 530:
#line 2697 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[1].value.list);
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4250 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 531:
#line 2703 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &args = (yystack_[3].value.list);
                      args->concat((yystack_[1].value.list));
                      args->concat((yystack_[0].value.list));
                      (yylhs.value.list) = args;
                    }
#line 4261 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 532:
#line 2710 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = (yystack_[0].value.list);
                    }
#line 4269 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 533:
#line 2714 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 4277 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 534:
#line 2719 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.diagnostics.emplace_back(dlevel::ERROR, dclass::ArgumentIvar, (yystack_[0].value.token));
                      YYERROR;
                    }
#line 4286 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 535:
#line 2724 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.diagnostics.emplace_back(dlevel::ERROR, dclass::ArgumentGvar, (yystack_[0].value.token));
                      YYERROR;
                    }
#line 4295 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 536:
#line 2729 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.diagnostics.emplace_back(dlevel::ERROR, dclass::ArgumentCvar, (yystack_[0].value.token));
                      YYERROR;
                    }
#line 4304 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 538:
#line 2736 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto ident = (yystack_[0].value.token);
                      driver.lex.declare(ident->string());
                      (yylhs.value.token) = ident;
                    }
#line 4314 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 539:
#line 2743 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.token) = (yystack_[0].value.token);
                    }
#line 4322 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 540:
#line 2748 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[1].value.node);
                      auto arg = driver.build.arg(self, (yystack_[0].value.token));

                      if (argsig) {
                        (yylhs.value.node) = driver.build.typed_arg(self, argsig, arg);
                      } else {
                        (yylhs.value.node) = arg;
                      }
                    }
#line 4337 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 541:
#line 2759 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.multi_lhs(self, (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 4345 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 542:
#line 2764 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 4353 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 543:
#line 2768 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 4363 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 544:
#line 2775 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto label = (yystack_[0].value.token);
                      if (!driver.valid_kwarg_name(label)) {
                        driver.diagnostics.emplace_back(dlevel::ERROR, dclass::ArgumentConst, label);
                        YYERROR;
                      }
                      driver.lex.declare(label->string());
                      (yylhs.value.token) = label;
                    }
#line 4377 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 545:
#line 2786 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[2].value.node);
                      auto arg = driver.build.kwoptarg(self, (yystack_[1].value.token), (yystack_[0].value.node));
                      if (argsig) {
                        (yylhs.value.node) = driver.build.typed_arg(self, argsig, arg);
                      } else {
                        (yylhs.value.node) = arg;
                      }
                    }
#line 4391 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 546:
#line 2796 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[1].value.node);
                      auto arg = driver.build.kwarg(self, (yystack_[0].value.token));
                      if (argsig) {
                        (yylhs.value.node) = driver.build.typed_arg(self, argsig, arg);
                      } else {
                        (yylhs.value.node) = arg;
                      }
                    }
#line 4405 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 547:
#line 2807 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[2].value.node);
                      auto arg = driver.build.kwoptarg(self, (yystack_[1].value.token), (yystack_[0].value.node));

                      if (argsig) {
                        (yylhs.value.node) = driver.build.typed_arg(self, argsig, arg);
                      } else {
                        (yylhs.value.node) = arg;
                      }
                    }
#line 4420 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 548:
#line 2818 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[1].value.node);
                      auto arg = driver.build.kwarg(self, (yystack_[0].value.token));

                      if (argsig) {
                        (yylhs.value.node) = driver.build.typed_arg(self, argsig, arg);
                      } else {
                        (yylhs.value.node) = arg;
                      }
                    }
#line 4435 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 549:
#line 2830 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 4443 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 550:
#line 2834 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 4453 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 551:
#line 2841 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 4461 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 552:
#line 2845 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 4471 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 555:
#line 2854 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[2].value.node);
                      auto ident = (yystack_[0].value.token);

                      driver.lex.declare(ident->string());

                      auto kwrestarg = driver.build.kwrestarg(self, (yystack_[1].value.token), ident);

                      if (argsig) {
                        kwrestarg = driver.build.typed_arg(self, argsig, kwrestarg);
                      }

                      (yylhs.value.list) = driver.alloc.node_list(kwrestarg);
                    }
#line 4490 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 556:
#line 2869 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[1].value.node);
                      auto kwrestarg = driver.build.kwrestarg(self, (yystack_[0].value.token), nullptr);

                      if (argsig) {
                        kwrestarg = driver.build.typed_arg(self, argsig, kwrestarg);
                      }

                      (yylhs.value.list) = driver.alloc.node_list(kwrestarg);
                    }
#line 4505 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 557:
#line 2881 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[3].value.node);
                      auto arg = driver.build.optarg(self, (yystack_[2].value.token), (yystack_[1].value.token), (yystack_[0].value.node));
                      if (argsig) {
                        (yylhs.value.node) = driver.build.typed_arg(self, argsig, arg);
                      } else {
                        (yylhs.value.node) = arg;
                      }
                    }
#line 4519 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 558:
#line 2892 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[3].value.node);
                      auto arg = driver.build.optarg(self, (yystack_[2].value.token), (yystack_[1].value.token), (yystack_[0].value.node));
                      if (argsig) {
                        (yylhs.value.node) = driver.build.typed_arg(self, argsig, arg);
                      } else {
                        (yylhs.value.node) = arg;
                      }
                    }
#line 4533 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 559:
#line 2903 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 4541 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 560:
#line 2907 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 4551 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 561:
#line 2914 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 4559 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 562:
#line 2918 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 4569 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 565:
#line 2927 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[2].value.node);
                      auto ident = (yystack_[0].value.token);

                      driver.lex.declare(ident->string());

                      auto restarg = driver.build.restarg(self, (yystack_[1].value.token), ident);

                      if (argsig) {
                        restarg = driver.build.typed_arg(self, argsig, restarg);
                      }

                      (yylhs.value.list) = driver.alloc.node_list(restarg);
                    }
#line 4588 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 566:
#line 2942 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[1].value.node);
                      auto restarg = driver.build.restarg(self, (yystack_[0].value.token), nullptr);

                      if (argsig) {
                        restarg = driver.build.typed_arg(self, argsig, restarg);
                      }

                      (yylhs.value.list) = driver.alloc.node_list(restarg);
                    }
#line 4603 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 569:
#line 2956 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[2].value.node);
                      auto ident = (yystack_[0].value.token);

                      driver.lex.declare(ident->string());

                      auto blockarg = driver.build.blockarg(self, (yystack_[1].value.token), ident);

                      if (argsig) {
                        blockarg = driver.build.typed_arg(self, argsig, blockarg);
                      }

                      (yylhs.value.list) = driver.alloc.node_list(blockarg);
                    }
#line 4622 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 570:
#line 2971 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto argsig = (yystack_[1].value.node);
                      auto blockarg = driver.build.blockarg(self, (yystack_[0].value.token), nullptr);

                      if (argsig) {
                        blockarg = driver.build.typed_arg(self, argsig, blockarg);
                      }

                      (yylhs.value.list) = driver.alloc.node_list(blockarg);
                    }
#line 4637 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 571:
#line 2983 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = (yystack_[0].value.list);
                    }
#line 4645 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 572:
#line 2987 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 4653 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 574:
#line 2993 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = (yystack_[1].value.node);
                    }
#line 4661 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 575:
#line 2998 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list();
                    }
#line 4669 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 577:
#line 3004 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 4677 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 578:
#line 3008 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 4687 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 579:
#line 3015 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.pair(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 4695 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 580:
#line 3019 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.pair_keyword(self, (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 4703 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 581:
#line 3023 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.pair_quoted(self, (yystack_[3].value.token), (yystack_[2].value.list), (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 4711 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 582:
#line 3027 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.kwsplat(self, (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 4719 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 595:
#line 3036 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      // XXX what is this???
                      // $$ = put(p, [:dot, $1[1]]
                      (yylhs.value.token) = (yystack_[0].value.token);
                    }
#line 4729 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 596:
#line 3042 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      // XXX what is this???
                      // $$ = [:anddot, $1[1]]
                      (yylhs.value.token) = (yystack_[0].value.token);
                    }
#line 4739 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 601:
#line 3050 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.token) = (yystack_[0].value.token);
                    }
#line 4747 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 602:
#line 3054 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.token) = (yystack_[0].value.token);
                    }
#line 4755 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 606:
#line 3060 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                    yyerrok;
                  }
#line 4763 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 610:
#line 3069 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                    (yylhs.value.node) = nullptr;
                  }
#line 4771 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 611:
#line 3074 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                    (yylhs.value.list) = nullptr;
                  }
#line 4779 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 612:
#line 3079 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.const_global(self, (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 4787 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 613:
#line 3083 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.const_(self, (yystack_[0].value.token));
                    }
#line 4795 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 614:
#line 3087 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.const_fetch(self, (yystack_[2].value.node), (yystack_[1].value.token), (yystack_[0].value.token));
                    }
#line 4803 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 615:
#line 3092 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 4813 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 616:
#line 3098 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 4821 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 617:
#line 3103 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_cpath(self, (yystack_[0].value.node));
                    }
#line 4829 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 618:
#line 3107 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_geninst(self, (yystack_[4].value.node), (yystack_[2].value.token), (yystack_[1].value.list), (yystack_[0].value.token));
                    }
#line 4837 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 619:
#line 3111 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_array(self, (yystack_[2].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 4845 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 620:
#line 3115 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &types = (yystack_[1].value.list);
                      types->push_front((yystack_[3].value.node));
                      (yylhs.value.node) = driver.build.tr_tuple(self, (yystack_[4].value.token), types, (yystack_[0].value.token));
                    }
#line 4855 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 621:
#line 3121 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_hash(self, (yystack_[4].value.token), (yystack_[3].value.node), (yystack_[2].value.token), (yystack_[1].value.node), (yystack_[0].value.token));
                    }
#line 4863 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 622:
#line 3125 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto blockproto = (yystack_[2].value.node);
                      auto returnsig = (yystack_[1].value.node);

                      auto prototype = returnsig
                        ? driver.build.prototype(self, nullptr, blockproto, returnsig)
                        : blockproto;

                      (yylhs.value.node) = driver.build.tr_proc(self, (yystack_[3].value.token), prototype, (yystack_[0].value.token));
                    }
#line 4878 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 623:
#line 3136 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_nillable(self, (yystack_[1].value.token), (yystack_[0].value.node));
                    }
#line 4886 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 624:
#line 3140 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_nil(self, (yystack_[0].value.token));
                    }
#line 4894 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 625:
#line 3144 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      if ((yystack_[0].value.token)->string() == "any") {
                        (yylhs.value.node) = driver.build.tr_any(self, (yystack_[0].value.token));
                      } else if ((yystack_[0].value.token)->string() == "class") {
                        (yylhs.value.node) = driver.build.tr_class(self, (yystack_[0].value.token));
                      } else if ((yystack_[0].value.token)->string() == "instance") {
                        (yylhs.value.node) = driver.build.tr_instance(self, (yystack_[0].value.token));
                      } else if ((yystack_[0].value.token)->string() == "self") {
                        (yylhs.value.node) = driver.build.tr_self(self, (yystack_[0].value.token));
                      } else {
                        driver.diagnostics.emplace_back(dlevel::ERROR, dclass::UnexpectedToken, (yystack_[0].value.token), (yystack_[0].value.token)->string());
                        YYERROR;
                      }
                    }
#line 4913 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 626:
#line 3159 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = (yystack_[1].value.node);
                    }
#line 4921 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 627:
#line 3164 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_or(self, (yystack_[2].value.node), (yystack_[0].value.node));
                    }
#line 4929 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 629:
#line 3170 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = (yystack_[0].value.node);
                      driver.lex.set_state_expr_beg();
                    }
#line 4938 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 630:
#line 3175 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = nullptr;
                    }
#line 4946 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 631:
#line 3180 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = (yystack_[0].value.node);
                    }
#line 4954 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 632:
#line 3184 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = nullptr;
                    }
#line 4962 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 633:
#line 3189 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_conunify(self, (yystack_[2].value.node), (yystack_[0].value.node));
                    }
#line 4970 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 634:
#line 3193 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_consubtype(self, (yystack_[2].value.node), (yystack_[0].value.node));
                    }
#line 4978 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 635:
#line 3198 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_gendeclarg(self, (yystack_[0].value.token), nullptr);
                    }
#line 4986 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 636:
#line 3202 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.node) = driver.build.tr_gendeclarg(self, (yystack_[2].value.token), (yystack_[0].value.node));
                    }
#line 4994 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 637:
#line 3207 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      auto &list = (yystack_[2].value.list);
                      list->push_back((yystack_[0].value.node));
                      (yylhs.value.list) = list;
                    }
#line 5004 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 638:
#line 3213 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      (yylhs.value.list) = driver.alloc.node_list((yystack_[0].value.node));
                    }
#line 5012 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 639:
#line 3217 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    { driver.lex.extend_dynamic(); }
#line 5018 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;

  case 640:
#line 3219 "cc/grammars/typedruby24.ypp" // lalr1.cc:859
    {
                      driver.lex.unextend();
                      (yylhs.value.node) = (yystack_[0].value.node);
                    }
#line 5027 "cc/grammars/typedruby24.cc" // lalr1.cc:859
    break;


#line 5031 "cc/grammars/typedruby24.cc" // lalr1.cc:859
            default:
              break;
            }
        }
      catch (const syntax_error& yyexc)
        {
          error (yyexc);
          YYERROR;
        }
      YY_SYMBOL_PRINT ("-> $$ =", yylhs);
      yypop_ (yylen);
      yylen = 0;
      YY_STACK_PRINT ();

      // Shift the result of the reduction.
      yypush_ (YY_NULLPTR, yylhs);
    }
    goto yynewstate;

  /*--------------------------------------.
  | yyerrlab -- here on detecting error.  |
  `--------------------------------------*/
  yyerrlab:
    // If not already recovering from an error, report this error.
    if (!yyerrstatus_)
      {
        ++yynerrs_;
        error (yysyntax_error_ (yystack_[0].state, yyla));
      }


    if (yyerrstatus_ == 3)
      {
        /* If just tried and failed to reuse lookahead token after an
           error, discard it.  */

        // Return failure if at end of input.
        if (yyla.type_get () == yyeof_)
          YYABORT;
        else if (!yyla.empty ())
          {
            yy_destroy_ ("Error: discarding", yyla);
            yyla.clear ();
          }
      }

    // Else will try to reuse lookahead token after shifting the error token.
    goto yyerrlab1;


  /*---------------------------------------------------.
  | yyerrorlab -- error raised explicitly by YYERROR.  |
  `---------------------------------------------------*/
  yyerrorlab:

    /* Pacify compilers like GCC when the user code never invokes
       YYERROR and the label yyerrorlab therefore never appears in user
       code.  */
    if (false)
      goto yyerrorlab;
    /* Do not reclaim the symbols of the rule whose action triggered
       this YYERROR.  */
    yypop_ (yylen);
    yylen = 0;
    goto yyerrlab1;

  /*-------------------------------------------------------------.
  | yyerrlab1 -- common code for both syntax error and YYERROR.  |
  `-------------------------------------------------------------*/
  yyerrlab1:
    yyerrstatus_ = 3;   // Each real token shifted decrements this.
    {
      stack_symbol_type error_token;
      for (;;)
        {
          yyn = yypact_[yystack_[0].state];
          if (!yy_pact_value_is_default_ (yyn))
            {
              yyn += yyterror_;
              if (0 <= yyn && yyn <= yylast_ && yycheck_[yyn] == yyterror_)
                {
                  yyn = yytable_[yyn];
                  if (0 < yyn)
                    break;
                }
            }

          // Pop the current state because it cannot handle the error token.
          if (yystack_.size () == 1)
            YYABORT;

          yy_destroy_ ("Error: popping", yystack_[0]);
          yypop_ ();
          YY_STACK_PRINT ();
        }


      // Shift the error token.
      error_token.state = yyn;
      yypush_ ("Shifting", error_token);
    }
    goto yynewstate;

    // Accept.
  yyacceptlab:
    yyresult = 0;
    goto yyreturn;

    // Abort.
  yyabortlab:
    yyresult = 1;
    goto yyreturn;

  yyreturn:
    if (!yyla.empty ())
      yy_destroy_ ("Cleanup: discarding lookahead", yyla);

    /* Do not reclaim the symbols of the rule whose action triggered
       this YYABORT or YYACCEPT.  */
    yypop_ (yylen);
    while (1 < yystack_.size ())
      {
        yy_destroy_ ("Cleanup: popping", yystack_[0]);
        yypop_ ();
      }

    return yyresult;
  }
    catch (...)
      {
        YYCDEBUG << "Exception caught: cleaning lookahead and stack"
                 << std::endl;
        // Do not try to display the values of the reclaimed symbols,
        // as their printer might throw an exception.
        if (!yyla.empty ())
          yy_destroy_ (YY_NULLPTR, yyla);

        while (1 < yystack_.size ())
          {
            yy_destroy_ (YY_NULLPTR, yystack_[0]);
            yypop_ ();
          }
        throw;
      }
  }

  void
  parser::error (const syntax_error& yyexc)
  {
    error (yyexc.what());
  }

  // Generate an error message.
  std::string
  parser::yysyntax_error_ (state_type, const symbol_type&) const
  {
    return YY_("syntax error");
  }


  const short int parser::yypact_ninf_ = -925;

  const short int parser::yytable_ninf_ = -600;

  const short int
  parser::yypact_[] =
  {
    2603,  6537,  9201,  9885, 10683, 10554,  -925,  8805,  8805,  7605,
    -925,  -925,  9333,  6801,  6801,  -925,  -925,  6801,  5590,  5176,
    -925,  -925,  -925,  -925,    79, 10425,   -48,     7,    46,  -925,
    -925,  -925,  4486,  5452,  -925,  -925,  4624,  -925,  -925,  -925,
    -925,  -925,  8937,  8937,   739,   113,  3479,    53,  7209,  7749,
    9465,  8937,  9069,  -925,  -925,  -925,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,   200,
    -925,     6,  -925,  1163,  -925,   648,  -925,  -925,   190,   136,
     162,  -925,   163,  9609,  -925,   252,  9334,   576,   782,   264,
     176,  -925,  -925,   462,  -925,  -925,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,  -925,  -925,   321,   328,  -925,   267,   406,
    -925,  -925, 10797,  -925,  -925,  -925,   279,   287,   291,   -48,
     484,   492,   739,  8805,   191,  3625,   190,  -925,   312,  -925,
     804,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,
    -925,  -925,   353,   453,   459,   528,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,   539,   620,   703,   834,  -925,   293,   855,
    -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,  -925,  -925,  8805,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,  -925,  -925,   264,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,   307,  -925,
    -925,  2749,   408,   648,    51,   429,   845,   239,   275,   405,
     331,    51,  -925,  -925,     6,   478,  -925,   427,  8805,  8805,
     487,  -925,  -925,   874,   533,    75,    82,    87,  8937,  8937,
    8937,  8937,  8937,  -925,  -925,  9334,   497,  -925,  -925,   474,
     482,  -925,  -925,  -925,  6669,  -925,  6801,  6801,  -925,  -925,
    7881,  -925,  8805,   766,  -925,  8013,  3771,  3917,  -925,   893,
     555,   559,   412,  6945,   525,  3479,   566,     6,  -925,  1163,
     118,   -48,   594,  6945,   -48,   579,   194,   371,  -925,   497,
     562,   371,   623, 10017,   563,   963,   981,  1002,  1003,  -925,
    -925,  -925,   180,   334,   589,   651,   707,    65,   835,   164,
    -925,  1293,  -925,  -925,  6249,  8805,  8805,  8805,  8805,  6945,
    8805,  8805,  -925,  -925,  -925,   615,  -925,  -925,  -925,  8145,
    -925,  3479,  9741,   586,  8145,  8937,  8937,  8937,  8937,  8937,
    8937,  8937,  8937,  8937,  8937,  8937,  8937,  8937,  8937,  8937,
    8937,  8937,  8937,  8937,  8937,  8937,  8937,  8937,  8937,  8937,
    8937,  -925, 11035,  6801,  -925, 11097,  -925, 11903,  -925,  -925,
    -925,  9069,  9069,   636,  -925,   428,  -925,  1004,  -925,  -925,
    -925, 10973,  6801, 11159,  2749,   941,   235,   640,  -925,  -925,
     738,   743,   401,  -925,  2895,   740,  8937, 11221,  6801, 11283,
    8937,  8937,  3187,   478,  8277,   749,  -925,    64,    64,   105,
   11345,  6801, 11407,  -925,  -925,  -925,  -925,  -925,   629,  8937,
    7077,  -925,  7341,  -925,   -48,   668,  -925,  -925,   -48,  -925,
     638,   645,   235,  -925,  -925,  -925,  -925,  -925, 10554,  8805,
    9334,   656,   669, 11221, 11283,  8937,  8937,  1163,  3771,   -48,
    -925,  -925,  6393,   941,   652,  1163,   691,  -925,  -925,  7473,
    -925,  7749,  -925,  -925,  -925,  1004,  -925,   672, 10017, 11469,
    6801, 11531,  -925,  -925,  1227,  -925,  -925,  -925,  -925,  -925,
     673,  -925,  -925,   249,  -925,   674,  -925,  -925,   386,   676,
    -925,  -925,  -925,   752,  1928,  1481,   941,   941,   941,  -925,
    -925,  -925,  -925,  -925,   682,  -925,   684,   686,  -925,   697,
     701,  -925,   756,  -925,  1428,  -925,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,   833,  -925,   729,  -925,  -925,  -925,   848,
    8937,  -925,   727,   730,  -925,  -925,   -48, 10017,   747,  -925,
    -925,  -925,   842,   806,  2146,  -925,  -925,  -925,  1042,   435,
     555,  5672,  5672,  5672,  5672,  1627,  1627,  5803,  2281,  5672,
    5672,  5934,  5934,  1237,  1237,   555,  1996,   555,   555,   915,
     915,  1627,  1627,  1738,  1738,  9202,  5721,  4762,  5983,  4900,
    -925,   287,  -925,   -48,   442,  -925,   513,  -925,  -925,  5314,
    -925,  -925,  1509,  -925,  2427,  -925,  -925,  -925,  -925,  8805,
    2749,   732,   647,   844,  -925,   287,   -48,   287,   879,   941,
    -925,  -925,   844,   800,  2749, 10911, 10554,  -925,  8409,   887,
     536,  -925,  4348,  5038,   -48,   454,   565,   887,   894,    41,
    -925,  -925,  -925,  -925,  -925,    83,    88,   -48,   124,   129,
    8805,  8937,  -925,  8937,   497,  -925,   482,  -925,  -925,  -925,
    -925,  7077,  7341,  -925,  -925,   235,  -925,  -925,   555,  -925,
     788,   179,  -925,   829,   -48,  -925,   371, 10017,   672,   815,
     647,   -48,   569,   604,  4063,  -925,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,
    -925,  1928,   319,  -925,   797,   -48,  -925,  -925,  -925,   131,
     -48,   420,   847,   857,   174,  -925,    85,  1499,  -925,   941,
    -925,   941,  -925,  1499,  -925,  1499,  -925,   206,  -925,  -925,
    -925,  -925,  -925,  -925,  -925,  -925,   861,  8937,   897,   899,
     906,  -925,   174,  -925,   174,   849,  -925,   824,  8541,  -925,
     672, 10017,  6945,  9069,  8937, 11593,  6801, 11655,   837,  9069,
    9069,  -925,   615,   871,   657,  9069,  9069,  -925,  -925,   615,
     176,   136,  -925,     6,   964,   839,  -925,   437,  -925,  -925,
     437,  1764,  1611,   967,  -925,  -925,   727,  -925,   891,  -925,
    3333,   976,  -925,  8805,   986,  -925,  8937,  8937,   617,  8937,
    8937,   990,  -925,  8673,  3041,  4209,  4209,   133,    64,  -925,
    -925,  -925,   866,  -925,  -925,  -925,  -925,  -925,  -925,   779,
     873,   -48,  1011,   868,  1337,  -925,   941,  -925,   916,   870,
     941,  -925,   941,   941,   895,  -925,  1813,  -925,  4209,  3917,
    -925,  -925,  -925,   876,   878,  -925,   686,  -925,  1056,   607,
     886,  -925,   892,   886,  1635,  -925,   941,  8937,  -925,  -925,
    -925,  -925,  4209,  -925,  3917,  -925,  8937,   901,   672,  -925,
    9334,  5852,  6114,   -48,   635,   658,  8937,  -925,  -925,  -925,
    -925,  -925,  -925,  9069,  -925,  -925,  -925,  -925,  -925,  -925,
    -925,  2749,  -925,  -925,   941,   844,  -925,  -925,   -48,   857,
    -925,   640, 10149,    51,  -925,  4209,  -925,    51,  -925,  8937,
    -925,  -925,   421,  1030,  1031,  -925,  7341,  -925,  -925,  1408,
     908,  1011,   473,  -925,  -925,  -925,   864,   449,  -925,   911,
    -925,  -925,  -925,   -48,   937,   918,  -925,   920,   686,  -925,
     924,   935,  -925,  1428,  1061,   959,  1499,  -925,  1499,  -925,
    -925,  1499,  -925,  1499,  -925,  -925,  -925,   449,  -925,  -925,
     857,  -925,   950,   659,  9334,  -925,  -925,  1077,   203,  -925,
    -925,  -925,     6,  2749,  1038,  -925,  1073,    66,    77,    89,
    2749,  -925,  2895,  -925,  -925,  -925,  -925,  -925,  4209,  1011,
     908,  1011,   961,  -925,   464,  -925,  -925,   941,  -925,  -925,
     991,  -925,  1864,  -925,   941,  -925,  -925,  1499,  -925,  1499,
    -925,  1032, 10281,  -925,  -925,   886,   970,   886,   886,  -925,
    -925,  -925,   941,   941,   857,  -925,  1096,    96, 11717,  6801,
   11779,   743,   536,  1098,   908,  1011,   864,  -925,  -925,  -925,
    -925,   974,   980,  -925,   686,  1056,   996,  -925,  1000,   996,
    1635, 10281,  1055,  1074,  -925,  1499,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,    60,    62,   -48,    98,   128,  -925,  -925,
    -925,   908,  -925,  1499,  -925,  1499,  -925,  -925,  1499,  -925,
    1499,  -925,  -925,  1074,  -925, 11841,   886,   134,   996,  1001,
     996,   996,  1035,  1047,  -925,  -925,  1499,  -925,  -925,  -925,
     996,  -925
  };

  const unsigned short int
  parser::yydefact_[] =
  {
       0,     0,     0,     0,     0,     0,   285,     0,     0,   597,
     311,   314,     0,   334,   335,   336,   337,   298,   260,   260,
     492,   491,   493,   494,   599,     0,   599,     0,     0,   496,
     495,   497,   583,   585,   488,   487,   584,   490,   502,   503,
     480,   481,     0,     0,     0,     0,     0,   287,   611,   575,
      83,     0,     0,   464,   462,   464,   466,   449,   458,   454,
     460,   444,   476,   396,   445,   482,   483,   484,   485,     0,
       2,   597,     5,     8,    30,    35,    47,    55,   260,    54,
       0,    71,     0,    75,    85,     0,    52,   239,     0,    56,
     306,   274,   275,   440,   441,   276,   277,   278,   280,   279,
     281,   438,   439,   437,   478,   498,   499,   282,     0,   283,
      59,     7,     0,   334,   335,   298,   301,   415,     0,   599,
     109,   110,     0,     0,     0,     0,     0,   112,   506,   338,
       0,   498,   499,   283,   327,   167,   178,   168,   191,   164,
     184,   174,   173,   194,   195,   189,   172,   171,   166,   192,
     196,   197,   176,   165,   179,   183,   185,   177,   170,   186,
     193,   188,   187,   180,   190,   175,   163,   182,   181,   162,
     169,   160,   161,   157,   158,   159,   115,   117,   487,   116,
     151,   152,   148,   130,   131,   132,   139,   136,   138,   133,
     134,   153,   154,   140,   141,     0,   145,   144,   129,   150,
     147,   146,   155,   142,   143,   137,   135,   127,   149,   128,
     156,   329,   118,   119,   573,     0,   187,   180,   190,   175,
     157,   158,   159,   115,   116,   120,   122,   124,    23,   121,
     123,     0,     0,    53,     0,     0,     0,   498,   499,     0,
     283,     0,   607,   606,   597,     0,   608,   598,     0,     0,
       0,   349,   348,     0,     0,   498,   499,   283,     0,     0,
       0,     0,     0,   462,   255,   240,   265,    69,   259,   264,
     264,   577,    70,    68,   599,    67,     0,   250,   414,    66,
     599,   600,     0,     0,    18,     0,     0,     0,   216,     0,
     217,   479,   295,     0,     0,     0,     0,   597,    13,    16,
      35,   599,    73,     0,   599,     0,   603,   603,   241,     0,
       0,   603,     0,     0,    81,     0,    91,    92,    99,   232,
      51,   231,     0,     0,     0,     0,     0,     0,     0,     0,
     308,   630,     1,     3,   598,     0,     0,     0,     0,     0,
       0,     0,   422,   419,   408,    60,   305,   422,   404,     0,
      87,     0,    79,    76,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   595,     0,   250,   596,     0,   594,     0,   593,   307,
     442,     0,     0,     0,   479,     0,   109,   110,   111,   504,
     322,     0,   250,     0,     0,     0,   599,   508,   331,   125,
       0,   429,   295,   340,     0,   339,     0,     0,   250,     0,
       0,     0,     0,     0,     0,     0,   609,     0,     0,   295,
       0,   250,     0,   319,   580,   266,   262,   582,     0,     0,
       0,   256,     0,   257,   599,     0,   300,   261,   599,   251,
     264,   264,   599,   304,    50,    20,    22,    21,     0,     0,
     237,     0,     0,     0,     0,     0,     0,    15,     0,   599,
     292,    11,   598,     0,    72,   288,     0,   296,   604,   605,
     242,   605,   244,   297,   576,    98,    89,    84,     0,     0,
     250,     0,   468,   470,     0,   477,   465,   443,   463,   446,
       0,   467,   448,     0,   451,     0,   456,   453,     0,     0,
     457,   624,   613,     0,     0,   630,     0,   639,     0,   625,
     397,   532,   400,   542,   518,   551,   572,   572,   561,   518,
     518,   516,   617,   629,     0,     6,    24,    25,    26,    27,
      28,    48,    49,     0,   423,     0,   420,   419,    61,     0,
       0,    31,   270,     0,    33,   269,   599,     0,    77,    88,
      46,    36,    44,     0,   245,   265,   198,    32,     0,   283,
     214,   221,   226,   227,   228,   223,   225,   235,   236,   229,
     230,   207,   208,   233,   234,   211,   220,   213,   212,   209,
     210,   224,   222,   218,   219,   599,   586,   588,   587,   589,
     413,   260,   411,   599,   586,   588,   587,   589,   412,   260,
     586,   587,   260,    37,   245,   199,    43,   206,   324,     0,
       0,   109,   110,     0,   114,     0,   599,   248,     0,     0,
      34,   574,     0,   511,     0,     0,     0,   286,   611,   610,
     610,   341,   586,   587,   599,   586,   587,   610,     0,     0,
     318,   343,   312,   342,   315,   590,   294,   599,   586,   587,
       0,     0,   579,     0,   267,   263,   264,   578,   299,   601,
     247,   252,   254,   303,    19,   599,     9,    29,   215,   205,
       0,    74,    14,     0,   599,   291,   603,     0,    82,   590,
      97,   599,   586,   587,     0,   472,   473,   474,   469,   475,
     447,   450,   452,   459,   455,   461,   612,   538,   535,   534,
     536,     0,   361,   352,   354,   599,   537,   350,   628,   599,
     599,   599,     0,   632,     0,   623,     0,   630,   525,   630,
     514,   630,   515,   630,   528,   630,   530,     0,   544,   553,
     564,   563,   568,   567,   554,   539,   540,   546,   556,   566,
     570,   418,   384,   417,   384,     0,   403,   273,     0,    86,
      80,     0,     0,     0,     0,     0,   250,     0,     0,     0,
       0,   410,    64,     0,   416,     0,     0,   249,   409,    62,
     405,    57,   325,     0,     0,   635,   638,   599,   416,   328,
     599,   630,   630,     0,   332,   126,   430,   431,   434,   432,
       0,   436,   346,     0,     0,   344,     0,     0,   416,     0,
       0,     0,   317,     0,     0,     0,     0,   416,     0,   581,
     268,   258,   264,   302,    17,   293,   289,   243,    90,   416,
       0,   599,     0,   359,     0,   541,     0,   626,     0,   390,
       0,   619,     0,     0,     0,   388,   630,   640,     0,     0,
     398,   517,   543,   518,   518,   552,   572,   571,     0,     0,
     518,   562,   518,   518,     0,   614,     0,     0,   545,   555,
     565,   569,     0,   385,     0,    58,     0,   271,    78,    45,
     246,   586,   587,   599,   586,   587,     0,    42,   203,    41,
     204,    65,   602,     0,    39,   201,    40,   202,    63,   406,
     407,     0,   505,   323,     0,     0,   113,   507,   599,   632,
     330,   508,     0,     0,   347,     0,    10,     0,   309,     0,
     310,   267,   610,     0,     0,   320,   253,   471,   351,     0,
     362,     0,   357,   353,   627,   399,     0,   599,   616,     0,
     631,   622,   383,   599,     0,   368,   549,   572,   572,   559,
     368,   368,   366,     0,     0,     0,   630,   521,   630,   523,
     513,   630,   529,   630,   526,   531,   540,   599,   557,   424,
     632,   421,   272,   416,   238,    38,   200,     0,     0,   636,
     637,   509,     0,     0,     0,   433,     0,   498,   499,   283,
       0,   435,     0,   426,   427,   425,   313,   316,     0,     0,
     360,     0,   355,   394,   599,   392,   395,     0,   620,   621,
       0,   387,   630,   376,   630,   364,   365,   630,   379,   630,
     381,   540,   548,   402,   401,   518,   518,   518,   518,   618,
     386,   326,     0,     0,   632,   512,     0,   295,     0,   250,
       0,   429,   610,     0,   358,     0,     0,   391,   615,   389,
     367,   368,   368,   550,   572,     0,   368,   560,   368,   368,
       0,     0,     0,   547,   522,   630,   519,   524,   527,   633,
     634,   510,   333,   590,   294,   599,   586,   587,   428,   345,
     321,   356,   393,   630,   371,   630,   373,   363,   630,   380,
     630,   377,   382,   558,   295,     0,   518,   416,   368,   368,
     368,   368,   590,   294,   520,   372,   630,   369,   375,   378,
     368,   370
  };

  const short int
  parser::yypgoto_[] =
  {
    -925,  -925,  -224,  -925,    52,  -367,   423,  -925,   670,    76,
    -925,  -202,  -293,  1802,    21,   -26,  -925,  -355,  1016,   122,
    1128,  -138,    -7,   -77,  -925,  -452,     8,  1277,   -68,  1141,
      12,  -925,   -21,  -925,  -925,    38,  -925,  1397,  1675,  -925,
     439,    55,   538,  -361,    72,    26,  -925,  -415,  -246,   241,
    -925,  -321,    32,  -925,  -925,  -925,  -925,  -925,  -925,  -925,
    -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,  -925,
       0,  -220,  -405,   112,  -580,  -925,  -762,  -740,   637,   310,
     169,  -925,   409,  -925,   440,  -736,  -925,   119,  -925,  -925,
    -925,  -925,  -925,   389,  -925,  -925,   -85,   624,  -925,  -925,
     826,  -925,  -925,  -413,  -925,   140,  -925,  -925,  -925,  -925,
    -925,  -925,  1081,  -925,  -925,  -925,  -925,   856,  -925,  -925,
    -925,  -925,  -925,  -925,   912,  1122,  -925,  -206,  -925,  -925,
      13,    23,  -925,   117,   318,   642,    74,  1598,    94,  -925,
    -925,  -925,   277,  -925,  -925,  -300,  -230,  -460,  -768,  -397,
    -519,  -716,   -33,  -873,   460,   177,  -925,  -925,  -925,   168,
    -673,  -924,   182,   470,  -925,  -562,  -925,   110,  -495,  -925,
    -925,   114,  -393,  -925,  -316,  -925,   995,   -69,    -6,   108,
    -240,   313,  -260,   109,    -3,  -925,   574,  -925,   335,   336,
    -925,   414,  -828,  -925,   308,   590,  -925,  -925
  };

  const short int
  parser::yydefgoto_[] =
  {
      -1,    69,    70,    71,    72,   410,   411,   297,   298,   299,
     458,    74,   561,    75,   234,    76,    77,   548,   126,    79,
      80,   301,    81,    82,    83,   487,    84,   235,   127,   128,
     225,   226,   227,   228,   636,   599,   213,    86,   266,   305,
     566,   600,   778,   448,   449,   275,   276,   268,   441,   450,
     554,   555,    87,   231,   303,   684,   304,   248,   815,   249,
     816,   660,   998,   620,   782,   901,   404,   407,   635,   911,
     289,   414,   652,   804,   805,   254,   713,   714,   831,  1050,
    1013,   943,   872,   970,   873,   838,  1004,  1005,   330,   331,
     726,   520,   850,   348,    89,    90,   346,   545,   546,   754,
     543,   544,   752,   425,   995,   639,   798,   913,   916,    91,
      92,    93,    94,    95,    96,    97,   326,   503,    98,   328,
      99,   100,   327,   329,   323,   322,   325,   496,   694,   698,
     101,   102,   103,   104,   237,   238,   107,   239,   240,   400,
     619,   633,   634,  1034,   792,   851,   728,   522,   716,   717,
     966,   523,   524,   747,   525,   946,   947,   526,   748,   527,
     528,   949,   950,   529,   749,   530,   750,   531,   730,   215,
     310,   451,   271,   110,   625,   602,   387,   403,   245,   445,
     446,   774,   480,   415,   247,   802,   308,   532,   937,   533,
     719,   534,   844,   979,   786,   787,   723,   724
  };

  const short int
  parser::yytable_[] =
  {
      88,    88,   130,   130,   284,   389,   353,   236,   236,   236,
     648,   852,   253,   236,   236,   746,   211,   236,   229,   385,
     388,   422,   603,   654,   443,   665,   320,   665,   230,   241,
     244,   521,   732,   567,   129,   129,   688,   628,   229,   302,
     453,   626,   212,   212,   129,   279,    88,   482,   230,   667,
     315,   484,   236,   111,   413,   720,   398,   644,   314,   801,
     861,   474,   461,   212,   413,   333,   601,   811,   334,   609,
     657,   612,   933,  -104,   278,  -106,    73,    73,   214,  -100,
    1022,   982,   129,   315,   281,   267,   272,   627,   667,   273,
    -101,   651,   930,  1057,   109,   109,   133,   133,   613,   616,
    -100,   601,  -108,   609,   345,   760,   257,  -101,  -104,  -107,
     944,  -103,  -108,  -106,   848,   129,   627,   498,   246,   501,
     504,   286,   504,   236,   211,    88,   505,   270,   270,   691,
    -107,   270,   282,   344,   285,   264,   264,   745,   242,   264,
     109,  -105,  1030,   243,   318,  -290,   388,  -102,   627,  -103,
     212,  -586,   560,  -587,  -105,   340,   341,   469,  -102,  1057,
     287,   291,   307,   311,   347,   854,   631,   419,  1006,   292,
     280,   862,   278,   242,  -586,   627,   813,   318,   243,  -587,
     246,   344,  1022,   242,   432,  -290,   214,  1022,   243,   560,
     560,  1000,  -104,   506,  -106,   236,   242,  -104,  -100,  -106,
     332,   243,   342,  -100,   668,   443,  1071,  1010,   670,  -101,
     -91,   281,   673,   556,  -101,   521,   342,   -92,   -95,   109,
     464,  -108,   -99,   -97,   849,   509,  -108,   393,  -107,   681,
    -103,    88,   498,  -107,   349,  -103,   486,   933,   423,   394,
     -98,   492,   836,   396,   680,   852,   491,   397,   236,   236,
     845,   -72,   473,   784,   269,   269,   665,   665,   269,   -94,
    -105,  1044,   865,   281,   -96,  -105,  -102,   793,   -93,   427,
     428,  -102,   340,   341,   236,  1032,   236,   236,  1006,   667,
     236,   277,   236,   861,   951,   846,    88,    88,   302,   306,
     343,   471,   510,    88,   472,    88,   852,   702,   350,   734,
     736,   866,   702,    88,   343,  1081,   493,   494,   495,   878,
     492,  -100,  1033,   315,   -86,   833,   759,   797,   105,   105,
     131,   131,   131,   551,   354,   109,   478,  -500,   562,   479,
     255,   908,   909,   624,    88,   236,   236,   236,   236,    88,
     236,   236,   993,   381,   302,   129,   444,  -101,   447,   236,
     386,    88,   315,   246,   568,   391,   536,   537,   538,   539,
     558,   960,    73,  -501,   105,   562,   562,   281,   316,   467,
     274,   707,   852,   708,   709,   493,   494,   710,   277,   475,
     109,   109,   280,   236,   129,   701,   535,   109,   270,   109,
     270,   568,   568,  -100,  1026,   492,   264,   109,   264,   264,
    -101,   316,   236,  -108,    88,   883,   246,   318,   384,  -500,
      73,   486,   476,   925,    88,   540,  -501,   891,   236,   421,
     821,   399,    88,   607,   898,   607,   827,   405,   109,   814,
     131,   236,  -492,   109,  1021,   823,   800,   674,   424,  -492,
     608,   607,   409,   105,   826,   109,   318,   492,   569,   601,
    1052,   609,  1015,  1016,   832,  1058,   -91,   607,   608,   236,
     493,   494,   497,   -92,   412,   340,   341,   745,    88,   296,
     607,   229,    88,  -107,   608,   835,   887,   889,  -108,   837,
     486,   230,   894,   896,  -107,   569,   569,   608,   315,   466,
     236,   521,   521,   420,   392,   424,   212,  -492,   109,   767,
     466,   416,   607,   478,   618,   264,   481,  -108,   109,   994,
    -583,   665,   493,   494,  -103,   269,   109,   269,  -584,   608,
     129,  1099,   704,   392,   264,   707,  -103,   708,   709,   607,
     775,   710,  -491,   667,   977,  1002,   653,   653,  -493,  -491,
     264,   -99,   809,   429,    73,  -493,   608,   -98,   296,   105,
     803,   800,   281,   264,   666,   840,   745,   315,   433,  1087,
     242,   560,   109,  -486,   426,   243,   109,   560,   560,   281,
    -486,  -489,   905,   560,   560,  -583,   821,   -94,  -489,  -486,
      54,   281,   318,  -584,  1007,  -105,   439,  -489,   699,   129,
     553,   928,    61,   686,   -94,   553,   281,  -491,  -583,  1046,
     975,   776,    64,  -493,   105,   105,  -584,  -494,  1001,   440,
     828,   105,   264,   105,  -494,   795,  1036,   442,  -496,   236,
      88,   105,   355,   957,   959,  -496,   465,   772,  -486,   -96,
     962,   316,   964,   965,    88,   779,  -489,  -105,   781,   468,
     783,   -94,   106,   106,   132,   132,   132,   794,   -96,   229,
     492,   318,   105,   810,   256,  -338,   771,   105,   470,   230,
     236,   -94,  -338,   745,   777,   649,   -71,   777,   981,   105,
     316,  -338,  -494,   212,   212,   477,   -96,   483,  1075,   485,
     771,   818,   777,  -496,   486,   340,   341,   315,   106,  -102,
     492,   560,   317,   990,    88,   899,   -96,   992,   488,  -495,
     860,   -94,   863,   768,   -94,   919,  -495,  -103,   742,   743,
     462,   773,   492,   547,   109,   493,   494,   499,   296,   129,
    -338,   559,   105,   775,   627,   317,  -294,   459,   109,  -102,
    -105,  -102,   105,  -294,   773,   632,   -96,   562,  -587,   -96,
     105,   630,  -294,   562,   562,   893,   776,   893,   637,   562,
     562,   638,   773,   641,   132,   493,   494,   497,  -590,   650,
     669,   315,    88,   568,  -495,   773,   236,   106,   492,   568,
     568,   676,   661,   671,   296,   568,   568,   493,   494,   500,
     672,   318,  -497,   685,   677,   822,   105,   -86,   109,  -497,
     105,  -294,   -93,   129,   700,  1064,  1066,  1067,  1068,   773,
      88,    40,    41,   236,   -93,   607,   316,   687,   706,   683,
     703,  -590,   705,   945,    88,    88,    88,   727,  -590,   729,
     455,   731,   608,  -586,   917,   456,   457,  -590,   839,   773,
     615,   617,   733,   493,   494,   502,   735,   640,   879,   857,
     -95,   857,   737,   751,   753,   647,  -590,  -497,    88,    88,
     718,   -93,   721,   722,   725,   318,   109,   569,   756,   615,
     617,   381,  -265,   569,   569,   758,  1104,   562,   382,   569,
     569,   -93,    88,   106,    88,   316,  -590,   383,   763,   553,
      65,    66,   761,   381,    67,    68,   762,   -95,   264,   789,
     401,   791,   902,   568,   109,   773,   492,   856,   773,   402,
     785,    88,   800,   824,   812,   679,  -586,   -95,   109,   109,
     109,   -93,   986,  -486,   -93,    88,  1003,  1040,   708,   709,
    -486,   825,   710,  1025,   381,  1027,   384,   653,   106,   106,
    1028,   417,   834,   867,  -489,   106,   842,   106,   105,   788,
     418,  -489,   109,   109,   129,   106,   843,   -95,   384,   869,
     -95,   870,   105,   381,   839,   317,   952,   808,   871,  -266,
     430,   493,   494,   507,   875,   718,   109,   892,   109,   431,
     817,   886,   381,   904,   903,   511,   106,   910,  -486,   463,
     912,   106,   355,    88,  1056,   915,  1059,   569,   418,   384,
      88,   773,    88,   106,   317,   109,   918,   512,    88,  -489,
     920,   926,   927,   931,   829,   316,   989,   936,   935,   109,
     941,   956,   105,   958,   948,   370,    78,    78,   384,   372,
     373,   961,  1063,    78,    78,    78,  1009,   963,   513,    78,
      78,   629,  1096,    78,   841,   516,  -267,   384,   517,   236,
     996,   997,   381,   999,   518,   773,   106,   718,  1011,   489,
    1098,   839,  1100,  1012,   129,  1014,   106,  1101,   490,  1017,
    -498,  1093,    78,   707,   106,   708,   709,  -498,    78,   710,
    1019,  1023,   519,  1110,  1024,   773,  -498,   109,   607,   316,
     105,  -499,  -283,  -295,   109,  -268,   109,  1031,  -499,  -283,
    -295,  1035,   109,   129,  1037,   608,  1045,  -499,  -283,  -295,
     906,   929,  1049,   907,  1061,  1065,  1072,   384,  1080,  1083,
     106,  1094,  1047,   738,   106,  1085,   133,   830,   105,  1018,
    1020,   381,   952,   739,   857,  -498,  -586,   952,   765,   952,
     317,  1088,   105,   105,   105,  1090,  1106,   766,  -587,    78,
     251,    78,   682,   858,   134,   859,  -499,  -283,  -295,   864,
     780,   715,   381,   381,  1079,   133,   942,   742,   743,  1038,
    1095,   264,   744,   874,   847,  1082,   105,   105,  1039,   402,
     900,   755,   934,   549,   390,   438,   938,   324,   939,   940,
     948,  1078,  1054,   773,   508,   948,   384,   948,   983,   855,
     105,  1053,   105,   952,  1051,   952,   973,   853,   952,   317,
     952,   967,   938,   335,   336,   337,   338,   339,   888,   890,
     408,    78,   799,   980,   895,   897,   952,   384,   384,   105,
    1084,  1086,   790,   914,     0,  1089,     0,  1091,  1092,     0,
     987,     0,     0,   105,     0,     0,     0,   922,   923,   924,
     978,     0,     0,     0,     0,   888,   890,    78,   895,   897,
    1008,   948,     0,   948,     0,     0,   948,     0,   948,     0,
     953,     0,   106,     0,    78,    78,     0,  1105,  1107,  1108,
    1109,   954,   955,     0,   948,     0,   106,    85,    85,  1111,
    1029,   695,   696,     0,     0,   697,    38,    39,     0,   252,
      78,     0,    78,    78,     0,   969,    78,   971,    78,     0,
       0,   105,    78,    78,   355,     0,     0,     0,   105,    78,
     105,    78,     0,     0,     0,     0,   105,     0,     0,    78,
       0,     0,  -533,    85,     0,     0,     0,   511,     0,   317,
       0,     0,   976,     0,     0,     0,   106,   370,   991,     0,
     131,   372,   373,  1048,   374,   375,     0,     0,     0,   512,
      78,    78,    78,    78,    78,    78,    78,    78,   976,     0,
       0,     0,     0,     0,     0,    78,     0,    78,  1069,  1070,
      78,     0,   864,     0,     0,   864,     0,   864,     0,   131,
     513,     0,     0,   514,   515,     0,     0,   516,  1097,   707,
     517,   708,   709,     0,     0,   710,   518,     0,     0,    78,
       0,     0,    85,   317,   106,     0,     0,    78,    78,     0,
     265,   265,     0,  1041,   265,  1042,     0,     0,    78,     0,
      78,  1043,     0,     0,   519,     0,   953,   929,  1055,     0,
      78,   953,  -533,  1060,    78,     0,   932,     0,    78,   288,
     290,     0,   106,     0,     0,   265,   265,    78,   319,   321,
       0,     0,     0,     0,     0,     0,   106,   106,   106,     0,
     707,     0,   708,   709,     0,     0,   710,     0,     0,     0,
       0,     0,     0,     0,     0,    78,     0,     0,     0,   864,
     707,     0,   708,   709,    78,   738,   710,     0,    78,     0,
     106,   106,     0,     0,     0,   739,     0,   953,   929,  1060,
       0,     0,  1060,     0,  1060,     0,    78,   712,    85,  -248,
       0,     0,     0,     0,   106,   511,   106,  -248,  -248,  -248,
    1060,     0,  -248,  -248,  -248,     0,  -248,   740,   741,   742,
     743,     0,     0,   511,   744,  -248,  -248,   512,     0,     0,
       0,     0,     0,   106,     0,     0,  -248,  -248,     0,  -248,
    -248,  -248,  -248,  -248,   988,   512,     0,   106,     0,     0,
       0,     0,     0,    85,    85,     0,     0,     0,   513,     0,
      85,   514,    85,  -533,     0,   516,     0,     0,   517,     0,
      85,     0,     0,     0,   518,     0,   513,     0,  -248,   514,
       0,     0,     0,   516,     0,  -248,   517,     0,   108,   108,
     277,  -248,   518,     0,     0,     0,     0,     0,     0,     0,
       0,    85,   519,  -533,     0,     0,    85,     0,  -533,     0,
       0,     0,     0,  -248,  -248,   106,     0,     0,    85,     0,
     519,   563,   106,     0,   106,    78,    78,     0,  -248,     0,
     106,  -248,     0,  -248,   108,   511,  -248,     0,     0,     0,
      78,     0,     0,  -248,     0,   265,   265,   265,   265,   321,
       0,     0,     0,     0,   132,     0,     0,   512,   563,   563,
       0,   265,     0,   265,   265,     0,    78,     0,     0,     0,
       0,    85,   460,     0,     0,     0,     0,   707,     0,   708,
     709,    85,   738,   710,   355,     0,     0,     0,   513,    85,
    -533,   514,   739,   132,     0,   516,     0,     0,   517,     0,
      78,   368,   369,     0,   518,     0,     0,     0,     0,     0,
       0,     0,     0,   108,   309,     0,     0,   370,     0,   371,
       0,   372,   373,     0,   374,   375,   742,   743,   378,     0,
     379,   744,   519,  -533,     0,    85,   265,     0,  -533,    85,
       0,   564,   570,   571,   572,   573,   574,   575,   576,   577,
     578,   579,   580,   581,   582,   583,   584,   585,   586,   587,
     588,   589,   590,   591,   592,   593,   594,   595,    78,    78,
     265,     0,    78,     0,     0,    78,    78,     0,   614,   614,
       0,    78,    78,     0,     0,     0,     0,     0,   511,   265,
       0,     0,     0,     0,     0,   355,     0,     0,     0,   233,
     233,   233,     0,   614,     0,   265,    78,   614,   614,    78,
     512,   265,   368,   369,     0,     0,     0,     0,   265,   108,
      78,    78,    78,     0,     0,     0,   265,   265,   370,   265,
     371,     0,   372,   373,     0,   374,   375,   511,   300,     0,
       0,   513,     0,     0,   514,     0,  -533,     0,   516,     0,
       0,   517,   678,   614,    78,    78,     0,   518,     0,   512,
       0,     0,     0,     0,     0,     0,   265,     0,   265,     0,
       0,     0,     0,     0,   108,   108,     0,   265,    78,     0,
      78,   108,     0,   108,     0,   519,  -533,    85,   511,     0,
     513,   108,     0,   514,     0,     0,     0,   516,     0,    78,
     517,    85,     0,     0,     0,     0,   518,    78,     0,     0,
     512,     0,     0,     0,  -599,   395,     0,   300,     0,     0,
       0,    78,   108,   434,   435,   436,   437,   108,     0,     0,
       0,     0,     0,     0,   519,   281,     0,   265,     0,   108,
    -599,   513,   108,     0,   514,     0,     0,     0,   516,     0,
       0,   517,   511,     0,     0,     0,     0,   518,     0,     0,
       0,    85,     0,     0,     0,  -374,     0,     0,     0,     0,
     707,     0,   708,   709,   512,     0,   710,     0,     0,   108,
     108,     0,     0,     0,     0,   519,  -374,   406,     0,    78,
       0,  -374,   108,     0,     0,     0,    78,     0,    78,     0,
       0,     0,   108,     0,    78,   513,     0,     0,   711,     0,
     108,     0,   516,     0,   552,   517,     0,   712,     0,   565,
       0,   518,     0,     0,     0,   265,     0,     0,     0,    85,
     563,     0,     0,     0,     0,     0,   563,   563,     0,     0,
     233,   233,   563,   563,     0,    78,     0,     0,   265,   519,
     265,     0,     0,   355,     0,     0,   108,     0,   265,   265,
     108,     0,     0,     0,     0,     0,     0,    85,     0,     0,
     368,   369,   452,     0,   454,     0,     0,     0,     0,     0,
       0,    85,    85,    85,     0,     0,   370,   300,     0,   565,
     372,   373,     0,   374,   375,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   662,   664,     0,   309,     0,     0,
       0,     0,     0,     0,     0,    85,    85,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   233,   233,   233,
     233,     0,   541,   542,   265,     0,     0,     0,     0,    85,
       0,    85,     0,   300,   664,   265,   309,     0,     0,     0,
     614,   880,     0,   265,     0,     0,   614,   614,     0,     0,
     563,     0,   614,   614,     0,     0,     0,     0,    85,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,   985,
     764,     0,    85,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   614,   614,     0,   614,   614,     0,     0,
     265,     0,     0,   355,   356,   357,   358,   359,   108,   360,
     361,   362,   363,   364,   365,   757,   366,   367,     0,     0,
     368,   369,   108,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   370,     0,   371,     0,
     372,   373,     0,   374,   375,   376,   377,   378,     0,   379,
      85,   675,     0,     0,   265,     0,     0,    85,     0,    85,
       0,     0,     0,   265,     0,    85,     0,     0,     0,   380,
       0,  -240,     0,   974,     0,     0,     0,     0,     0,     0,
     614,     0,   108,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   796,     0,     0,   614,     0,     0,     0,
       0,     0,     0,   265,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   819,     0,   820,     0,
       0,     0,     0,     0,     0,     0,   664,   309,   355,   356,
     357,   358,   359,     0,   360,   361,   362,     0,   364,   365,
     108,   108,     0,     0,     0,   368,   369,   108,   108,     0,
       0,     0,     0,   108,   108,     0,     0,     0,     0,     0,
       0,   370,     0,   371,     0,   372,   373,     0,   374,   375,
     376,   377,   378,     0,   379,     0,     0,     0,   108,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   108,   108,   108,     0,     0,     0,     0,     0,
       0,   233,   868,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   877,     0,     0,   265,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   108,   108,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   233,     0,     0,     0,     0,     0,     0,     0,
     108,   764,   108,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   921,     0,
       0,   108,     0,     0,   355,   356,   357,   358,   359,   108,
     360,   361,   362,   363,   364,   365,     0,   366,   367,     0,
       0,   368,   369,   108,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   370,     0,   371,
       0,   372,   373,     0,   374,   375,   376,   377,   378,     0,
     379,     0,   968,     0,     0,     0,     0,     0,     0,     0,
       0,   972,     0,     0,     0,     0,     0,     0,     0,     0,
     380,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   108,     0,     0,     0,     0,     0,     0,   108,     0,
     108,     0,     0,     0,     0,     0,   108,     0,     0,     0,
       0,   309,     0,    -4,     1,   233,     2,     3,     4,     5,
       6,     0,     0,     0,     7,     8,     0,     0,     0,     9,
       0,    10,    11,    12,    13,    14,    15,    16,     0,     0,
       0,     0,     0,    17,    18,    19,    20,    21,    22,    23,
       0,     0,    24,     0,     0,     0,     0,     0,    25,    26,
      27,    28,    29,    30,    31,    32,    33,    34,    35,    36,
       0,    37,    38,    39,     0,    40,    41,    42,    43,    44,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      45,     0,     0,    46,     0,     0,    47,    48,     0,     0,
      49,     0,    50,     0,     0,     0,    51,     0,     0,     0,
       0,     0,     0,     0,     0,    52,     0,     0,     0,     0,
      53,    54,    55,    56,     0,    57,    58,    59,    60,     0,
       0,     0,     0,    61,    62,    -4,     0,     0,     0,     0,
      -4,    63,     0,    64,    65,    66,     0,     0,    67,    68,
     293,     0,     2,     3,     4,     5,     6,   -12,   -12,   -12,
       7,     8,     0,     0,   -12,     9,     0,    10,    11,    12,
      13,    14,    15,    16,     0,     0,     0,     0,     0,    17,
      18,    19,    20,    21,    22,    23,     0,     0,    24,     0,
       0,     0,     0,     0,    25,    26,   294,    28,    29,    30,
      31,    32,    33,    34,    35,    36,     0,    37,    38,    39,
       0,    40,    41,    42,    43,    44,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    45,     0,     0,    46,
       0,     0,    47,    48,     0,     0,    49,     0,    50,     0,
       0,     0,    51,     0,     0,     0,     0,     0,     0,     0,
       0,    52,     0,     0,     0,     0,    53,    54,    55,    56,
       0,    57,    58,    59,    60,     0,     0,     0,     0,    61,
      62,   -12,     0,     0,     0,     0,   -12,    63,     0,    64,
      65,    66,     0,     0,    67,    68,   293,     0,     2,     3,
       4,     5,     6,     0,     0,   -12,     7,     8,     0,   -12,
     -12,     9,     0,    10,    11,    12,    13,    14,    15,    16,
       0,     0,     0,     0,     0,    17,    18,    19,    20,    21,
      22,    23,     0,     0,    24,     0,     0,     0,     0,     0,
      25,    26,   294,    28,    29,    30,    31,    32,    33,    34,
      35,    36,     0,    37,    38,    39,     0,    40,    41,    42,
      43,    44,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    45,     0,     0,    46,     0,     0,    47,    48,
       0,     0,    49,     0,    50,     0,     0,     0,    51,     0,
       0,     0,     0,     0,     0,     0,     0,    52,     0,     0,
       0,     0,    53,    54,    55,    56,     0,    57,    58,    59,
      60,     0,     0,     0,     0,    61,    62,   -12,     0,     0,
       0,     0,   -12,    63,     0,    64,    65,    66,     0,     0,
      67,    68,   293,     0,     2,     3,     4,     5,     6,     0,
       0,   -12,     7,     8,     0,     0,   -12,     9,   -12,    10,
      11,    12,    13,    14,    15,    16,     0,     0,     0,     0,
       0,    17,    18,    19,    20,    21,    22,    23,     0,     0,
      24,     0,     0,     0,     0,     0,    25,    26,   294,    28,
      29,    30,    31,    32,    33,    34,    35,    36,     0,    37,
      38,    39,     0,    40,    41,    42,    43,    44,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    45,     0,
       0,    46,     0,     0,    47,    48,     0,     0,    49,     0,
      50,     0,     0,     0,    51,     0,     0,     0,     0,     0,
       0,     0,     0,    52,     0,     0,     0,     0,    53,    54,
      55,    56,     0,    57,    58,    59,    60,     0,     0,     0,
       0,    61,    62,   -12,     0,     0,     0,     0,   -12,    63,
       0,    64,    65,    66,     0,     0,    67,    68,   293,     0,
       2,     3,     4,     5,     6,     0,     0,   -12,     7,     8,
       0,     0,   -12,     9,     0,    10,    11,    12,    13,    14,
      15,    16,     0,     0,     0,     0,     0,    17,    18,    19,
      20,    21,    22,    23,     0,     0,    24,     0,     0,     0,
       0,     0,    25,    26,   294,    28,    29,    30,    31,    32,
      33,    34,    35,    36,     0,    37,    38,    39,     0,    40,
      41,    42,    43,    44,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    45,     0,     0,    46,     0,     0,
      47,    48,     0,     0,    49,     0,    50,     0,     0,     0,
      51,     0,     0,     0,     0,     0,     0,     0,     0,    52,
       0,     0,     0,     0,    53,    54,    55,    56,     0,    57,
      58,    59,    60,     0,     0,     0,     0,    61,    62,   -12,
       0,     0,     0,     0,   -12,    63,     0,    64,    65,    66,
       0,     0,    67,    68,   293,     0,     2,     3,     4,     5,
       6,     0,   -12,   -12,     7,     8,     0,     0,     0,     9,
       0,    10,    11,    12,    13,    14,    15,    16,     0,     0,
       0,     0,     0,    17,    18,    19,    20,    21,    22,    23,
       0,     0,    24,     0,     0,     0,     0,     0,    25,    26,
     294,    28,    29,    30,    31,    32,    33,    34,    35,    36,
       0,    37,    38,    39,     0,    40,    41,    42,    43,    44,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      45,     0,     0,    46,     0,     0,    47,    48,     0,     0,
      49,     0,    50,     0,     0,     0,    51,     0,     0,     0,
       0,     0,     0,     0,     0,    52,     0,     0,     0,     0,
      53,    54,    55,    56,     0,    57,    58,    59,    60,     0,
       0,     0,     0,    61,    62,   -12,     0,     0,     0,     0,
     -12,    63,     0,    64,    65,    66,     0,     0,    67,    68,
     293,     0,     2,     3,     4,     5,     6,     0,     0,     0,
       7,     8,     0,     0,     0,     9,     0,    10,    11,    12,
      13,    14,    15,    16,     0,     0,     0,     0,     0,    17,
      18,    19,    20,    21,    22,    23,     0,     0,    24,     0,
       0,     0,     0,     0,    25,    26,   294,    28,    29,    30,
      31,    32,    33,    34,    35,    36,     0,    37,    38,    39,
       0,    40,    41,    42,    43,    44,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    45,     0,     0,   295,
       0,   -12,    47,    48,     0,     0,    49,     0,    50,     0,
       0,     0,    51,     0,     0,     0,     0,     0,     0,     0,
       0,    52,     0,     0,     0,     0,    53,    54,    55,    56,
       0,    57,    58,    59,    60,     0,     0,     0,     0,    61,
      62,   -12,     0,     0,     0,     0,   -12,    63,     0,    64,
      65,    66,     0,     0,    67,    68,   293,     0,     2,     3,
       4,     5,     6,     0,     0,     0,     7,     8,     0,     0,
       0,     9,     0,    10,    11,    12,    13,    14,    15,    16,
       0,     0,     0,     0,     0,    17,    18,    19,    20,    21,
      22,    23,     0,     0,    24,     0,     0,     0,     0,     0,
      25,    26,   294,    28,    29,    30,    31,    32,    33,    34,
      35,    36,     0,    37,    38,    39,     0,    40,    41,    42,
      43,    44,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    45,     0,     0,    46,     0,   -12,    47,    48,
       0,     0,    49,     0,    50,     0,     0,     0,    51,     0,
       0,     0,     0,     0,     0,     0,     0,    52,     0,     0,
       0,     0,    53,    54,    55,    56,     0,    57,    58,    59,
      60,     0,     0,     0,     0,    61,    62,   -12,     0,     0,
       0,     0,   -12,    63,     0,    64,    65,    66,     0,     0,
      67,    68,     1,     0,     2,     3,     4,     5,     6,     0,
       0,     0,     7,     8,     0,     0,     0,     9,     0,    10,
      11,    12,    13,    14,    15,    16,     0,     0,     0,     0,
       0,    17,    18,    19,    20,    21,    22,    23,     0,     0,
      24,     0,     0,     0,     0,     0,    25,    26,    27,    28,
      29,    30,    31,    32,    33,    34,    35,    36,     0,    37,
      38,    39,     0,    40,    41,    42,    43,    44,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    45,     0,
       0,    46,     0,     0,    47,    48,     0,     0,    49,     0,
      50,     0,     0,     0,    51,     0,     0,     0,     0,     0,
       0,     0,     0,    52,     0,     0,    -4,     0,    53,    54,
      55,    56,     0,    57,    58,    59,    60,     0,     0,     0,
       0,    61,    62,    -4,     0,     0,     0,     0,    -4,    63,
       0,    64,    65,    66,     0,     0,    67,    68,   293,     0,
       2,     3,     4,     5,     6,     0,     0,     0,     7,     8,
       0,     0,     0,     9,     0,    10,    11,    12,    13,    14,
      15,    16,     0,     0,     0,     0,     0,    17,    18,    19,
      20,    21,    22,    23,     0,     0,    24,     0,     0,     0,
       0,     0,    25,    26,   294,    28,    29,    30,    31,    32,
      33,    34,    35,    36,     0,    37,    38,    39,     0,    40,
      41,    42,    43,    44,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    45,     0,     0,    46,     0,     0,
      47,    48,     0,     0,    49,     0,    50,     0,     0,     0,
      51,     0,     0,     0,     0,     0,     0,     0,     0,    52,
       0,     0,   -12,     0,    53,    54,    55,    56,     0,    57,
      58,    59,    60,     0,     0,     0,     0,    61,    62,   -12,
       0,     0,     0,     0,   -12,    63,     0,    64,    65,    66,
       0,     0,    67,    68,   293,     0,     2,     3,     4,     5,
       6,     0,     0,     0,     7,     8,     0,     0,     0,     9,
       0,    10,    11,    12,    13,    14,    15,    16,     0,     0,
       0,     0,     0,    17,    18,    19,    20,    21,    22,    23,
       0,     0,    24,     0,     0,     0,     0,     0,    25,    26,
     294,    28,    29,    30,    31,    32,    33,    34,    35,    36,
       0,    37,    38,    39,     0,    40,    41,    42,    43,    44,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      45,     0,     0,    46,     0,     0,    47,    48,     0,     0,
      49,     0,    50,     0,     0,     0,    51,     0,     0,     0,
       0,     0,     0,     0,     0,    52,     0,     0,     0,     0,
      53,    54,    55,    56,     0,    57,    58,    59,    60,     0,
       0,     0,   -12,    61,    62,   -12,     0,     0,     0,     0,
     -12,    63,     0,    64,    65,    66,     0,     0,    67,    68,
     293,     0,     2,     3,     4,     5,     6,     0,     0,   -12,
       7,     8,     0,     0,     0,     9,     0,    10,    11,    12,
      13,    14,    15,    16,     0,     0,     0,     0,     0,    17,
      18,    19,    20,    21,    22,    23,     0,     0,    24,     0,
       0,     0,     0,     0,    25,    26,   294,    28,    29,    30,
      31,    32,    33,    34,    35,    36,     0,    37,    38,    39,
       0,    40,    41,    42,    43,    44,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    45,     0,     0,    46,
       0,     0,    47,    48,     0,     0,    49,     0,    50,     0,
       0,     0,    51,     0,     0,     0,     0,     0,     0,     0,
       0,    52,     0,     0,     0,     0,    53,    54,    55,    56,
       0,    57,    58,    59,    60,     0,     0,     0,     0,    61,
      62,   -12,     0,     0,     0,     0,   -12,    63,  -590,    64,
      65,    66,     0,     0,    67,    68,  -590,  -590,  -590,     0,
       0,  -590,  -590,  -590,     0,  -590,     0,     0,     0,     0,
       0,     0,     0,     0,  -590,  -590,  -590,  -590,     0,     0,
       0,     0,     0,     0,     0,  -590,  -590,     0,  -590,  -590,
    -590,  -590,  -590,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,  -590,  -590,  -590,  -590,  -590,
    -104,  -590,  -590,  -590,  -590,  -590,  -590,  -590,  -590,  -590,
       0,     0,  -590,  -590,  -590,     0,   806,  -590,     0,     0,
    -590,     0,     0,  -590,  -590,     0,  -590,     0,  -590,     0,
    -590,     0,  -590,  -590,     0,  -590,  -590,  -590,  -590,  -590,
       0,  -590,  -590,  -590,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,  -590,     0,     0,
    -590,  -590,  -590,  -590,     0,  -590,  -486,  -590,     0,     0,
       0,     0,  -590,     0,  -486,  -486,  -486,     0,     0,  -486,
    -486,  -486,     0,  -486,     0,     0,     0,     0,     0,     0,
       0,  -486,     0,  -486,  -486,  -486,     0,     0,     0,     0,
       0,     0,     0,  -486,  -486,     0,  -486,  -486,  -486,  -486,
    -486,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,  -486,  -486,  -486,  -486,  -486,  -486,  -486,
    -486,  -486,  -486,  -486,  -486,  -486,  -486,  -486,     0,     0,
    -486,  -486,  -486,     0,  -486,  -486,     0,     0,  -486,     0,
       0,  -486,  -486,     0,  -486,     0,  -486,     0,  -486,     0,
    -486,  -486,     0,  -486,  -486,  -486,  -486,  -486,     0,  -486,
       0,  -486,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,  -486,     0,     0,  -486,  -486,
    -486,  -486,     0,  -486,  -489,  -486,     0,     0,     0,     0,
    -486,     0,  -489,  -489,  -489,     0,     0,  -489,  -489,  -489,
       0,  -489,     0,     0,     0,     0,     0,     0,     0,  -489,
       0,  -489,  -489,  -489,     0,     0,     0,     0,     0,     0,
       0,  -489,  -489,     0,  -489,  -489,  -489,  -489,  -489,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,  -489,  -489,  -489,  -489,  -489,  -489,  -489,  -489,  -489,
    -489,  -489,  -489,  -489,  -489,  -489,     0,     0,  -489,  -489,
    -489,     0,  -489,  -489,     0,     0,  -489,     0,     0,  -489,
    -489,     0,  -489,     0,  -489,     0,  -489,     0,  -489,  -489,
       0,  -489,  -489,  -489,  -489,  -489,     0,  -489,     0,  -489,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,  -489,     0,     0,  -489,  -489,  -489,  -489,
       0,  -489,  -591,  -489,     0,     0,     0,     0,  -489,     0,
    -591,  -591,  -591,     0,     0,  -591,  -591,  -591,     0,  -591,
       0,     0,     0,     0,     0,     0,     0,     0,  -591,  -591,
    -591,  -591,     0,     0,     0,     0,     0,     0,     0,  -591,
    -591,     0,  -591,  -591,  -591,  -591,  -591,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,  -591,
    -591,  -591,  -591,  -591,     0,  -591,  -591,  -591,  -591,  -591,
    -591,  -591,  -591,  -591,     0,     0,  -591,  -591,  -591,     0,
       0,  -591,     0,     0,  -591,     0,     0,  -591,  -591,     0,
    -591,     0,  -591,     0,  -591,     0,  -591,  -591,     0,  -591,
    -591,  -591,  -591,  -591,     0,  -591,  -591,  -591,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,  -591,     0,     0,  -591,  -591,  -591,  -591,     0,  -591,
    -592,  -591,     0,     0,     0,     0,  -591,     0,  -592,  -592,
    -592,     0,     0,  -592,  -592,  -592,     0,  -592,     0,     0,
       0,     0,     0,     0,     0,     0,  -592,  -592,  -592,  -592,
       0,     0,     0,     0,     0,     0,     0,  -592,  -592,     0,
    -592,  -592,  -592,  -592,  -592,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,  -592,  -592,  -592,
    -592,  -592,     0,  -592,  -592,  -592,  -592,  -592,  -592,  -592,
    -592,  -592,     0,     0,  -592,  -592,  -592,     0,     0,  -592,
       0,     0,  -592,     0,     0,  -592,  -592,     0,  -592,     0,
    -592,     0,  -592,     0,  -592,  -592,     0,  -592,  -592,  -592,
    -592,  -592,     0,  -592,  -592,  -592,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,  -592,
       0,     0,  -592,  -592,  -592,  -592,     0,  -592,  -294,  -592,
       0,     0,     0,     0,  -592,     0,  -294,  -294,  -294,     0,
       0,  -294,  -294,  -294,     0,  -294,     0,     0,     0,     0,
       0,     0,     0,     0,     0,  -294,  -294,  -294,     0,     0,
       0,     0,     0,     0,     0,  -294,  -294,     0,  -294,  -294,
    -294,  -294,  -294,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,  -294,  -294,  -294,  -294,  -294,
    -106,  -294,  -294,  -294,  -294,  -294,  -294,  -294,  -294,  -294,
       0,     0,  -294,  -294,  -294,     0,   807,  -294,     0,     0,
    -294,     0,     0,  -294,  -294,     0,  -294,     0,  -294,     0,
    -294,     0,  -294,  -294,     0,  -294,  -294,  -294,  -294,  -294,
       0,  -294,     0,  -294,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,  -294,     0,     0,
    -294,  -294,  -294,  -294,     0,  -294,  -415,  -294,     0,     0,
       0,     0,  -294,     0,  -415,  -415,  -415,     0,     0,  -415,
    -415,  -415,     0,  -415,     0,     0,     0,     0,     0,     0,
       0,     0,  -415,  -415,  -415,     0,     0,     0,     0,     0,
       0,     0,     0,  -415,  -415,     0,  -415,  -415,  -415,  -415,
    -415,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,  -415,  -415,  -415,  -415,  -415,     0,  -415,
    -415,  -415,  -415,  -415,  -415,  -415,  -415,  -415,     0,     0,
    -415,  -415,  -415,     0,     0,  -415,     0,   277,  -415,     0,
       0,  -415,  -415,     0,  -415,     0,  -415,     0,  -415,     0,
    -415,  -415,     0,  -415,  -415,  -415,  -415,  -415,     0,  -415,
    -415,  -415,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,  -415,     0,     0,  -415,  -415,
    -415,  -415,     0,  -415,  -248,     0,     0,     0,     0,     0,
    -415,     0,  -248,  -248,  -248,     0,     0,  -248,  -248,  -248,
       0,  -248,     0,     0,     0,     0,     0,     0,     0,     0,
    -248,  -248,  -248,     0,     0,     0,     0,     0,     0,     0,
       0,  -248,  -248,     0,  -248,  -248,  -248,  -248,  -248,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,  -248,  -248,  -248,  -248,  -248,     0,  -248,  -248,  -248,
    -248,  -248,  -248,  -248,  -248,  -248,     0,     0,  -248,  -248,
    -248,     0,     0,  -248,     0,   277,  -248,     0,     0,  -248,
    -248,     0,  -248,     0,  -248,     0,  -248,     0,  -248,  -248,
       0,  -248,  -248,  -248,  -248,  -248,     0,  -248,  -248,  -248,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,  -248,     0,     0,  -248,  -248,  -248,  -248,
       0,  -248,  -284,     0,     0,     0,     0,     0,  -248,     0,
    -284,  -284,  -284,     0,     0,  -284,  -284,  -284,     0,  -284,
       0,     0,     0,     0,     0,     0,     0,     0,     0,  -284,
    -284,  -284,     0,     0,     0,     0,     0,     0,     0,  -284,
    -284,     0,  -284,  -284,  -284,  -284,  -284,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,  -284,
    -284,  -284,  -284,  -284,     0,  -284,  -284,  -284,  -284,  -284,
    -284,  -284,  -284,  -284,     0,     0,  -284,  -284,  -284,     0,
       0,  -284,     0,     0,  -284,     0,     0,  -284,  -284,     0,
    -284,     0,  -284,     0,  -284,     0,  -284,  -284,     0,  -284,
    -284,  -284,  -284,  -284,     0,  -284,     0,  -284,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,  -284,     0,     0,  -284,  -284,  -284,  -284,     0,  -284,
    -301,  -284,     0,     0,     0,     0,  -284,     0,  -301,  -301,
    -301,     0,     0,  -301,  -301,  -301,     0,  -301,     0,     0,
       0,     0,     0,     0,     0,     0,     0,  -301,  -301,     0,
       0,     0,     0,     0,     0,     0,     0,  -301,  -301,     0,
    -301,  -301,  -301,  -301,  -301,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,  -301,  -301,  -301,
    -301,  -301,     0,  -301,  -301,  -301,  -301,  -301,  -301,  -301,
    -301,  -301,     0,     0,  -301,  -301,  -301,     0,     0,  -301,
       0,   274,  -301,     0,     0,  -301,  -301,     0,  -301,     0,
    -301,     0,  -301,     0,  -301,  -301,     0,  -301,  -301,  -301,
    -301,  -301,     0,  -301,     0,  -301,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,  -301,
       0,  -590,  -301,  -301,  -301,  -301,     0,  -301,     0,  -590,
    -590,  -590,     0,     0,  -301,  -590,  -590,     0,  -590,   355,
    -600,  -600,  -600,  -600,     0,   360,   361,  -590,     0,  -600,
    -600,     0,     0,     0,     0,     0,   368,   369,  -590,  -590,
       0,  -590,  -590,  -590,  -590,  -590,     0,     0,     0,     0,
       0,     0,   370,     0,   371,     0,   372,   373,     0,   374,
     375,   376,   377,   378,     0,   379,     0,     0,  -590,  -590,
    -590,  -590,  -590,  -104,  -590,  -590,  -590,  -590,  -590,  -590,
    -590,  -590,  -590,     0,     0,  -590,  -590,  -590,     0,   769,
       0,     0,     0,  -590,     0,     0,  -590,     0,     0,     0,
       0,  -590,     0,  -590,     0,  -590,  -590,     0,  -590,  -590,
    -590,  -590,  -590,     0,  -590,  -590,  -590,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
    -590,     0,  -590,  -590,  -590,  -590,   -95,     0,  -590,     0,
    -590,  -590,  -590,     0,     0,  -590,  -590,  -590,     0,  -590,
     355,   356,   357,   358,   359,     0,   360,   361,  -590,     0,
     364,   365,     0,     0,     0,     0,     0,   368,   369,  -590,
    -590,     0,  -590,  -590,  -590,  -590,  -590,     0,     0,     0,
       0,     0,     0,   370,     0,   371,     0,   372,   373,     0,
     374,   375,   376,   377,   378,     0,   379,     0,     0,  -590,
    -590,  -590,  -590,  -590,  -104,  -590,  -590,  -590,  -590,  -590,
    -590,  -590,  -590,  -590,     0,     0,  -590,  -590,  -590,     0,
     769,     0,     0,     0,  -590,     0,     0,  -590,     0,     0,
       0,     0,  -590,     0,  -590,     0,  -590,  -590,     0,  -590,
    -590,  -590,  -590,  -590,     0,  -590,  -590,  -590,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,  -590,     0,  -294,  -590,  -590,  -590,  -590,     0,  -590,
       0,  -294,  -294,  -294,     0,     0,  -590,  -294,  -294,     0,
    -294,   355,   356,   357,   358,   359,     0,   360,   361,   362,
     363,   364,   365,     0,  -600,  -600,     0,     0,   368,   369,
    -294,  -294,     0,  -294,  -294,  -294,  -294,  -294,     0,     0,
       0,     0,     0,     0,   370,     0,   371,     0,   372,   373,
       0,   374,   375,   376,   377,   378,     0,   379,     0,     0,
    -294,  -294,  -294,  -294,  -294,  -106,  -294,  -294,  -294,  -294,
    -294,  -294,  -294,  -294,  -294,     0,     0,  -294,  -294,  -294,
       0,   770,     0,     0,     0,  -294,     0,     0,  -294,     0,
       0,     0,     0,  -294,     0,  -294,     0,  -294,  -294,     0,
    -294,  -294,  -294,  -294,  -294,     0,  -294,     0,  -294,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,  -294,     0,  -294,  -294,  -294,  -294,   -97,     0,
    -294,     0,  -294,  -294,  -294,     0,     0,  -294,  -294,  -294,
       0,  -294,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,  -294,  -294,     0,  -294,  -294,  -294,  -294,  -294,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,  -294,  -294,  -294,  -294,  -294,  -106,  -294,  -294,  -294,
    -294,  -294,  -294,  -294,  -294,  -294,     0,     0,  -294,  -294,
    -294,     0,   770,     0,     0,     0,  -294,     0,     0,  -294,
       0,     0,     0,     0,  -294,     0,  -294,     0,  -294,  -294,
       0,  -294,  -294,  -294,  -294,  -294,     0,  -294,     0,  -294,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,  -294,     0,     0,  -294,  -294,  -294,  -294,
       0,  -294,     2,     3,     4,     5,     6,     0,  -294,     0,
       7,     8,     0,     0,     0,     9,     0,    10,    11,    12,
      13,    14,    15,    16,     0,     0,     0,     0,     0,    17,
      18,    19,    20,    21,    22,    23,     0,     0,    24,     0,
       0,     0,     0,     0,    25,    26,    27,    28,    29,    30,
      31,    32,    33,    34,    35,    36,     0,    37,    38,    39,
       0,    40,    41,    42,    43,    44,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    45,     0,     0,    46,
       0,     0,    47,    48,     0,     0,    49,     0,    50,     0,
       0,     0,    51,     0,     0,     0,     0,     0,     0,     0,
       0,    52,     0,     0,     0,     0,    53,    54,    55,    56,
       0,    57,    58,    59,    60,     0,     0,     0,     0,    61,
      62,     0,     0,     0,     0,     0,   426,    63,     0,    64,
      65,    66,     0,     0,    67,    68,     2,     3,     4,     5,
       6,     0,     0,     0,     7,     8,     0,     0,     0,     9,
       0,    10,    11,    12,    13,    14,    15,    16,     0,     0,
       0,     0,     0,    17,    18,    19,    20,    21,    22,    23,
       0,     0,    24,     0,     0,     0,     0,     0,    25,    26,
     294,    28,    29,    30,    31,    32,    33,    34,    35,    36,
       0,    37,    38,    39,     0,    40,    41,    42,    43,    44,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
      45,     0,     0,    46,     0,     0,    47,    48,     0,     0,
      49,     0,    50,     0,     0,     0,    51,     0,     0,     0,
       0,     0,     0,     0,     0,    52,     0,     0,     0,     0,
      53,    54,    55,    56,     0,    57,    58,    59,    60,     0,
       0,     0,     0,    61,    62,     0,     0,     0,     0,     0,
     426,    63,     0,    64,    65,    66,     0,     0,    67,    68,
       2,     3,     4,     5,     6,     0,     0,     0,     7,     8,
       0,     0,     0,     9,     0,    10,    11,    12,    13,    14,
      15,    16,     0,     0,     0,     0,     0,    17,    18,    19,
      20,    21,    22,    23,     0,     0,    24,     0,     0,     0,
       0,     0,    25,    26,    27,    28,    29,    30,    31,    32,
      33,    34,    35,    36,     0,    37,    38,    39,     0,    40,
      41,    42,    43,    44,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    45,     0,     0,    46,     0,     0,
      47,    48,     0,     0,    49,     0,    50,     0,     0,     0,
      51,     0,     0,     0,     0,     0,     0,     0,     0,    52,
       0,     0,     0,     0,    53,    54,    55,    56,     0,    57,
      58,    59,    60,     0,     0,     0,     0,    61,    62,     0,
       0,     0,     2,     3,   112,    63,     6,    64,    65,    66,
       7,     8,    67,    68,     0,     9,     0,    10,    11,    12,
      13,    14,    15,    16,     0,     0,     0,     0,     0,    17,
      18,    19,    20,    21,    22,    23,     0,     0,   118,     0,
       0,     0,     0,     0,     0,    26,     0,     0,    29,    30,
      31,    32,    33,    34,    35,    36,   258,    37,    38,    39,
       0,    40,    41,    42,    43,    44,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   232,     0,     0,   125,
       0,     0,    47,    48,     0,     0,    49,     0,   259,     0,
     260,     0,    51,     0,     0,   261,     0,     0,     0,     0,
       0,   262,     0,     0,     0,     0,    53,   263,    55,    56,
       0,    57,    58,    59,    60,     0,     0,     0,     0,    61,
      62,   281,     0,     0,     2,     3,   112,    63,     6,    64,
      65,    66,     7,     8,    67,    68,     0,     9,     0,    10,
      11,    12,    13,    14,    15,    16,     0,     0,     0,     0,
       0,    17,    18,    19,    20,    21,    22,    23,     0,     0,
     118,     0,     0,     0,     0,     0,     0,    26,     0,     0,
      29,    30,    31,    32,    33,    34,    35,    36,   258,    37,
      38,    39,     0,    40,    41,    42,    43,    44,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   232,     0,
       0,   125,     0,     0,    47,    48,     0,     0,    49,     0,
     259,     0,   260,     0,    51,     0,     0,   261,     0,     0,
       0,     0,     0,   262,     0,     0,     0,     0,    53,   263,
      55,    56,     0,    57,    58,    59,    60,     0,     0,     0,
       0,    61,    62,     0,     0,     0,     0,     0,     0,    63,
       0,    64,    65,    66,     0,     0,    67,    68,     2,     3,
       4,     5,     6,     0,     0,     0,     7,     8,     0,     0,
       0,     9,     0,    10,    11,    12,    13,    14,    15,    16,
       0,     0,     0,     0,     0,    17,    18,    19,    20,    21,
      22,    23,     0,     0,    24,     0,     0,     0,     0,     0,
      25,    26,     0,    28,    29,    30,    31,    32,    33,    34,
      35,    36,     0,    37,    38,    39,     0,    40,    41,    42,
      43,    44,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    45,     0,     0,    46,     0,     0,    47,    48,
       0,     0,    49,     0,    50,     0,     0,     0,    51,     0,
       0,     0,     0,     0,     0,     0,     0,    52,     0,     0,
       0,     0,    53,    54,    55,    56,     0,    57,    58,    59,
      60,     0,     0,     0,     0,    61,    62,     0,     0,     0,
       2,     3,   112,    63,     6,    64,    65,    66,     7,     8,
      67,    68,     0,     9,     0,    10,    11,    12,   113,   114,
      15,    16,     0,     0,     0,     0,     0,   115,   116,   117,
      20,    21,    22,    23,     0,     0,   118,     0,     0,     0,
       0,     0,     0,    26,     0,     0,    29,    30,    31,    32,
      33,    34,    35,    36,   258,    37,    38,    39,     0,    40,
      41,    42,    43,    44,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   232,     0,     0,   125,     0,     0,
      47,    48,     0,     0,    49,     0,   663,     0,   260,     0,
      51,     0,     0,   261,     0,     0,     0,     0,     0,   262,
       0,     0,     0,     0,    53,   263,    55,    56,     0,    57,
      58,    59,    60,     0,     0,     0,     0,    61,    62,     0,
       0,     0,     2,     3,   112,    63,     6,    64,    65,    66,
       7,     8,    67,    68,     0,     9,     0,    10,    11,    12,
     113,   114,    15,    16,     0,     0,     0,     0,     0,   115,
     116,   117,    20,    21,    22,    23,     0,     0,   118,     0,
       0,     0,     0,     0,     0,    26,     0,     0,    29,    30,
      31,    32,    33,    34,    35,    36,   258,    37,    38,    39,
       0,    40,    41,    42,    43,    44,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   232,     0,     0,   125,
       0,     0,    47,    48,     0,     0,    49,     0,   259,     0,
       0,     0,    51,     0,     0,   261,     0,     0,     0,     0,
       0,   262,     0,     0,     0,     0,    53,   263,    55,    56,
       0,    57,    58,    59,    60,     0,     0,     0,     0,    61,
      62,     0,     0,     0,     2,     3,   112,    63,     6,    64,
      65,    66,     7,     8,    67,    68,     0,     9,     0,    10,
      11,    12,   113,   114,    15,    16,     0,     0,     0,     0,
       0,   115,   116,   117,    20,    21,    22,    23,     0,     0,
     118,     0,     0,     0,     0,     0,     0,    26,     0,     0,
      29,    30,    31,    32,    33,    34,    35,    36,   258,    37,
      38,    39,     0,    40,    41,    42,    43,    44,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   232,     0,
       0,   125,     0,     0,    47,    48,     0,     0,    49,     0,
       0,     0,   260,     0,    51,     0,     0,   261,     0,     0,
       0,     0,     0,   262,     0,     0,     0,     0,    53,   263,
      55,    56,     0,    57,    58,    59,    60,     0,     0,     0,
       0,    61,    62,     0,     0,     0,     2,     3,   112,    63,
       6,    64,    65,    66,     7,     8,    67,    68,     0,     9,
       0,    10,    11,    12,   113,   114,    15,    16,     0,     0,
       0,     0,     0,   115,   116,   117,    20,    21,    22,    23,
       0,     0,   118,     0,     0,     0,     0,     0,     0,    26,
       0,     0,    29,    30,    31,    32,    33,    34,    35,    36,
     258,    37,    38,    39,     0,    40,    41,    42,    43,    44,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     232,     0,     0,   125,     0,     0,    47,    48,     0,     0,
      49,     0,   663,     0,     0,     0,    51,     0,     0,   261,
       0,     0,     0,     0,     0,   262,     0,     0,     0,     0,
      53,   263,    55,    56,     0,    57,    58,    59,    60,     0,
       0,     0,     0,    61,    62,     0,     0,     0,     2,     3,
     112,    63,     6,    64,    65,    66,     7,     8,    67,    68,
       0,     9,     0,    10,    11,    12,    13,    14,    15,    16,
       0,     0,     0,     0,     0,    17,    18,    19,    20,    21,
      22,    23,     0,     0,    24,     0,     0,     0,     0,     0,
       0,    26,     0,     0,    29,    30,    31,    32,    33,    34,
      35,    36,     0,    37,    38,    39,     0,    40,    41,    42,
      43,    44,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   232,     0,     0,   125,     0,     0,    47,    48,
       0,     0,    49,     0,     0,     0,     0,     0,    51,     0,
       0,     0,     0,     0,     0,     0,     0,    52,     0,     0,
       0,     0,    53,    54,    55,    56,     0,    57,    58,    59,
      60,     0,     0,     0,     0,    61,    62,   242,     0,     0,
       0,     0,   243,    63,     0,    64,    65,    66,     0,     0,
      67,    68,     2,     3,   112,     0,     6,     0,     0,     0,
       7,     8,     0,     0,     0,     9,     0,    10,    11,    12,
     113,   114,    15,    16,     0,     0,     0,     0,     0,   115,
     116,   117,    20,    21,    22,    23,     0,     0,   118,     0,
       0,     0,     0,     0,     0,    26,     0,     0,    29,    30,
      31,    32,    33,    34,    35,    36,   258,    37,    38,    39,
       0,    40,    41,    42,    43,    44,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   232,     0,     0,   125,
       0,     0,    47,    48,     0,     0,    49,     0,     0,     0,
       0,     0,    51,     0,     0,   261,     0,     0,     0,     0,
       0,   262,     0,     0,     0,     0,    53,   263,    55,    56,
       0,    57,    58,    59,    60,     0,     0,     0,     0,    61,
      62,     0,     0,     0,     2,     3,   112,    63,     6,    64,
      65,    66,     7,     8,    67,    68,     0,     9,     0,    10,
      11,    12,    13,    14,    15,    16,     0,     0,     0,     0,
       0,    17,    18,    19,    20,    21,    22,    23,     0,     0,
      24,     0,     0,     0,     0,     0,     0,    26,     0,     0,
      29,    30,    31,    32,    33,    34,    35,    36,     0,    37,
      38,    39,     0,    40,    41,    42,    43,    44,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   232,     0,
       0,   125,     0,     0,    47,    48,     0,     0,    49,     0,
       0,     0,     0,     0,    51,     0,     0,     0,     0,     0,
       0,     0,     0,    52,     0,     0,     0,     0,    53,    54,
      55,    56,     0,    57,    58,    59,    60,     0,     0,     0,
       0,    61,    62,   281,     0,     0,     2,     3,   112,    63,
       6,    64,    65,    66,     7,     8,    67,    68,     0,     9,
       0,    10,    11,    12,   113,   114,    15,    16,     0,     0,
       0,     0,     0,   115,   116,   117,    20,    21,    22,    23,
       0,     0,   118,     0,     0,     0,     0,     0,     0,    26,
       0,     0,    29,    30,    31,    32,    33,    34,    35,    36,
       0,    37,    38,    39,     0,    40,    41,    42,    43,    44,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     232,     0,     0,   125,   459,     0,    47,    48,     0,     0,
      49,     0,     0,     0,     0,     0,    51,     0,     0,     0,
       0,     0,     0,     0,     0,   262,     0,     0,     0,     0,
      53,    54,    55,    56,     0,    57,    58,    59,    60,     0,
       0,     0,     0,    61,    62,     0,     0,     0,     2,     3,
     112,    63,     6,    64,    65,    66,     7,     8,    67,    68,
       0,     9,     0,    10,    11,    12,    13,    14,    15,    16,
       0,     0,     0,     0,     0,    17,    18,    19,    20,    21,
      22,    23,     0,     0,   118,     0,     0,     0,     0,     0,
       0,    26,     0,     0,    29,    30,    31,    32,    33,    34,
      35,    36,     0,    37,    38,    39,     0,    40,    41,    42,
      43,    44,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   232,     0,     0,   125,     0,     0,    47,    48,
       0,     0,    49,     0,   550,     0,     0,     0,    51,     0,
       0,     0,     0,     0,     0,     0,     0,   262,     0,     0,
       0,     0,    53,    54,    55,    56,     0,    57,    58,    59,
      60,     0,     0,     0,     0,    61,    62,     0,     0,     0,
       2,     3,   112,    63,     6,    64,    65,    66,     7,     8,
      67,    68,     0,     9,     0,    10,    11,    12,   113,   114,
      15,    16,     0,     0,     0,     0,     0,   115,   116,   117,
      20,    21,    22,    23,     0,     0,   118,     0,     0,     0,
       0,     0,     0,    26,     0,     0,    29,    30,    31,    32,
      33,    34,    35,    36,     0,    37,    38,    39,     0,    40,
      41,    42,    43,    44,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   232,     0,     0,   125,     0,     0,
      47,    48,     0,     0,    49,     0,   259,     0,     0,     0,
      51,     0,     0,     0,     0,     0,     0,     0,     0,   262,
       0,     0,     0,     0,    53,    54,    55,    56,     0,    57,
      58,    59,    60,     0,     0,     0,     0,    61,    62,     0,
       0,     0,     2,     3,   112,    63,     6,    64,    65,    66,
       7,     8,    67,    68,     0,     9,     0,    10,    11,    12,
     113,   114,    15,    16,     0,     0,     0,     0,     0,   115,
     116,   117,    20,    21,    22,    23,     0,     0,   118,     0,
       0,     0,     0,     0,     0,    26,     0,     0,    29,    30,
      31,    32,    33,    34,    35,    36,     0,    37,    38,    39,
       0,    40,    41,    42,    43,    44,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   232,     0,     0,   125,
       0,     0,    47,    48,     0,     0,    49,     0,   550,     0,
       0,     0,    51,     0,     0,     0,     0,     0,     0,     0,
       0,   262,     0,     0,     0,     0,    53,    54,    55,    56,
       0,    57,    58,    59,    60,     0,     0,     0,     0,    61,
      62,     0,     0,     0,     2,     3,   112,    63,     6,    64,
      65,    66,     7,     8,    67,    68,     0,     9,     0,    10,
      11,    12,   113,   114,    15,    16,     0,     0,     0,     0,
       0,   115,   116,   117,    20,    21,    22,    23,     0,     0,
     118,     0,     0,     0,     0,     0,     0,    26,     0,     0,
      29,    30,    31,    32,    33,    34,    35,    36,     0,    37,
      38,    39,     0,    40,    41,    42,    43,    44,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   232,     0,
       0,   125,     0,     0,    47,    48,     0,     0,    49,     0,
     876,     0,     0,     0,    51,     0,     0,     0,     0,     0,
       0,     0,     0,   262,     0,     0,     0,     0,    53,    54,
      55,    56,     0,    57,    58,    59,    60,     0,     0,     0,
       0,    61,    62,     0,     0,     0,     2,     3,   112,    63,
       6,    64,    65,    66,     7,     8,    67,    68,     0,     9,
       0,    10,    11,    12,   113,   114,    15,    16,     0,     0,
       0,     0,     0,   115,   116,   117,    20,    21,    22,    23,
       0,     0,   118,     0,     0,     0,     0,     0,     0,    26,
       0,     0,    29,    30,    31,    32,    33,    34,    35,    36,
       0,    37,    38,    39,     0,    40,    41,    42,    43,    44,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     232,     0,     0,   125,     0,     0,    47,    48,     0,     0,
      49,     0,   663,     0,     0,     0,    51,     0,     0,     0,
       0,     0,     0,     0,     0,   262,     0,     0,     0,     0,
      53,    54,    55,    56,     0,    57,    58,    59,    60,     0,
       0,     0,     0,    61,    62,     0,     0,     0,     2,     3,
     112,    63,     6,    64,    65,    66,     7,     8,    67,    68,
       0,     9,     0,    10,    11,    12,    13,    14,    15,    16,
       0,     0,     0,     0,     0,    17,    18,    19,    20,    21,
      22,    23,     0,     0,    24,     0,     0,     0,     0,     0,
       0,    26,     0,     0,    29,    30,    31,    32,    33,    34,
      35,    36,     0,    37,    38,    39,     0,    40,    41,    42,
      43,    44,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   232,     0,     0,   125,     0,     0,    47,    48,
       0,     0,    49,     0,     0,     0,     0,     0,    51,     0,
       0,     0,     0,     0,     0,     0,     0,    52,     0,     0,
       0,     0,    53,    54,    55,    56,     0,    57,    58,    59,
      60,     0,     0,     0,     0,    61,    62,     0,     0,     0,
       2,     3,   112,    63,     6,    64,    65,    66,     7,     8,
      67,    68,     0,     9,     0,    10,    11,    12,   113,   114,
      15,    16,     0,     0,     0,     0,     0,   115,   116,   117,
      20,    21,    22,    23,     0,     0,   118,     0,     0,     0,
       0,     0,     0,    26,     0,     0,    29,    30,    31,    32,
      33,    34,    35,    36,     0,    37,    38,    39,     0,    40,
      41,    42,    43,    44,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   232,     0,     0,   125,     0,     0,
      47,    48,     0,     0,    49,     0,     0,     0,     0,     0,
      51,     0,     0,     0,     0,     0,     0,     0,     0,   262,
       0,     0,     0,     0,    53,    54,    55,    56,     0,    57,
      58,    59,    60,     0,     0,     0,     0,    61,    62,     0,
       0,     0,     2,     3,   112,    63,     6,    64,    65,    66,
       7,     8,    67,    68,     0,     9,     0,    10,    11,    12,
      13,    14,    15,    16,     0,     0,     0,     0,     0,    17,
      18,    19,    20,    21,    22,    23,     0,     0,   118,     0,
       0,     0,     0,     0,     0,    26,     0,     0,    29,    30,
      31,    32,    33,    34,    35,    36,     0,    37,    38,    39,
       0,    40,    41,    42,    43,    44,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   232,     0,     0,   125,
       0,     0,    47,    48,     0,     0,    49,     0,     0,     0,
       0,     0,    51,     0,     0,     0,     0,     0,     0,     0,
       0,   262,     0,     0,     0,     0,    53,    54,    55,    56,
       0,    57,    58,    59,    60,     0,     0,     0,     0,    61,
      62,     0,     0,     0,     2,     3,   112,    63,     6,    64,
      65,    66,     7,     8,    67,    68,     0,     9,     0,    10,
      11,    12,   113,   114,    15,    16,     0,     0,     0,     0,
       0,   115,   116,   117,    20,    21,    22,    23,     0,     0,
     118,     0,     0,     0,     0,     0,     0,   119,     0,     0,
      29,    30,    31,   120,    33,    34,    35,   121,     0,    37,
      38,    39,     0,    40,    41,     0,     0,   122,     0,   355,
     356,   357,   358,   359,     0,   360,   361,   362,   363,   364,
     365,     0,   366,   367,     0,   123,   368,   369,   124,     0,
       0,   125,     0,     0,    47,    48,     0,     0,    49,     0,
       0,     0,   370,     0,   371,     0,   372,   373,     0,   374,
     375,   376,   377,   378,     0,   379,     0,     0,    53,    54,
      55,    56,     0,    57,    58,    59,    60,     0,     0,     0,
       0,    61,    62,     0,   281,   380,     2,     3,   112,    63,
       6,    64,    65,    66,     7,     8,    67,    68,     0,     9,
       0,    10,    11,    12,   113,   114,    15,    16,     0,     0,
       0,     0,     0,   115,   116,   117,    20,    21,    22,    23,
       0,     0,   118,     0,     0,     0,     0,     0,     0,   119,
       0,     0,    29,    30,    31,    32,    33,    34,    35,    36,
       0,    37,    38,    39,     0,    40,    41,     0,     0,   122,
       0,   355,   356,   357,   358,   359,     0,   360,   361,   362,
     363,   364,   365,     0,   366,   367,     0,     0,   368,   369,
     250,     0,     0,    46,     0,     0,    47,    48,     0,     0,
      49,     0,    50,     0,   370,     0,   371,     0,   372,   373,
       0,   374,   375,   376,   377,   378,     0,   379,     0,     0,
      53,    54,    55,    56,     0,    57,    58,    59,    60,     0,
       0,     0,     0,    61,    62,     0,     0,   380,     2,     3,
     112,    63,     6,    64,    65,    66,     7,     8,    67,    68,
       0,     9,     0,    10,    11,    12,   113,   114,    15,    16,
       0,     0,     0,     0,     0,   115,   116,   117,    20,    21,
      22,    23,     0,     0,   118,     0,     0,     0,     0,     0,
       0,   119,     0,     0,    29,    30,    31,    32,    33,    34,
      35,    36,     0,    37,    38,    39,     0,    40,    41,     0,
       0,   122,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   312,     0,     0,   125,     0,     0,    47,    48,
       0,     0,    49,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    53,    54,    55,    56,     0,    57,    58,    59,
      60,     0,     0,     0,     0,    61,    62,     0,     0,     0,
     313,     0,     0,    63,     0,    64,    65,    66,     0,     0,
      67,    68,     2,     3,   112,     0,     6,     0,     0,     0,
       7,     8,     0,     0,     0,     9,     0,    10,    11,    12,
     113,   114,    15,    16,     0,     0,     0,     0,     0,   115,
     116,   117,    20,    21,    22,    23,     0,     0,   118,     0,
       0,     0,     0,     0,     0,   119,     0,     0,    29,    30,
      31,    32,    33,    34,    35,    36,     0,    37,    38,    39,
       0,    40,    41,     0,     0,   122,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   312,     0,     0,   351,
       0,     0,    47,    48,     0,     0,    49,     0,   352,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    53,    54,    55,    56,
       0,    57,    58,    59,    60,     0,     0,     0,     0,    61,
      62,     0,     0,     0,     2,     3,   112,    63,     6,    64,
      65,    66,     7,     8,    67,    68,     0,     9,     0,    10,
      11,    12,   113,   114,    15,    16,     0,     0,     0,     0,
       0,   115,   116,   117,    20,    21,    22,    23,     0,     0,
     118,     0,     0,     0,     0,     0,     0,   119,     0,     0,
      29,    30,    31,    32,    33,    34,    35,    36,     0,    37,
      38,    39,     0,    40,    41,     0,     0,   122,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   312,     0,
       0,   125,     0,     0,    47,    48,     0,     0,    49,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    53,    54,
      55,    56,     0,    57,    58,    59,    60,     0,     0,     0,
       0,    61,    62,     0,     0,     0,   557,     0,     0,    63,
       0,    64,    65,    66,     0,     0,    67,    68,     2,     3,
     112,     0,     6,     0,     0,     0,     7,     8,     0,     0,
       0,     9,     0,    10,    11,    12,   113,   114,    15,    16,
       0,     0,     0,     0,     0,   115,   116,   117,    20,    21,
      22,    23,     0,     0,   118,     0,     0,     0,     0,     0,
       0,   119,     0,     0,    29,    30,    31,   120,    33,    34,
      35,   121,     0,    37,    38,    39,     0,    40,    41,     0,
       0,   122,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   124,     0,     0,   125,     0,     0,    47,    48,
       0,     0,    49,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    53,    54,    55,    56,     0,    57,    58,    59,
      60,     0,     0,     0,     0,    61,    62,     0,     0,     0,
       2,     3,   112,    63,     6,    64,    65,    66,     7,     8,
      67,    68,     0,     9,     0,    10,    11,    12,   113,   114,
      15,    16,     0,     0,     0,     0,     0,   115,   116,   117,
      20,    21,    22,    23,     0,     0,   118,     0,     0,     0,
       0,     0,     0,   119,     0,     0,    29,    30,    31,    32,
      33,    34,    35,    36,     0,    37,    38,    39,     0,    40,
      41,     0,     0,   122,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   312,     0,     0,   351,     0,     0,
      47,    48,     0,     0,    49,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    53,    54,    55,    56,     0,    57,
      58,    59,    60,     0,     0,     0,     0,    61,    62,     0,
       0,     0,     2,     3,   112,    63,     6,    64,    65,    66,
       7,     8,    67,    68,     0,     9,     0,    10,    11,    12,
     113,   114,    15,    16,     0,     0,     0,     0,     0,   115,
     116,   117,    20,    21,    22,    23,     0,     0,   118,     0,
       0,     0,     0,     0,     0,   119,     0,     0,    29,    30,
      31,    32,    33,    34,    35,    36,     0,    37,    38,    39,
       0,    40,    41,     0,     0,   122,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   984,     0,     0,   125,
       0,     0,    47,    48,     0,     0,    49,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    53,    54,    55,    56,
       0,    57,    58,    59,    60,     0,     0,     0,     0,    61,
      62,     0,     0,     0,     2,     3,   112,    63,     6,    64,
      65,    66,     7,     8,    67,    68,     0,     9,     0,    10,
      11,    12,   113,   114,    15,    16,     0,     0,     0,     0,
       0,   115,   116,   117,    20,    21,    22,    23,     0,     0,
     118,     0,     0,     0,     0,     0,     0,   119,     0,     0,
      29,    30,    31,    32,    33,    34,    35,    36,     0,    37,
      38,    39,     0,    40,    41,     0,     0,   122,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,  1062,     0,
       0,   125,     0,     0,    47,    48,     0,     0,    49,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,    53,    54,
      55,    56,     0,    57,    58,    59,    60,     0,     0,     0,
       0,    61,    62,     0,     0,     0,     0,     0,     0,    63,
       0,    64,    65,    66,     0,     0,    67,    68,   135,   136,
     137,   138,   139,   140,   141,   142,   143,   144,   145,   146,
     147,   148,   149,   150,   151,   152,   153,   154,   155,   156,
     157,   158,     0,     0,     0,   159,   160,   161,   216,   217,
     218,   219,   166,   167,   168,     0,     0,     0,     0,     0,
     169,   170,   171,   172,   220,   221,   222,   223,   177,   283,
       0,   224,     0,     0,     0,     0,     0,     0,     0,   180,
     181,     0,   182,   183,   184,   185,   186,     0,   187,   188,
       0,     0,   189,   190,     0,     0,     0,   191,   192,   193,
     194,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   196,   197,     0,   198,   199,   200,
     201,   202,   203,   204,   205,   206,   207,   208,   209,     0,
       0,   210,    53,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    62,   135,   136,   137,
     138,   139,   140,   141,   142,   143,   144,   145,   146,   147,
     148,   149,   150,   151,   152,   153,   154,   155,   156,   157,
     158,     0,     0,     0,   159,   160,   161,   216,   217,   218,
     219,   166,   167,   168,     0,     0,     0,     0,     0,   169,
     170,   171,   172,   220,   221,   222,   223,   177,     0,     0,
     224,     0,     0,     0,     0,     0,     0,     0,   180,   181,
       0,   182,   183,   184,   185,   186,     0,   187,   188,     0,
       0,   189,   190,     0,     0,     0,   191,   192,   193,   194,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   196,   197,     0,   198,   199,   200,   201,
     202,   203,   204,   205,   206,   207,   208,   209,     0,     0,
     210,    53,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,    62,   135,   136,   137,   138,
     139,   140,   141,   142,   143,   144,   145,   146,   147,   148,
     149,   150,   151,   152,   153,   154,   155,   156,   157,   158,
       0,     0,     0,   159,   160,   161,   162,   163,   164,   165,
     166,   167,   168,     0,     0,     0,     0,     0,   169,   170,
     171,   172,   173,   174,   175,   176,   177,    34,   178,   179,
       0,    37,     0,     0,     0,     0,     0,   180,   181,     0,
     182,   183,   184,   185,   186,     0,   187,   188,     0,     0,
     189,   190,     0,     0,     0,   191,   192,   193,   194,     0,
       0,     0,     0,     0,   195,     0,     0,     0,     0,     0,
       0,     0,   196,   197,     0,   198,   199,   200,   201,   202,
     203,   204,   205,   206,   207,   208,   209,     0,     0,   210,
     135,   136,   137,   138,   139,   140,   141,   142,   143,   144,
     145,   146,   147,   148,   149,   150,   151,   152,   153,   154,
     155,   156,   157,   158,     0,     0,     0,   159,   160,   161,
     162,   163,   164,   165,   166,   167,   168,     0,     0,     0,
       0,     0,   169,   170,   171,   172,   173,   174,   175,   176,
     177,    34,    35,   179,     0,    37,     0,     0,     0,     0,
       0,   180,   181,     0,   182,   183,   184,   185,   186,     0,
     187,   188,     0,     0,   189,   190,     0,     0,     0,   191,
     192,   193,   194,     0,     0,     0,     0,     0,   195,     0,
       0,     0,     0,     0,     0,     0,   196,   197,     0,   198,
     199,   200,   201,   202,   203,   204,   205,   206,   207,   208,
     209,     0,     0,   210,   135,   136,   137,   138,   139,   140,
     141,   142,   143,   144,   145,   146,   147,   148,   149,   150,
     151,   152,   153,   154,   155,   156,   157,   158,     0,     0,
       0,   159,   160,   161,   216,   217,   218,   219,   166,   167,
     168,     0,     0,     0,     0,     0,   169,   170,   171,   172,
     220,   221,   222,   223,   177,     0,     0,   224,     0,     0,
       0,     0,     0,     0,     0,   180,   181,     0,   182,   183,
     184,   185,   186,     0,   187,   188,     0,     0,   189,   190,
       0,     0,     0,   191,   192,   193,   194,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     196,   197,     0,   198,   199,   200,   201,   202,   203,   204,
     205,   206,   207,   208,   209,   621,   597,   210,     0,   622,
       0,     0,     0,     0,     0,     0,     0,   180,   181,     0,
     182,   183,   184,   185,   186,     0,   187,   188,     0,     0,
     189,   190,     0,     0,     0,   191,   192,   193,   194,     0,
       0,     0,     0,     0,   277,     0,     0,     0,   623,     0,
       0,     0,   196,   197,     0,   198,   199,   200,   201,   202,
     203,   204,   205,   206,   207,   208,   209,   596,   597,   210,
       0,   598,     0,     0,     0,     0,     0,     0,     0,   180,
     181,     0,   182,   183,   184,   185,   186,     0,   187,   188,
       0,     0,   189,   190,     0,     0,     0,   191,   192,   193,
     194,     0,     0,     0,     0,     0,   277,     0,     0,     0,
       0,     0,     0,     0,   196,   197,     0,   198,   199,   200,
     201,   202,   203,   204,   205,   206,   207,   208,   209,   604,
     605,   210,     0,   606,     0,     0,     0,     0,     0,     0,
       0,   180,   181,     0,   182,   183,   184,   185,   186,     0,
     187,   188,     0,     0,   189,   190,     0,     0,     0,   191,
     192,   193,   194,     0,     0,     0,     0,     0,   277,     0,
       0,     0,     0,     0,     0,     0,   196,   197,     0,   198,
     199,   200,   201,   202,   203,   204,   205,   206,   207,   208,
     209,   610,   605,   210,     0,   611,     0,     0,     0,     0,
       0,     0,     0,   180,   181,     0,   182,   183,   184,   185,
     186,     0,   187,   188,     0,     0,   189,   190,     0,     0,
       0,   191,   192,   193,   194,     0,     0,     0,     0,     0,
     277,     0,     0,     0,     0,     0,     0,     0,   196,   197,
       0,   198,   199,   200,   201,   202,   203,   204,   205,   206,
     207,   208,   209,   642,   597,   210,     0,   643,     0,     0,
       0,     0,     0,     0,     0,   180,   181,     0,   182,   183,
     184,   185,   186,     0,   187,   188,     0,     0,   189,   190,
       0,     0,     0,   191,   192,   193,   194,     0,     0,     0,
       0,     0,   277,     0,     0,     0,     0,     0,     0,     0,
     196,   197,     0,   198,   199,   200,   201,   202,   203,   204,
     205,   206,   207,   208,   209,   645,   605,   210,     0,   646,
       0,     0,     0,     0,     0,     0,     0,   180,   181,     0,
     182,   183,   184,   185,   186,     0,   187,   188,     0,     0,
     189,   190,     0,     0,     0,   191,   192,   193,   194,     0,
       0,     0,     0,     0,   277,     0,     0,     0,     0,     0,
       0,     0,   196,   197,     0,   198,   199,   200,   201,   202,
     203,   204,   205,   206,   207,   208,   209,   655,   597,   210,
       0,   656,     0,     0,     0,     0,     0,     0,     0,   180,
     181,     0,   182,   183,   184,   185,   186,     0,   187,   188,
       0,     0,   189,   190,     0,     0,     0,   191,   192,   193,
     194,     0,     0,     0,     0,     0,   277,     0,     0,     0,
       0,     0,     0,     0,   196,   197,     0,   198,   199,   200,
     201,   202,   203,   204,   205,   206,   207,   208,   209,   658,
     605,   210,     0,   659,     0,     0,     0,     0,     0,     0,
       0,   180,   181,     0,   182,   183,   184,   185,   186,     0,
     187,   188,     0,     0,   189,   190,     0,     0,     0,   191,
     192,   193,   194,     0,     0,     0,     0,     0,   277,     0,
       0,     0,     0,     0,     0,     0,   196,   197,     0,   198,
     199,   200,   201,   202,   203,   204,   205,   206,   207,   208,
     209,   689,   597,   210,     0,   690,     0,     0,     0,     0,
       0,     0,     0,   180,   181,     0,   182,   183,   184,   185,
     186,     0,   187,   188,     0,     0,   189,   190,     0,     0,
       0,   191,   192,   193,   194,     0,     0,     0,     0,     0,
     277,     0,     0,     0,     0,     0,     0,     0,   196,   197,
       0,   198,   199,   200,   201,   202,   203,   204,   205,   206,
     207,   208,   209,   692,   605,   210,     0,   693,     0,     0,
       0,     0,     0,     0,     0,   180,   181,     0,   182,   183,
     184,   185,   186,     0,   187,   188,     0,     0,   189,   190,
       0,     0,     0,   191,   192,   193,   194,     0,     0,     0,
       0,     0,   277,     0,     0,     0,     0,     0,     0,     0,
     196,   197,     0,   198,   199,   200,   201,   202,   203,   204,
     205,   206,   207,   208,   209,   881,   597,   210,     0,   882,
       0,     0,     0,     0,     0,     0,     0,   180,   181,     0,
     182,   183,   184,   185,   186,     0,   187,   188,     0,     0,
     189,   190,     0,     0,     0,   191,   192,   193,   194,     0,
       0,     0,     0,     0,   277,     0,     0,     0,     0,     0,
       0,     0,   196,   197,     0,   198,   199,   200,   201,   202,
     203,   204,   205,   206,   207,   208,   209,   884,   605,   210,
       0,   885,     0,     0,     0,     0,     0,     0,     0,   180,
     181,     0,   182,   183,   184,   185,   186,     0,   187,   188,
       0,     0,   189,   190,     0,     0,     0,   191,   192,   193,
     194,     0,     0,     0,     0,     0,   277,     0,     0,     0,
       0,     0,     0,     0,   196,   197,     0,   198,   199,   200,
     201,   202,   203,   204,   205,   206,   207,   208,   209,  1073,
     597,   210,     0,  1074,     0,     0,     0,     0,     0,     0,
       0,   180,   181,     0,   182,   183,   184,   185,   186,     0,
     187,   188,     0,     0,   189,   190,     0,     0,     0,   191,
     192,   193,   194,     0,     0,     0,     0,     0,   277,     0,
       0,     0,     0,     0,     0,     0,   196,   197,     0,   198,
     199,   200,   201,   202,   203,   204,   205,   206,   207,   208,
     209,  1076,   605,   210,     0,  1077,     0,     0,     0,     0,
       0,     0,     0,   180,   181,     0,   182,   183,   184,   185,
     186,     0,   187,   188,     0,     0,   189,   190,     0,     0,
       0,   191,   192,   193,   194,     0,     0,     0,     0,     0,
     277,     0,     0,     0,     0,     0,     0,     0,   196,   197,
       0,   198,   199,   200,   201,   202,   203,   204,   205,   206,
     207,   208,   209,  1102,   597,   210,     0,  1103,     0,     0,
       0,     0,     0,     0,     0,   180,   181,     0,   182,   183,
     184,   185,   186,     0,   187,   188,     0,     0,   189,   190,
       0,     0,     0,   191,   192,   193,   194,     0,     0,     0,
       0,     0,   277,     0,     0,     0,     0,     0,     0,     0,
     196,   197,     0,   198,   199,   200,   201,   202,   203,   204,
     205,   206,   207,   208,   209,   610,   605,   210,     0,   611,
       0,     0,     0,     0,     0,     0,     0,   180,   181,     0,
     182,   183,   184,   185,   186,     0,   187,   188,     0,     0,
     189,   190,     0,     0,     0,   191,   192,   193,   194,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   196,   197,     0,   198,   199,   200,   201,   202,
     203,   204,   205,   206,   207,   208,   209,     0,     0,   210
  };

  const short int
  parser::yycheck_[] =
  {
       0,     1,     2,     3,    25,    90,    83,     7,     8,     9,
     423,   727,    12,    13,    14,   534,     4,    17,     5,    88,
      89,   241,   383,   428,   270,   440,    52,   442,     5,     8,
       9,   331,   527,   354,     2,     3,   488,   404,    25,    46,
     280,   402,     4,     5,    12,    19,    46,   307,    25,   442,
      50,   311,    52,     1,    13,   515,   124,   418,    50,   639,
     733,   301,   286,    25,    13,    71,   382,   647,    71,   385,
     431,   387,   834,    13,    19,    13,     0,     1,     4,    13,
     953,   909,    50,    83,   132,    13,    14,   403,   481,    17,
      13,    27,   832,  1017,     0,     1,     2,     3,   391,   392,
      25,   417,    13,   419,    78,   557,    12,    25,    25,    13,
     846,    13,    25,    25,    29,    83,   432,   323,     9,   325,
     326,   114,   328,   123,   112,   125,    61,    13,    14,   490,
      25,    17,    24,    78,    26,    13,    14,   534,   132,    17,
      46,    13,   970,   137,    50,    92,   215,    13,   464,    25,
     112,    91,   354,    91,    25,    37,    38,   295,    25,  1083,
     114,    44,    48,    49,    28,   727,   406,   236,   936,    56,
      91,   733,   117,   132,    91,   491,   135,    83,   137,    91,
      71,   126,  1055,   132,   253,   132,   112,  1060,   137,   391,
     392,   931,   132,   128,   132,   195,   132,   137,   132,   137,
       0,   137,    26,   137,   444,   451,  1034,   943,   448,   132,
     135,   132,   452,   351,   137,   515,    26,   135,   135,   125,
     289,   132,   135,   135,   139,    61,   137,   119,   132,   469,
     132,   231,   438,   137,    72,   137,   313,   999,   244,   122,
     135,    61,   111,    52,   468,   961,   315,    56,   248,   249,
      76,    72,   134,   620,    13,    14,   671,   672,    17,   135,
     132,  1001,    56,   132,   135,   137,   132,   634,   135,   248,
     249,   137,    37,    38,   274,    72,   276,   277,  1046,   672,
     280,    91,   282,   956,   846,   111,   286,   287,   295,    48,
     114,   297,   128,   293,   297,   295,  1012,   503,   135,   529,
     530,    95,   508,   303,   114,  1045,   126,   127,   128,   761,
      61,    72,   109,   313,   135,   712,   556,   638,     0,     1,
       2,     3,     4,   349,    72,   231,   132,    88,   354,   135,
      12,   791,   792,   401,   334,   335,   336,   337,   338,   339,
     340,   341,   922,    79,   351,   313,   274,    72,   276,   349,
      86,   351,   352,   244,   354,    88,   335,   336,   337,   338,
     352,   856,   286,    88,    46,   391,   392,   132,    50,   293,
      91,    52,  1088,    54,    55,   126,   127,    58,    91,   303,
     286,   287,    91,   383,   352,   136,   334,   293,   274,   295,
     276,   391,   392,    72,   956,    61,   274,   303,   276,   277,
      72,    83,   402,    72,   404,   766,   297,   313,   144,    88,
     334,   488,   304,   818,   414,   339,    88,   772,   418,    88,
     666,   109,   422,   385,   779,   387,   686,   134,   334,   649,
     112,   431,    79,   339,   953,   675,    15,   458,    17,    86,
     385,   403,   135,   125,   684,   351,   352,    61,   354,   765,
    1012,   767,   947,   948,   135,  1017,   135,   419,   403,   459,
     126,   127,   128,   135,    56,    37,    38,   864,   468,    46,
     432,   458,   472,    72,   419,   715,   769,   770,    72,   719,
     557,   458,   775,   776,    72,   391,   392,   432,   488,    88,
     490,   791,   792,    88,    88,    17,   458,   144,   404,   568,
      88,    72,   464,   132,   395,   383,   135,    72,   414,   922,
      26,   926,   126,   127,    72,   274,   422,   276,    26,   464,
     488,  1083,   136,    88,   402,    52,    72,    54,    55,   491,
      88,    58,    79,   926,   901,   932,   427,   428,    79,    86,
     418,   135,    88,    56,   468,    86,   491,   135,   125,   231,
      14,    15,   132,   431,   440,   135,   953,   557,    25,  1054,
     132,   763,   468,    79,   137,   137,   472,   769,   770,   132,
      86,    79,   135,   775,   776,    91,   822,   135,    86,    95,
     118,   132,   488,    91,   135,    72,    89,    95,   494,   557,
     349,   831,   130,   479,    25,   354,   132,   144,   114,   135,
     893,    88,   140,   144,   286,   287,   114,    79,   135,   135,
     687,   293,   490,   295,    86,   636,   983,   135,    79,   619,
     620,   303,    67,   853,   854,    86,    67,   601,   144,    25,
     860,   313,   862,   863,   634,   609,   144,    72,   612,   114,
     619,    72,     0,     1,     2,     3,     4,   635,   135,   636,
      61,   557,   334,    88,    12,    79,   601,   339,    92,   636,
     660,    92,    86,  1060,   609,   424,    72,   612,   908,   351,
     352,    95,   144,   635,   636,    96,    72,   115,  1039,    56,
     625,   660,   627,   144,   761,    37,    38,   687,    46,    72,
      61,   893,    50,   913,   694,   780,    92,   917,   135,    79,
     733,   132,   735,   595,   135,    88,    86,    72,   101,   102,
     287,   603,    61,    98,   620,   126,   127,   128,   295,   687,
     144,   135,   404,    88,  1040,    83,    79,    91,   634,    72,
      72,    72,   414,    86,   626,    95,   132,   763,    91,   135,
     422,   405,    95,   769,   770,    88,    88,    88,    10,   775,
     776,     8,   644,    13,   112,   126,   127,   128,    26,    10,
      92,   761,   762,   763,   144,   657,   766,   125,    61,   769,
     770,   115,   143,   135,   351,   775,   776,   126,   127,   128,
     135,   687,    79,    92,   115,   671,   468,   135,   694,    86,
     472,   144,   135,   761,   121,  1025,  1026,  1027,  1028,   691,
     800,    62,    63,   803,    25,   767,   488,   135,    56,   473,
     136,    79,   136,   846,   814,   815,   816,   135,    86,   135,
      54,   135,   767,    91,   803,    59,    60,    95,   720,   721,
     391,   392,   135,   126,   127,   128,   135,   414,   762,   729,
      25,   731,    86,    10,   115,   422,   114,   144,   848,   849,
     514,    72,   516,   517,   518,   761,   762,   763,    10,   420,
     421,    79,   135,   769,   770,   135,  1096,   893,    86,   775,
     776,    92,   872,   231,   874,   557,   144,    95,    72,   638,
     141,   142,   135,    79,   145,   146,    44,    72,   766,    10,
      86,    91,   783,   893,   800,   787,    61,   729,   790,    95,
      56,   901,    15,   115,    10,   466,    91,    92,   814,   815,
     816,   132,   912,    79,   135,   915,    52,   986,    54,    55,
      86,    92,    58,   956,    79,   958,   144,   818,   286,   287,
     963,    86,   135,    72,    79,   293,    89,   295,   620,   626,
      95,    86,   848,   849,   912,   303,    89,   132,   144,    52,
     135,    52,   634,    79,   846,   313,   846,   644,    52,   135,
      86,   126,   127,   128,   115,   629,   872,    96,   874,    95,
     657,   134,    79,   134,    10,    34,   334,    10,   144,    86,
      89,   339,    67,   983,  1017,     9,  1019,   893,    95,   144,
     990,   883,   992,   351,   352,   901,    10,    56,   998,   144,
      10,   135,   129,   135,   691,   687,   912,   137,    92,   915,
     115,   135,   694,   135,   846,   100,     0,     1,   144,   104,
     105,   135,  1022,     7,     8,     9,   115,   135,    87,    13,
      14,    90,  1065,    17,   721,    94,   135,   144,    97,  1039,
      10,    10,    79,   135,   103,   937,   404,   711,   111,    86,
    1083,   943,  1085,   135,  1022,   135,   414,  1090,    95,   135,
      79,  1061,    46,    52,   422,    54,    55,    86,    52,    58,
     135,    10,   131,  1106,   115,   967,    95,   983,  1040,   761,
     762,    79,    79,    79,   990,   135,   992,    10,    86,    86,
      86,   982,   998,  1061,    56,  1040,   135,    95,    95,    95,
     787,    90,   111,   790,    72,   135,    10,   144,    10,   135,
     468,    56,  1004,    57,   472,   135,  1022,   694,   800,   950,
     951,    79,  1012,    67,  1014,   144,    91,  1017,    86,  1019,
     488,   135,   814,   815,   816,   135,   135,    95,    91,   123,
      12,   125,   472,   729,     3,   731,   144,   144,   144,   735,
     612,   514,    79,    79,  1042,  1061,   846,   101,   102,    86,
      86,  1039,   106,   754,   724,  1046,   848,   849,    95,    95,
     781,   547,   836,   347,    93,   263,   840,    55,   842,   843,
    1012,  1041,  1014,  1075,   328,  1017,   144,  1019,   911,   729,
     872,  1014,   874,  1083,  1012,  1085,   883,   727,  1088,   557,
    1090,   866,   866,    40,    41,    42,    43,    44,   769,   770,
     215,   195,   638,   905,   775,   776,  1106,   144,   144,   901,
    1051,  1052,   632,   800,    -1,  1056,    -1,  1058,  1059,    -1,
     912,    -1,    -1,   915,    -1,    -1,    -1,   814,   815,   816,
     904,    -1,    -1,    -1,    -1,   806,   807,   231,   809,   810,
     937,  1083,    -1,  1085,    -1,    -1,  1088,    -1,  1090,    -1,
     846,    -1,   620,    -1,   248,   249,    -1,  1098,  1099,  1100,
    1101,   848,   849,    -1,  1106,    -1,   634,     0,     1,  1110,
     967,    54,    55,    -1,    -1,    58,    59,    60,    -1,    12,
     274,    -1,   276,   277,    -1,   872,   280,   874,   282,    -1,
      -1,   983,   286,   287,    67,    -1,    -1,    -1,   990,   293,
     992,   295,    -1,    -1,    -1,    -1,   998,    -1,    -1,   303,
      -1,    -1,    29,    46,    -1,    -1,    -1,    34,    -1,   687,
      -1,    -1,   893,    -1,    -1,    -1,   694,   100,   915,    -1,
    1022,   104,   105,  1007,   107,   108,    -1,    -1,    -1,    56,
     334,   335,   336,   337,   338,   339,   340,   341,   919,    -1,
      -1,    -1,    -1,    -1,    -1,   349,    -1,   351,  1032,  1033,
     354,    -1,   958,    -1,    -1,   961,    -1,   963,    -1,  1061,
      87,    -1,    -1,    90,    91,    -1,    -1,    94,  1075,    52,
      97,    54,    55,    -1,    -1,    58,   103,    -1,    -1,   383,
      -1,    -1,   125,   761,   762,    -1,    -1,   391,   392,    -1,
      13,    14,    -1,   990,    17,   992,    -1,    -1,   402,    -1,
     404,   998,    -1,    -1,   131,    -1,  1012,    90,  1014,    -1,
     414,  1017,   139,  1019,   418,    -1,    99,    -1,   422,    42,
      43,    -1,   800,    -1,    -1,    48,    49,   431,    51,    52,
      -1,    -1,    -1,    -1,    -1,    -1,   814,   815,   816,    -1,
      52,    -1,    54,    55,    -1,    -1,    58,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   459,    -1,    -1,    -1,  1065,
      52,    -1,    54,    55,   468,    57,    58,    -1,   472,    -1,
     848,   849,    -1,    -1,    -1,    67,    -1,  1083,    90,  1085,
      -1,    -1,  1088,    -1,  1090,    -1,   490,    99,   231,     0,
      -1,    -1,    -1,    -1,   872,    34,   874,     8,     9,    10,
    1106,    -1,    13,    14,    15,    -1,    17,    99,   100,   101,
     102,    -1,    -1,    34,   106,    26,    27,    56,    -1,    -1,
      -1,    -1,    -1,   901,    -1,    -1,    37,    38,    -1,    40,
      41,    42,    43,    44,   912,    56,    -1,   915,    -1,    -1,
      -1,    -1,    -1,   286,   287,    -1,    -1,    -1,    87,    -1,
     293,    90,   295,    92,    -1,    94,    -1,    -1,    97,    -1,
     303,    -1,    -1,    -1,   103,    -1,    87,    -1,    79,    90,
      -1,    -1,    -1,    94,    -1,    86,    97,    -1,     0,     1,
      91,    92,   103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   334,   131,   132,    -1,    -1,   339,    -1,   137,    -1,
      -1,    -1,    -1,   114,   115,   983,    -1,    -1,   351,    -1,
     131,   354,   990,    -1,   992,   619,   620,    -1,   129,    -1,
     998,   132,    -1,   134,    46,    34,   137,    -1,    -1,    -1,
     634,    -1,    -1,   144,    -1,   258,   259,   260,   261,   262,
      -1,    -1,    -1,    -1,  1022,    -1,    -1,    56,   391,   392,
      -1,   274,    -1,   276,   277,    -1,   660,    -1,    -1,    -1,
      -1,   404,   285,    -1,    -1,    -1,    -1,    52,    -1,    54,
      55,   414,    57,    58,    67,    -1,    -1,    -1,    87,   422,
      89,    90,    67,  1061,    -1,    94,    -1,    -1,    97,    -1,
     694,    84,    85,    -1,   103,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   125,    49,    -1,    -1,   100,    -1,   102,
      -1,   104,   105,    -1,   107,   108,   101,   102,   111,    -1,
     113,   106,   131,   132,    -1,   468,   349,    -1,   137,   472,
      -1,   354,   355,   356,   357,   358,   359,   360,   361,   362,
     363,   364,   365,   366,   367,   368,   369,   370,   371,   372,
     373,   374,   375,   376,   377,   378,   379,   380,   762,   763,
     383,    -1,   766,    -1,    -1,   769,   770,    -1,   391,   392,
      -1,   775,   776,    -1,    -1,    -1,    -1,    -1,    34,   402,
      -1,    -1,    -1,    -1,    -1,    67,    -1,    -1,    -1,     7,
       8,     9,    -1,   416,    -1,   418,   800,   420,   421,   803,
      56,   424,    84,    85,    -1,    -1,    -1,    -1,   431,   231,
     814,   815,   816,    -1,    -1,    -1,   439,   440,   100,   442,
     102,    -1,   104,   105,    -1,   107,   108,    34,    46,    -1,
      -1,    87,    -1,    -1,    90,    -1,    92,    -1,    94,    -1,
      -1,    97,   465,   466,   848,   849,    -1,   103,    -1,    56,
      -1,    -1,    -1,    -1,    -1,    -1,   479,    -1,   481,    -1,
      -1,    -1,    -1,    -1,   286,   287,    -1,   490,   872,    -1,
     874,   293,    -1,   295,    -1,   131,   132,   620,    34,    -1,
      87,   303,    -1,    90,    -1,    -1,    -1,    94,    -1,   893,
      97,   634,    -1,    -1,    -1,    -1,   103,   901,    -1,    -1,
      56,    -1,    -1,    -1,   111,   123,    -1,   125,    -1,    -1,
      -1,   915,   334,   258,   259,   260,   261,   339,    -1,    -1,
      -1,    -1,    -1,    -1,   131,   132,    -1,   550,    -1,   351,
     137,    87,   354,    -1,    90,    -1,    -1,    -1,    94,    -1,
      -1,    97,    34,    -1,    -1,    -1,    -1,   103,    -1,    -1,
      -1,   694,    -1,    -1,    -1,   111,    -1,    -1,    -1,    -1,
      52,    -1,    54,    55,    56,    -1,    58,    -1,    -1,   391,
     392,    -1,    -1,    -1,    -1,   131,   132,   195,    -1,   983,
      -1,   137,   404,    -1,    -1,    -1,   990,    -1,   992,    -1,
      -1,    -1,   414,    -1,   998,    87,    -1,    -1,    90,    -1,
     422,    -1,    94,    -1,   349,    97,    -1,    99,    -1,   354,
      -1,   103,    -1,    -1,    -1,   638,    -1,    -1,    -1,   762,
     763,    -1,    -1,    -1,    -1,    -1,   769,   770,    -1,    -1,
     248,   249,   775,   776,    -1,  1039,    -1,    -1,   661,   131,
     663,    -1,    -1,    67,    -1,    -1,   468,    -1,   671,   672,
     472,    -1,    -1,    -1,    -1,    -1,    -1,   800,    -1,    -1,
      84,    85,   280,    -1,   282,    -1,    -1,    -1,    -1,    -1,
      -1,   814,   815,   816,    -1,    -1,   100,   295,    -1,   424,
     104,   105,    -1,   107,   108,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   439,   440,    -1,   442,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   848,   849,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   335,   336,   337,
     338,    -1,   340,   341,   747,    -1,    -1,    -1,    -1,   872,
      -1,   874,    -1,   351,   479,   758,   481,    -1,    -1,    -1,
     763,   764,    -1,   766,    -1,    -1,   769,   770,    -1,    -1,
     893,    -1,   775,   776,    -1,    -1,    -1,    -1,   901,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   912,
      44,    -1,   915,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   806,   807,    -1,   809,   810,    -1,    -1,
     813,    -1,    -1,    67,    68,    69,    70,    71,   620,    73,
      74,    75,    76,    77,    78,   550,    80,    81,    -1,    -1,
      84,    85,   634,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   100,    -1,   102,    -1,
     104,   105,    -1,   107,   108,   109,   110,   111,    -1,   113,
     983,   459,    -1,    -1,   867,    -1,    -1,   990,    -1,   992,
      -1,    -1,    -1,   876,    -1,   998,    -1,    -1,    -1,   133,
      -1,   135,    -1,   886,    -1,    -1,    -1,    -1,    -1,    -1,
     893,    -1,   694,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   638,    -1,    -1,   919,    -1,    -1,    -1,
      -1,    -1,    -1,   926,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   661,    -1,   663,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   671,   672,    67,    68,
      69,    70,    71,    -1,    73,    74,    75,    -1,    77,    78,
     762,   763,    -1,    -1,    -1,    84,    85,   769,   770,    -1,
      -1,    -1,    -1,   775,   776,    -1,    -1,    -1,    -1,    -1,
      -1,   100,    -1,   102,    -1,   104,   105,    -1,   107,   108,
     109,   110,   111,    -1,   113,    -1,    -1,    -1,   800,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   814,   815,   816,    -1,    -1,    -1,    -1,    -1,
      -1,   619,   747,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   758,    -1,    -1,  1039,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   848,   849,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   660,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     872,    44,   874,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   813,    -1,
      -1,   893,    -1,    -1,    67,    68,    69,    70,    71,   901,
      73,    74,    75,    76,    77,    78,    -1,    80,    81,    -1,
      -1,    84,    85,   915,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   100,    -1,   102,
      -1,   104,   105,    -1,   107,   108,   109,   110,   111,    -1,
     113,    -1,   867,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   876,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     133,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   983,    -1,    -1,    -1,    -1,    -1,    -1,   990,    -1,
     992,    -1,    -1,    -1,    -1,    -1,   998,    -1,    -1,    -1,
      -1,   926,    -1,     0,     1,   803,     3,     4,     5,     6,
       7,    -1,    -1,    -1,    11,    12,    -1,    -1,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    45,    46,
      47,    48,    49,    50,    51,    52,    53,    54,    55,    56,
      -1,    58,    59,    60,    -1,    62,    63,    64,    65,    66,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      87,    -1,    -1,    90,    -1,    -1,    93,    94,    -1,    -1,
      97,    -1,    99,    -1,    -1,    -1,   103,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,
     117,   118,   119,   120,    -1,   122,   123,   124,   125,    -1,
      -1,    -1,    -1,   130,   131,   132,    -1,    -1,    -1,    -1,
     137,   138,    -1,   140,   141,   142,    -1,    -1,   145,   146,
       1,    -1,     3,     4,     5,     6,     7,     8,     9,    10,
      11,    12,    -1,    -1,    15,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    45,    46,    47,    48,    49,    50,
      51,    52,    53,    54,    55,    56,    -1,    58,    59,    60,
      -1,    62,    63,    64,    65,    66,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,    -1,    90,
      -1,    -1,    93,    94,    -1,    -1,    97,    -1,    99,    -1,
      -1,    -1,   103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   112,    -1,    -1,    -1,    -1,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,    -1,    -1,    -1,    -1,   130,
     131,   132,    -1,    -1,    -1,    -1,   137,   138,    -1,   140,
     141,   142,    -1,    -1,   145,   146,     1,    -1,     3,     4,
       5,     6,     7,    -1,    -1,    10,    11,    12,    -1,    14,
      15,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      45,    46,    47,    48,    49,    50,    51,    52,    53,    54,
      55,    56,    -1,    58,    59,    60,    -1,    62,    63,    64,
      65,    66,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    87,    -1,    -1,    90,    -1,    -1,    93,    94,
      -1,    -1,    97,    -1,    99,    -1,    -1,    -1,   103,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,
      -1,    -1,   117,   118,   119,   120,    -1,   122,   123,   124,
     125,    -1,    -1,    -1,    -1,   130,   131,   132,    -1,    -1,
      -1,    -1,   137,   138,    -1,   140,   141,   142,    -1,    -1,
     145,   146,     1,    -1,     3,     4,     5,     6,     7,    -1,
      -1,    10,    11,    12,    -1,    -1,    15,    16,    17,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    45,    46,    47,    48,
      49,    50,    51,    52,    53,    54,    55,    56,    -1,    58,
      59,    60,    -1,    62,    63,    64,    65,    66,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,
      -1,    90,    -1,    -1,    93,    94,    -1,    -1,    97,    -1,
      99,    -1,    -1,    -1,   103,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,   117,   118,
     119,   120,    -1,   122,   123,   124,   125,    -1,    -1,    -1,
      -1,   130,   131,   132,    -1,    -1,    -1,    -1,   137,   138,
      -1,   140,   141,   142,    -1,    -1,   145,   146,     1,    -1,
       3,     4,     5,     6,     7,    -1,    -1,    10,    11,    12,
      -1,    -1,    15,    16,    -1,    18,    19,    20,    21,    22,
      23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,
      -1,    -1,    45,    46,    47,    48,    49,    50,    51,    52,
      53,    54,    55,    56,    -1,    58,    59,    60,    -1,    62,
      63,    64,    65,    66,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    87,    -1,    -1,    90,    -1,    -1,
      93,    94,    -1,    -1,    97,    -1,    99,    -1,    -1,    -1,
     103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   112,
      -1,    -1,    -1,    -1,   117,   118,   119,   120,    -1,   122,
     123,   124,   125,    -1,    -1,    -1,    -1,   130,   131,   132,
      -1,    -1,    -1,    -1,   137,   138,    -1,   140,   141,   142,
      -1,    -1,   145,   146,     1,    -1,     3,     4,     5,     6,
       7,    -1,     9,    10,    11,    12,    -1,    -1,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    45,    46,
      47,    48,    49,    50,    51,    52,    53,    54,    55,    56,
      -1,    58,    59,    60,    -1,    62,    63,    64,    65,    66,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      87,    -1,    -1,    90,    -1,    -1,    93,    94,    -1,    -1,
      97,    -1,    99,    -1,    -1,    -1,   103,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,
     117,   118,   119,   120,    -1,   122,   123,   124,   125,    -1,
      -1,    -1,    -1,   130,   131,   132,    -1,    -1,    -1,    -1,
     137,   138,    -1,   140,   141,   142,    -1,    -1,   145,   146,
       1,    -1,     3,     4,     5,     6,     7,    -1,    -1,    -1,
      11,    12,    -1,    -1,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    45,    46,    47,    48,    49,    50,
      51,    52,    53,    54,    55,    56,    -1,    58,    59,    60,
      -1,    62,    63,    64,    65,    66,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,    -1,    90,
      -1,    92,    93,    94,    -1,    -1,    97,    -1,    99,    -1,
      -1,    -1,   103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   112,    -1,    -1,    -1,    -1,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,    -1,    -1,    -1,    -1,   130,
     131,   132,    -1,    -1,    -1,    -1,   137,   138,    -1,   140,
     141,   142,    -1,    -1,   145,   146,     1,    -1,     3,     4,
       5,     6,     7,    -1,    -1,    -1,    11,    12,    -1,    -1,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      45,    46,    47,    48,    49,    50,    51,    52,    53,    54,
      55,    56,    -1,    58,    59,    60,    -1,    62,    63,    64,
      65,    66,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    87,    -1,    -1,    90,    -1,    92,    93,    94,
      -1,    -1,    97,    -1,    99,    -1,    -1,    -1,   103,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,
      -1,    -1,   117,   118,   119,   120,    -1,   122,   123,   124,
     125,    -1,    -1,    -1,    -1,   130,   131,   132,    -1,    -1,
      -1,    -1,   137,   138,    -1,   140,   141,   142,    -1,    -1,
     145,   146,     1,    -1,     3,     4,     5,     6,     7,    -1,
      -1,    -1,    11,    12,    -1,    -1,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    45,    46,    47,    48,
      49,    50,    51,    52,    53,    54,    55,    56,    -1,    58,
      59,    60,    -1,    62,    63,    64,    65,    66,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,
      -1,    90,    -1,    -1,    93,    94,    -1,    -1,    97,    -1,
      99,    -1,    -1,    -1,   103,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   112,    -1,    -1,   115,    -1,   117,   118,
     119,   120,    -1,   122,   123,   124,   125,    -1,    -1,    -1,
      -1,   130,   131,   132,    -1,    -1,    -1,    -1,   137,   138,
      -1,   140,   141,   142,    -1,    -1,   145,   146,     1,    -1,
       3,     4,     5,     6,     7,    -1,    -1,    -1,    11,    12,
      -1,    -1,    -1,    16,    -1,    18,    19,    20,    21,    22,
      23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,
      -1,    -1,    45,    46,    47,    48,    49,    50,    51,    52,
      53,    54,    55,    56,    -1,    58,    59,    60,    -1,    62,
      63,    64,    65,    66,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    87,    -1,    -1,    90,    -1,    -1,
      93,    94,    -1,    -1,    97,    -1,    99,    -1,    -1,    -1,
     103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   112,
      -1,    -1,   115,    -1,   117,   118,   119,   120,    -1,   122,
     123,   124,   125,    -1,    -1,    -1,    -1,   130,   131,   132,
      -1,    -1,    -1,    -1,   137,   138,    -1,   140,   141,   142,
      -1,    -1,   145,   146,     1,    -1,     3,     4,     5,     6,
       7,    -1,    -1,    -1,    11,    12,    -1,    -1,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    45,    46,
      47,    48,    49,    50,    51,    52,    53,    54,    55,    56,
      -1,    58,    59,    60,    -1,    62,    63,    64,    65,    66,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      87,    -1,    -1,    90,    -1,    -1,    93,    94,    -1,    -1,
      97,    -1,    99,    -1,    -1,    -1,   103,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,
     117,   118,   119,   120,    -1,   122,   123,   124,   125,    -1,
      -1,    -1,   129,   130,   131,   132,    -1,    -1,    -1,    -1,
     137,   138,    -1,   140,   141,   142,    -1,    -1,   145,   146,
       1,    -1,     3,     4,     5,     6,     7,    -1,    -1,    10,
      11,    12,    -1,    -1,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    45,    46,    47,    48,    49,    50,
      51,    52,    53,    54,    55,    56,    -1,    58,    59,    60,
      -1,    62,    63,    64,    65,    66,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,    -1,    90,
      -1,    -1,    93,    94,    -1,    -1,    97,    -1,    99,    -1,
      -1,    -1,   103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   112,    -1,    -1,    -1,    -1,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,    -1,    -1,    -1,    -1,   130,
     131,   132,    -1,    -1,    -1,    -1,   137,   138,     0,   140,
     141,   142,    -1,    -1,   145,   146,     8,     9,    10,    -1,
      -1,    13,    14,    15,    -1,    17,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    26,    27,    28,    29,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    37,    38,    -1,    40,    41,
      42,    43,    44,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    67,    68,    69,    70,    71,
      72,    73,    74,    75,    76,    77,    78,    79,    80,    81,
      -1,    -1,    84,    85,    86,    -1,    88,    89,    -1,    -1,
      92,    -1,    -1,    95,    96,    -1,    98,    -1,   100,    -1,
     102,    -1,   104,   105,    -1,   107,   108,   109,   110,   111,
      -1,   113,   114,   115,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   129,    -1,    -1,
     132,   133,   134,   135,    -1,   137,     0,   139,    -1,    -1,
      -1,    -1,   144,    -1,     8,     9,    10,    -1,    -1,    13,
      14,    15,    -1,    17,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    25,    -1,    27,    28,    29,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    37,    38,    -1,    40,    41,    42,    43,
      44,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    67,    68,    69,    70,    71,    72,    73,
      74,    75,    76,    77,    78,    79,    80,    81,    -1,    -1,
      84,    85,    86,    -1,    88,    89,    -1,    -1,    92,    -1,
      -1,    95,    96,    -1,    98,    -1,   100,    -1,   102,    -1,
     104,   105,    -1,   107,   108,   109,   110,   111,    -1,   113,
      -1,   115,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   129,    -1,    -1,   132,   133,
     134,   135,    -1,   137,     0,   139,    -1,    -1,    -1,    -1,
     144,    -1,     8,     9,    10,    -1,    -1,    13,    14,    15,
      -1,    17,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    25,
      -1,    27,    28,    29,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    37,    38,    -1,    40,    41,    42,    43,    44,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    67,    68,    69,    70,    71,    72,    73,    74,    75,
      76,    77,    78,    79,    80,    81,    -1,    -1,    84,    85,
      86,    -1,    88,    89,    -1,    -1,    92,    -1,    -1,    95,
      96,    -1,    98,    -1,   100,    -1,   102,    -1,   104,   105,
      -1,   107,   108,   109,   110,   111,    -1,   113,    -1,   115,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   129,    -1,    -1,   132,   133,   134,   135,
      -1,   137,     0,   139,    -1,    -1,    -1,    -1,   144,    -1,
       8,     9,    10,    -1,    -1,    13,    14,    15,    -1,    17,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    26,    27,
      28,    29,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    37,
      38,    -1,    40,    41,    42,    43,    44,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    67,
      68,    69,    70,    71,    -1,    73,    74,    75,    76,    77,
      78,    79,    80,    81,    -1,    -1,    84,    85,    86,    -1,
      -1,    89,    -1,    -1,    92,    -1,    -1,    95,    96,    -1,
      98,    -1,   100,    -1,   102,    -1,   104,   105,    -1,   107,
     108,   109,   110,   111,    -1,   113,   114,   115,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   129,    -1,    -1,   132,   133,   134,   135,    -1,   137,
       0,   139,    -1,    -1,    -1,    -1,   144,    -1,     8,     9,
      10,    -1,    -1,    13,    14,    15,    -1,    17,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    26,    27,    28,    29,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    37,    38,    -1,
      40,    41,    42,    43,    44,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    67,    68,    69,
      70,    71,    -1,    73,    74,    75,    76,    77,    78,    79,
      80,    81,    -1,    -1,    84,    85,    86,    -1,    -1,    89,
      -1,    -1,    92,    -1,    -1,    95,    96,    -1,    98,    -1,
     100,    -1,   102,    -1,   104,   105,    -1,   107,   108,   109,
     110,   111,    -1,   113,   114,   115,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   129,
      -1,    -1,   132,   133,   134,   135,    -1,   137,     0,   139,
      -1,    -1,    -1,    -1,   144,    -1,     8,     9,    10,    -1,
      -1,    13,    14,    15,    -1,    17,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    27,    28,    29,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    37,    38,    -1,    40,    41,
      42,    43,    44,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    67,    68,    69,    70,    71,
      72,    73,    74,    75,    76,    77,    78,    79,    80,    81,
      -1,    -1,    84,    85,    86,    -1,    88,    89,    -1,    -1,
      92,    -1,    -1,    95,    96,    -1,    98,    -1,   100,    -1,
     102,    -1,   104,   105,    -1,   107,   108,   109,   110,   111,
      -1,   113,    -1,   115,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   129,    -1,    -1,
     132,   133,   134,   135,    -1,   137,     0,   139,    -1,    -1,
      -1,    -1,   144,    -1,     8,     9,    10,    -1,    -1,    13,
      14,    15,    -1,    17,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    26,    27,    28,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    37,    38,    -1,    40,    41,    42,    43,
      44,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    67,    68,    69,    70,    71,    -1,    73,
      74,    75,    76,    77,    78,    79,    80,    81,    -1,    -1,
      84,    85,    86,    -1,    -1,    89,    -1,    91,    92,    -1,
      -1,    95,    96,    -1,    98,    -1,   100,    -1,   102,    -1,
     104,   105,    -1,   107,   108,   109,   110,   111,    -1,   113,
     114,   115,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   129,    -1,    -1,   132,   133,
     134,   135,    -1,   137,     0,    -1,    -1,    -1,    -1,    -1,
     144,    -1,     8,     9,    10,    -1,    -1,    13,    14,    15,
      -1,    17,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      26,    27,    28,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    37,    38,    -1,    40,    41,    42,    43,    44,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    67,    68,    69,    70,    71,    -1,    73,    74,    75,
      76,    77,    78,    79,    80,    81,    -1,    -1,    84,    85,
      86,    -1,    -1,    89,    -1,    91,    92,    -1,    -1,    95,
      96,    -1,    98,    -1,   100,    -1,   102,    -1,   104,   105,
      -1,   107,   108,   109,   110,   111,    -1,   113,   114,   115,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   129,    -1,    -1,   132,   133,   134,   135,
      -1,   137,     0,    -1,    -1,    -1,    -1,    -1,   144,    -1,
       8,     9,    10,    -1,    -1,    13,    14,    15,    -1,    17,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    27,
      28,    29,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    37,
      38,    -1,    40,    41,    42,    43,    44,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    67,
      68,    69,    70,    71,    -1,    73,    74,    75,    76,    77,
      78,    79,    80,    81,    -1,    -1,    84,    85,    86,    -1,
      -1,    89,    -1,    -1,    92,    -1,    -1,    95,    96,    -1,
      98,    -1,   100,    -1,   102,    -1,   104,   105,    -1,   107,
     108,   109,   110,   111,    -1,   113,    -1,   115,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   129,    -1,    -1,   132,   133,   134,   135,    -1,   137,
       0,   139,    -1,    -1,    -1,    -1,   144,    -1,     8,     9,
      10,    -1,    -1,    13,    14,    15,    -1,    17,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    27,    28,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    37,    38,    -1,
      40,    41,    42,    43,    44,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    67,    68,    69,
      70,    71,    -1,    73,    74,    75,    76,    77,    78,    79,
      80,    81,    -1,    -1,    84,    85,    86,    -1,    -1,    89,
      -1,    91,    92,    -1,    -1,    95,    96,    -1,    98,    -1,
     100,    -1,   102,    -1,   104,   105,    -1,   107,   108,   109,
     110,   111,    -1,   113,    -1,   115,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   129,
      -1,     0,   132,   133,   134,   135,    -1,   137,    -1,     8,
       9,    10,    -1,    -1,   144,    14,    15,    -1,    17,    67,
      68,    69,    70,    71,    -1,    73,    74,    26,    -1,    77,
      78,    -1,    -1,    -1,    -1,    -1,    84,    85,    37,    38,
      -1,    40,    41,    42,    43,    44,    -1,    -1,    -1,    -1,
      -1,    -1,   100,    -1,   102,    -1,   104,   105,    -1,   107,
     108,   109,   110,   111,    -1,   113,    -1,    -1,    67,    68,
      69,    70,    71,    72,    73,    74,    75,    76,    77,    78,
      79,    80,    81,    -1,    -1,    84,    85,    86,    -1,    88,
      -1,    -1,    -1,    92,    -1,    -1,    95,    -1,    -1,    -1,
      -1,   100,    -1,   102,    -1,   104,   105,    -1,   107,   108,
     109,   110,   111,    -1,   113,   114,   115,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     129,    -1,     0,   132,   133,   134,   135,    -1,   137,    -1,
       8,     9,    10,    -1,    -1,   144,    14,    15,    -1,    17,
      67,    68,    69,    70,    71,    -1,    73,    74,    26,    -1,
      77,    78,    -1,    -1,    -1,    -1,    -1,    84,    85,    37,
      38,    -1,    40,    41,    42,    43,    44,    -1,    -1,    -1,
      -1,    -1,    -1,   100,    -1,   102,    -1,   104,   105,    -1,
     107,   108,   109,   110,   111,    -1,   113,    -1,    -1,    67,
      68,    69,    70,    71,    72,    73,    74,    75,    76,    77,
      78,    79,    80,    81,    -1,    -1,    84,    85,    86,    -1,
      88,    -1,    -1,    -1,    92,    -1,    -1,    95,    -1,    -1,
      -1,    -1,   100,    -1,   102,    -1,   104,   105,    -1,   107,
     108,   109,   110,   111,    -1,   113,   114,   115,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   129,    -1,     0,   132,   133,   134,   135,    -1,   137,
      -1,     8,     9,    10,    -1,    -1,   144,    14,    15,    -1,
      17,    67,    68,    69,    70,    71,    -1,    73,    74,    75,
      76,    77,    78,    -1,    80,    81,    -1,    -1,    84,    85,
      37,    38,    -1,    40,    41,    42,    43,    44,    -1,    -1,
      -1,    -1,    -1,    -1,   100,    -1,   102,    -1,   104,   105,
      -1,   107,   108,   109,   110,   111,    -1,   113,    -1,    -1,
      67,    68,    69,    70,    71,    72,    73,    74,    75,    76,
      77,    78,    79,    80,    81,    -1,    -1,    84,    85,    86,
      -1,    88,    -1,    -1,    -1,    92,    -1,    -1,    95,    -1,
      -1,    -1,    -1,   100,    -1,   102,    -1,   104,   105,    -1,
     107,   108,   109,   110,   111,    -1,   113,    -1,   115,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   129,    -1,     0,   132,   133,   134,   135,    -1,
     137,    -1,     8,     9,    10,    -1,    -1,   144,    14,    15,
      -1,    17,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    37,    38,    -1,    40,    41,    42,    43,    44,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    67,    68,    69,    70,    71,    72,    73,    74,    75,
      76,    77,    78,    79,    80,    81,    -1,    -1,    84,    85,
      86,    -1,    88,    -1,    -1,    -1,    92,    -1,    -1,    95,
      -1,    -1,    -1,    -1,   100,    -1,   102,    -1,   104,   105,
      -1,   107,   108,   109,   110,   111,    -1,   113,    -1,   115,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   129,    -1,    -1,   132,   133,   134,   135,
      -1,   137,     3,     4,     5,     6,     7,    -1,   144,    -1,
      11,    12,    -1,    -1,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    45,    46,    47,    48,    49,    50,
      51,    52,    53,    54,    55,    56,    -1,    58,    59,    60,
      -1,    62,    63,    64,    65,    66,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,    -1,    90,
      -1,    -1,    93,    94,    -1,    -1,    97,    -1,    99,    -1,
      -1,    -1,   103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   112,    -1,    -1,    -1,    -1,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,    -1,    -1,    -1,    -1,   130,
     131,    -1,    -1,    -1,    -1,    -1,   137,   138,    -1,   140,
     141,   142,    -1,    -1,   145,   146,     3,     4,     5,     6,
       7,    -1,    -1,    -1,    11,    12,    -1,    -1,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    45,    46,
      47,    48,    49,    50,    51,    52,    53,    54,    55,    56,
      -1,    58,    59,    60,    -1,    62,    63,    64,    65,    66,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      87,    -1,    -1,    90,    -1,    -1,    93,    94,    -1,    -1,
      97,    -1,    99,    -1,    -1,    -1,   103,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,
     117,   118,   119,   120,    -1,   122,   123,   124,   125,    -1,
      -1,    -1,    -1,   130,   131,    -1,    -1,    -1,    -1,    -1,
     137,   138,    -1,   140,   141,   142,    -1,    -1,   145,   146,
       3,     4,     5,     6,     7,    -1,    -1,    -1,    11,    12,
      -1,    -1,    -1,    16,    -1,    18,    19,    20,    21,    22,
      23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,
      -1,    -1,    45,    46,    47,    48,    49,    50,    51,    52,
      53,    54,    55,    56,    -1,    58,    59,    60,    -1,    62,
      63,    64,    65,    66,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    87,    -1,    -1,    90,    -1,    -1,
      93,    94,    -1,    -1,    97,    -1,    99,    -1,    -1,    -1,
     103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   112,
      -1,    -1,    -1,    -1,   117,   118,   119,   120,    -1,   122,
     123,   124,   125,    -1,    -1,    -1,    -1,   130,   131,    -1,
      -1,    -1,     3,     4,     5,   138,     7,   140,   141,   142,
      11,    12,   145,   146,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,
      51,    52,    53,    54,    55,    56,    57,    58,    59,    60,
      -1,    62,    63,    64,    65,    66,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,    -1,    90,
      -1,    -1,    93,    94,    -1,    -1,    97,    -1,    99,    -1,
     101,    -1,   103,    -1,    -1,   106,    -1,    -1,    -1,    -1,
      -1,   112,    -1,    -1,    -1,    -1,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,    -1,    -1,    -1,    -1,   130,
     131,   132,    -1,    -1,     3,     4,     5,   138,     7,   140,
     141,   142,    11,    12,   145,   146,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,
      49,    50,    51,    52,    53,    54,    55,    56,    57,    58,
      59,    60,    -1,    62,    63,    64,    65,    66,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,
      -1,    90,    -1,    -1,    93,    94,    -1,    -1,    97,    -1,
      99,    -1,   101,    -1,   103,    -1,    -1,   106,    -1,    -1,
      -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,   117,   118,
     119,   120,    -1,   122,   123,   124,   125,    -1,    -1,    -1,
      -1,   130,   131,    -1,    -1,    -1,    -1,    -1,    -1,   138,
      -1,   140,   141,   142,    -1,    -1,   145,   146,     3,     4,
       5,     6,     7,    -1,    -1,    -1,    11,    12,    -1,    -1,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      45,    46,    -1,    48,    49,    50,    51,    52,    53,    54,
      55,    56,    -1,    58,    59,    60,    -1,    62,    63,    64,
      65,    66,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    87,    -1,    -1,    90,    -1,    -1,    93,    94,
      -1,    -1,    97,    -1,    99,    -1,    -1,    -1,   103,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,
      -1,    -1,   117,   118,   119,   120,    -1,   122,   123,   124,
     125,    -1,    -1,    -1,    -1,   130,   131,    -1,    -1,    -1,
       3,     4,     5,   138,     7,   140,   141,   142,    11,    12,
     145,   146,    -1,    16,    -1,    18,    19,    20,    21,    22,
      23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,
      -1,    -1,    -1,    46,    -1,    -1,    49,    50,    51,    52,
      53,    54,    55,    56,    57,    58,    59,    60,    -1,    62,
      63,    64,    65,    66,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    87,    -1,    -1,    90,    -1,    -1,
      93,    94,    -1,    -1,    97,    -1,    99,    -1,   101,    -1,
     103,    -1,    -1,   106,    -1,    -1,    -1,    -1,    -1,   112,
      -1,    -1,    -1,    -1,   117,   118,   119,   120,    -1,   122,
     123,   124,   125,    -1,    -1,    -1,    -1,   130,   131,    -1,
      -1,    -1,     3,     4,     5,   138,     7,   140,   141,   142,
      11,    12,   145,   146,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,
      51,    52,    53,    54,    55,    56,    57,    58,    59,    60,
      -1,    62,    63,    64,    65,    66,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,    -1,    90,
      -1,    -1,    93,    94,    -1,    -1,    97,    -1,    99,    -1,
      -1,    -1,   103,    -1,    -1,   106,    -1,    -1,    -1,    -1,
      -1,   112,    -1,    -1,    -1,    -1,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,    -1,    -1,    -1,    -1,   130,
     131,    -1,    -1,    -1,     3,     4,     5,   138,     7,   140,
     141,   142,    11,    12,   145,   146,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,
      49,    50,    51,    52,    53,    54,    55,    56,    57,    58,
      59,    60,    -1,    62,    63,    64,    65,    66,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,
      -1,    90,    -1,    -1,    93,    94,    -1,    -1,    97,    -1,
      -1,    -1,   101,    -1,   103,    -1,    -1,   106,    -1,    -1,
      -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,   117,   118,
     119,   120,    -1,   122,   123,   124,   125,    -1,    -1,    -1,
      -1,   130,   131,    -1,    -1,    -1,     3,     4,     5,   138,
       7,   140,   141,   142,    11,    12,   145,   146,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    -1,    46,
      -1,    -1,    49,    50,    51,    52,    53,    54,    55,    56,
      57,    58,    59,    60,    -1,    62,    63,    64,    65,    66,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      87,    -1,    -1,    90,    -1,    -1,    93,    94,    -1,    -1,
      97,    -1,    99,    -1,    -1,    -1,   103,    -1,    -1,   106,
      -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,
     117,   118,   119,   120,    -1,   122,   123,   124,   125,    -1,
      -1,    -1,    -1,   130,   131,    -1,    -1,    -1,     3,     4,
       5,   138,     7,   140,   141,   142,    11,    12,   145,   146,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      -1,    46,    -1,    -1,    49,    50,    51,    52,    53,    54,
      55,    56,    -1,    58,    59,    60,    -1,    62,    63,    64,
      65,    66,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    87,    -1,    -1,    90,    -1,    -1,    93,    94,
      -1,    -1,    97,    -1,    -1,    -1,    -1,    -1,   103,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,
      -1,    -1,   117,   118,   119,   120,    -1,   122,   123,   124,
     125,    -1,    -1,    -1,    -1,   130,   131,   132,    -1,    -1,
      -1,    -1,   137,   138,    -1,   140,   141,   142,    -1,    -1,
     145,   146,     3,     4,     5,    -1,     7,    -1,    -1,    -1,
      11,    12,    -1,    -1,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,
      51,    52,    53,    54,    55,    56,    57,    58,    59,    60,
      -1,    62,    63,    64,    65,    66,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,    -1,    90,
      -1,    -1,    93,    94,    -1,    -1,    97,    -1,    -1,    -1,
      -1,    -1,   103,    -1,    -1,   106,    -1,    -1,    -1,    -1,
      -1,   112,    -1,    -1,    -1,    -1,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,    -1,    -1,    -1,    -1,   130,
     131,    -1,    -1,    -1,     3,     4,     5,   138,     7,   140,
     141,   142,    11,    12,   145,   146,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,
      49,    50,    51,    52,    53,    54,    55,    56,    -1,    58,
      59,    60,    -1,    62,    63,    64,    65,    66,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,
      -1,    90,    -1,    -1,    93,    94,    -1,    -1,    97,    -1,
      -1,    -1,    -1,    -1,   103,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,   117,   118,
     119,   120,    -1,   122,   123,   124,   125,    -1,    -1,    -1,
      -1,   130,   131,   132,    -1,    -1,     3,     4,     5,   138,
       7,   140,   141,   142,    11,    12,   145,   146,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    -1,    46,
      -1,    -1,    49,    50,    51,    52,    53,    54,    55,    56,
      -1,    58,    59,    60,    -1,    62,    63,    64,    65,    66,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      87,    -1,    -1,    90,    91,    -1,    93,    94,    -1,    -1,
      97,    -1,    -1,    -1,    -1,    -1,   103,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,
     117,   118,   119,   120,    -1,   122,   123,   124,   125,    -1,
      -1,    -1,    -1,   130,   131,    -1,    -1,    -1,     3,     4,
       5,   138,     7,   140,   141,   142,    11,    12,   145,   146,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      -1,    46,    -1,    -1,    49,    50,    51,    52,    53,    54,
      55,    56,    -1,    58,    59,    60,    -1,    62,    63,    64,
      65,    66,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    87,    -1,    -1,    90,    -1,    -1,    93,    94,
      -1,    -1,    97,    -1,    99,    -1,    -1,    -1,   103,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,
      -1,    -1,   117,   118,   119,   120,    -1,   122,   123,   124,
     125,    -1,    -1,    -1,    -1,   130,   131,    -1,    -1,    -1,
       3,     4,     5,   138,     7,   140,   141,   142,    11,    12,
     145,   146,    -1,    16,    -1,    18,    19,    20,    21,    22,
      23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,
      -1,    -1,    -1,    46,    -1,    -1,    49,    50,    51,    52,
      53,    54,    55,    56,    -1,    58,    59,    60,    -1,    62,
      63,    64,    65,    66,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    87,    -1,    -1,    90,    -1,    -1,
      93,    94,    -1,    -1,    97,    -1,    99,    -1,    -1,    -1,
     103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   112,
      -1,    -1,    -1,    -1,   117,   118,   119,   120,    -1,   122,
     123,   124,   125,    -1,    -1,    -1,    -1,   130,   131,    -1,
      -1,    -1,     3,     4,     5,   138,     7,   140,   141,   142,
      11,    12,   145,   146,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,
      51,    52,    53,    54,    55,    56,    -1,    58,    59,    60,
      -1,    62,    63,    64,    65,    66,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,    -1,    90,
      -1,    -1,    93,    94,    -1,    -1,    97,    -1,    99,    -1,
      -1,    -1,   103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   112,    -1,    -1,    -1,    -1,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,    -1,    -1,    -1,    -1,   130,
     131,    -1,    -1,    -1,     3,     4,     5,   138,     7,   140,
     141,   142,    11,    12,   145,   146,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,
      49,    50,    51,    52,    53,    54,    55,    56,    -1,    58,
      59,    60,    -1,    62,    63,    64,    65,    66,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,
      -1,    90,    -1,    -1,    93,    94,    -1,    -1,    97,    -1,
      99,    -1,    -1,    -1,   103,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,   117,   118,
     119,   120,    -1,   122,   123,   124,   125,    -1,    -1,    -1,
      -1,   130,   131,    -1,    -1,    -1,     3,     4,     5,   138,
       7,   140,   141,   142,    11,    12,   145,   146,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    -1,    46,
      -1,    -1,    49,    50,    51,    52,    53,    54,    55,    56,
      -1,    58,    59,    60,    -1,    62,    63,    64,    65,    66,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      87,    -1,    -1,    90,    -1,    -1,    93,    94,    -1,    -1,
      97,    -1,    99,    -1,    -1,    -1,   103,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,    -1,    -1,
     117,   118,   119,   120,    -1,   122,   123,   124,   125,    -1,
      -1,    -1,    -1,   130,   131,    -1,    -1,    -1,     3,     4,
       5,   138,     7,   140,   141,   142,    11,    12,   145,   146,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      -1,    46,    -1,    -1,    49,    50,    51,    52,    53,    54,
      55,    56,    -1,    58,    59,    60,    -1,    62,    63,    64,
      65,    66,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    87,    -1,    -1,    90,    -1,    -1,    93,    94,
      -1,    -1,    97,    -1,    -1,    -1,    -1,    -1,   103,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   112,    -1,    -1,
      -1,    -1,   117,   118,   119,   120,    -1,   122,   123,   124,
     125,    -1,    -1,    -1,    -1,   130,   131,    -1,    -1,    -1,
       3,     4,     5,   138,     7,   140,   141,   142,    11,    12,
     145,   146,    -1,    16,    -1,    18,    19,    20,    21,    22,
      23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,
      -1,    -1,    -1,    46,    -1,    -1,    49,    50,    51,    52,
      53,    54,    55,    56,    -1,    58,    59,    60,    -1,    62,
      63,    64,    65,    66,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    87,    -1,    -1,    90,    -1,    -1,
      93,    94,    -1,    -1,    97,    -1,    -1,    -1,    -1,    -1,
     103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   112,
      -1,    -1,    -1,    -1,   117,   118,   119,   120,    -1,   122,
     123,   124,   125,    -1,    -1,    -1,    -1,   130,   131,    -1,
      -1,    -1,     3,     4,     5,   138,     7,   140,   141,   142,
      11,    12,   145,   146,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,
      51,    52,    53,    54,    55,    56,    -1,    58,    59,    60,
      -1,    62,    63,    64,    65,    66,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,    -1,    90,
      -1,    -1,    93,    94,    -1,    -1,    97,    -1,    -1,    -1,
      -1,    -1,   103,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   112,    -1,    -1,    -1,    -1,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,    -1,    -1,    -1,    -1,   130,
     131,    -1,    -1,    -1,     3,     4,     5,   138,     7,   140,
     141,   142,    11,    12,   145,   146,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,
      49,    50,    51,    52,    53,    54,    55,    56,    -1,    58,
      59,    60,    -1,    62,    63,    -1,    -1,    66,    -1,    67,
      68,    69,    70,    71,    -1,    73,    74,    75,    76,    77,
      78,    -1,    80,    81,    -1,    84,    84,    85,    87,    -1,
      -1,    90,    -1,    -1,    93,    94,    -1,    -1,    97,    -1,
      -1,    -1,   100,    -1,   102,    -1,   104,   105,    -1,   107,
     108,   109,   110,   111,    -1,   113,    -1,    -1,   117,   118,
     119,   120,    -1,   122,   123,   124,   125,    -1,    -1,    -1,
      -1,   130,   131,    -1,   132,   133,     3,     4,     5,   138,
       7,   140,   141,   142,    11,    12,   145,   146,    -1,    16,
      -1,    18,    19,    20,    21,    22,    23,    24,    -1,    -1,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,    -1,    46,
      -1,    -1,    49,    50,    51,    52,    53,    54,    55,    56,
      -1,    58,    59,    60,    -1,    62,    63,    -1,    -1,    66,
      -1,    67,    68,    69,    70,    71,    -1,    73,    74,    75,
      76,    77,    78,    -1,    80,    81,    -1,    -1,    84,    85,
      87,    -1,    -1,    90,    -1,    -1,    93,    94,    -1,    -1,
      97,    -1,    99,    -1,   100,    -1,   102,    -1,   104,   105,
      -1,   107,   108,   109,   110,   111,    -1,   113,    -1,    -1,
     117,   118,   119,   120,    -1,   122,   123,   124,   125,    -1,
      -1,    -1,    -1,   130,   131,    -1,    -1,   133,     3,     4,
       5,   138,     7,   140,   141,   142,    11,    12,   145,   146,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      -1,    46,    -1,    -1,    49,    50,    51,    52,    53,    54,
      55,    56,    -1,    58,    59,    60,    -1,    62,    63,    -1,
      -1,    66,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    87,    -1,    -1,    90,    -1,    -1,    93,    94,
      -1,    -1,    97,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   117,   118,   119,   120,    -1,   122,   123,   124,
     125,    -1,    -1,    -1,    -1,   130,   131,    -1,    -1,    -1,
     135,    -1,    -1,   138,    -1,   140,   141,   142,    -1,    -1,
     145,   146,     3,     4,     5,    -1,     7,    -1,    -1,    -1,
      11,    12,    -1,    -1,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,
      51,    52,    53,    54,    55,    56,    -1,    58,    59,    60,
      -1,    62,    63,    -1,    -1,    66,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,    -1,    90,
      -1,    -1,    93,    94,    -1,    -1,    97,    -1,    99,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,    -1,    -1,    -1,    -1,   130,
     131,    -1,    -1,    -1,     3,     4,     5,   138,     7,   140,
     141,   142,    11,    12,   145,   146,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,
      49,    50,    51,    52,    53,    54,    55,    56,    -1,    58,
      59,    60,    -1,    62,    63,    -1,    -1,    66,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,
      -1,    90,    -1,    -1,    93,    94,    -1,    -1,    97,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   117,   118,
     119,   120,    -1,   122,   123,   124,   125,    -1,    -1,    -1,
      -1,   130,   131,    -1,    -1,    -1,   135,    -1,    -1,   138,
      -1,   140,   141,   142,    -1,    -1,   145,   146,     3,     4,
       5,    -1,     7,    -1,    -1,    -1,    11,    12,    -1,    -1,
      -1,    16,    -1,    18,    19,    20,    21,    22,    23,    24,
      -1,    -1,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    -1,    -1,    39,    -1,    -1,    -1,    -1,    -1,
      -1,    46,    -1,    -1,    49,    50,    51,    52,    53,    54,
      55,    56,    -1,    58,    59,    60,    -1,    62,    63,    -1,
      -1,    66,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    87,    -1,    -1,    90,    -1,    -1,    93,    94,
      -1,    -1,    97,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   117,   118,   119,   120,    -1,   122,   123,   124,
     125,    -1,    -1,    -1,    -1,   130,   131,    -1,    -1,    -1,
       3,     4,     5,   138,     7,   140,   141,   142,    11,    12,
     145,   146,    -1,    16,    -1,    18,    19,    20,    21,    22,
      23,    24,    -1,    -1,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    -1,    -1,    39,    -1,    -1,    -1,
      -1,    -1,    -1,    46,    -1,    -1,    49,    50,    51,    52,
      53,    54,    55,    56,    -1,    58,    59,    60,    -1,    62,
      63,    -1,    -1,    66,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    87,    -1,    -1,    90,    -1,    -1,
      93,    94,    -1,    -1,    97,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   117,   118,   119,   120,    -1,   122,
     123,   124,   125,    -1,    -1,    -1,    -1,   130,   131,    -1,
      -1,    -1,     3,     4,     5,   138,     7,   140,   141,   142,
      11,    12,   145,   146,    -1,    16,    -1,    18,    19,    20,
      21,    22,    23,    24,    -1,    -1,    -1,    -1,    -1,    30,
      31,    32,    33,    34,    35,    36,    -1,    -1,    39,    -1,
      -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,    49,    50,
      51,    52,    53,    54,    55,    56,    -1,    58,    59,    60,
      -1,    62,    63,    -1,    -1,    66,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,    -1,    90,
      -1,    -1,    93,    94,    -1,    -1,    97,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,    -1,    -1,    -1,    -1,   130,
     131,    -1,    -1,    -1,     3,     4,     5,   138,     7,   140,
     141,   142,    11,    12,   145,   146,    -1,    16,    -1,    18,
      19,    20,    21,    22,    23,    24,    -1,    -1,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    -1,    -1,
      39,    -1,    -1,    -1,    -1,    -1,    -1,    46,    -1,    -1,
      49,    50,    51,    52,    53,    54,    55,    56,    -1,    58,
      59,    60,    -1,    62,    63,    -1,    -1,    66,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    87,    -1,
      -1,    90,    -1,    -1,    93,    94,    -1,    -1,    97,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   117,   118,
     119,   120,    -1,   122,   123,   124,   125,    -1,    -1,    -1,
      -1,   130,   131,    -1,    -1,    -1,    -1,    -1,    -1,   138,
      -1,   140,   141,   142,    -1,    -1,   145,   146,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
      25,    26,    -1,    -1,    -1,    30,    31,    32,    33,    34,
      35,    36,    37,    38,    39,    -1,    -1,    -1,    -1,    -1,
      45,    46,    47,    48,    49,    50,    51,    52,    53,    54,
      -1,    56,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    64,
      65,    -1,    67,    68,    69,    70,    71,    -1,    73,    74,
      -1,    -1,    77,    78,    -1,    -1,    -1,    82,    83,    84,
      85,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    99,   100,    -1,   102,   103,   104,
     105,   106,   107,   108,   109,   110,   111,   112,   113,    -1,
      -1,   116,   117,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   131,     3,     4,     5,
       6,     7,     8,     9,    10,    11,    12,    13,    14,    15,
      16,    17,    18,    19,    20,    21,    22,    23,    24,    25,
      26,    -1,    -1,    -1,    30,    31,    32,    33,    34,    35,
      36,    37,    38,    39,    -1,    -1,    -1,    -1,    -1,    45,
      46,    47,    48,    49,    50,    51,    52,    53,    -1,    -1,
      56,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    64,    65,
      -1,    67,    68,    69,    70,    71,    -1,    73,    74,    -1,
      -1,    77,    78,    -1,    -1,    -1,    82,    83,    84,    85,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    99,   100,    -1,   102,   103,   104,   105,
     106,   107,   108,   109,   110,   111,   112,   113,    -1,    -1,
     116,   117,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   131,     3,     4,     5,     6,
       7,     8,     9,    10,    11,    12,    13,    14,    15,    16,
      17,    18,    19,    20,    21,    22,    23,    24,    25,    26,
      -1,    -1,    -1,    30,    31,    32,    33,    34,    35,    36,
      37,    38,    39,    -1,    -1,    -1,    -1,    -1,    45,    46,
      47,    48,    49,    50,    51,    52,    53,    54,    55,    56,
      -1,    58,    -1,    -1,    -1,    -1,    -1,    64,    65,    -1,
      67,    68,    69,    70,    71,    -1,    73,    74,    -1,    -1,
      77,    78,    -1,    -1,    -1,    82,    83,    84,    85,    -1,
      -1,    -1,    -1,    -1,    91,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    99,   100,    -1,   102,   103,   104,   105,   106,
     107,   108,   109,   110,   111,   112,   113,    -1,    -1,   116,
       3,     4,     5,     6,     7,     8,     9,    10,    11,    12,
      13,    14,    15,    16,    17,    18,    19,    20,    21,    22,
      23,    24,    25,    26,    -1,    -1,    -1,    30,    31,    32,
      33,    34,    35,    36,    37,    38,    39,    -1,    -1,    -1,
      -1,    -1,    45,    46,    47,    48,    49,    50,    51,    52,
      53,    54,    55,    56,    -1,    58,    -1,    -1,    -1,    -1,
      -1,    64,    65,    -1,    67,    68,    69,    70,    71,    -1,
      73,    74,    -1,    -1,    77,    78,    -1,    -1,    -1,    82,
      83,    84,    85,    -1,    -1,    -1,    -1,    -1,    91,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    99,   100,    -1,   102,
     103,   104,   105,   106,   107,   108,   109,   110,   111,   112,
     113,    -1,    -1,   116,     3,     4,     5,     6,     7,     8,
       9,    10,    11,    12,    13,    14,    15,    16,    17,    18,
      19,    20,    21,    22,    23,    24,    25,    26,    -1,    -1,
      -1,    30,    31,    32,    33,    34,    35,    36,    37,    38,
      39,    -1,    -1,    -1,    -1,    -1,    45,    46,    47,    48,
      49,    50,    51,    52,    53,    -1,    -1,    56,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    64,    65,    -1,    67,    68,
      69,    70,    71,    -1,    73,    74,    -1,    -1,    77,    78,
      -1,    -1,    -1,    82,    83,    84,    85,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      99,   100,    -1,   102,   103,   104,   105,   106,   107,   108,
     109,   110,   111,   112,   113,    52,    53,   116,    -1,    56,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    64,    65,    -1,
      67,    68,    69,    70,    71,    -1,    73,    74,    -1,    -1,
      77,    78,    -1,    -1,    -1,    82,    83,    84,    85,    -1,
      -1,    -1,    -1,    -1,    91,    -1,    -1,    -1,    95,    -1,
      -1,    -1,    99,   100,    -1,   102,   103,   104,   105,   106,
     107,   108,   109,   110,   111,   112,   113,    52,    53,   116,
      -1,    56,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    64,
      65,    -1,    67,    68,    69,    70,    71,    -1,    73,    74,
      -1,    -1,    77,    78,    -1,    -1,    -1,    82,    83,    84,
      85,    -1,    -1,    -1,    -1,    -1,    91,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    99,   100,    -1,   102,   103,   104,
     105,   106,   107,   108,   109,   110,   111,   112,   113,    52,
      53,   116,    -1,    56,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    64,    65,    -1,    67,    68,    69,    70,    71,    -1,
      73,    74,    -1,    -1,    77,    78,    -1,    -1,    -1,    82,
      83,    84,    85,    -1,    -1,    -1,    -1,    -1,    91,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    99,   100,    -1,   102,
     103,   104,   105,   106,   107,   108,   109,   110,   111,   112,
     113,    52,    53,   116,    -1,    56,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    64,    65,    -1,    67,    68,    69,    70,
      71,    -1,    73,    74,    -1,    -1,    77,    78,    -1,    -1,
      -1,    82,    83,    84,    85,    -1,    -1,    -1,    -1,    -1,
      91,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    99,   100,
      -1,   102,   103,   104,   105,   106,   107,   108,   109,   110,
     111,   112,   113,    52,    53,   116,    -1,    56,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    64,    65,    -1,    67,    68,
      69,    70,    71,    -1,    73,    74,    -1,    -1,    77,    78,
      -1,    -1,    -1,    82,    83,    84,    85,    -1,    -1,    -1,
      -1,    -1,    91,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      99,   100,    -1,   102,   103,   104,   105,   106,   107,   108,
     109,   110,   111,   112,   113,    52,    53,   116,    -1,    56,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    64,    65,    -1,
      67,    68,    69,    70,    71,    -1,    73,    74,    -1,    -1,
      77,    78,    -1,    -1,    -1,    82,    83,    84,    85,    -1,
      -1,    -1,    -1,    -1,    91,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    99,   100,    -1,   102,   103,   104,   105,   106,
     107,   108,   109,   110,   111,   112,   113,    52,    53,   116,
      -1,    56,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    64,
      65,    -1,    67,    68,    69,    70,    71,    -1,    73,    74,
      -1,    -1,    77,    78,    -1,    -1,    -1,    82,    83,    84,
      85,    -1,    -1,    -1,    -1,    -1,    91,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    99,   100,    -1,   102,   103,   104,
     105,   106,   107,   108,   109,   110,   111,   112,   113,    52,
      53,   116,    -1,    56,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    64,    65,    -1,    67,    68,    69,    70,    71,    -1,
      73,    74,    -1,    -1,    77,    78,    -1,    -1,    -1,    82,
      83,    84,    85,    -1,    -1,    -1,    -1,    -1,    91,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    99,   100,    -1,   102,
     103,   104,   105,   106,   107,   108,   109,   110,   111,   112,
     113,    52,    53,   116,    -1,    56,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    64,    65,    -1,    67,    68,    69,    70,
      71,    -1,    73,    74,    -1,    -1,    77,    78,    -1,    -1,
      -1,    82,    83,    84,    85,    -1,    -1,    -1,    -1,    -1,
      91,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    99,   100,
      -1,   102,   103,   104,   105,   106,   107,   108,   109,   110,
     111,   112,   113,    52,    53,   116,    -1,    56,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    64,    65,    -1,    67,    68,
      69,    70,    71,    -1,    73,    74,    -1,    -1,    77,    78,
      -1,    -1,    -1,    82,    83,    84,    85,    -1,    -1,    -1,
      -1,    -1,    91,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      99,   100,    -1,   102,   103,   104,   105,   106,   107,   108,
     109,   110,   111,   112,   113,    52,    53,   116,    -1,    56,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    64,    65,    -1,
      67,    68,    69,    70,    71,    -1,    73,    74,    -1,    -1,
      77,    78,    -1,    -1,    -1,    82,    83,    84,    85,    -1,
      -1,    -1,    -1,    -1,    91,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    99,   100,    -1,   102,   103,   104,   105,   106,
     107,   108,   109,   110,   111,   112,   113,    52,    53,   116,
      -1,    56,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    64,
      65,    -1,    67,    68,    69,    70,    71,    -1,    73,    74,
      -1,    -1,    77,    78,    -1,    -1,    -1,    82,    83,    84,
      85,    -1,    -1,    -1,    -1,    -1,    91,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    99,   100,    -1,   102,   103,   104,
     105,   106,   107,   108,   109,   110,   111,   112,   113,    52,
      53,   116,    -1,    56,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    64,    65,    -1,    67,    68,    69,    70,    71,    -1,
      73,    74,    -1,    -1,    77,    78,    -1,    -1,    -1,    82,
      83,    84,    85,    -1,    -1,    -1,    -1,    -1,    91,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    99,   100,    -1,   102,
     103,   104,   105,   106,   107,   108,   109,   110,   111,   112,
     113,    52,    53,   116,    -1,    56,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    64,    65,    -1,    67,    68,    69,    70,
      71,    -1,    73,    74,    -1,    -1,    77,    78,    -1,    -1,
      -1,    82,    83,    84,    85,    -1,    -1,    -1,    -1,    -1,
      91,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    99,   100,
      -1,   102,   103,   104,   105,   106,   107,   108,   109,   110,
     111,   112,   113,    52,    53,   116,    -1,    56,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    64,    65,    -1,    67,    68,
      69,    70,    71,    -1,    73,    74,    -1,    -1,    77,    78,
      -1,    -1,    -1,    82,    83,    84,    85,    -1,    -1,    -1,
      -1,    -1,    91,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      99,   100,    -1,   102,   103,   104,   105,   106,   107,   108,
     109,   110,   111,   112,   113,    52,    53,   116,    -1,    56,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    64,    65,    -1,
      67,    68,    69,    70,    71,    -1,    73,    74,    -1,    -1,
      77,    78,    -1,    -1,    -1,    82,    83,    84,    85,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    99,   100,    -1,   102,   103,   104,   105,   106,
     107,   108,   109,   110,   111,   112,   113,    -1,    -1,   116
  };

  const unsigned short int
  parser::yystos_[] =
  {
       0,     1,     3,     4,     5,     6,     7,    11,    12,    16,
      18,    19,    20,    21,    22,    23,    24,    30,    31,    32,
      33,    34,    35,    36,    39,    45,    46,    47,    48,    49,
      50,    51,    52,    53,    54,    55,    56,    58,    59,    60,
      62,    63,    64,    65,    66,    87,    90,    93,    94,    97,
      99,   103,   112,   117,   118,   119,   120,   122,   123,   124,
     125,   130,   131,   138,   140,   141,   142,   145,   146,   149,
     150,   151,   152,   157,   159,   161,   163,   164,   166,   167,
     168,   170,   171,   172,   174,   175,   185,   200,   218,   242,
     243,   257,   258,   259,   260,   261,   262,   263,   266,   268,
     269,   278,   279,   280,   281,   282,   283,   284,   285,   286,
     321,   152,     5,    21,    22,    30,    31,    32,    39,    46,
      52,    56,    66,    84,    87,    90,   166,   176,   177,   200,
     218,   282,   283,   286,   177,     3,     4,     5,     6,     7,
       8,     9,    10,    11,    12,    13,    14,    15,    16,    17,
      18,    19,    20,    21,    22,    23,    24,    25,    26,    30,
      31,    32,    33,    34,    35,    36,    37,    38,    39,    45,
      46,    47,    48,    49,    50,    51,    52,    53,    55,    56,
      64,    65,    67,    68,    69,    70,    71,    73,    74,    77,
      78,    82,    83,    84,    85,    91,    99,   100,   102,   103,
     104,   105,   106,   107,   108,   109,   110,   111,   112,   113,
     116,   178,   183,   184,   284,   317,    33,    34,    35,    36,
      49,    50,    51,    52,    56,   178,   179,   180,   181,   278,
     279,   201,    87,   161,   162,   175,   218,   282,   283,   285,
     286,   162,   132,   137,   162,   326,   331,   332,   205,   207,
      87,   168,   175,   218,   223,   282,   283,   286,    57,    99,
     101,   106,   112,   118,   167,   185,   186,   192,   195,   197,
     319,   320,   192,   192,    91,   193,   194,    91,   189,   193,
      91,   132,   327,    54,   180,   327,   114,   114,   185,   218,
     185,   281,    56,     1,    47,    90,   154,   155,   156,   157,
     161,   169,   170,   202,   204,   187,   197,   319,   334,   186,
     318,   319,    87,   135,   174,   218,   282,   283,   286,   185,
     163,   185,   273,   272,   273,   274,   264,   270,   267,   271,
     236,   237,     0,   326,   332,    40,    41,    42,    43,    44,
      37,    38,    26,   114,   189,   193,   244,    28,   241,    72,
     135,    90,    99,   171,    72,    67,    68,    69,    70,    71,
      73,    74,    75,    76,    77,    78,    80,    81,    84,    85,
     100,   102,   104,   105,   107,   108,   109,   110,   111,   113,
     133,    79,    86,    95,   144,   325,    86,   324,   325,   244,
     260,    88,    88,   327,   281,   161,    52,    56,   176,   109,
     287,    86,    95,   325,   214,   134,   161,   215,   324,   135,
     153,   154,    56,    13,   219,   331,    72,    86,    95,   325,
      88,    88,   219,   326,    17,   251,   137,   162,   162,    56,
      86,    95,   325,    25,   186,   186,   186,   186,   272,    89,
     135,   196,   135,   196,   192,   327,   328,   192,   191,   192,
     197,   319,   161,   328,   161,    54,    59,    60,   158,    91,
     185,   150,   154,    86,   325,    67,    88,   157,   114,   169,
      92,   326,   332,   134,   328,   157,   327,    96,   132,   135,
     330,   135,   330,   115,   330,    56,   171,   173,   135,    86,
      95,   325,    61,   126,   127,   128,   275,   128,   275,   128,
     128,   275,   128,   265,   275,    61,   128,   128,   265,    61,
     128,    34,    56,    87,    90,    91,    94,    97,   103,   131,
     239,   293,   295,   299,   300,   302,   305,   307,   308,   311,
     313,   315,   335,   337,   339,   152,   162,   162,   162,   162,
     157,   161,   161,   248,   249,   245,   246,    98,   165,   248,
      99,   163,   186,   197,   198,   199,   169,   135,   174,   135,
     159,   160,   163,   175,   185,   186,   188,   199,   218,   286,
     185,   185,   185,   185,   185,   185,   185,   185,   185,   185,
     185,   185,   185,   185,   185,   185,   185,   185,   185,   185,
     185,   185,   185,   185,   185,   185,    52,    53,    56,   183,
     189,   322,   323,   191,    52,    53,    56,   183,   189,   322,
      52,    56,   322,   160,   185,   188,   160,   188,   331,   288,
     211,    52,    56,    95,   176,   322,   191,   322,   153,    90,
     337,   328,    95,   289,   290,   216,   182,    10,     8,   253,
     154,    13,    52,    56,   191,    52,    56,   154,   251,   197,
      10,    27,   220,   331,   220,    52,    56,   191,    52,    56,
     209,   143,   186,    99,   186,   195,   319,   320,   328,    92,
     328,   135,   135,   328,   180,   161,   115,   115,   185,   188,
     150,   328,   156,   337,   203,    92,   319,   135,   173,    52,
      56,   191,    52,    56,   276,    54,    55,    58,   277,   286,
     121,   136,   275,   136,   136,   136,    56,    52,    54,    55,
      58,    90,    99,   224,   225,   226,   296,   297,   337,   338,
     295,   337,   337,   344,   345,   337,   238,   135,   294,   135,
     316,   135,   316,   135,   294,   135,   294,    86,    57,    67,
      99,   100,   101,   102,   106,   297,   298,   301,   306,   312,
     314,    10,   250,   115,   247,   245,    10,   186,   135,   328,
     173,   135,    44,    72,    44,    86,    95,   325,   327,    88,
      88,   189,   193,   327,   329,    88,    88,   189,   190,   193,
     190,   193,   212,   162,   153,    56,   342,   343,   329,    10,
     343,    91,   292,   153,   178,   180,   186,   199,   254,   334,
      15,   222,   333,    14,   221,   222,    88,    88,   329,    88,
      88,   222,    10,   135,   219,   206,   208,   329,   162,   186,
     186,   196,   319,   328,   115,    92,   328,   330,   171,   329,
     154,   226,   135,   297,   135,   328,   111,   328,   233,   327,
     135,   329,    89,    89,   340,    76,   111,   232,    29,   139,
     240,   293,   299,   311,   313,   302,   307,   315,   339,   339,
     300,   308,   313,   300,   339,    56,    95,    72,   186,    52,
      52,    52,   230,   232,   230,   115,    99,   186,   173,   157,
     185,    52,    56,   191,    52,    56,   134,   160,   188,   160,
     188,   165,    96,    88,   160,   188,   160,   188,   165,   244,
     241,   213,   331,    10,   134,   135,   329,   329,   295,   295,
      10,   217,    89,   255,   154,     9,   256,   162,    10,    88,
      10,   186,   154,   154,   154,   220,   135,   129,   328,    90,
     225,   135,    99,   224,   337,    92,   137,   336,   337,   337,
     337,   115,   227,   229,   233,   300,   303,   304,   307,   309,
     310,   313,   315,   339,   154,   154,   135,   294,   135,   294,
     316,   135,   294,   135,   294,   294,   298,   336,   186,   154,
     231,   154,   186,   329,   185,   160,   188,   153,   337,   341,
     342,   328,   340,   290,    87,   175,   218,   282,   283,   286,
     219,   154,   219,   222,   251,   252,    10,    10,   210,   135,
     225,   135,   297,    52,   234,   235,   296,   135,   329,   115,
     233,   111,   135,   228,   135,   316,   316,   135,   228,   135,
     228,   298,   301,    10,   115,   300,   313,   300,   300,   329,
     340,    10,    72,   109,   291,   331,   153,    56,    86,    95,
     325,   154,   154,   154,   225,   135,   135,   327,   337,   111,
     227,   310,   313,   303,   307,   339,   300,   309,   313,   300,
     339,    72,    87,   218,   294,   135,   294,   294,   294,   337,
     337,   340,    10,    52,    56,   191,    52,    56,   253,   221,
      10,   225,   235,   135,   228,   135,   228,   316,   135,   228,
     135,   228,   228,   218,    56,    86,   300,   329,   300,   313,
     300,   300,    52,    56,   294,   228,   135,   228,   228,   228,
     300,   228
  };

  const unsigned short int
  parser::yyr1_[] =
  {
       0,   148,   149,   150,   151,   151,   151,   151,   152,   152,
     153,   154,   155,   155,   155,   155,   156,   156,   158,   157,
     157,   157,   157,   157,   157,   157,   157,   157,   157,   157,
     157,   157,   157,   157,   157,   157,   159,   159,   159,   159,
     159,   159,   159,   159,   160,   160,   160,   161,   161,   161,
     161,   161,   161,   162,   163,   163,   164,   164,   165,   166,
     167,   167,   167,   167,   167,   167,   167,   167,   167,   167,
     167,   168,   168,   169,   169,   170,   170,   170,   170,   170,
     170,   170,   170,   170,   170,   171,   171,   172,   172,   173,
     173,   174,   174,   174,   174,   174,   174,   174,   174,   174,
     175,   175,   175,   175,   175,   175,   175,   175,   175,   176,
     176,   177,   177,   177,   177,   178,   178,   178,   178,   178,
     179,   179,   180,   180,   181,   182,   181,   183,   183,   183,
     183,   183,   183,   183,   183,   183,   183,   183,   183,   183,
     183,   183,   183,   183,   183,   183,   183,   183,   183,   183,
     183,   183,   183,   183,   183,   183,   183,   184,   184,   184,
     184,   184,   184,   184,   184,   184,   184,   184,   184,   184,
     184,   184,   184,   184,   184,   184,   184,   184,   184,   184,
     184,   184,   184,   184,   184,   184,   184,   184,   184,   184,
     184,   184,   184,   184,   184,   184,   184,   184,   185,   185,
     185,   185,   185,   185,   185,   185,   185,   185,   185,   185,
     185,   185,   185,   185,   185,   185,   185,   185,   185,   185,
     185,   185,   185,   185,   185,   185,   185,   185,   185,   185,
     185,   185,   185,   185,   185,   185,   185,   185,   185,   185,
     186,   187,   187,   187,   187,   188,   188,   189,   190,   190,
     191,   191,   191,   191,   191,   192,   192,   192,   192,   192,
     194,   193,   195,   196,   196,   197,   197,   197,   197,   198,
     198,   199,   199,   199,   200,   200,   200,   200,   200,   200,
     200,   200,   200,   200,   200,   201,   200,   202,   203,   200,
     204,   200,   200,   200,   200,   200,   200,   200,   200,   200,
     200,   200,   200,   200,   200,   200,   200,   200,   200,   200,
     200,   205,   206,   200,   207,   208,   200,   200,   200,   209,
     210,   200,   211,   200,   212,   213,   200,   214,   200,   215,
     200,   216,   217,   200,   200,   200,   200,   200,   218,   219,
     219,   219,   220,   220,   221,   221,   222,   222,   223,   223,
     224,   224,   225,   225,   226,   226,   226,   226,   226,   226,
     226,   226,   226,   227,   227,   227,   227,   228,   228,   229,
     229,   229,   229,   229,   229,   229,   229,   229,   229,   229,
     229,   229,   229,   229,   230,   231,   230,   232,   232,   232,
     233,   233,   234,   234,   235,   235,   237,   238,   236,   239,
     239,   240,   240,   241,   242,   242,   242,   242,   243,   243,
     243,   243,   243,   243,   243,   243,   243,   244,   244,   246,
     247,   245,   249,   250,   248,   251,   252,   252,   253,   253,
     254,   254,   254,   255,   255,   256,   256,   257,   257,   257,
     258,   259,   259,   260,   260,   260,   261,   262,   263,   264,
     264,   265,   265,   266,   267,   267,   268,   269,   270,   270,
     271,   271,   272,   272,   273,   273,   274,   274,   275,   275,
     276,   275,   277,   277,   277,   277,   278,   279,   280,   280,
     281,   281,   281,   281,   281,   281,   282,   282,   282,   282,
     282,   283,   283,   283,   283,   283,   283,   283,   284,   284,
     285,   285,   286,   286,   288,   287,   287,   289,   289,   291,
     290,   292,   290,   293,   293,   293,   293,   294,   294,   295,
     295,   295,   295,   295,   295,   295,   295,   295,   295,   295,
     295,   295,   295,   295,   296,   296,   296,   297,   297,   298,
     299,   299,   300,   300,   301,   302,   302,   303,   303,   304,
     304,   305,   305,   306,   306,   307,   307,   308,   309,   310,
     310,   311,   311,   312,   312,   313,   313,   314,   314,   315,
     315,   316,   316,   317,   317,   318,   318,   319,   319,   320,
     320,   320,   320,   321,   321,   321,   322,   322,   322,   322,
     323,   323,   323,   324,   324,   325,   325,   326,   326,   327,
     327,   328,   329,   330,   330,   330,   331,   331,   332,   332,
     333,   334,   335,   335,   335,   336,   336,   337,   337,   337,
     337,   337,   337,   337,   337,   337,   337,   338,   338,   339,
     339,   340,   340,   341,   341,   342,   342,   343,   343,   345,
     344
  };

  const unsigned char
  parser::yyr2_[] =
  {
       0,     2,     1,     2,     0,     1,     3,     2,     1,     4,
       4,     2,     0,     1,     3,     2,     1,     4,     0,     4,
       3,     3,     3,     2,     3,     3,     3,     3,     3,     4,
       1,     3,     3,     3,     4,     1,     3,     3,     6,     5,
       5,     5,     5,     3,     1,     3,     1,     1,     3,     3,
       3,     2,     1,     1,     1,     1,     1,     4,     3,     1,
       2,     3,     4,     5,     4,     5,     2,     2,     2,     2,
       2,     1,     3,     1,     3,     1,     2,     3,     5,     2,
       4,     2,     4,     1,     3,     1,     3,     2,     3,     1,
       3,     1,     1,     4,     3,     3,     3,     3,     2,     1,
       1,     1,     4,     3,     3,     3,     3,     2,     1,     1,
       1,     2,     1,     5,     3,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     0,     4,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     3,     3,
       6,     5,     5,     5,     5,     4,     3,     3,     3,     3,
       3,     3,     3,     3,     3,     4,     2,     2,     3,     3,
       3,     3,     3,     3,     3,     3,     3,     3,     3,     3,
       3,     2,     2,     3,     3,     3,     3,     3,     6,     1,
       1,     1,     2,     4,     2,     1,     3,     3,     0,     1,
       0,     1,     2,     4,     2,     1,     2,     2,     4,     1,
       0,     2,     2,     2,     0,     1,     2,     3,     4,     1,
       1,     3,     4,     2,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     0,     4,     0,     0,     5,
       0,     4,     3,     5,     3,     2,     3,     3,     1,     4,
       3,     1,     5,     4,     3,     2,     1,     2,     2,     6,
       6,     0,     0,     7,     0,     0,     7,     5,     4,     0,
       0,     9,     0,     6,     0,     0,     8,     0,     5,     0,
       6,     0,     0,     9,     1,     1,     1,     1,     1,     1,
       1,     2,     1,     1,     1,     5,     1,     2,     1,     1,
       1,     3,     1,     3,     1,     4,     6,     3,     5,     2,
       4,     1,     3,     4,     2,     2,     1,     2,     0,     6,
       8,     4,     6,     4,     2,     6,     2,     4,     6,     2,
       4,     2,     4,     1,     0,     0,     3,     3,     1,     4,
       1,     4,     1,     3,     1,     1,     0,     0,     4,     4,
       1,     3,     3,     3,     2,     4,     5,     5,     2,     4,
       4,     3,     3,     3,     2,     1,     4,     3,     3,     0,
       0,     4,     0,     0,     4,     5,     1,     1,     6,     0,
       1,     1,     1,     2,     0,     2,     0,     1,     1,     1,
       1,     1,     2,     3,     1,     1,     3,     4,     3,     0,
       3,     1,     2,     3,     0,     3,     3,     3,     0,     3,
       0,     3,     0,     2,     0,     2,     0,     2,     1,     2,
       0,     4,     1,     1,     1,     1,     1,     3,     1,     2,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     0,     4,     0,     3,     0,     0,
       6,     0,     5,     4,     2,     2,     1,     2,     0,     6,
       8,     4,     6,     4,     6,     2,     4,     6,     2,     4,
       2,     4,     1,     0,     1,     1,     1,     1,     1,     1,
       2,     3,     1,     3,     1,     3,     2,     3,     2,     1,
       3,     1,     3,     1,     1,     3,     2,     4,     4,     1,
       3,     1,     3,     1,     1,     3,     2,     1,     1,     3,
       2,     2,     0,     1,     3,     0,     2,     1,     3,     3,
       2,     4,     2,     1,     1,     1,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     0,     1,     0,
       1,     2,     2,     0,     1,     1,     1,     1,     1,     2,
       0,     0,     2,     1,     3,     3,     1,     1,     5,     3,
       5,     5,     4,     2,     1,     1,     3,     3,     1,     1,
       0,     2,     0,     3,     3,     1,     3,     3,     1,     0,
       2
  };


#if TYPEDRUBY24DEBUG
  // YYTNAME[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
  // First, the terminals, then, starting at \a yyntokens_, nonterminals.
  const char*
  const parser::yytname_[] =
  {
  "$end", "error", "$undefined", "kCLASS", "kMODULE", "kDEF", "kUNDEF",
  "kBEGIN", "kRESCUE", "kENSURE", "kEND", "kIF", "kUNLESS", "kTHEN",
  "kELSIF", "kELSE", "kCASE", "kWHEN", "kWHILE", "kUNTIL", "kFOR",
  "kBREAK", "kNEXT", "kREDO", "kRETRY", "kIN", "kDO", "kDO_COND",
  "kDO_BLOCK", "kDO_LAMBDA", "kRETURN", "kYIELD", "kSUPER", "kSELF",
  "kNIL", "kTRUE", "kFALSE", "kAND", "kOR", "kNOT", "kIF_MOD",
  "kUNLESS_MOD", "kWHILE_MOD", "kUNTIL_MOD", "kRESCUE_MOD", "kALIAS",
  "kDEFINED", "klBEGIN", "klEND", "k__LINE__", "k__FILE__",
  "k__ENCODING__", "tIDENTIFIER", "tFID", "tGVAR", "tIVAR", "tCONSTANT",
  "tLABEL", "tCVAR", "tNTH_REF", "tBACK_REF", "tSTRING_CONTENT",
  "tINTEGER", "tFLOAT", "tUPLUS", "tUMINUS", "tUMINUS_NUM", "tPOW", "tCMP",
  "tEQ", "tEQQ", "tNEQ", "tEQL", "tGEQ", "tLEQ", "tANDOP", "tOROP",
  "tMATCH", "tNMATCH", "tDOT", "tDOT2", "tDOT3", "tAREF", "tASET",
  "tLSHFT", "tRSHFT", "tCOLON2", "tCOLON3", "tOP_ASGN", "tASSOC",
  "tLPAREN", "tLPAREN2", "tRPAREN", "tLPAREN_ARG", "tLBRACK", "tLBRACK2",
  "tRBRACK", "tLBRACE", "tLBRACE_ARG", "tSTAR", "tSTAR2", "tAMPER",
  "tAMPER2", "tTILDE", "tPERCENT", "tDIVIDE", "tDSTAR", "tPLUS", "tMINUS",
  "tLT", "tGT", "tPIPE", "tBANG", "tCARET", "tLCURLY", "tRCURLY",
  "tBACK_REF2", "tSYMBEG", "tSTRING_BEG", "tXSTRING_BEG", "tREGEXP_BEG",
  "tREGEXP_OPT", "tWORDS_BEG", "tQWORDS_BEG", "tSYMBOLS_BEG",
  "tQSYMBOLS_BEG", "tSTRING_DBEG", "tSTRING_DVAR", "tSTRING_END",
  "tSTRING_DEND", "tSTRING", "tSYMBOL", "tNL", "tEH", "tCOLON", "tCOMMA",
  "tSPACE", "tSEMI", "tLAMBDA", "tLAMBEG", "tCHARACTER", "tRATIONAL",
  "tIMAGINARY", "tLABEL_END", "tANDDOT", "tRATIONAL_IMAGINARY",
  "tFLOAT_IMAGINARY", "tLOWEST", "$accept", "program", "top_compstmt",
  "top_stmts", "top_stmt", "bodystmt", "compstmt", "stmts",
  "stmt_or_begin", "stmt", "$@1", "command_asgn", "command_rhs", "expr",
  "expr_value", "command_call", "block_command", "cmd_brace_block",
  "fcall", "command", "mlhs", "mlhs_inner", "mlhs_basic", "mlhs_item",
  "mlhs_head", "mlhs_post", "mlhs_node", "lhs", "cname", "cpath", "fname",
  "fsym", "fitem", "undef_list", "$@2", "op", "reswords", "arg",
  "arg_value", "aref_args", "arg_rhs", "paren_args", "opt_paren_args",
  "opt_call_args", "call_args", "command_args", "@3", "block_arg",
  "opt_block_arg", "args", "mrhs_arg", "mrhs", "primary", "@4", "@5",
  "$@6", "$@7", "$@8", "$@9", "$@10", "$@11", "$@12", "$@13", "@14", "@15",
  "@16", "@17", "@18", "$@19", "@20", "primary_value", "then", "do",
  "if_tail", "opt_else", "for_var", "f_marg", "f_marg_list", "f_margs",
  "block_args_tail", "opt_block_args_tail", "block_param",
  "opt_block_param", "$@21", "block_param_def", "opt_bv_decl", "bv_decls",
  "bvar", "lambda", "$@22", "@23", "f_larglist", "lambda_body", "do_block",
  "block_call", "method_call", "brace_block", "brace_body", "$@24", "@25",
  "do_body", "$@26", "@27", "case_body", "cases", "opt_rescue", "exc_list",
  "exc_var", "opt_ensure", "literal", "strings", "string", "string1",
  "xstring", "regexp", "words", "word_list", "word", "symbols",
  "symbol_list", "qwords", "qsymbols", "qword_list", "qsym_list",
  "string_contents", "xstring_contents", "regexp_contents",
  "string_content", "$@28", "string_dvar", "symbol", "dsym", "numeric",
  "simple_numeric", "user_variable", "keyword_variable", "var_ref",
  "var_lhs", "backref", "superclass", "$@29", "tr_methodgenargs",
  "f_arglist", "$@30", "@31", "args_tail", "opt_args_tail", "f_args",
  "f_bad_arg", "f_norm_arg", "f_arg_asgn", "f_arg_item", "f_arg",
  "f_label", "f_kw", "f_block_kw", "f_block_kwarg", "f_kwarg",
  "kwrest_mark", "f_kwrest", "f_opt", "f_block_opt", "f_block_optarg",
  "f_optarg", "restarg_mark", "f_rest_arg", "blkarg_mark", "f_block_arg",
  "opt_f_block_arg", "singleton", "assoc_list", "assocs", "assoc",
  "operation", "operation2", "operation3", "dot_or_colon", "call_op",
  "opt_terms", "opt_nl", "rparen", "rbracket", "trailer", "term", "terms",
  "none", "list_none", "tr_cpath", "tr_types", "tr_type", "tr_union_type",
  "tr_argsig", "tr_returnsig", "tr_constraint", "tr_gendeclarg",
  "tr_gendeclargs", "tr_blockproto", "$@32", YY_NULLPTR
  };


  const unsigned short int
  parser::yyrline_[] =
  {
       0,   426,   426,   431,   437,   440,   444,   449,   454,   455,
     460,   478,   484,   487,   491,   496,   501,   502,   510,   509,
     517,   521,   525,   530,   534,   538,   542,   546,   550,   556,
     560,   561,   565,   569,   573,   577,   579,   583,   588,   593,
     598,   603,   609,   614,   620,   621,   627,   629,   630,   634,
     638,   642,   646,   648,   650,   651,   653,   654,   659,   667,
     669,   673,   684,   688,   699,   703,   714,   718,   723,   727,
     731,   736,   740,   745,   749,   754,   755,   761,   767,   774,
     780,   787,   791,   797,   801,   808,   809,   814,   818,   825,
     829,   836,   841,   846,   850,   854,   858,   862,   867,   872,
     878,   883,   888,   892,   896,   900,   904,   909,   914,   920,
     925,   927,   931,   935,   939,   944,   944,   944,   945,   946,
     948,   952,   954,   955,   957,   962,   961,   972,   972,   972,
     972,   972,   972,   973,   973,   973,   973,   973,   973,   974,
     974,   974,   974,   974,   974,   975,   975,   975,   975,   975,
     975,   976,   976,   976,   976,   976,   976,   978,   978,   978,
     978,   978,   979,   979,   979,   979,   979,   980,   980,   980,
     980,   980,   981,   981,   981,   981,   981,   982,   982,   982,
     982,   982,   983,   983,   983,   983,   983,   984,   984,   984,
     984,   984,   985,   985,   985,   985,   985,   986,   988,   992,
     997,  1002,  1007,  1012,  1017,  1023,  1029,  1034,  1038,  1042,
    1046,  1050,  1054,  1058,  1062,  1066,  1070,  1074,  1078,  1082,
    1086,  1090,  1094,  1098,  1102,  1106,  1110,  1114,  1118,  1122,
    1127,  1131,  1135,  1139,  1143,  1147,  1151,  1155,  1159,  1163,
    1165,  1167,  1168,  1169,  1175,  1180,  1181,  1187,  1193,  1196,
    1199,  1202,  1203,  1204,  1210,  1215,  1219,  1225,  1231,  1238,
    1243,  1243,  1253,  1258,  1263,  1267,  1271,  1275,  1281,  1288,
    1292,  1294,  1300,  1306,  1311,  1312,  1313,  1314,  1315,  1316,
    1317,  1318,  1319,  1320,  1321,  1326,  1325,  1336,  1341,  1335,
    1350,  1349,  1357,  1361,  1365,  1369,  1373,  1377,  1381,  1385,
    1390,  1396,  1401,  1405,  1409,  1413,  1425,  1426,  1436,  1447,
    1454,  1462,  1466,  1461,  1474,  1478,  1473,  1485,  1494,  1504,
    1508,  1503,  1516,  1515,  1540,  1544,  1539,  1557,  1556,  1577,
    1576,  1591,  1595,  1590,  1610,  1614,  1618,  1622,  1627,  1629,
    1630,  1631,  1636,  1637,  1639,  1640,  1652,  1656,  1661,  1662,
    1664,  1668,  1673,  1677,  1684,  1685,  1691,  1698,  1704,  1711,
    1715,  1721,  1725,  1732,  1739,  1745,  1751,  1757,  1762,  1766,
    1774,  1783,  1790,  1798,  1805,  1806,  1814,  1826,  1833,  1841,
    1847,  1854,  1860,  1867,  1870,  1875,  1874,  1890,  1895,  1900,
    1908,  1912,  1917,  1921,  1928,  1934,  1940,  1944,  1940,  1959,
    1966,  1972,  1976,  1981,  1989,  2000,  2008,  2023,  2031,  2039,
    2047,  2055,  2059,  2067,  2075,  2083,  2087,  2092,  2099,  2107,
    2110,  2107,  2123,  2126,  2123,  2139,  2146,  2150,  2152,  2170,
    2174,  2178,  2179,  2181,  2186,  2190,  2195,  2199,  2200,  2201,
    2203,  2208,  2212,  2219,  2224,  2229,  2234,  2240,  2247,  2253,
    2256,  2263,  2267,  2274,  2280,  2283,  2290,  2295,  2301,  2304,
    2312,  2315,  2323,  2326,  2334,  2337,  2345,  2348,  2355,  2359,
    2364,  2363,  2375,  2379,  2383,  2387,  2390,  2396,  2402,  2406,
    2411,  2416,  2421,  2426,  2431,  2436,  2442,  2446,  2450,  2454,
    2458,  2463,  2467,  2471,  2475,  2479,  2483,  2487,  2492,  2496,
    2501,  2506,  2512,  2516,  2522,  2521,  2530,  2534,  2539,  2544,
    2543,  2561,  2560,  2581,  2588,  2594,  2600,  2605,  2610,  2614,
    2622,  2631,  2638,  2646,  2653,  2661,  2667,  2674,  2682,  2689,
    2696,  2702,  2709,  2714,  2718,  2723,  2728,  2734,  2735,  2742,
    2747,  2758,  2763,  2767,  2774,  2785,  2795,  2806,  2817,  2829,
    2833,  2840,  2844,  2851,  2851,  2853,  2868,  2880,  2891,  2902,
    2906,  2913,  2917,  2924,  2924,  2926,  2941,  2953,  2953,  2955,
    2970,  2982,  2987,  2991,  2992,  2998,  3001,  3003,  3007,  3014,
    3018,  3022,  3026,  3031,  3031,  3031,  3032,  3032,  3032,  3032,
    3033,  3033,  3033,  3034,  3034,  3035,  3041,  3047,  3047,  3048,
    3048,  3049,  3053,  3057,  3057,  3057,  3059,  3063,  3065,  3066,
    3069,  3074,  3078,  3082,  3086,  3091,  3097,  3102,  3106,  3110,
    3114,  3120,  3124,  3135,  3139,  3143,  3158,  3163,  3167,  3169,
    3175,  3179,  3184,  3188,  3192,  3197,  3201,  3206,  3212,  3217,
    3217
  };

  // Print the state stack on the debug stream.
  void
  parser::yystack_print_ ()
  {
    *yycdebug_ << "Stack now";
    for (stack_type::const_iterator
           i = yystack_.begin (),
           i_end = yystack_.end ();
         i != i_end; ++i)
      *yycdebug_ << ' ' << i->state;
    *yycdebug_ << std::endl;
  }

  // Report on the debug stream that the rule \a yyrule is going to be reduced.
  void
  parser::yy_reduce_print_ (int yyrule)
  {
    unsigned int yylno = yyrline_[yyrule];
    int yynrhs = yyr2_[yyrule];
    // Print the symbols being reduced, and their result.
    *yycdebug_ << "Reducing stack by rule " << yyrule - 1
               << " (line " << yylno << "):" << std::endl;
    // The symbols being reduced.
    for (int yyi = 0; yyi < yynrhs; yyi++)
      YY_SYMBOL_PRINT ("   $" << yyi + 1 << " =",
                       yystack_[(yynrhs) - (yyi + 1)]);
  }
#endif // TYPEDRUBY24DEBUG

  // Symbol number corresponding to token number t.
  inline
  parser::token_number_type
  parser::yytranslate_ (int t)
  {
    static
    const token_number_type
    translate_table[] =
    {
     0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     3,     4,     5,     6,     7,     8,     9,    10,    11,
      12,    13,    14,    15,    16,    17,    18,    19,    20,    21,
      22,    23,    24,    25,    26,    27,    28,    29,    30,    31,
      32,    33,    34,    35,    36,    37,    38,    39,    40,    41,
      42,    43,    44,    45,    46,    47,    48,    49,    50,    51,
      52,    53,    54,    55,    56,    57,    58,    59,    60,    61,
      62,    63,    64,    65,    66,    67,    68,    69,    70,    71,
      72,    73,    74,    75,    76,    77,    78,    79,    80,    81,
      82,    83,    84,    85,    86,    87,    88,    89,    90,    91,
      92,    93,    94,    95,    96,    97,    98,    99,   100,   101,
     102,   103,   104,   105,   106,   107,   108,   109,   110,   111,
     112,   113,   114,   115,   116,   117,   118,   119,   120,   121,
     122,   123,   124,   125,   126,   127,   128,   129,   130,   131,
     132,   133,   134,   135,   136,   137,   138,   139,   140,   141,
     142,   143,   144,   145,   146,     2,   147
    };
    const unsigned int user_token_number_max_ = 1146;
    const token_number_type undef_token_ = 2;

    if (static_cast<int>(t) <= yyeof_)
      return yyeof_;
    else if (static_cast<unsigned int> (t) <= user_token_number_max_)
      return translate_table[t];
    else
      return undef_token_;
  }

#line 26 "cc/grammars/typedruby24.ypp" // lalr1.cc:1167
} } } // ruby_parser::bison::typedruby24
#line 8458 "cc/grammars/typedruby24.cc" // lalr1.cc:1167
#line 3224 "cc/grammars/typedruby24.ypp" // lalr1.cc:1168

