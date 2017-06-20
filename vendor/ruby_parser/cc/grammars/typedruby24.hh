// A Bison parser, made by GNU Bison 3.0.4.

// Skeleton interface for Bison LALR(1) parsers in C++

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

/**
 ** \file cc/grammars/typedruby24.hh
 ** Define the ruby_parser::bison::typedruby24::parser class.
 */

// C++ LALR(1) parser skeleton written by Akim Demaille.

#ifndef YY_TYPEDRUBY24_CC_GRAMMARS_TYPEDRUBY24_HH_INCLUDED
# define YY_TYPEDRUBY24_CC_GRAMMARS_TYPEDRUBY24_HH_INCLUDED
// //                    "%code requires" blocks.
#line 4 "cc/grammars/typedruby24.ypp" // lalr1.cc:392

  #include <ruby_parser/builder.hh>
  #include <ruby_parser/node.hh>
  #include <ruby_parser/token.hh>
  #include <ruby_parser/lexer.hh>
  #include <ruby_parser/driver.hh>
  #include <ruby_parser/state_stack.hh>
  #include <iterator>
  #include <iostream>
  #include <utility>
  #include <cstdlib>

using namespace ruby_parser;
using namespace std::string_literals;

#ifndef YY_NULLPTR
#define YY_NULLPTR nullptr
#endif
#line 378 "cc/grammars/typedruby24.ypp" // lalr1.cc:392


union parser_value {
  ruby_parser::token *token;
  ruby_parser::delimited_node_list *delimited_list;
  ruby_parser::delimited_block *delimited_block;
  ruby_parser::node_with_token *with_token;
  ruby_parser::case_body *case_body;
  ruby_parser::foreign_ptr node;
  ruby_parser::node_list *list;
  ruby_parser::state_stack *stack;
  size_t size;
  bool boolean;
};


#line 80 "cc/grammars/typedruby24.hh" // lalr1.cc:392

# include <cassert>
# include <cstdlib> // std::abort
# include <iostream>
# include <stdexcept>
# include <string>
# include <vector>
# include "stack.hh"



#ifndef YY_ATTRIBUTE
# if (defined __GNUC__                                               \
      && (2 < __GNUC__ || (__GNUC__ == 2 && 96 <= __GNUC_MINOR__)))  \
     || defined __SUNPRO_C && 0x5110 <= __SUNPRO_C
#  define YY_ATTRIBUTE(Spec) __attribute__(Spec)
# else
#  define YY_ATTRIBUTE(Spec) /* empty */
# endif
#endif

#ifndef YY_ATTRIBUTE_PURE
# define YY_ATTRIBUTE_PURE   YY_ATTRIBUTE ((__pure__))
#endif

#ifndef YY_ATTRIBUTE_UNUSED
# define YY_ATTRIBUTE_UNUSED YY_ATTRIBUTE ((__unused__))
#endif

#if !defined _Noreturn \
     && (!defined __STDC_VERSION__ || __STDC_VERSION__ < 201112)
# if defined _MSC_VER && 1200 <= _MSC_VER
#  define _Noreturn __declspec (noreturn)
# else
#  define _Noreturn YY_ATTRIBUTE ((__noreturn__))
# endif
#endif

/* Suppress unused-variable warnings by "using" E.  */
#if ! defined lint || defined __GNUC__
# define YYUSE(E) ((void) (E))
#else
# define YYUSE(E) /* empty */
#endif

#if defined __GNUC__ && 407 <= __GNUC__ * 100 + __GNUC_MINOR__
/* Suppress an incorrect diagnostic about yylval being uninitialized.  */
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN \
    _Pragma ("GCC diagnostic push") \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")\
    _Pragma ("GCC diagnostic ignored \"-Wmaybe-uninitialized\"")
# define YY_IGNORE_MAYBE_UNINITIALIZED_END \
    _Pragma ("GCC diagnostic pop")
#else
# define YY_INITIAL_VALUE(Value) Value
#endif
#ifndef YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_END
#endif
#ifndef YY_INITIAL_VALUE
# define YY_INITIAL_VALUE(Value) /* Nothing. */
#endif

/* Debug traces.  */
#ifndef TYPEDRUBY24DEBUG
# if defined YYDEBUG
#if YYDEBUG
#   define TYPEDRUBY24DEBUG 1
#  else
#   define TYPEDRUBY24DEBUG 0
#  endif
# else /* ! defined YYDEBUG */
#  define TYPEDRUBY24DEBUG 0
# endif /* ! defined YYDEBUG */
#endif  /* ! defined TYPEDRUBY24DEBUG */

#line 26 "cc/grammars/typedruby24.ypp" // lalr1.cc:392
namespace ruby_parser { namespace bison { namespace typedruby24 {
#line 160 "cc/grammars/typedruby24.hh" // lalr1.cc:392





