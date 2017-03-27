#include <ruby_parser/token.hh>

using namespace ruby_parser;

token_type token::type() const {
    return _type;
}

size_t token::start() const {
    return _start;
}

size_t token::end() const {
    return _end;
}

const std::string& token::string() const {
    return _string;
}
