#ifndef RUBY_PARSER_PARSER_HH
#define RUBY_PARSER_PARSER_HH

#include <memory>

#include "lexer.hh"
#include "node.hh"
#include "diagnostic.hh"

namespace ruby_parser {
  struct builder;

  typedef void* foreign_ptr;

  struct node_list {
    std::vector<foreign_ptr> nodes;

    node_list() {}
    node_list(decltype(nodes)&& nodes) : nodes(std::move(nodes)) {}
  };

  struct delimited_node_list {
    token *begin;
    node_list *inner;
    token *end;

    delimited_node_list(token *begin, node_list *inner, token *end)
      : begin(begin), inner(inner), end(end) {}
  };

  struct delimited_block {
    token *begin;
    foreign_ptr args;
    foreign_ptr body;
    token *end;

    delimited_block(token *begin, foreign_ptr args, foreign_ptr body, token *end)
      : begin(begin), args(args), body(body), end(end) {}
  };

  struct node_with_token {
    token *token_;
    foreign_ptr node_;

    node_with_token(token *token_, foreign_ptr node_)
      : token_(token_), node_(node_) {}
  };

  struct case_body {
    node_list *whens;
    node_with_token *else_;

    case_body(node_with_token *else_) : whens(new node_list()), else_(else_) {}
  };

  namespace parser {
    class base {
    public:
      std::unique_ptr<lexer> lexer_;
      std::vector<diagnostic> diagnostics;
      size_t def_level;
      const struct builder& builder;
      foreign_ptr ast;

      base(ruby_version version, const std::string& source, const struct builder& builder);
      virtual ~base();

      virtual foreign_ptr parse() = 0;

      void check_kwarg_name(const token *name);

      template<typename... Args>
      void diagnostic_(Args&&... args) {
        diagnostics.emplace_back(std::forward<Args>(args)...);
      }
    };

    class typedruby24 : public base {
    public:
      typedruby24(const std::string& source, const struct builder& builder);
      virtual foreign_ptr parse();
    };
  }
}

#endif
