#ifndef RUBY_PARSER_LEXER_HH
#define RUBY_PARSER_LEXER_HH

#include <string>
#include <stack>
#include <queue>
#include <set>
#include <memory>
#include <map>

#include "literal.hh"
#include "token.hh"
#include "state_stack.hh"

namespace ruby_parser {
  enum class ruby_version {
    RUBY_18,
    RUBY_19,
    RUBY_20,
    RUBY_21,
    RUBY_22,
    RUBY_23,
    RUBY_24,
  };

  class lexer {
  public:
    using environment = std::set<std::string>;
    using token_table = std::map<std::string, token_type>;

    enum class num_xfrm_type {
      NONE,
      RATIONAL,
      IMAGINARY,
      RATIONAL_IMAGINARY,
      FLOAT,
      FLOAT_IMAGINARY,
    };

  private:
    ruby_version version;
    const std::string source_buffer;

    std::stack<environment> static_env;
    std::stack<literal> literal_stack;
    std::queue<token_ptr> token_queue;

    int cs;
    const char* _p;
    const char* _pe;
    const char* ts;
    const char* te;
    int act;

    std::vector<int> stack;
    int top;

    const char* eq_begin_s; // location of last encountered =begin
    const char* sharp_s;    // location of last encountered #
    const char* newline_s;  // location of last encountered newline

    // Ruby 1.9 ->() lambdas emit a distinct token if do/{ is
    // encountered after a matching closing parenthesis.
    size_t paren_nest;
    std::stack<size_t> lambda_stack;

    // If the lexer is in `command state' (aka expr_value)
    // at the entry to #advance, it will transition to expr_cmdarg
    // instead of expr_arg at certain points.
    bool command_state;

    int num_base;             // last numeric base
    const char* num_digits_s; // starting position of numeric digits
    const char* num_suffix_s; // starting position of numeric suffix
    num_xfrm_type num_xfrm;   // numeric suffix-induced transformation

    const char* escape_s;                // starting position of current sequence
    std::unique_ptr<std::string> escape; // last escaped sequence, as string

    const char* herebody_s;   // starting position of current heredoc line

    void check_stack_capacity();
    int stack_pop();
    int arg_or_cmdarg();
    void emit_comment(const char* s, const char* e);
    std::string tok();
    std::string tok(const char* start);
    std::string tok(const char* start, const char* end);
    bool static_env_declared(std::string& identifier);
    void emit(token_type type);
    void emit(token_type type, const std::string& str);
    void emit(token_type type, const std::string& str, const char* start, const char* end);
    void emit_do(bool do_block = false);
    void emit_table(const token_table& table);
    void emit_num(const std::string& num);
    template<typename... Args> int push_literal(Args&&... args);
    literal& literal();
    int pop_literal();

  public:
    state_stack cond;
    state_stack cmdarg;

    bool in_kwarg;            // true at the end of "def foo a:"

    lexer(ruby_version version, const std::string& source_buffer_);

    token_ptr advance();

    void set_state_expr_beg();
    void set_state_expr_endarg();
    void set_state_expr_fname();
    void set_state_expr_value();

    void extend_static();
    void extend_dynamic();
    void unextend();
    void declare(std::string& name);
  };
}

#endif
