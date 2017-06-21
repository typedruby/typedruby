#ifndef RUBY_PARSER_DRIVER_HH
#define RUBY_PARSER_DRIVER_HH

#include <memory>

#include "lexer.hh"
#include "node.hh"
#include "diagnostic.hh"

namespace ruby_parser {

struct builder;

using foreign_ptr = const void*;
using self_ptr = const void *;

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

class mempool {
	pool<ruby_parser::node_list, 16> _node_list;
	pool<ruby_parser::delimited_node_list, 32> _delimited_node_list;
	pool<ruby_parser::delimited_block, 32> _delimited_block;
	pool<ruby_parser::node_with_token, 32> _node_with_token;
	pool<ruby_parser::case_body, 32> _case_body;
	pool<ruby_parser::state_stack, 8> _stacks;
	friend class base_driver;

public:
	mempool() = default;

	template <typename... Args>
	ruby_parser::node_list *node_list(Args&&... args) {
		return _node_list.alloc(std::forward<Args>(args)...);
	}

	template <typename... Args>
	ruby_parser::delimited_node_list *delimited_node_list(Args&&... args) {
		return _delimited_node_list.alloc(std::forward<Args>(args)...);
	}

	template <typename... Args>
	ruby_parser::delimited_block *delimited_block(Args&&... args) {
		return _delimited_block.alloc(std::forward<Args>(args)...);
	}

	template <typename... Args>
	ruby_parser::node_with_token *node_with_token(Args&&... args) {
		return _node_with_token.alloc(std::forward<Args>(args)...);
	}

	template <typename... Args>
	ruby_parser::case_body *case_body(Args&&... args) {
		return _case_body.alloc(std::forward<Args>(args)...);
	}
};

class base_driver {
public:
	diagnostics_t diagnostics;
	const builder& build;
	lexer lex;
	mempool alloc;

	bool pending_error;
	size_t def_level;
	foreign_ptr ast;

	base_driver(ruby_version version, const std::string& source, const struct builder& builder);
	virtual ~base_driver() {}
	virtual foreign_ptr parse(self_ptr self) = 0;

	bool valid_kwarg_name(const token *name) {
		char c = name->string().at(0);
		return !(c >= 'A' && c <= 'Z');
	}

	ruby_parser::state_stack *copy_stack() {
		return alloc._stacks.alloc(lex.cmdarg);
	}

	void replace_stack(ruby_parser::state_stack *stack) {
		lex.cmdarg = *stack;
	}

	void external_diagnostic(dlevel lvl, dclass cls, size_t begin, size_t end, const std::string &msg) {
		diagnostics.emplace_back(lvl, cls, diagnostic::range(begin, end), msg);
		if (lvl == dlevel::ERROR) {
			pending_error = true;
		}
	}
};

class typedruby24 : public base_driver {
public:
	typedruby24(const std::string& source, const struct builder& builder);
	virtual foreign_ptr parse(self_ptr self);
	~typedruby24() {}
};

}

#endif
