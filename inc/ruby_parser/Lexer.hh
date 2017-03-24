#ifndef RUBY_PARSER_LEXER_HH
#define RUBY_PARSER_LEXER_HH

#include <string>
#include <stack>
#include <queue>
#include <set>
#include <memory>

#include "Literal.hh"
#include "Token.hh"

namespace ruby_parser {
  enum class RubyVersion {
    RUBY_18,
    RUBY_19,
    RUBY_20,
    RUBY_21,
    RUBY_22,
    RUBY_23,
    RUBY_24,
  };

  class Lexer {
    using Environment = std::set<std::string>;

    enum class NumXfrm {
      NONE,
      RATIONAL,
      IMAGINARY,
      IMAGINARY_RATIONAL,
      FLOAT,
      IMAGINARY_FLOAT
    };

    RubyVersion version;
    std::string source_buffer;

    std::stack<bool> cond;
    std::stack<bool> cmdarg;
    std::stack<Environment> static_env;
    std::stack<Literal> literal_stack;
    std::queue<std::unique_ptr<Token>> token_queue;

    int cs;
    const char* _p;
    const char* _pe;
    const char* ts;
    const char* te;
    int act;

    std::vector<int> stack;
    int top;

    const char* eq_begin_s;   // location of last encountered =begin
    const char* sharp_s;    // location of last encountered #
    const char* newline_s;    // location of last encountered newline

    // Ruby 1.9 ->() lambdas emit a distinct token if do/{ is
    // encountered after a matching closing parenthesis.
    size_t paren_nest;
    std::stack<size_t> lambda_stack;

    // If the lexer is in `command state' (aka expr_value)
    // at the entry to #advance, it will transition to expr_cmdarg
    // instead of expr_arg at certain points.
    bool command_state;

    bool in_kwarg;        // true at the end of "def foo a:"

    int num_base;         // last numeric base
    const char* num_digits_s;   // starting position of numeric digits
    const char* num_suffix_s;   // starting position of numeric suffix
    NumXfrm num_xfrm;       // numeric suffix-induced transformation

    const char* escape_s;     // starting position of current sequence
    std::string escape;     // last escaped sequence, as string

    const char* herebody_s;   // starting position of current heredoc line

    void check_stack_capacity();
    bool active(std::stack<bool>& state_stack) const;
    void lexpop(std::stack<bool>& state_stack);
    int stack_pop();
    int arg_or_cmdarg();
    void emit_comment(const char* s, const char* e);
    std::string tok_as_string();
    bool static_env_declared(std::string&& identifier);
    void emit0(TokenType token_type);
    void emit1(TokenType token_type, const char* start, const char* end);
    void emit(TokenType token_type, const char* start, const char* end, const char* ptr, size_t len);
    template<typename... Args> int push_literal(Args&&... args);
    Literal& literal();
    int pop_literal();

  public:
    Lexer(RubyVersion version, std::string source);

    std::unique_ptr<Token> advance();

    void extend_static();
    void extend_dynamic();
    void unextend();
    void declare(std::string& name);
  };
}

#endif
