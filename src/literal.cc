#include <ruby_parser/literal.hh>

using namespace ruby_parser;

literal::literal(literal_type type, std::string delimiter, const char* str_s, const char* heredoc_e, bool indent, bool dedent_body, bool label_allowed)
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

bool literal::words() const {
  return _type == literal_type::UPPERW_WORDS
      || _type == literal_type::LOWERW_WORDS
      || _type == literal_type::UPPERI_SYMBOLS
      || _type == literal_type::LOWERI_SYMBOLS
      ;
}

bool literal::backslash_delimited() const {
  return end_delim == "\\";
}

bool literal::interpolate() const {
  return _type == literal_type::DQUOTE_STRING
      || _type == literal_type::DQUOTE_HEREDOC
      || _type == literal_type::PERCENT_STRING
      || _type == literal_type::UPPERQ_STRING
      || _type == literal_type::UPPERW_WORDS
      || _type == literal_type::UPPERI_SYMBOLS
      || _type == literal_type::DQUOTE_SYMBOL
      || _type == literal_type::SLASH_REGEXP
      || _type == literal_type::PERCENT_REGEXP
      || _type == literal_type::LOWERX_XSTRING
      || _type == literal_type::BACKTICK_XSTRING
      || _type == literal_type::BACKTICK_HEREDOC
      ;
}

bool literal::regexp() const {
  return _type == literal_type::SLASH_REGEXP
      || _type == literal_type::PERCENT_REGEXP
      ;
}
