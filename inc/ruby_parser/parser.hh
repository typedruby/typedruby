#ifndef RUBY_PARSER_PARSER_HH
#define RUBY_PARSER_PARSER_HH

#include <memory>

#include "lexer.hh"

namespace ruby_parser {
    namespace parser {
        class base {
        public:
            std::unique_ptr<lexer> lexer;
            size_t def_level;
            base(ruby_version version, const std::string& source);
        };

        class typedruby24 : public base {
        public:
            typedruby24(const std::string& source);
        };
    }
};

#endif
