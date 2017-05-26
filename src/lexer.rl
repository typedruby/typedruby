/*
Copyright (c) 2013-2016 whitequark  <whitequark@whitequark.org>

Parts of the source are derived from ruby_parser:
Copyright (c) Ryan Davis, seattle.rb

This lexer is a rewrite of the original in Ragel/C:
Copyright (c) Hailey Somerville, GitHub

MIT License

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

%%machine lex; # % fix highlighting

/*
#
# === BEFORE YOU START ===
#
# Read the Ruby Hacking Guide chapter 11, available in English at
# http://whitequark.org/blog/2013/04/01/ruby-hacking-guide-ch-11-finite-state-lexer/
#
# Remember two things about Ragel scanners:
#
#   1) Longest match wins.
#
#   2) If two matches have the same length, the first
#      in source code wins.
#
# General rules of making Ragel and Bison happy:
#
#  * `p` (position) and `@te` contain the index of the character
#    they're pointing to ("current"), plus one. `@ts` contains the index
#    of the corresponding character. The code for extracting matched token is:
#
#       @source_buffer.slice(@ts...@te)
#
#  * If your input is `foooooooobar` and the rule is:
#
#       'f' 'o'+
#
#    the result will be:
#
#       foooooooobar
#       ^ ts=0   ^ p=te=9
#
#  * A Ragel lexer action should not emit more than one token, unless
#    you know what you are doing.
#
#  * All Ragel commands (fnext, fgoto, ...) end with a semicolon.
#
#  * If an action emits the token and transitions to another state, use
#    these Ragel commands:
#
#       emit($whatever)
#       fnext $next_state; fbreak;
#
#    If you perform `fgoto` in an action which does not emit a token nor
#    rewinds the stream pointer, the parser's side-effectful,
#    context-sensitive lookahead actions will break in a hard to detect
#    and debug way.
#
#  * If an action does not emit a token:
#
#       fgoto $next_state;
#
#  * If an action features lookbehind, i.e. matches characters with the
#    intent of passing them to another action:
#
#       p = @ts - 1
#       fgoto $next_state;
#
#    or, if the lookbehind consists of a single character:
#
#       fhold; fgoto $next_state;
#
#  * Ragel merges actions. So, if you have `e_lparen = '(' %act` and
#    `c_lparen = '('` and a lexer action `e_lparen | c_lparen`, the result
#    _will_ invoke the action `act`.
#
#    e_something stands for "something with **e**mbedded action".
#
#  * EOF is explicit and is matched by `c_eof`. If you want to introspect
#    the state of the lexer, add this rule to the state:
#
#       c_eof => do_eof;
#
#  * If you proceed past EOF, the lexer will complain:
#
#       NoMethodError: undefined method `ord' for nil:NilClass
#
*/

#include <ruby_parser/lexer.hh>
#include <cassert>

%% write data nofinal;

using namespace ruby_parser;
using namespace std::string_literals;

%% prepush { check_stack_capacity(); }

lexer::lexer(parser::base& parser, ruby_version version, const std::string& source_buffer_)
  : parser(parser)
  , version(version)
  , source_buffer(source_buffer_ + std::string("\0\0", 2))
  , cs(lex_en_line_begin)
  , _p(source_buffer.data())
  , _pe(source_buffer.data() + source_buffer.size())
  , ts(nullptr)
  , te(nullptr)
  , act(0)
  , top(0)
  , eq_begin_s(nullptr)
  , sharp_s(nullptr)
  , newline_s(nullptr)
  , paren_nest(0)
  , command_state(false)
  , num_base(0)
  , num_digits_s(nullptr)
  , num_suffix_s(nullptr)
  , num_xfrm(num_xfrm_type::NONE)
  , escape_s(nullptr)
  , herebody_s(nullptr)
  , in_kwarg(false)
{
  // ensure the stack capacity is non-zero so we can just double in
  // check_stack_capacity:
  stack.reserve(16);

  static_env.push(environment());
}

void lexer::check_stack_capacity() {
  if (stack.size() == stack.capacity()) {
    stack.reserve(stack.capacity() * 2);
  }
}

int lexer::stack_pop() {
  return stack[--top];
}

int lexer::arg_or_cmdarg() {
  if (command_state) {
    return lex_en_expr_cmdarg;
  } else {
    return lex_en_expr_arg;
  }
}

void lexer::emit_comment(const char* s, const char* e) {
  /* unused for now */
  (void)s;
  (void)e;
}

std::string lexer::tok() {
  return tok(ts);
}

std::string lexer::tok(const char* start) {
  return tok(start, te);
}

std::string lexer::tok(const char* start, const char* end) {
  assert(start <= end);

  return std::string(start, (size_t)(end - start));
}

static const lexer::token_table PUNCTUATION = {
  { "=", token_type::tEQL },
  { "&", token_type::tAMPER2 },
  { "|", token_type::tPIPE },
  { "!", token_type::tBANG },
  { "^", token_type::tCARET },
  { "+", token_type::tPLUS },
  { "-", token_type::tMINUS },
  { "*", token_type::tSTAR2 },
  { "/", token_type::tDIVIDE },
  { "%", token_type::tPERCENT },
  { "~", token_type::tTILDE },
  { ",", token_type::tCOMMA },
  { ";", token_type::tSEMI },
  { ".", token_type::tDOT },
  { "..", token_type::tDOT2 },
  { "...", token_type::tDOT3 },
  { "[", token_type::tLBRACK2 },
  { "]", token_type::tRBRACK },
  { "(", token_type::tLPAREN2 },
  { ")", token_type::tRPAREN },
  { "?", token_type::tEH },
  { ":", token_type::tCOLON },
  { "&&", token_type::tANDOP },
  { "||", token_type::tOROP },
  { "-@", token_type::tUMINUS },
  { "+@", token_type::tUPLUS },
  { "~@", token_type::tTILDE },
  { "**", token_type::tPOW },
  { "->", token_type::tLAMBDA },
  { "=~", token_type::tMATCH },
  { "!~", token_type::tNMATCH },
  { "==", token_type::tEQ },
  { "!=", token_type::tNEQ },
  { ">", token_type::tGT },
  { ">>", token_type::tRSHFT },
  { ">=", token_type::tGEQ },
  { "<", token_type::tLT },
  { "<<", token_type::tLSHFT },
  { "<=", token_type::tLEQ },
  { "=>", token_type::tASSOC },
  { "::", token_type::tCOLON2 },
  { "===", token_type::tEQQ },
  { "<=>", token_type::tCMP },
  { "[]", token_type::tAREF },
  { "[]=", token_type::tASET },
  { "{", token_type::tLCURLY },
  { "}", token_type::tRCURLY },
  { "`", token_type::tBACK_REF2 },
  { "!@", token_type::tBANG },
  { "&.", token_type::tANDDOT },
};

static const lexer::token_table PUNCTUATION_BEGIN = {
  { "&", token_type::tAMPER },
  { "*", token_type::tSTAR },
  { "**", token_type::tDSTAR },
  { "+", token_type::tUPLUS },
  { "-", token_type::tUMINUS },
  { "::", token_type::tCOLON3 },
  { "(", token_type::tLPAREN },
  { "{", token_type::tLBRACE },
  { "[", token_type::tLBRACK },
};

static const lexer::token_table KEYWORDS = {
  { "if", token_type::kIF_MOD },
  { "unless", token_type::kUNLESS_MOD },
  { "while", token_type::kWHILE_MOD },
  { "until", token_type::kUNTIL_MOD },
  { "rescue", token_type::kRESCUE_MOD },
  { "defined?", token_type::kDEFINED },
  { "BEGIN", token_type::klBEGIN },
  { "END", token_type::klEND },
  { "class", token_type::kCLASS },
  { "module", token_type::kMODULE },
  { "def", token_type::kDEF },
  { "undef", token_type::kUNDEF },
  { "begin", token_type::kBEGIN },
  { "end", token_type::kEND },
  { "then", token_type::kTHEN },
  { "elsif", token_type::kELSIF },
  { "else", token_type::kELSE },
  { "ensure", token_type::kENSURE },
  { "case", token_type::kCASE },
  { "when", token_type::kWHEN },
  { "for", token_type::kFOR },
  { "break", token_type::kBREAK },
  { "next", token_type::kNEXT },
  { "redo", token_type::kREDO },
  { "retry", token_type::kRETRY },
  { "in", token_type::kIN },
  { "do", token_type::kDO },
  { "return", token_type::kRETURN },
  { "yield", token_type::kYIELD },
  { "super", token_type::kSUPER },
  { "self", token_type::kSELF },
  { "nil", token_type::kNIL },
  { "true", token_type::kTRUE },
  { "false", token_type::kFALSE },
  { "and", token_type::kAND },
  { "or", token_type::kOR },
  { "not", token_type::kNOT },
  { "alias", token_type::kALIAS },
  { "__FILE__", token_type::k__FILE__ },
  { "__LINE__", token_type::k__LINE__ },
  { "__ENCODING__", token_type::k__ENCODING__ },
};

