#ifndef RUBY_PARSER_TYPEDRUBY24_HH
#define RUBY_PARSER_TYPEDRUBY24_HH

#include <memory>

#include "Node.hh"
#include "Lexer.hh"

namespace ruby_parser {
    class TypedRuby24 {
        std::string filename;
        Lexer lexer;
    public:
        TypedRuby24(std::string& filename, std::string& source);
        std::unique_ptr<Node> parse();
    };
}

#endif