  /// A Bison parser.
  class parser
  {
  public:
#ifndef TYPEDRUBY24STYPE
    /// Symbol semantic values.
    typedef  union parser_value  semantic_type;
#else
    typedef TYPEDRUBY24STYPE semantic_type;
#endif

    /// Syntax errors thrown from user actions.
    struct syntax_error : std::runtime_error
    {
      syntax_error (const std::string& m);
    };

    /// Tokens.
    struct token
    {
      enum yytokentype
      {
        kCLASS = 1001,
        kMODULE = 1002,
        kDEF = 1003,
        kUNDEF = 1004,
        kBEGIN = 1005,
        kRESCUE = 1006,
        kENSURE = 1007,
        kEND = 1008,
        kIF = 1009,
        kUNLESS = 1010,
        kTHEN = 1011,
        kELSIF = 1012,
        kELSE = 1013,
        kCASE = 1014,
        kWHEN = 1015,
        kWHILE = 1016,
        kUNTIL = 1017,
        kFOR = 1018,
        kBREAK = 1019,
        kNEXT = 1020,
        kREDO = 1021,
        kRETRY = 1022,
        kIN = 1023,
        kDO = 1024,
        kDO_COND = 1025,
        kDO_BLOCK = 1026,
        kDO_LAMBDA = 1027,
        kRETURN = 1028,
        kYIELD = 1029,
        kSUPER = 1030,
        kSELF = 1031,
        kNIL = 1032,
        kTRUE = 1033,
        kFALSE = 1034,
        kAND = 1035,
        kOR = 1036,
        kNOT = 1037,
        kIF_MOD = 1038,
        kUNLESS_MOD = 1039,
        kWHILE_MOD = 1040,
        kUNTIL_MOD = 1041,
        kRESCUE_MOD = 1042,
        kALIAS = 1043,
        kDEFINED = 1044,
        klBEGIN = 1045,
        klEND = 1046,
        k__LINE__ = 1047,
        k__FILE__ = 1048,
        k__ENCODING__ = 1049,
        tIDENTIFIER = 1050,
        tFID = 1051,
        tGVAR = 1052,
        tIVAR = 1053,
        tCONSTANT = 1054,
        tLABEL = 1055,
        tCVAR = 1056,
        tNTH_REF = 1057,
        tBACK_REF = 1058,
        tSTRING_CONTENT = 1059,
        tINTEGER = 1060,
        tFLOAT = 1061,
        tUPLUS = 1062,
        tUMINUS = 1063,
        tUMINUS_NUM = 1064,
        tPOW = 1065,
        tCMP = 1066,
        tEQ = 1067,
        tEQQ = 1068,
        tNEQ = 1069,
        tEQL = 1070,
        tGEQ = 1071,
        tLEQ = 1072,
        tANDOP = 1073,
        tOROP = 1074,
        tMATCH = 1075,
        tNMATCH = 1076,
        tDOT = 1077,
        tDOT2 = 1078,
        tDOT3 = 1079,
        tAREF = 1080,
        tASET = 1081,
        tLSHFT = 1082,
        tRSHFT = 1083,
        tCOLON2 = 1084,
        tCOLON3 = 1085,
        tOP_ASGN = 1086,
        tASSOC = 1087,
        tLPAREN = 1088,
        tLPAREN2 = 1089,
        tRPAREN = 1090,
        tLPAREN_ARG = 1091,
        tLBRACK = 1092,
        tLBRACK2 = 1093,
        tRBRACK = 1094,
        tLBRACE = 1095,
        tLBRACE_ARG = 1096,
        tSTAR = 1097,
        tSTAR2 = 1098,
        tAMPER = 1099,
        tAMPER2 = 1100,
        tTILDE = 1101,
        tPERCENT = 1102,
        tDIVIDE = 1103,
        tDSTAR = 1104,
        tPLUS = 1105,
        tMINUS = 1106,
        tLT = 1107,
        tGT = 1108,
        tPIPE = 1109,
        tBANG = 1110,
        tCARET = 1111,
        tLCURLY = 1112,
        tRCURLY = 1113,
        tBACK_REF2 = 1114,
        tSYMBEG = 1115,
        tSTRING_BEG = 1116,
        tXSTRING_BEG = 1117,
        tREGEXP_BEG = 1118,
        tREGEXP_OPT = 1119,
        tWORDS_BEG = 1120,
        tQWORDS_BEG = 1121,
        tSYMBOLS_BEG = 1122,
        tQSYMBOLS_BEG = 1123,
        tSTRING_DBEG = 1124,
        tSTRING_DVAR = 1125,
        tSTRING_END = 1126,
        tSTRING_DEND = 1127,
        tSTRING = 1128,
        tSYMBOL = 1129,
        tNL = 1130,
        tEH = 1131,
        tCOLON = 1132,
        tCOMMA = 1133,
        tSPACE = 1134,
        tSEMI = 1135,
        tLAMBDA = 1136,
        tLAMBEG = 1137,
        tCHARACTER = 1138,
        tRATIONAL = 1139,
        tIMAGINARY = 1140,
        tLABEL_END = 1141,
        tANDDOT = 1142,
        tRATIONAL_IMAGINARY = 1143,
        tFLOAT_IMAGINARY = 1144,
        tLOWEST = 1146
      };
    };

