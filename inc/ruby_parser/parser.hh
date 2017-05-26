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
		node_list() : exists(true) {}
		node_list(bool exists) : exists(exists) {}
		node_list(foreign_ptr node) : nodes(), exists(true) {
			nodes.push_back(node);
		}

		node_list& operator=(const foreign_ptr &other) = delete;
		node_list& operator=(foreign_ptr &&other) = delete;

		inline size_t size() const { return nodes.size(); }

		inline void push_back(const foreign_ptr &ptr) {
			assert(exists);
			nodes.push_back(ptr);
		}

		inline void push_front(const foreign_ptr &ptr) {
			assert(exists);
			nodes.insert(nodes.begin(), ptr);
		}

		inline foreign_ptr &first() { return nodes[0]; }

		inline void concat(node_list &other) {
			assert(exists);
			nodes.insert(nodes.end(),
				std::make_move_iterator(other.nodes.begin()),
				std::make_move_iterator(other.nodes.end())
			);
		}

		inline operator bool() const { return exists; }

		std::vector<foreign_ptr> nodes;
		bool exists;
	};

	struct delimited_node_list {
		delimited_node_list(const token_t &begin, const node_list &inner, const token_t &end)
			: begin(begin), inner(inner), end(end) {}

		delimited_node_list()
			: begin(nullptr), inner(), end(nullptr) {}

		token_t begin;
		node_list inner;
		token_t end;
	};

	struct delimited_block {
		delimited_block(const token_t &begin, foreign_ptr args, foreign_ptr body, const token_t &end)
			: begin(begin), args(args), body(body), end(end) {}

		delimited_block()
			: begin(nullptr), args(nullptr), body(nullptr), end(nullptr) {}

		token_t begin;
		foreign_ptr args;
		foreign_ptr body;
		token_t end;
	};

	struct node_with_token {
		node_with_token(const token_t &token_, foreign_ptr node_)
			: tok(token_), nod(node_) {}

		node_with_token()
			: tok(nullptr), nod(nullptr) {}

		operator bool() const { return tok && nod; }

		token_t tok;
		foreign_ptr nod;
	};

	struct case_body {
		case_body(const node_with_token &else_) : whens(), els(else_) {}
		case_body() : whens(), els() {}

		node_list whens;
		node_with_token els;
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
