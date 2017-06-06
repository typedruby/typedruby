#ifndef RUBY_PARSER_DIAGNOSTIC_HH
#define RUBY_PARSER_DIAGNOSTIC_HH

#include <cstddef>
#include <string>
#include <vector>

#include "token.hh"

namespace ruby_parser {

enum class dlevel {
	NOTE    = 1,
	WARNING = 2,
	ERROR   = 3,
	FATAL   = 4,
};

#include "diagnostic_class.hh"

class diagnostic {
public:
	struct range {
		const size_t begin_pos;
		const size_t end_pos;

		range(size_t begin_pos, size_t end_pos)
			: begin_pos(begin_pos)
			  , end_pos(end_pos)
		{}
	};

private:
	dlevel level_;
	dclass type_;
	range location_;
	std::string message_;

public:
	diagnostic(dlevel lvl, dclass type, range location, const std::string& message = "")
		: level_(lvl)
		  , type_(type)
		  , location_(location)
		  , message_(message)
	{}

	diagnostic(dlevel lvl, dclass type, const token *token, const std::string& message = "")
		: level_(lvl)
		  , type_(type)
		  , location_(token->start(), token->end())
		  , message_(message)
	{}

	dlevel level() const {
		return level_;
	}

	dclass error_class() const {
		return type_;
	}

	const std::string& message() const {
		return message_;
	}

	const range& location() const {
		return location_;
	}
};

using diagnostics_t = std::vector<diagnostic>;

}

#endif
