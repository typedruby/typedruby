#ifndef RUBY_PARSER_DRIVER_HH
#define RUBY_PARSER_DRIVER_HH

#include <memory>

#include "lexer.hh"
#include "node.hh"
#include "diagnostic.hh"
#include "optional.hpp"

namespace ruby_parser {

struct builder;

typedef void* foreign_ptr;

struct node_list {
	node_list() : nodes(std::in_place) {}
	node_list(std::nullopt_t) : nodes(std::nullopt) {}
	node_list(foreign_ptr node) : nodes(std::in_place) {
		nodes->push_back(node);
	}

	node_list& operator=(const foreign_ptr &other) = delete;
	node_list& operator=(foreign_ptr &&other) = delete;

	inline size_t size() const {
		if (!nodes.has_value())
			return 0;
		return nodes->size();
	}

	inline void push_back(const foreign_ptr &ptr) {
		nodes->push_back(ptr);
	}

	inline void push_front(const foreign_ptr &ptr) {
		nodes->insert(nodes->begin(), ptr);
	}

	inline foreign_ptr &at(size_t n) { return nodes->at(n); }

	inline void concat(node_list &other) {
		nodes->insert(nodes->end(),
			std::make_move_iterator(other.nodes->begin()),
			std::make_move_iterator(other.nodes->end())
		);
	}

	inline operator bool() const { return nodes.has_value(); }

protected:
	std::optional<std::vector<foreign_ptr>> nodes;
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

class base_driver {
public:
	diagnostics_t diagnostics;
	const builder& build;
	lexer lex;

	size_t def_level;
	foreign_ptr ast;

	base_driver(ruby_version version, const std::string& source, const struct builder& builder);
	virtual ~base_driver() {}
	virtual foreign_ptr parse() = 0;

	void check_kwarg_name(const token *name);
};

class typedruby24 : public base_driver {
public:
	typedruby24(const std::string& source, const struct builder& builder);
	virtual foreign_ptr parse();
	~typedruby24() {}
};

}

#endif
