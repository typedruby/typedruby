#include <ruby_parser/Literal.hh>

using namespace ruby_parser;

Literal::Literal(LiteralType type, std::string delimiter, const char* str_s, const char* heredoc_e, bool indent, bool dedent_body, bool label_allowed)
  : _type(type)
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
  return _type == LiteralType::UPPERW_WORDS
      || _type == LiteralType::LOWERW_WORDS
      || _type == LiteralType::UPPERI_SYMBOLS
      || _type == LiteralType::LOWERI_SYMBOLS
      ;
}

bool Literal::backslash_delimited() const {
  return end_delim == "\\";
}

bool Literal::interpolate() const {
  return _type == LiteralType::DQUOTE_STRING
      || _type == LiteralType::DQUOTE_HEREDOC
      || _type == LiteralType::PERCENT_STRING
      || _type == LiteralType::UPPERQ_STRING
      || _type == LiteralType::UPPERW_WORDS
      || _type == LiteralType::UPPERI_SYMBOLS
      || _type == LiteralType::DQUOTE_SYMBOL
      || _type == LiteralType::SLASH_REGEXP
      || _type == LiteralType::PERCENT_REGEXP
      || _type == LiteralType::LOWERX_XSTRING
      || _type == LiteralType::BACKTICK_XSTRING
      || _type == LiteralType::BACKTICK_HEREDOC
      ;
}

bool Literal::regexp() const {
  return _type == LiteralType::SLASH_REGEXP
      || _type == LiteralType::PERCENT_REGEXP
      ;
}
