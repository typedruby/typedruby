#ifndef RUBY_PARSER_PARSER_HH
#define RUBY_PARSER_PARSER_HH

#include <memory>

#include "lexer.hh"
#include "node.hh"
#include "builder.hh"

namespace ruby_parser {
  namespace parser {
    class base {
    public:
      node_ptr ast;
      std::unique_ptr<lexer> lexer;
      size_t def_level;
      const builder& builder;

      base(ruby_version version, const std::string& source, const struct builder& builder);

      void check_kwarg_name(const token_ptr& name);
    };

    class typedruby24 : public base {
    public:
      typedruby24(const std::string& source, const struct builder& builder);

      node_ptr parse();
    };
  }
};

#endif