    /// (External) token type, as returned by yylex.
    typedef token::yytokentype token_type;

    /// Symbol type: an internal symbol number.
    typedef int symbol_number_type;

    /// The symbol type number to denote an empty symbol.
    enum { empty_symbol = -2 };

    /// Internal symbol number for tokens (subsumed by symbol_number_type).
    typedef unsigned char token_number_type;

    /// A complete symbol.
    ///
    /// Expects its Base type to provide access to the symbol type
    /// via type_get().
    ///
    /// Provide access to semantic value.
    template <typename Base>
    struct basic_symbol : Base
    {
      /// Alias to Base.
      typedef Base super_type;

      /// Default constructor.
      basic_symbol ();

      /// Copy constructor.
      basic_symbol (const basic_symbol& other);

      /// Constructor for valueless symbols.
      basic_symbol (typename Base::kind_type t);

      /// Constructor for symbols with semantic value.
      basic_symbol (typename Base::kind_type t,
                    const semantic_type& v);

      /// Destroy the symbol.
      ~basic_symbol ();

      /// Destroy contents, and record that is empty.
      void clear ();

      /// Whether empty.
      bool empty () const;

      /// Destructive move, \a s is emptied into this.
      void move (basic_symbol& s);

      /// The semantic value.
      semantic_type value;

    private:
      /// Assignment operator.
      basic_symbol& operator= (const basic_symbol& other);
    };

    /// Type access provider for token (enum) based symbols.
    struct by_type
    {
      /// Default constructor.
      by_type ();

      /// Copy constructor.
      by_type (const by_type& other);

      /// The symbol type as needed by the constructor.
      typedef token_type kind_type;

      /// Constructor from (external) token numbers.
      by_type (kind_type t);

      /// Record that this symbol is empty.
      void clear ();

      /// Steal the symbol type from \a that.
      void move (by_type& that);

      /// The (internal) type number (corresponding to \a type).
      /// \a empty when empty.
      symbol_number_type type_get () const;

      /// The token.
      token_type token () const;

      /// The symbol type.
      /// \a empty_symbol when empty.
      /// An int, not token_number_type, to be able to store empty_symbol.
      int type;
    };

    /// "External" symbols: returned by the scanner.
    typedef basic_symbol<by_type> symbol_type;


    /// Build a parser object.
    parser (ruby_parser::typedruby24& driver_yyarg, ruby_parser::self_ptr self_yyarg);
    virtual ~parser ();

    /// Parse.
    /// \returns  0 iff parsing succeeded.
    virtual int parse ();

#if TYPEDRUBY24DEBUG
    /// The current debugging stream.
    std::ostream& debug_stream () const YY_ATTRIBUTE_PURE;
    /// Set the current debugging stream.
    void set_debug_stream (std::ostream &);

    /// Type for debugging levels.
    typedef int debug_level_type;
    /// The current debugging level.
    debug_level_type debug_level () const YY_ATTRIBUTE_PURE;
    /// Set the current debugging level.
    void set_debug_level (debug_level_type l);
#endif

    /// Report a syntax error.
    /// \param msg    a description of the syntax error.
    virtual void error (const std::string& msg);

    /// Report a syntax error.
    void error (const syntax_error& err);

  private:
    /// This class is not copyable.
    parser (const parser&);
    parser& operator= (const parser&);

    /// State numbers.
    typedef int state_type;

    /// Generate an error message.
    /// \param yystate   the state where the error occurred.
    /// \param yyla      the lookahead token.
    virtual std::string yysyntax_error_ (state_type yystate,
                                         const symbol_type& yyla) const;

    /// Compute post-reduction state.
    /// \param yystate   the current state
    /// \param yysym     the nonterminal to push on the stack
    state_type yy_lr_goto_state_ (state_type yystate, int yysym);

    /// Whether the given \c yypact_ value indicates a defaulted state.
    /// \param yyvalue   the value to check
    static bool yy_pact_value_is_default_ (int yyvalue);

    /// Whether the given \c yytable_ value indicates a syntax error.
    /// \param yyvalue   the value to check
    static bool yy_table_value_is_error_ (int yyvalue);

    static const short int yypact_ninf_;
    static const short int yytable_ninf_;

    /// Convert a scanner token number \a t to a symbol number.
    static token_number_type yytranslate_ (int t);

    // Tables.
  // YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
  // STATE-NUM.
  static const short int yypact_[];

  // YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
  // Performed when YYTABLE does not specify something else to do.  Zero
  // means the default is an error.
  static const unsigned short int yydefact_[];

  // YYPGOTO[NTERM-NUM].
  static const short int yypgoto_[];

  // YYDEFGOTO[NTERM-NUM].
  static const short int yydefgoto_[];

  // YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
  // positive, shift that token.  If negative, reduce the rule whose
  // number is the opposite.  If YYTABLE_NINF, syntax error.
  static const short int yytable_[];

  static const short int yycheck_[];

  // YYSTOS[STATE-NUM] -- The (internal number of the) accessing
  // symbol of state STATE-NUM.
  static const unsigned short int yystos_[];

  // YYR1[YYN] -- Symbol number of symbol that rule YYN derives.
  static const unsigned short int yyr1_[];

  // YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.
  static const unsigned char yyr2_[];


#if TYPEDRUBY24DEBUG
    /// For a symbol, its name in clear.
    static const char* const yytname_[];

  // YYRLINE[YYN] -- Source line where rule number YYN was defined.
  static const unsigned short int yyrline_[];
    /// Report on the debug stream that the rule \a r is going to be reduced.
    virtual void yy_reduce_print_ (int r);
    /// Print the state stack on the debug stream.
    virtual void yystack_print_ ();

    // Debugging.
    int yydebug_;
    std::ostream* yycdebug_;

    /// \brief Display a symbol type, value and location.
    /// \param yyo    The output stream.
    /// \param yysym  The symbol.
    template <typename Base>
    void yy_print_ (std::ostream& yyo, const basic_symbol<Base>& yysym) const;
#endif

    /// \brief Reclaim the memory associated to a symbol.
    /// \param yymsg     Why this token is reclaimed.
    ///                  If null, print nothing.
    /// \param yysym     The symbol.
    template <typename Base>
    void yy_destroy_ (const char* yymsg, basic_symbol<Base>& yysym) const;

  private:
    /// Type access provider for state based symbols.
    struct by_state
    {
      /// Default constructor.
      by_state ();

      /// The symbol type as needed by the constructor.
      typedef state_type kind_type;

      /// Constructor.
      by_state (kind_type s);

      /// Copy constructor.
      by_state (const by_state& other);

      /// Record that this symbol is empty.
      void clear ();

      /// Steal the symbol type from \a that.
      void move (by_state& that);

      /// The (internal) type number (corresponding to \a state).
      /// \a empty_symbol when empty.
      symbol_number_type type_get () const;

      /// The state number used to denote an empty symbol.
      enum { empty_state = -1 };

      /// The state.
      /// \a empty when empty.
      state_type state;
    };

    /// "Internal" symbol: element of the stack.
    struct stack_symbol_type : basic_symbol<by_state>
    {
      /// Superclass.
      typedef basic_symbol<by_state> super_type;
      /// Construct an empty symbol.
      stack_symbol_type ();
      /// Steal the contents from \a sym to build this.
      stack_symbol_type (state_type s, symbol_type& sym);
      /// Assignment, needed by push_back.
      stack_symbol_type& operator= (const stack_symbol_type& that);
    };

    /// Stack type.
    typedef stack<stack_symbol_type> stack_type;

    /// The stack.
    stack_type yystack_;

    /// Push a new state on the stack.
    /// \param m    a debug message to display
    ///             if null, no trace is output.
    /// \param s    the symbol
    /// \warning the contents of \a s.value is stolen.
    void yypush_ (const char* m, stack_symbol_type& s);

    /// Push a new look ahead token on the state on the stack.
    /// \param m    a debug message to display
    ///             if null, no trace is output.
    /// \param s    the state
    /// \param sym  the symbol (for its value and location).
    /// \warning the contents of \a s.value is stolen.
    void yypush_ (const char* m, state_type s, symbol_type& sym);

    /// Pop \a n symbols the three stacks.
    void yypop_ (unsigned int n = 1);

    /// Constants.
    enum
    {
      yyeof_ = 0,
      yylast_ = 12019,     ///< Last index in yytable_.
      yynnts_ = 198,  ///< Number of nonterminal symbols.
      yyfinal_ = 332, ///< Termination state number.
      yyterror_ = 1,
      yyerrcode_ = 256,
      yyntokens_ = 148  ///< Number of tokens.
    };


    // User arguments.
    ruby_parser::typedruby24& driver;
    ruby_parser::self_ptr self;
  };


#line 26 "cc/grammars/typedruby24.ypp" // lalr1.cc:392
} } } // ruby_parser::bison::typedruby24
#line 648 "cc/grammars/typedruby24.hh" // lalr1.cc:392




#endif // !YY_TYPEDRUBY24_CC_GRAMMARS_TYPEDRUBY24_HH_INCLUDED
