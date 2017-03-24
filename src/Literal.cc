#include <ruby_parser/Literal.hh>

using namespace ruby_parser;

Literal::Literal(LiteralType str_type, std::string delimiter, const char* str_s, const char* heredoc_e, bool indent, bool dedent_body, bool label_allowed)
  : str_type(str_type)
  , str_s(str_s)
  , heredoc_e(heredoc_e)
  , indent(indent)
  , dedent_body(dedent_body)
  , label_allowed(label_allowed)
{
  if (delimiter == "(") {
    start_delim = "(";
    end_delim = ")";
  } else if (delimiter == "[") {
    start_delim = "[";
    end_delim = "]";
  } else if (delimiter == "{") {
    start_delim = "{";
    end_delim = "}";
  } else if (delimiter == "<") {
    start_delim = "<";
    end_delim = ">";
  } else {
    start_delim = "";
    end_delim = delimiter;
  }
}

bool Literal::words() const {
  return str_type == LiteralType::UPPERW_WORDS
      || str_type == LiteralType::LOWERW_WORDS
      || str_type == LiteralType::UPPERI_SYMBOLS
      || str_type == LiteralType::LOWERI_SYMBOLS
      ;
}

bool Literal::backslash_delimited() const {
  return end_delim == "\\";
}

bool Literal::interpolate() const {
  return str_type == LiteralType::DQUOTE_STRING
      || str_type == LiteralType::DQUOTE_HEREDOC
      || str_type == LiteralType::PERCENT_STRING
      || str_type == LiteralType::UPPERQ_STRING
      || str_type == LiteralType::UPPERW_WORDS
      || str_type == LiteralType::UPPERI_SYMBOLS
      || str_type == LiteralType::DQUOTE_SYMBOL
      || str_type == LiteralType::SLASH_REGEXP
      || str_type == LiteralType::PERCENT_REGEXP
      || str_type == LiteralType::LOWERX_XSTRING
      || str_type == LiteralType::BACKTICK_XSTRING
      || str_type == LiteralType::BACKTICK_HEREDOC
      ;
}