static const lexer::token_table KEYWORDS_BEGIN = {
  { "if", token_type::kIF },
  { "unless", token_type::kUNLESS },
  { "while", token_type::kWHILE },
  { "until", token_type::kUNTIL },
  { "rescue", token_type::kRESCUE },
  { "defined?", token_type::kDEFINED },
  { "class", token_type::kCLASS },
  { "module", token_type::kMODULE },
  { "def", token_type::kDEF },
  { "undef", token_type::kUNDEF },
  { "begin", token_type::kBEGIN },
  { "end", token_type::kEND },
  { "then", token_type::kTHEN },
  { "elsif", token_type::kELSIF },
  { "else", token_type::kELSE },
  { "ensure", token_type::kENSURE },
  { "case", token_type::kCASE },
  { "when", token_type::kWHEN },
  { "for", token_type::kFOR },
  { "break", token_type::kBREAK },
  { "next", token_type::kNEXT },
  { "redo", token_type::kREDO },
  { "retry", token_type::kRETRY },
  { "in", token_type::kIN },
  { "do", token_type::kDO },
  { "return", token_type::kRETURN },
  { "yield", token_type::kYIELD },
  { "super", token_type::kSUPER },
  { "self", token_type::kSELF },
  { "nil", token_type::kNIL },
  { "true", token_type::kTRUE },
  { "false", token_type::kFALSE },
  { "and", token_type::kAND },
  { "or", token_type::kOR },
  { "not", token_type::kNOT },
  { "alias", token_type::kALIAS },
  { "__FILE__", token_type::k__FILE__ },
  { "__LINE__", token_type::k__LINE__ },
  { "__ENCODING__", token_type::k__ENCODING__ },
};

static std::string gsub(const std::string&& str, const std::string&& search, const std::string&& replace) {
  std::string result;

  std::string::size_type from = 0;

  while (true) {
    auto index = str.find(search, from);

    if (index == std::string::npos) {
      result += str.substr(from);
      break;
    } else {
      result += str.substr(from, index - from);
      result += replace;
      from = index + search.size();
    }
  }

  return result;
}

static bool eof_codepoint(char c) {
  return c == 0 || c == 0x04 || c == 0x1a;
}

token_t lexer::advance_() {
  if (!token_queue.empty()) {
    token_t token = token_queue.front();
    token_queue.pop();
    return token;
  }

  command_state = (cs == lex_en_expr_value || cs == lex_en_line_begin);

  const char* p = _p;
  const char* pe = _pe;
  const char* eof = _pe;

  const char* tm = NULL;
  const char* heredoc_e = NULL;
  const char* new_herebody_s = NULL;

  %% write exec;

  _p = p;

  if (!token_queue.empty()) {
    token_t token = token_queue.front();
    token_queue.pop();
    return token;
  }

  if (cs == lex_error) {
    size_t start = (size_t)(p - source_buffer.data());
    return std::make_shared<token>(token_type::error, start, start + 1, std::string(p - 1, 1));
  }

  return std::make_shared<token>(token_type::eof, source_buffer.size(), source_buffer.size(), "");
}

void lexer::emit(token_type type) {
  emit(type, tok());
}

void lexer::emit(token_type type, const std::string& str) {
  emit(type, str, ts, te);
}

void lexer::emit(token_type type, const std::string& str, const char* start, const char* end) {
  size_t offset_start = (size_t)(start - source_buffer.data());
  size_t offset_end = (size_t)(end - source_buffer.data());

  token_queue.push(std::make_shared<token>(type, offset_start, offset_end, str));
}

void lexer::emit_do(bool do_block) {
  if (cond.active()) {
    emit(token_type::kDO_COND, "do");
  } else if (cmdarg.active() || do_block) {
    emit(token_type::kDO_BLOCK, "do");
  } else {
    emit(token_type::kDO, "do");
  }
}

void lexer::emit_table(const token_table& table) {
  auto value = tok();
  emit(table.at(value), value);
}

void lexer::emit_num(const std::string& num) {
  switch (num_xfrm) {
    case num_xfrm_type::NONE:
      emit(token_type::tINTEGER, num);
      break;
    case num_xfrm_type::RATIONAL:
      emit(token_type::tRATIONAL, num);
      break;
    case num_xfrm_type::IMAGINARY:
      emit(token_type::tIMAGINARY, num);
      break;
    case num_xfrm_type::RATIONAL_IMAGINARY:
      emit(token_type::tRATIONAL_IMAGINARY, num);
      break;
    case num_xfrm_type::FLOAT:
      emit(token_type::tFLOAT, num);
      break;
    case num_xfrm_type::FLOAT_IMAGINARY:
      emit(token_type::tFLOAT_IMAGINARY, num);
      break;
  }
}

void lexer::diagnostic_(diagnostic_level level, std::string&& message) {
  diagnostic_(level, std::forward<std::string>(message), ts, te);
}

void lexer::diagnostic_(diagnostic_level level, std::string&& message, const char* start, const char* end) {
  size_t token_start = (size_t)(start - source_buffer.data());
  size_t token_end = (size_t)(end - source_buffer.data());

  parser.diagnostic_(level, std::forward<std::string>(message), diagnostic::range(token_start, token_end));
}

//
// === LITERAL STACK ===
//

template<typename... Args>
int lexer::push_literal(Args&&... args) {
  literal_stack.emplace(*this, std::forward<Args>(args)...);

  auto& literal = literal_stack.top();

  if (literal.words() && literal.backslash_delimited()) {
    if (literal.interpolate()) {
      return lex_en_interp_backslash_delimited_words;
    } else {
      return lex_en_plain_backslash_delimited_words;
    }
  } else if (literal.words() && !literal.backslash_delimited()) {
    if (literal.interpolate()) {
      return lex_en_interp_words;
    } else {
      return lex_en_plain_words;
    }
  } else if (!literal.words() && literal.backslash_delimited()) {
    if (literal.interpolate()) {
      return lex_en_interp_backslash_delimited;
    } else {
      return lex_en_plain_backslash_delimited;
    }
  } else {
    if (literal.interpolate()) {
      return lex_en_interp_string;
    } else {
      return lex_en_plain_string;
    }
  }
}

literal& lexer::literal_() {
  return literal_stack.top();
}

int lexer::pop_literal() {
  bool was_regexp;

  {
    auto& old_literal = literal_stack.top();

    was_regexp = old_literal.regexp();
    dedent_level_ = old_literal.dedent_level();
  }

  literal_stack.pop();

  if (was_regexp) {
    return lex_en_regexp_modifiers;
  } else {
    return lex_en_expr_end;
  }
}

void lexer::set_state_expr_beg() {
  cs = lex_en_expr_beg;
}

void lexer::set_state_expr_endarg() {
  cs = lex_en_expr_endarg;
}

void lexer::set_state_expr_fname() {
  cs = lex_en_expr_fname;
}

void lexer::set_state_expr_value() {
  cs = lex_en_expr_value;
}

