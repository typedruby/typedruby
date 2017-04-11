#ifndef RUBY_PARSER_PARSER_HH
#define RUBY_PARSER_PARSER_HH

#include <memory>

#include "lexer.hh"
#include "node.hh"
#include "diagnostic.hh"

namespace ruby_parser {
  struct builder;

  typedef void* foreign_ptr;

  struct object {
    object() {};
  public:
    virtual ~object() {};
  };

  struct node_list : public object {
    std::vector<foreign_ptr> nodes;

    node_list() {}
    node_list(decltype(nodes)&& nodes) : nodes(std::move(nodes)) {}
  };
  using node_list_ptr = std::unique_ptr<node_list>;

  struct delimited_node_list : public object {
    token_ptr begin;
    node_list_ptr inner;
    token_ptr end;

    delimited_node_list(token_ptr&& begin, node_list_ptr&& inner, token_ptr&& end)
      : begin(std::move(begin)), inner(std::move(inner)), end(std::move(end)) {}
  };
  using delimited_node_list_ptr = std::unique_ptr<delimited_node_list>;

  struct delimited_block : public object {
    token_ptr begin;
    foreign_ptr args;
    foreign_ptr body;
    token_ptr end;

    delimited_block(token_ptr&& begin, foreign_ptr&& args, foreign_ptr&& body, token_ptr&& end)
      : begin(std::move(begin)), args(std::move(args)), body(std::move(body)), end(std::move(end)) {}
  };
  using delimited_block_ptr = std::unique_ptr<delimited_block>;

  struct node_with_token : public object {
    token_ptr token_;
    foreign_ptr node_;

    node_with_token(token_ptr&& token_, foreign_ptr&& node_)
      : token_(std::move(token_)), node_(std::move(node_)) {}
  };
  using node_with_token_ptr = std::unique_ptr<node_with_token>;

  struct case_body : public object {
    node_list_ptr whens;
    node_with_token_ptr else_;

    case_body(node_with_token_ptr else_) : whens(std::make_unique<node_list>()), else_(std::move(else_)) {}
  };
  using case_body_ptr = std::unique_ptr<case_body>;

  namespace parser {
    class base {
    public:
      foreign_ptr ast;
      std::unique_ptr<lexer> lexer_;
      std::vector<diagnostic> diagnostics;
      size_t def_level;
      const struct builder& builder;
      std::set<void*> saved_pointers;

      base(ruby_version version, const std::string& source, const struct builder& builder);
      virtual ~base();

      virtual foreign_ptr parse() = 0;

      void check_kwarg_name(const token_ptr& name);

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
