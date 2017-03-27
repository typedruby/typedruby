#ifndef RUBY_PARSER_LITERAL_HH
#define RUBY_PARSER_LITERAL_HH

#include <string>

namespace ruby_parser {
  enum class literal_type {
    SQUOTE_STRING,
    SQUOTE_HEREDOC,
    LOWERQ_STRING,
    DQUOTE_STRING,
    DQUOTE_HEREDOC,
    PERCENT_STRING,
    UPPERQ_STRING,
    LOWERW_WORDS,
    UPPERW_WORDS,
    LOWERI_SYMBOLS,
    UPPERI_SYMBOLS,
    SQUOTE_SYMBOL,
    LOWERS_SYMBOL,
    DQUOTE_SYMBOL,
    SLASH_REGEXP,
    PERCENT_REGEXP,
    LOWERX_XSTRING,
    BACKTICK_XSTRING,
    BACKTICK_HEREDOC,
  };

  class literal {
    literal_type _type;
    const char* str_s;
    std::string start_delim;
    std::string end_delim;
    const char* heredoc_e;
    bool indent;
    bool dedent_body;
    bool label_allowed;

  public:
    literal(literal_type type, std::string delimiter, const char* str_s, const char* heredoc_e = nullptr, bool indent = false, bool dedent_body = false, bool label_allowed = false);

    bool words() const;

    bool backslash_delimited() const;

    bool interpolate() const;

    bool regexp() const;
  };
}


#endif