%%{
  # access @;
  # getkey (@source_pts[p] || 0);

  # === CHARACTER CLASSES ===
  #
  # Pay close attention to the differences between c_any and any.
  # c_any does not include EOF and so will cause incorrect behavior
  # for machine subtraction (any-except rules) and default transitions
  # for scanners.

  action do_nl {
    // Record position of a newline for precise location reporting on tNL
    // tokens.
    //
    // This action is embedded directly into c_nl, as it is idempotent and
    // there are no cases when we need to skip it.
    newline_s = p;
  }

  c_nl       = '\n' $ do_nl;
  c_space    = [ \t\r\f\v];
  c_space_nl = c_space | c_nl;

  c_eof      = 0x04 | 0x1a | 0 | zlen; # ^D, ^Z, \0, EOF
  c_eol      = c_nl | c_eof;
  c_any      = any - c_eof;

  c_nl_zlen  = c_nl | zlen;
  c_line     = any - c_nl_zlen;

  c_unicode  = c_any - 0x00..0x7f;
  c_upper    = [A-Z];
  c_lower    = [a-z_]  | c_unicode;
  c_alpha    = c_lower | c_upper;
  c_alnum    = c_alpha | [0-9];

  action do_eof {
    // Sit at EOF indefinitely. #advance would return $eof each time.
    // This allows to feed the lexer more data if needed; this is only used
    // in tests.
    //
    // Note that this action is not embedded into e_eof like e_heredoc_nl and e_bs
    // below. This is due to the fact that scanner state at EOF is observed
    // by tests, and encapsulating it in a rule would break the introspection.
    fhold; fbreak;
  }

  #
  # === TOKEN DEFINITIONS ===
  #

  # All operators are punctuation. There is more to punctuation
  # than just operators. Operators can be overridden by user;
  # punctuation can not.

  # A list of operators which are valid in the function name context, but
  # have different semantics in others.
  operator_fname      = '[]' | '[]=' | '`'  | '-@' | '+@' | '~@'  | '!@' ;

  # A list of operators which can occur within an assignment shortcut (+ → +=).
  operator_arithmetic = '&'  | '|'   | '&&' | '||' | '^'  | '+'   | '-'  |
                        '*'  | '/'   | '**' | '~'  | '<<' | '>>'  | '%'  ;

  # A list of all user-definable operators not covered by groups above.
  operator_rest       = '=~' | '!~' | '==' | '!=' | '!'   | '===' |
                        '<'  | '<=' | '>'  | '>=' | '<=>' | '=>'  ;

  # Note that `{` and `}` need to be referred to as e_lbrace and e_rbrace,
  # as they are ambiguous with interpolation `#{}` and should be counted.
  # These braces are not present in punctuation lists.

  # A list of punctuation which has different meaning when used at the
  # beginning of expression.
  punctuation_begin   = '-'  | '+'  | '::' | '('  | '['  |
                        '*'  | '**' | '&'  ;

  # A list of all punctuation except punctuation_begin.
  punctuation_end     = ','  | '='  | '->' | '('  | '['  | ']'   |
                        '::' | '?'  | ':'  | '.'  | '..' | '...' ;

  # A list of keywords which have different meaning at the beginning of expression.
  keyword_modifier    = 'if'     | 'unless' | 'while'  | 'until' | 'rescue' ;

  # A list of keywords which accept an argument-like expression, i.e. have the
  # same post-processing as method calls or commands. Example: `yield 1`,
  # `yield (1)`, `yield(1)`, are interpreted as if `yield` was a function.
  keyword_with_arg    = 'yield'  | 'super'  | 'not'    | 'defined?' ;

  # A list of keywords which accept a literal function name as an argument.
  keyword_with_fname  = 'def'    | 'undef'  | 'alias'  ;

  # A list of keywords which accept an expression after them.
  keyword_with_value  = 'else'   | 'case'   | 'ensure' | 'module' | 'elsif' | 'then'  |
                        'for'    | 'in'     | 'do'     | 'when'   | 'begin' | 'class' |
                        'and'    | 'or'     ;

  # A list of keywords which accept a value, and treat the keywords from
  # `keyword_modifier` list as modifiers.
  keyword_with_mid    = 'rescue' | 'return' | 'break'  | 'next'   ;

  # A list of keywords which do not accept an expression after them.
  keyword_with_end    = 'end'    | 'self'   | 'true'   | 'false'  | 'retry'    |
                        'redo'   | 'nil'    | 'BEGIN'  | 'END'    | '__FILE__' |
                        '__LINE__' | '__ENCODING__';

  # All keywords.
  keyword             = keyword_with_value | keyword_with_mid |
                        keyword_with_end   | keyword_with_arg |
                        keyword_with_fname | keyword_modifier ;

  constant       = c_upper c_alnum*;
  bareword       = c_alpha c_alnum*;

  call_or_var    = c_lower c_alnum*;
  class_var      = '@@' bareword;
  instance_var   = '@' bareword;
  global_var     = '$'
      ( bareword | digit+
      | [`'+~*$&?!@/\\;,.=:<>"] # `
      | '-' c_alnum
      )
  ;

  # Ruby accepts (and fails on) variables with leading digit
  # in literal context, but not in unquoted symbol body.
  class_var_v    = '@@' c_alnum+;
  instance_var_v = '@' c_alnum+;

  label          = bareword [?!]? ':';

  #
  # === NUMERIC PARSING ===
  #

  int_hex  = ( xdigit+ '_' )* xdigit* '_'? ;
  int_dec  = ( digit+ '_' )* digit* '_'? ;
  int_bin  = ( [01]+ '_' )* [01]* '_'? ;

  flo_int  = [1-9] [0-9]* ( '_' digit+ )* | '0';
  flo_frac = '.' ( digit+ '_' )* digit+;
  flo_pow  = [eE] [+\-]? ( digit+ '_' )* digit+;

  int_suffix =
    ''   % { num_xfrm = num_xfrm_type::NONE; }
  | 'r'  % { num_xfrm = num_xfrm_type::RATIONAL; }
  | 'i'  % { num_xfrm = num_xfrm_type::IMAGINARY; }
  | 'ri' % { num_xfrm = num_xfrm_type::RATIONAL_IMAGINARY; };

  flo_pow_suffix =
    ''   % { num_xfrm = num_xfrm_type::FLOAT; }
  | 'i'  % { num_xfrm = num_xfrm_type::FLOAT_IMAGINARY; };

  flo_suffix =
    flo_pow_suffix
  | 'r'  % { num_xfrm = num_xfrm_type::RATIONAL; }
  | 'ri' % { num_xfrm = num_xfrm_type::RATIONAL_IMAGINARY; };

  #
  # === ESCAPE SEQUENCE PARSING ===
  #

  # Escape parsing code is a Ragel pattern, not a scanner, and therefore
  # it shouldn't directly raise errors or perform other actions with side effects.
  # In reality this would probably just mess up error reporting in pathological
  # cases, through.

  # The amount of code required to parse \M\C stuff correctly is ridiculous.

  escaped_nl = "\\" c_nl;

  action unicode_points {
    /* TODO
    @escape = ""

    codepoints  = tok(@escape_s + 2, p - 1)
    codepoint_s = @escape_s + 2

    codepoints.split(/[ \t]/).each do |codepoint_str|
      codepoint = codepoint_str.to_i(16)

      if codepoint >= 0x110000
        diagnostic :error, :unicode_point_too_large, nil,
                   range(codepoint_s, codepoint_s + codepoint_str.length)
        break
      end

      @escape     += codepoint.chr(Encoding::UTF_8)
      codepoint_s += codepoint_str.length + 1
    end
    */
  }

  action unescape_char {
    /* TODO
    codepoint = @source_pts[p - 1]
    if (@escape = ESCAPES[codepoint]).nil?
      @escape = encode_escape(@source_buffer.slice(p - 1))
    end
    */
  }

  action invalid_complex_escape {
    diagnostic_(diagnostic_level::FATAL, "invalid escape character syntax"s);
  }

  action slash_c_char {
    // TODO multibyte
    char c = escape->at(0) & 0x9f;
    escape = std::make_unique<std::string>(&c, 1);
  }

  action slash_m_char {
    // TODO multibyte
    char c = escape->at(0) | 0x80;
    escape = std::make_unique<std::string>(&c, 1);
  }

  maybe_escaped_char = (
        '\\' c_any      %unescape_char
    | ( c_any - [\\] )  % { escape = std::make_unique<std::string>(p - 1, 1); /* TODO multibyte */ }
  );

  maybe_escaped_ctrl_char = ( # why?!
        '\\' c_any      %unescape_char %slash_c_char
    |   '?'             % { escape = std::make_unique<std::string>("\x7f"); }
    | ( c_any - [\\?] ) % { escape = std::make_unique<std::string>(p - 1, 1); /* TODO multibyte */ } %slash_c_char
  );

  escape = (
      # \377
      [0-7]{1,3}
      % { /* TODO @escape = encode_escape(tok(@escape_s, p).to_i(8) % 0x100) */ }

      # \xff
    | 'x' xdigit{1,2}
        % { /* TODO @escape = encode_escape(tok(@escape_s + 1, p).to_i(16)) */ }

      # \u263a
    | 'u' xdigit{4}
      % { /* TODO @escape = tok(@escape_s + 1, p).to_i(16).chr(Encoding::UTF_8) */ }

      # %q[\x]
    | 'x' ( c_any - xdigit )
      % {
        diagnostic_(diagnostic_level::FATAL, "invalid hex escape"s, escape_s - 1, p + 2);
      }

      # %q[\u123] %q[\u{12]
    | 'u' ( c_any{0,4}  -
            xdigit{4}   -            # \u1234 is valid
            ( '{' xdigit{1,3}        # \u{1 \u{12 \u{123 are valid
            | '{' xdigit [ \t}] any? # \u{1. \u{1} are valid
            | '{' xdigit{2} [ \t}]   # \u{12. \u{12} are valid
            )
          )
      % {
        diagnostic_(diagnostic_level::FATAL, "invalid Unicode escape"s, escape_s - 1, p);
      }

      # \u{123 456}
    | 'u{' ( xdigit{1,6} [ \t] )*
      ( xdigit{1,6} '}'
        %unicode_points
      | ( xdigit* ( c_any - xdigit - '}' )+ '}'
        | ( c_any - '}' )* c_eof
        | xdigit{7,}
        ) % {
          diagnostic_(diagnostic_level::FATAL, "unterminated Unicode escape"s, p - 1, p);
        }
      )

      # \C-\a \cx
    | ( 'C-' | 'c' ) escaped_nl?
      maybe_escaped_ctrl_char

      # \M-a
    | 'M-' escaped_nl?
      maybe_escaped_char
      %slash_m_char

      # \C-\M-f \M-\cf \c\M-f
    | ( ( 'C-'   | 'c' ) escaped_nl?   '\\M-'
      |   'M-\\'         escaped_nl? ( 'C-'   | 'c' ) ) escaped_nl?
      maybe_escaped_ctrl_char
      %slash_m_char

    | 'C' c_any %invalid_complex_escape
    | 'M' c_any %invalid_complex_escape
    | ( 'M-\\C' | 'C-\\M' ) c_any %invalid_complex_escape

    | ( c_any - [0-7xuCMc] ) %unescape_char

    | c_eof % {
      diagnostic_(diagnostic_level::FATAL, "escape sequence meets end of file", p - 1, p);
    }
  );

  # Use rules in form of `e_bs escape' when you need to parse a sequence.
  e_bs = '\\' % {
    escape_s = p;
    escape   = nullptr;
  };

  #
  # === STRING AND HEREDOC PARSING ===
  #

  # Heredoc parsing is quite a complex topic. First, consider that heredocs
  # can be arbitrarily nested. For example:
  #
  #     puts <<CODE
  #     the result is: #{<<RESULT.inspect
  #       i am a heredoc
  #     RESULT
  #     }
  #     CODE
  #
  # which, incidentally, evaluates to:
  #
  #     the result is: "  i am a heredoc\n"
  #
  # To parse them, lexer refers to two kinds (remember, nested heredocs)
  # of positions in the input stream, namely heredoc_e
  # (HEREDOC declaration End) and @herebody_s (HEREdoc BODY line Start).
  #
  # heredoc_e is simply contained inside the corresponding Literal, and
  # when the heredoc is closed, the lexing is restarted from that position.
  #
  # @herebody_s is quite more complex. First, @herebody_s changes after each
  # heredoc line is lexed. This way, at '\n' tok(@herebody_s, @te) always
  # contains the current line, and also when a heredoc is started, @herebody_s
  # contains the position from which the heredoc will be lexed.
  #
  # Second, as (insanity) there are nested heredocs, we need to maintain a
  # stack of these positions. Each time #push_literal is called, it saves current
  # @heredoc_s to literal.saved_herebody_s, and after an interpolation (possibly
  # containing another heredocs) is closed, the previous value is restored.

  e_heredoc_nl = c_nl % {
    // After every heredoc was parsed, herebody_s contains the
    // position of next token after all heredocs.
    if (herebody_s) {
      p = herebody_s;
      herebody_s = NULL;
    }
  };

  action extend_string {
    auto str = tok();
    std::string lookahead;

    // tLABEL_END is only possible in non-cond context on >= 2.2
    if (version >= ruby_version::RUBY_22 && !cond.active()) {
      const char* lookahead_s = te;
      const char* lookahead_e = te + 2;

      if (lookahead_e > eof) {
        lookahead_e = eof;
      }

      lookahead = std::string(lookahead_s, (size_t)(lookahead_e - lookahead_s));
    }

    auto& current_literal = literal_();

    if (!current_literal.heredoc() && current_literal.nest_and_try_closing(str, ts, te, lookahead)) {
      if (token_queue.back()->type() == token_type::tLABEL_END) {
        p += 1;
        pop_literal();
        fnext expr_labelarg;
      } else {
        fnext *pop_literal();
      }
      fbreak;
    } else {
      current_literal.extend_string(str, ts, te);
    }
  }

  action extend_string_escaped {
    auto& current_literal = literal_();

    // TODO multibyte
    auto escaped_char = *escape_s;

    if (current_literal.munge_escape(escaped_char)) {
      // If this particular literal uses this character as an opening
      // or closing delimiter, it is an escape sequence for that
      // particular character. Write it without the backslash.

      if (current_literal.regexp()
          && (escaped_char == '\\' ||
              escaped_char == '$'  ||
              escaped_char == '$'  ||
              escaped_char == '('  ||
              escaped_char == ')'  ||
              escaped_char == '*'  ||
              escaped_char == '+'  ||
              escaped_char == '.'  ||
              escaped_char == '<'  ||
              escaped_char == '>'  ||
              escaped_char == '?'  ||
              escaped_char == '['  ||
              escaped_char == ']'  ||
              escaped_char == '^'  ||
              escaped_char == '{'  ||
              escaped_char == '|'  ||
              escaped_char == '}')) {
        // Regular expressions should include escaped delimiters in their
        // escaped form, except when the escaped character is
        // a closing delimiter but not a regexp metacharacter.
        //
        // The backslash itself cannot be used as a closing delimiter
        // at the same time as an escape symbol, but it is always munged,
        // so this branch also executes for the non-closing-delimiter case
        // for the backslash.
        auto str = tok();
        current_literal.extend_string(str, ts, te);
      } else {
        auto str = std::string(&escaped_char, 1);
        current_literal.extend_string(str, ts, te);
      }
    } else {
      // It does not. So this is an actual escape sequence, yay!
      if (current_literal.regexp()) {
        // Regular expressions should include escape sequences in their
        // escaped form. On the other hand, escaped newlines are removed.
        std::string str = gsub(tok(), "\\\n", "");
        current_literal.extend_string(str, ts, te);
      } else {
        auto str = escape ? *escape : tok();
        current_literal.extend_string(str, ts, te);
      }
    }
  }

  # Extend a string with a newline or a EOF character.
  # As heredoc closing line can immediately precede EOF, this action
  # has to handle such case specially.
  action extend_string_eol {
    auto& current_literal = literal_();

    if (te == pe) {
      diagnostic_(diagnostic_level::FATAL, "unterminated string meets end of file"s, current_literal.str_s, current_literal.str_s + 1);
    }

    if (current_literal.heredoc()) {
      auto line = tok(herebody_s, ts);

      while (line.back() == '\r') {
        line.pop_back();
      }

      if (version <= ruby_version::RUBY_20) {
        // See ruby:c48b4209c
        auto riter = line.rfind('\r');

        if (riter != std::string::npos) {
          line.erase(riter);
        }
      }

      // Try ending the heredoc with the complete most recently
      // scanned line. @herebody_s always refers to the start of such line.
      if (current_literal.nest_and_try_closing(line, herebody_s, ts)) {
        herebody_s = te;

        // Continue regular lexing after the heredoc reference (<<END).
        p = current_literal.heredoc_e - 1;
        fnext *pop_literal(); fbreak;
      } else {
        // Calculate indentation level for <<~HEREDOCs.
        current_literal.infer_indent_level(line);

        // Ditto.
        herebody_s = te;
      }
    } else {
      // Try ending the literal with a newline.
      auto str = tok();
      if (current_literal.nest_and_try_closing(str, ts, te)) {
        fnext *pop_literal(); fbreak;
      }

      if (herebody_s) {
        // This is a regular literal intertwined with a heredoc. Like:
        //
        //     p <<-foo+"1
        //     bar
        //     foo
        //     2"
        //
        // which, incidentally, evaluates to "bar\n1\n2".
        p = herebody_s - 1;
        herebody_s = nullptr;
      }
    }

    if (current_literal.words() && !eof_codepoint(*p)) {
      current_literal.extend_space(ts, te);
    } else {
      // A literal newline is appended if the heredoc was _not_ closed
      // this time (see f break above). See also Literal#nest_and_try_closing
      // for rationale of calling #flush_string here.
      std::string str = tok();
      current_literal.extend_string(str, ts, te);
      current_literal.flush_string();
    }
  }

  action extend_string_space {
    literal_().extend_space(ts, te);
  }

  #
  # === INTERPOLATION PARSING ===
  #

  # Interpolations with immediate variable names simply call into
  # the corresponding machine.

  interp_var = '#' ( global_var | class_var_v | instance_var_v );

  action extend_interp_var {
    auto& current_literal = literal_();
    current_literal.flush_string();
    current_literal.extend_content();

    emit(token_type::tSTRING_DVAR, "", ts, ts + 1);

    p = ts;
    fcall expr_variable;
  }

  # Interpolations with code blocks must match nested curly braces, as
  # interpolation ending is ambiguous with a block ending. So, every
  # opening and closing brace should be matched with e_[lr]brace rules,
  # which automatically perform the counting.
  #
  # Note that interpolations can themselves be nested, so brace balance
  # is tied to the innermost literal.
  #
  # Also note that literals themselves should not use e_[lr]brace rules
  # when matching their opening and closing delimiters, as the amount of
  # braces inside the characters of a string literal is independent.

  interp_code = '#{';

  e_lbrace = '{' % {
    cond.push(false); cmdarg.push(false);

    if (!literal_stack.empty()) {
      literal_().start_interp_brace();
    }
  };

  e_rbrace = '}' % {
    if (!literal_stack.empty()) {
      auto& current_literal = literal_();

      if (current_literal.end_interp_brace_and_try_closing()) {
        if (version == ruby_version::RUBY_18 || version == ruby_version::RUBY_19) {
          emit(token_type::tRCURLY, "}", p - 1, p);
        } else {
          emit(token_type::tSTRING_DEND, "}", p - 1, p);
        }

        if (current_literal.saved_herebody_s) {
          herebody_s = current_literal.saved_herebody_s;
        }

        fhold;
        fnext *stack_pop();
        fbreak;
      }
    }
  };

  action extend_interp_code {
    auto& current_literal = literal_();
    current_literal.flush_string();
    current_literal.extend_content();

    emit(token_type::tSTRING_DBEG, "#{");

    if (current_literal.heredoc()) {
      current_literal.saved_herebody_s = herebody_s;
      herebody_s = nullptr;
    }

    current_literal.start_interp_brace();
    fcall expr_value;
  }

  # Actual string parsers are simply combined from the primitives defined
  # above.

  interp_words := |*
      interp_code => extend_interp_code;
      interp_var  => extend_interp_var;
      e_bs escape => extend_string_escaped;
      c_space+    => extend_string_space;
      c_eol       => extend_string_eol;
      c_any       => extend_string;
  *|;

  interp_string := |*
      interp_code => extend_interp_code;
      interp_var  => extend_interp_var;
      e_bs escape => extend_string_escaped;
      c_eol       => extend_string_eol;
      c_any       => extend_string;
  *|;

  plain_words := |*
      e_bs c_any  => extend_string_escaped;
      c_space+    => extend_string_space;
      c_eol       => extend_string_eol;
      c_any       => extend_string;
  *|;

  plain_string := |*
      '\\' c_nl   => extend_string_eol;
      e_bs c_any  => extend_string_escaped;
      c_eol       => extend_string_eol;
      c_any       => extend_string;
  *|;

  interp_backslash_delimited := |*
      interp_code => extend_interp_code;
      interp_var  => extend_interp_var;
      c_eol       => extend_string_eol;
      c_any       => extend_string;
  *|;

  plain_backslash_delimited := |*
      c_eol       => extend_string_eol;
      c_any       => extend_string;
  *|;

  interp_backslash_delimited_words := |*
      interp_code => extend_interp_code;
      interp_var  => extend_interp_var;
      c_space+    => extend_string_space;
      c_eol       => extend_string_eol;
      c_any       => extend_string;
  *|;

  plain_backslash_delimited_words := |*
      c_space+    => extend_string_space;
      c_eol       => extend_string_eol;
      c_any       => extend_string;
  *|;

  regexp_modifiers := |*
      [A-Za-z]+
      => {
        auto options = tok();
        std::string unknown_options;

        for (auto i = options.cbegin(); i != options.cend(); ++i) {
          switch (char opt = *i) {
            case 'i':
            case 'm':
            case 'x':
            case 'o':
            case 'u':
            case 'e':
            case 's':
            case 'n':
              continue;
            default:
              unknown_options += opt;
              break;
          }
        }

        if (!unknown_options.empty()) {
          diagnostic_(diagnostic_level::ERROR, "unknown regexp options: "s + unknown_options);
        }

        emit(token_type::tREGEXP_OPT, options);
        fnext expr_end; fbreak;
      };

      any
      => {
        emit(token_type::tREGEXP_OPT, tok(ts, te - 1), ts, te - 1);
        fhold; fgoto expr_end;
      };
  *|;

  #
  # === WHITESPACE HANDLING ===
  #

  # Various contexts in Ruby allow various kinds of whitespace
  # to be used. They are grouped to clarify the lexing machines
  # and ease collection of comments.

  # A line of code with inline #comment at end is always equivalent
  # to a line of code ending with just a newline, so an inline
  # comment is deemed equivalent to non-newline whitespace
  # (c_space character class).

  w_space =
      c_space+
    | '\\' e_heredoc_nl
    ;

  w_comment =
      '#'     %{ sharp_s = p - 1; }
      # The (p == pe) condition compensates for added "\0" and
      # the way Ragel handles EOF.
      c_line* %{ emit_comment(sharp_s, p == pe ? p - 2 : p); }
    ;

  w_space_comment =
      w_space
    | w_comment
    ;

  # A newline in non-literal context always interoperates with
  # here document logic and can always be escaped by a backslash,
  # still interoperating with here document logic in the same way,
  # yet being invisible to anything else.
  #
  # To demonstrate:
  #
  #     foo = <<FOO \
  #     bar
  #     FOO
  #      + 2
  #
  # is equivalent to `foo = "bar\n" + 2`.

  w_newline =
      e_heredoc_nl;

  w_any =
      w_space
    | w_comment
    | w_newline
    ;


  #
  # === EXPRESSION PARSING ===
  #

  # These rules implement a form of manually defined lookahead.
  # The default longest-match scanning does not work here due
  # to sheer ambiguity.

  ambiguous_fid_suffix =         # actual    parsed
      [?!]    %{ tm = p; }     | # a?        a?
      [?!]'=' %{ tm = p - 2; }   # a!=b      a != b
  ;

  ambiguous_ident_suffix =       # actual    parsed
      ambiguous_fid_suffix     |
      '='     %{ tm = p; }     | # a=        a=
      '=='    %{ tm = p - 2; } | # a==b      a == b
      '=~'    %{ tm = p - 2; } | # a=~b      a =~ b
      '=>'    %{ tm = p - 2; } | # a=>b      a => b
      '==='   %{ tm = p - 3; }   # a===b     a === b
  ;

  ambiguous_symbol_suffix =      # actual    parsed
      ambiguous_ident_suffix |
      '==>'   %{ tm = p - 2; }   # :a==>b    :a= => b
  ;

  # Ambiguous with 1.9 hash labels.
  ambiguous_const_suffix =       # actual    parsed
      '::'    %{ tm = p - 2; }   # A::B      A :: B
  ;

  # Resolving kDO/kDO_COND/kDO_BLOCK ambiguity requires embedding
  # @cond/@cmdarg-related code to e_lbrack, e_lparen and e_lbrace.

  e_lbrack = '[' % {
    cond.push(false); cmdarg.push(false);
  };

  # Ruby 1.9 lambdas require parentheses counting in order to
  # emit correct opening kDO/tLBRACE.

  e_lparen = '(' % {
    cond.push(false); cmdarg.push(false);

    paren_nest += 1;
  };

  e_rparen = ')' % {
    paren_nest -= 1;
  };

  # Ruby is context-sensitive wrt/ local identifiers.
  action local_ident {
    auto ident = tok();

    emit(token_type::tIDENTIFIER, ident);

    if (is_declared(ident)) {
      fnext expr_endfn; fbreak;
    } else {
      fnext *arg_or_cmdarg(); fbreak;
    }
  }

  # Variable lexing code is accessed from both expressions and
  # string interpolation related code.
  #
  expr_variable := |*
      global_var
      => {
        if (ts[1] >= '1' && ts[1] <= '9') {
          emit(token_type::tNTH_REF, tok(ts + 1));
        } else if (ts[1] == '&' || ts[1] == '`' || ts[1] == '\'' || ts[1] == '+') {
          emit(token_type::tBACK_REF);
        } else {
          emit(token_type::tGVAR);
        }

        fnext *stack_pop(); fbreak;
      };

      class_var_v
      => {
        if (ts[2] >= '0' && ts[2] <= '9') {
          diagnostic_(diagnostic_level::ERROR, "`" + tok() + "' is not allowed as a class variable name");
        }

        emit(token_type::tCVAR);
        fnext *stack_pop(); fbreak;
      };

      instance_var_v
      => {
        if (ts[1] >= '0' && ts[1] <= '9') {
          diagnostic_(diagnostic_level::ERROR, "`" + tok() + "' is not allowed as an instance variable name");
        }

        emit(token_type::tIVAR);
        fnext *stack_pop(); fbreak;
      };
  *|;

  # Literal function name in definition (e.g. `def class`).
  # Keywords are returned as their respective tokens; this is used
  # to support singleton def `def self.foo`. Global variables are
  # returned as `tGVAR`; this is used in global variable alias
  # statements `alias $a $b`. Symbols are returned verbatim; this
  # is used in `alias :a :"b#{foo}"` and `undef :a`.
  #
  # Transitions to `expr_endfn` afterwards.
  #
  expr_fname := |*
      keyword
      => { emit_table(KEYWORDS_BEGIN);
           fnext expr_endfn; fbreak; };

      constant
      => { emit(token_type::tCONSTANT);
           fnext expr_endfn; fbreak; };

      bareword [?=!]?
      => { emit(token_type::tIDENTIFIER);
           fnext expr_endfn; fbreak; };

      global_var
      => { p = ts - 1;
           fnext expr_end; fcall expr_variable; };

      # If the handling was to be delegated to expr_end,
      # these cases would transition to something else than
      # expr_endfn, which is incorrect.
      operator_fname      |
      operator_arithmetic |
      operator_rest
      => { emit_table(PUNCTUATION);
           fnext expr_endfn; fbreak; };

      '::'
      => { fhold; fhold; fgoto expr_end; };

      ':'
      => { fhold; fgoto expr_beg; };

      '%s' c_any
      => {
        if (version == ruby_version::RUBY_23) {
          fgoto *push_literal(literal_type::LOWERS_SYMBOL, std::string(ts + 2, 1), ts);
        } else {
          p = ts - 1;
          fgoto expr_end;
        }
      };

      w_any;

      c_any
      => { fhold; fgoto expr_end; };

      c_eof => do_eof;
  *|;

  # After literal function name in definition. Behaves like `expr_end`,
  # but allows a tLABEL.
  #
  # Transitions to `expr_end` afterwards.
  #
  expr_endfn := |*
      label ( any - ':' )
      => { emit(token_type::tLABEL, tok(ts, te - 2), ts, te - 1);
           fhold; fnext expr_labelarg; fbreak; };

      w_space_comment;

      c_any
      => { fhold; fgoto expr_end; };

      c_eof => do_eof;
  *|;

  # Literal function name in method call (e.g. `a.class`).
  #
  # Transitions to `expr_arg` afterwards.
  #
  expr_dot := |*
      constant
      => { emit(token_type::tCONSTANT);
           fnext *arg_or_cmdarg(); fbreak; };

      call_or_var
      => { emit(token_type::tIDENTIFIER);
           fnext *arg_or_cmdarg(); fbreak; };

      bareword ambiguous_fid_suffix
      => { emit(token_type::tFID, tok(ts, tm), ts, tm);
           fnext *arg_or_cmdarg(); p = tm - 1; fbreak; };

      # See the comment in `expr_fname`.
      operator_fname      |
      operator_arithmetic |
      operator_rest
      => { emit_table(PUNCTUATION);
           fnext expr_arg; fbreak; };

      w_any;

      c_any
      => { fhold; fgoto expr_end; };

      c_eof => do_eof;
  *|;

  # The previous token emitted was a `tIDENTIFIER` or `tFID`; no space
  # is consumed; the current expression is a command or method call.
  #
  expr_arg := |*
      #
      # COMMAND MODE SPECIFIC TOKENS
      #

      # cmd (1 + 2)
      # See below the rationale about expr_endarg.
      w_space+ e_lparen
      => {
        if (version == ruby_version::RUBY_18) {
          emit(token_type::tLPAREN2, "(", te - 1, te);
          fnext expr_value; fbreak;
        } else {
          emit(token_type::tLPAREN_ARG, "(", te - 1, te);
          fnext expr_beg; fbreak;
        }
      };

      # meth(1 + 2)
      # Regular method call.
      e_lparen
      => { emit(token_type::tLPAREN2, "(");
           fnext expr_beg; fbreak; };

      # meth [...]
      # Array argument. Compare with indexing `meth[...]`.
      w_space+ e_lbrack
      => { emit(token_type::tLBRACK, "[", te - 1, te);
           fnext expr_beg; fbreak; };

      # cmd {}
      # Command: method call without parentheses.
      w_space* e_lbrace
      => {
        if (!lambda_stack.empty() && lambda_stack.top() == paren_nest) {
          p = ts - 1;
          fgoto expr_end;
        } else {
          emit(token_type::tLCURLY, "{", te - 1, te);
          fnext expr_value; fbreak;
        }
      };

      #
      # AMBIGUOUS TOKENS RESOLVED VIA EXPR_BEG
      #

      # a??
      # Ternary operator
      '?' c_space_nl
      => {
        // Unlike expr_beg as invoked in the next rule, do not warn
        p = ts - 1;
        fgoto expr_end;
      };

      # a ?b, a? ?
      # Character literal or ternary operator
      w_space* '?'
      => { fhold; fgoto expr_beg; };

      # a %{1}, a %[1] (but not "a %=1=" or "a % foo")
      # a /foo/ (but not "a / foo" or "a /=foo")
      # a <<HEREDOC
      w_space+ %{ tm = p; }
      ( [%/] ( c_any - c_space_nl - '=' ) # /
      | '<<'
      )
      => {
        if (*tm == '/') {
          // Ambiguous regexp literal.
          diagnostic_(diagnostic_level::WARNING,
            "ambiguous first argument; put parentheses or a space even after the operator"s,
            tm, tm + 1);
        }

        p = tm - 1;
        fgoto expr_beg;
      };

      # x *1
      # Ambiguous splat, kwsplat or block-pass.
      w_space+ %{ tm = p; } ( '+' | '-' | '*' | '&' | '**' )
      => {
        diagnostic_(diagnostic_level::WARNING, "`"s + tok(tm, te) + "' interpreted as argument prefix"s, tm, te);

        p = tm - 1;
        fgoto expr_beg;
      };

      # x ::Foo
      # Ambiguous toplevel constant access.
      w_space+ '::'
      => { fhold; fhold; fgoto expr_beg; };

      # x:b
      # Symbol.
      w_space* ':'
      => { fhold; fgoto expr_beg; };

      w_space+ label
      => { p = ts - 1; fgoto expr_beg; };

      #
      # AMBIGUOUS TOKENS RESOLVED VIA EXPR_END
      #

      # a ? b
      # Ternary operator.
      w_space+ %{ tm = p; } '?' c_space_nl
      => { p = tm - 1; fgoto expr_end; };

      # x + 1: Binary operator or operator-assignment.
      w_space* operator_arithmetic
                  ( '=' | c_space_nl )?    |
      # x rescue y: Modifier keyword.
      w_space* keyword_modifier            |
      # a &. b: Safe navigation operator.
      w_space* '&.'                        |
      # Miscellanea.
      w_space* punctuation_end
      => {
        p = ts - 1;
        fgoto expr_end;
      };

      w_space;

      w_comment
      => { fgoto expr_end; };

      w_newline
      => { fhold; fgoto expr_end; };

      c_any
      => { fhold; fgoto expr_beg; };

      c_eof => do_eof;
  *|;

  # The previous token was an identifier which was seen while in the
  # command mode (that is, the state at the beginning of #advance was
  # expr_value). This state is very similar to expr_arg, but disambiguates
  # two very rare and specific condition:
  #   * In 1.8 mode, "foo (lambda do end)".
  #   * In 1.9+ mode, "f x: -> do foo do end end".
  expr_cmdarg := |*
      w_space+ e_lparen
      => {
        emit(token_type::tLPAREN_ARG, "(", te - 1, te);

        if (version == ruby_version::RUBY_18) {
          fnext expr_value; fbreak;
        } else {
          fnext expr_beg; fbreak;
        }
      };

      w_space* 'do'
      => {
        if (cond.active()) {
          emit(token_type::kDO_COND, "do", te - 2, te);
        } else {
          emit(token_type::kDO, "do", te - 2, te);
        }
        fnext expr_value; fbreak;
      };

      c_any             |
      # Disambiguate with the `do' rule above.
      w_space* bareword |
      w_space* label
      => { p = ts - 1;
           fgoto expr_arg; };

      c_eof => do_eof;
  *|;

  # The rationale for this state is pretty complex. Normally, if an argument
  # is passed to a command and then there is a block (tLCURLY...tRCURLY),
  # the block is attached to the innermost argument (`f` in `m f {}`), or it
  # is a parse error (`m 1 {}`). But there is a special case for passing a single
  # primary expression grouped with parentheses: if you write `m (1) {}` or
  # (2.0 only) `m () {}`, then the block is attached to `m`.
  #
  # Thus, we recognize the opening `(` of a command (remember, a command is
  # a method call without parens) as a tLPAREN_ARG; then, in parser, we recognize
  # `tLPAREN_ARG expr rparen` as a `primary_expr` and before rparen, set the
  # lexer's state to `expr_endarg`, which makes it emit the possibly following
  # `{` as `tLBRACE_ARG`.
  #
  # The default post-`expr_endarg` state is `expr_end`, so this state also handles
  # `do` (as `kDO_BLOCK` in `expr_beg`).
  expr_endarg := |*
      e_lbrace
      => {
        if (!lambda_stack.empty() && lambda_stack.top() == paren_nest) {
          lambda_stack.pop();
          emit(token_type::tLAMBEG, "{");
        } else {
          emit(token_type::tLBRACE_ARG, "{");
        }
        fnext expr_value;
      };

      'do'
      => { emit_do(true);
           fnext expr_value; fbreak; };

      w_space_comment;

      c_any
      => { fhold; fgoto expr_end; };

      c_eof => do_eof;
  *|;

  # The rationale for this state is that several keywords accept value
  # (i.e. should transition to `expr_beg`), do not accept it like a command
  # (i.e. not an `expr_arg`), and must behave like a statement, that is,
  # accept a modifier if/while/etc.
  #
  expr_mid := |*
      keyword_modifier
      => { emit_table(KEYWORDS);
           fnext expr_beg; fbreak; };

      bareword
      => { p = ts - 1; fgoto expr_beg; };

      w_space_comment;

      w_newline
      => { fhold; fgoto expr_end; };

      c_any
      => { fhold; fgoto expr_beg; };

      c_eof => do_eof;
  *|;

  # Beginning of an expression.
  #
  # Don't fallthrough to this state from `c_any`; make sure to handle
  # `c_space* c_nl` and let `expr_end` handle the newline.
  # Otherwise code like `f\ndef x` gets glued together and the parser
  # explodes.
  #
  expr_beg := |*
      # Numeric processing. Converts:
      #   +5 to [tINTEGER, 5]
      #   -5 to [tUMINUS_NUM] [tINTEGER, 5]
      [+\-][0-9]
      => {
        fhold;
        if (*ts == '-') {
          emit(token_type::tUMINUS_NUM, "-", ts, ts + 1);
          fnext expr_end; fbreak;
        }
      };

      # splat *a
      '*'
      => { emit(token_type::tSTAR, "*");
           fbreak; };

      #
      # STRING AND REGEXP LITERALS
      #

      # /regexp/oui
      # /=/ (disambiguation with /=)
      '/' c_any
      => {
        fhold; fgoto *push_literal(literal_type::SLASH_REGEXP, std::string(ts + 0, 1), ts);
      };

      # %<string>
      '%' ( any - [A-Za-z] )
      => {
        fgoto *push_literal(literal_type::PERCENT_STRING, std::string(ts + 1, 1), ts);
      };

      # %w(we are the people)
      '%' [A-Za-z]+ c_any
      => {
        literal_type type;

        bool single_char_type = (ts + 3 == te);

        if (single_char_type && ts[1] == 'q') {
          type = literal_type::LOWERQ_STRING;
        } else if (single_char_type && ts[1] == 'Q') {
          type = literal_type::UPPERQ_STRING;
        } else if (single_char_type && ts[1] == 'w') {
          type = literal_type::LOWERW_WORDS;
        } else if (single_char_type && ts[1] == 'W') {
          type = literal_type::UPPERW_WORDS;
        } else if (single_char_type && ts[1] == 'i') {
          type = literal_type::LOWERI_SYMBOLS;
        } else if (single_char_type && ts[1] == 'I') {
          type = literal_type::UPPERI_SYMBOLS;
        } else if (single_char_type && ts[1] == 's') {
          type = literal_type::LOWERS_SYMBOL;
        } else if (single_char_type && ts[1] == 'r') {
          type = literal_type::PERCENT_REGEXP;
        } else if (single_char_type && ts[1] == 'x') {
          type = literal_type::LOWERX_XSTRING;
        } else {
          type = literal_type::PERCENT_STRING;
          diagnostic_(diagnostic_level::ERROR,
            tok(ts, te - 1) + ": unknown type of percent-literal",
            ts, te - 1);
        }

        fgoto *push_literal(type, std::string(te - 1, 1), ts);
      };

      '%' c_eof
      => {
        diagnostic_(diagnostic_level::FATAL, "unterminated string meets end of file",
          ts, ts + 1);
      };

      # Heredoc start.
      # <<END  | <<'END'  | <<"END"  | <<`END`  |
      # <<-END | <<-'END' | <<-"END" | <<-`END` |
      # <<~END | <<~'END' | <<~"END" | <<~`END`
      '<<' [~\-]?
        ( '"' ( c_line - '"' )* '"'
        | "'" ( c_line - "'" )* "'"
        | "`" ( c_line - "`" )* "`"
        | bareword ) % { heredoc_e      = p; }
        c_line* c_nl % { new_herebody_s = p; }
      => {
        bool indent;
        bool dedent_body;

        const char* delim_s = ts + 2;
        const char* delim_e = heredoc_e;

        if (*delim_s == '-') {
          indent = true;
          dedent_body = false;
          delim_s++;
        } else if (*delim_s == '~') {
          indent = true;
          dedent_body = true;
          delim_s++;
        } else {
          indent = false;
          dedent_body = false;
        }

        literal_type type;

        if (*delim_s == '"') {
          type = literal_type::DQUOTE_HEREDOC;
          delim_s++;
          delim_e--;
        } else if (*delim_s == '\'') {
          type = literal_type::SQUOTE_HEREDOC;
          delim_s++;
          delim_e--;
        } else if (*delim_s == '`') {
          type = literal_type::BACKTICK_HEREDOC;
          delim_s++;
          delim_e--;
        } else {
          type = literal_type::DQUOTE_HEREDOC;
        }

        if (dedent_body && (version == ruby_version::RUBY_18 ||
                            version == ruby_version::RUBY_19 ||
                            version == ruby_version::RUBY_20 ||
                            version == ruby_version::RUBY_21 ||
                            version == ruby_version::RUBY_22)) {
          emit(token_type::tLSHFT, "<<", ts, ts + 2);
          p = ts + 1;
          fnext expr_beg; fbreak;
        } else {
          fnext *push_literal(type, std::string(delim_s, (size_t)(delim_e - delim_s)), ts, heredoc_e, indent, dedent_body);

          if (!herebody_s) {
            herebody_s = new_herebody_s;
          }

          p = herebody_s - 1;
        }
      };

      #
      # SYMBOL LITERALS
      #

      # :"bar", :'baz'
      ':' ['"] # '
      => {
        literal_type type;

        if (ts[1] == '\'') {
          type = literal_type::SQUOTE_SYMBOL;
        } else { // '"'
          type = literal_type::DQUOTE_SYMBOL;
        }

        fgoto *push_literal(type, std::string(ts + 1, 1), ts);
      };

      ':' bareword ambiguous_symbol_suffix
      => {
        emit(token_type::tSYMBOL, tok(ts + 1, tm), ts, tm);
        p = tm - 1;
        fnext expr_end; fbreak;
      };

      ':' ( bareword | global_var | class_var | instance_var |
            operator_fname | operator_arithmetic | operator_rest )
      => {
        emit(token_type::tSYMBOL, tok(ts + 1), ts, te);
        fnext expr_end; fbreak;
      };

      #
      # AMBIGUOUS TERNARY OPERATOR
      #

      # Character constant, like ?a, ?\n, ?\u1000, and so on
      # Don't accept \u escape with multiple codepoints, like \u{1 2 3}
      '?' ( e_bs ( escape - ( '\u{' (xdigit+ [ \t]+)+ xdigit+ '}' ))
          | (c_any - c_space_nl - e_bs) % { escape = nullptr; }
          )
      => {
        if (version == ruby_version::RUBY_18) {
          emit(token_type::tINTEGER, std::to_string(static_cast<unsigned char>(ts[1])));
        } else {
          emit(token_type::tCHARACTER, escape ? *escape : tok(ts + 1));
        }

        fnext expr_end; fbreak;
      };

      '?' c_space_nl
      => {
        static const std::map<char, std::string> escape_map {
          { ' ',  "\\s" },
          { '\r', "\\r" },
          { '\n', "\\n" },
          { '\t', "\\t" },
          { '\v', "\\v" },
          { '\f', "\\f" },
        };

        auto& escape = escape_map.at(ts[1]);

        diagnostic_(diagnostic_level::WARNING, "invalid character syntax; use ?"s + escape);

        p = ts - 1;
        fgoto expr_end;
      };

      '?' c_eof
      => {
        diagnostic_(diagnostic_level::FATAL, "incomplete character syntax", ts, ts + 1);
      };

      # f ?aa : b: Disambiguate with a character literal.
      '?' [A-Za-z_] bareword
      => {
        p = ts - 1;
        fgoto expr_end;
      };

      #
      # KEYWORDS AND PUNCTUATION
      #

      # a({b=>c})
      e_lbrace
      => {
        if (!lambda_stack.empty() && lambda_stack.top() == paren_nest) {
          lambda_stack.pop();
          emit(token_type::tLAMBEG, "{");
        } else {
          emit(token_type::tLBRACE, "{");
        }
        fbreak;
      };

      # a([1, 2])
      e_lbrack
      => { emit(token_type::tLBRACK, "[");
           fbreak; };

      # a()
      e_lparen
      => { emit(token_type::tLPAREN, "(");
           fbreak; };

      # a(+b)
      punctuation_begin
      => { emit_table(PUNCTUATION_BEGIN);
           fbreak; };

      # rescue Exception => e: Block rescue.
      # Special because it should transition to expr_mid.
      'rescue' %{ tm = p; } '=>'?
      => { emit(token_type::kRESCUE, "rescue", ts, tm);
           p = tm - 1;
           fnext expr_mid; fbreak; };

      # if a: Statement if.
      keyword_modifier
      => { emit_table(KEYWORDS_BEGIN);
           fnext expr_value; fbreak; };

      #
      # RUBY 1.9 HASH LABELS
      #

      label ( any - ':' )
      => {
        fhold;

        if (version == ruby_version::RUBY_18) {
          auto ident = tok(ts, te - 2);

          if (*ts >= 'A' && *ts <= 'Z') {
            emit(token_type::tCONSTANT, ident, ts, te - 2);
          } else {
            emit(token_type::tIDENTIFIER, ident, ts, te - 2);
          }
          fhold; // continue as a symbol

          if (is_declared(ident)) {
            fnext expr_end;
          } else {
            fnext *arg_or_cmdarg();
          }
        } else {
          emit(token_type::tLABEL, tok(ts, te - 2), ts, te - 1);
          fnext expr_labelarg;
        }

        fbreak;
      };

      #
      # CONTEXT-DEPENDENT VARIABLE LOOKUP OR COMMAND INVOCATION
      #

      # foo= bar:  Disambiguate with bareword rule below.
      bareword ambiguous_ident_suffix |
      # def foo:   Disambiguate with bareword rule below.
      keyword
      => { p = ts - 1;
           fgoto expr_end; };

      # a = 42;     a [42]: Indexing.
      # def a; end; a [42]: Array argument.
      call_or_var
      => local_ident;

      #
      # WHITESPACE
      #

      w_any;

      e_heredoc_nl '=begin' ( c_space | c_nl_zlen )
      => { p = ts - 1;
           fgoto line_begin; };

      #
      # DEFAULT TRANSITION
      #

      # The following rules match most binary and all unary operators.
      # Rules for binary operators provide better error reporting.
      operator_arithmetic '='    |
      operator_rest              |
      punctuation_end            |
      c_any
      => { p = ts - 1; fgoto expr_end; };

      c_eof => do_eof;
  *|;

  # Special newline handling for "def a b:"
  #
  expr_labelarg := |*
    w_space_comment;

    w_newline
    => {
      if (in_kwarg) {
        fhold; fgoto expr_end;
      } else {
        fgoto line_begin;
      }
    };

    c_any
    => { fhold; fgoto expr_beg; };

    c_eof => do_eof;
  *|;

  # Like expr_beg, but no 1.9 label or 2.2 quoted label possible.
  #
  expr_value := |*
      # a:b: a(:b), a::B, A::B
      label (any - ':')
      => { p = ts - 1;
           fgoto expr_end; };

      # "bar", 'baz'
      ['"] # '
      => {
        literal_type type;

        if (ts[0] == '\'') {
          type = literal_type::SQUOTE_STRING;
        } else { // '"'
          type = literal_type::DQUOTE_STRING;
        }

        fgoto *push_literal(type, tok(), ts);
      };

      w_space_comment;

      w_newline
      => { fgoto line_begin; };

      c_any
      => { fhold; fgoto expr_beg; };

      c_eof => do_eof;
  *|;

  expr_end := |*
      #
      # STABBY LAMBDA
      #

      '->'
      => {
        emit(token_type::tLAMBDA, "->", ts, ts + 2);

        lambda_stack.push(paren_nest);
        fnext expr_endfn; fbreak;
      };

      e_lbrace | 'do'
      => {
        if (!lambda_stack.empty() && lambda_stack.top() == paren_nest) {
          lambda_stack.pop();

          if (ts[0] == '{') {
            emit(token_type::tLAMBEG, "{");
          } else { // 'do'
            emit(token_type::kDO_LAMBDA, "do");
          }
        } else {
          if (ts[0] == '{') {
            emit(token_type::tLCURLY, "{");
          } else { // 'do'
            emit_do();
          }
        }

        fnext expr_value; fbreak;
      };

      #
      # KEYWORDS
      #

      keyword_with_fname
      => { emit_table(KEYWORDS);
           fnext expr_fname; fbreak; };

      'class' w_any* '<<'
      => { emit(token_type::kCLASS, "class", ts, ts + 5);
           emit(token_type::tLSHFT, "<<",    te - 2, te);
           fnext expr_value; fbreak; };

      # a if b:c: Syntax error.
      keyword_modifier
      => { emit_table(KEYWORDS);
           fnext expr_beg; fbreak; };

      # elsif b:c: elsif b(:c)
      keyword_with_value
      => { emit_table(KEYWORDS);
           fnext expr_value; fbreak; };

      keyword_with_mid
      => { emit_table(KEYWORDS);
           fnext expr_mid; fbreak; };

      keyword_with_arg
      => {
        emit_table(KEYWORDS);

        if (version == ruby_version::RUBY_18 && ts + 3 == te && ts[0] == 'n' && ts[1] == 'o' && ts[2] == 't') {
          fnext expr_beg; fbreak;
        } else {
          fnext expr_arg; fbreak;
        }
      };

      '__ENCODING__'
      => {
        if (version == ruby_version::RUBY_18) {
          auto ident = tok();

          emit(token_type::tIDENTIFIER, ident);

          if (!is_declared(ident)) {
            fnext *arg_or_cmdarg();
          }
        } else {
          emit(token_type::k__ENCODING__, "__ENCODING__");
        }
        fbreak;
      };

      keyword_with_end
      => { emit_table(KEYWORDS);
           fbreak; };

      #
      # NUMERIC LITERALS
      #

      ( '0' [Xx] %{ num_base = 16; num_digits_s = p; } int_hex
      | '0' [Dd] %{ num_base = 10; num_digits_s = p; } int_dec
      | '0' [Oo] %{ num_base = 8;  num_digits_s = p; } int_dec
      | '0' [Bb] %{ num_base = 2;  num_digits_s = p; } int_bin
      | [1-9] digit* '_'? %{ num_base = 10; num_digits_s = ts; } int_dec
      | '0'   digit* '_'? %{ num_base = 8;  num_digits_s = ts; } int_dec
      ) %{ num_suffix_s = p; } int_suffix
      => {
        auto digits = tok(num_digits_s, num_suffix_s);

        if (num_suffix_s[-1] == '_') {
          diagnostic_(diagnostic_level::ERROR, "trailing `_' in number"s, te - 1, te);
        } else if (num_digits_s == num_suffix_s && num_base == 8 && version == ruby_version::RUBY_18) {
          // 1.8 did not raise an error on 0o.
        } else if (num_digits_s == num_suffix_s) {
          diagnostic_(diagnostic_level::ERROR, "numeric literal without digits"s);
        } else if (num_base == 8) {
          for (const char* digit_p = num_digits_s; digit_p < num_suffix_s; digit_p++) {
            if (*digit_p == '8' || *digit_p == '9') {
              diagnostic_(diagnostic_level::ERROR, "invalid octal digit",
                digit_p, digit_p + 1);
            }
          }
        }

        if (version == ruby_version::RUBY_18 || version == ruby_version::RUBY_19 || version == ruby_version::RUBY_20) {
          emit(token_type::tINTEGER, digits, ts, num_suffix_s);
          p = num_suffix_s - 1;
        } else {
          emit_num(digits);
        }
        fbreak;
      };

      flo_frac flo_pow?
      => {
        diagnostic_(diagnostic_level::ERROR, "no .<digit> floating literal anymore; put 0 before dot");
      };

      flo_int [eE]
      => {
        if (version == ruby_version::RUBY_18 || version == ruby_version::RUBY_19 || version == ruby_version::RUBY_20) {
          diagnostic_(diagnostic_level::ERROR,
            "trailing `"s + tok(te - 1, te) + "' in number",
            te - 1, te);
        } else {
          emit(token_type::tINTEGER, tok(ts, te - 1), ts, te - 1);
          fhold; fbreak;
        }
      };

      flo_int flo_frac [eE]
      => {
        if (version == ruby_version::RUBY_18 || version == ruby_version::RUBY_19 || version == ruby_version::RUBY_20) {
          diagnostic_(diagnostic_level::ERROR,
            "trailing `"s + tok(te - 1, te) + "' in number",
            te - 1, te);
        } else {
          emit(token_type::tFLOAT, tok(ts, te - 1), ts, te - 1);
          fhold; fbreak;
        }
      };

      flo_int
      ( flo_frac? flo_pow %{ num_suffix_s = p; } flo_pow_suffix
      | flo_frac          %{ num_suffix_s = p; } flo_suffix
      )
      => {
        auto digits = tok(ts, num_suffix_s);

        if (version == ruby_version::RUBY_18 || version == ruby_version::RUBY_19 || version == ruby_version::RUBY_20) {
          emit(token_type::tFLOAT, digits, ts, num_suffix_s);
          p = num_suffix_s - 1;
        } else {
          emit_num(digits);
        }
        fbreak;
      };

      #
      # STRING AND XSTRING LITERALS
      #

      # `echo foo`, "bar", 'baz'
      '`' | ['"] # '
      => {
        literal_type type;

        if (ts[0] == '`') {
          type = literal_type::BACKTICK_XSTRING;
        } else if (ts[0] == '\'') {
          type = literal_type::SQUOTE_STRING;
        } else { // '"'
          type = literal_type::DQUOTE_STRING;
        }

        fgoto *push_literal(type, std::string(te - 1, 1), ts, nullptr, false, false, true);
      };

      #
      # CONSTANTS AND VARIABLES
      #

      constant
      => { emit(token_type::tCONSTANT);
           fnext *arg_or_cmdarg(); fbreak; };

      constant ambiguous_const_suffix
      => { emit(token_type::tCONSTANT, tok(ts, tm), ts, tm);
           p = tm - 1; fbreak; };

      global_var | class_var_v | instance_var_v
      => { p = ts - 1; fcall expr_variable; };

      #
      # METHOD CALLS
      #

      '.' | '&.' | '::'
      => { emit_table(PUNCTUATION);
           fnext expr_dot; fbreak; };

      call_or_var
      => local_ident;

      bareword ambiguous_fid_suffix
      => {
        if (tm == te) {
          // Suffix was consumed, e.g. foo!
          emit(token_type::tFID);
        } else {
          // Suffix was not consumed, e.g. foo!=
          emit(token_type::tIDENTIFIER, tok(ts, tm), ts, tm);
          p = tm - 1;
        }
        fnext expr_arg; fbreak;
      };

      #
      # OPERATORS
      #

      ( e_lparen
      | operator_arithmetic
      | operator_rest
      )
      => { emit_table(PUNCTUATION);
           fnext expr_beg; fbreak; };

      e_rbrace | e_rparen | ']'
      => {
        emit_table(PUNCTUATION);
        cond.lexpop(); cmdarg.lexpop();

        if (ts[0] == '}' || ts[0] == ']') {
          fnext expr_endarg;
        } else { // ')'
          // this was commented out in the original lexer.rl:
          // fnext expr_endfn; ?
        }

        fbreak;
      };

      operator_arithmetic '='
      => { emit(token_type::tOP_ASGN, tok(ts, te - 1));
           fnext expr_beg; fbreak; };

      '?'
      => { emit(token_type::tEH, "?");
           fnext expr_value; fbreak; };

      e_lbrack
      => { emit(token_type::tLBRACK2, "[");
           fnext expr_beg; fbreak; };

      punctuation_end
      => { emit_table(PUNCTUATION);
           fnext expr_beg; fbreak; };

      #
      # WHITESPACE
      #

      w_space_comment;

      w_newline
      => { fgoto leading_dot; };

      ';'
      => { emit(token_type::tSEMI, ";");
           fnext expr_value; fbreak; };

      '\\' c_line {
        diagnostic_(diagnostic_level::ERROR,
          "bare backslash only allowed before newline",
          ts, ts + 1);
        fhold;
      };

      c_any
      => {
        diagnostic_(diagnostic_level::ERROR,
          "unexpected `"s + tok() + "'"s);
      };

      c_eof => do_eof;
  *|;

  leading_dot := |*
      # Insane leading dots:
      # a #comment
      #  .b: a.b
      c_space* %{ tm = p; } ('.' | '&.')
      => { p = tm - 1; fgoto expr_end; };

      any
      => { emit(token_type::tNL, std::string(), newline_s, newline_s + 1);
           fhold; fnext line_begin; fbreak; };
  *|;

  #
  # === EMBEDDED DOCUMENT (aka BLOCK COMMENT) PARSING ===
  #

  line_comment := |*
      '=end' c_line* c_nl_zlen
      => {
        emit_comment(eq_begin_s, te);
        fgoto line_begin;
      };

      c_line* c_nl;

      c_line* zlen
      => {
        diagnostic_(diagnostic_level::FATAL,
          "embedded document meets end of file (and they embark on a romantic journey)"s,
          eq_begin_s, eq_begin_s + "=begin"s.size());
      };
  *|;

  line_begin := |*
      w_any;

      '=begin' ( c_space | c_nl_zlen )
      => { eq_begin_s = ts;
           fgoto line_comment; };

      '__END__' ( c_eol - zlen )
      => { p = pe - 3; };

      c_any
      => { fhold; fgoto expr_value; };

      c_eof => do_eof;
  *|;

}%%

token_t lexer::advance() {
  auto tok = advance_();
  last_token_s = tok->start();
  last_token_e = tok->end();
  return tok;
}

void lexer::extend_static() {
  static_env.emplace();
}

void lexer::extend_dynamic() {
  if (static_env.empty()) {
    static_env.emplace();
  } else {
    environment& env = static_env.top();
    static_env.push(env);
  }
}

void lexer::unextend() {
  static_env.pop();
}

void lexer::declare(const std::string& name) {
  static_env.top().insert(name);
}

bool lexer::is_declared(const std::string& identifier) const {
  const environment& env = static_env.top();

  return env.find(identifier) != env.end();
}

optional_size lexer::dedent_level() {
  // We erase @dedent_level as a precaution to avoid accidentally
  // using a stale value.
  auto ret = dedent_level_;
  dedent_level_ = optional_size::none();
  return ret;
}
