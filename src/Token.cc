#include <ruby_parser/Token.hh>

using namespace ruby_parser;

TokenType Token::type() const {
    return _type;
}

size_t Token::start() const {
    return _start;
}

size_t Token::end() const {
    return _end;
}

const std::string& Token::string() const {
    return _string;
}
