#ifndef RUBY_PARSER_DRIVER_HH
#define RUBY_PARSER_DRIVER_HH

#include <memory>

#include "lexer.hh"
#include "node.hh"
#include "diagnostic.hh"

namespace ruby_parser {

struct builder;

using foreign_ptr = const void*;

struct node_list {
	node_list() = default;
	node_list(foreign_ptr node) {
		nodes.push_back(node);
	}

	node_list& operator=(const foreign_ptr &other) = delete;
	node_list& operator=(foreign_ptr &&other) = delete;

	inline size_t size() const {
		return nodes.size();
	}

	inline void push_back(const foreign_ptr &ptr) {
		nodes.push_back(ptr);
	}

	inline void push_front(const foreign_ptr &ptr) {
		nodes.insert(nodes.begin(), ptr);
	}

	inline foreign_ptr &at(size_t n) { return nodes.at(n); }

	inline void concat(node_list *other) {
		nodes.insert(nodes.end(),
			std::make_move_iterator(other->nodes.begin()),
			std::make_move_iterator(other->nodes.end())
		);
	}

protected:
	std::vector<foreign_ptr> nodes;
};

struct delimited_node_list {
	delimited_node_list() = default;
	delimited_node_list(const token_t &begin, node_list *inner, const token_t &end)
		: begin(begin), inner(inner), end(end) {}

	token_t begin = nullptr;
	node_list *inner = nullptr;
	token_t end = nullptr;
};

struct delimited_block {
	delimited_block() = default;
	delimited_block(const token_t &begin, foreign_ptr args, foreign_ptr body, const token_t &end)
		: begin(begin), args(args), body(body), end(end) {}

	token_t begin = nullptr;
	foreign_ptr args = nullptr;
	foreign_ptr body = nullptr;
	token_t end = nullptr;
};

struct node_with_token {
	node_with_token() = default;
	node_with_token(const token_t &token_, foreign_ptr node_)
		: tok(token_), nod(node_) {}

	token_t tok = nullptr;
	foreign_ptr nod = nullptr;
};

struct case_body {
	case_body() = default;
	case_body(node_with_token *else_) : els(else_) {}
	node_list whens;
	node_with_token *els = nullptr;
};

struct mempool {
	pool<ruby_parser::node_list, 16> node_list;
	pool<ruby_parser::delimited_node_list, 32> delimited_node_list;
	pool<ruby_parser::delimited_block, 32> delimited_block;
	pool<ruby_parser::node_with_token, 32> node_with_token;
	pool<ruby_parser::case_body, 32> case_body;
	pool<ruby_parser::state_stack, 8> stacks;
};

class base_driver {
public:
	diagnostics_t diagnostics;
	const builder& build;
	lexer lex;
	mempool pool;

	size_t def_level;
	foreign_ptr ast;

	base_driver(ruby_version version, const std::string& source, const struct builder& builder);
	virtual ~base_driver() {}
	virtual foreign_ptr parse() = 0;

	void check_kwarg_name(const token *name);

	ruby_parser::state_stack *copy_stack() {
		return pool.stacks.alloc(lex.cmdarg);
	}

	void replace_stack(ruby_parser::state_stack *stack) {
		lex.cmdarg = *stack;
	}
};

class typedruby24 : public base_driver {
public:
	typedruby24(const std::string& source, const struct builder& builder);
	virtual foreign_ptr parse();
	~typedruby24() {}
};

}

#endif
