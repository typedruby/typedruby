#ifndef RUBY_PARSER_TYPEDRUBY24_HH
#define RUBY_PARSER_TYPEDRUBY24_HH

#include <memory>

#include "node.hh"
#include "lexer.hh"

namespace ruby_parser {
    class typedruby24 {
        std::string filename;
        lexer lexer;
    public:
        typedruby24(std::string& filename, std::string& source);
        std::unique_ptr<Node> parse();
    };
}

#endif
