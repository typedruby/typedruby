#include <ruby_parser/parser.hh>
#include <ruby_parser/lexer.hh>
#include "grammars/typedruby24.hh"

namespace ruby_parser {

base_driver::base_driver(ruby_version version, const std::string& source, const struct builder& builder)
	: build(builder),
	lex(diagnostics, version, source),
	def_level(0),
	ast(nullptr)
{
}

void base_driver::check_kwarg_name(const token *name) {
	char c = name->string().at(0);

	if (c >= 'A' && c <= 'Z') {
		// XXX: todo
		// diagnostic :error, :argument_const, nil, name_t
	}
}

typedruby24::typedruby24(const std::string& source, const struct builder& builder)
	: base_driver(ruby_version::RUBY_24, source, builder)
{}

foreign_ptr typedruby24::parse() {
	bison::typedruby24::parser p(*this);
	p.parse();
	return std::move(ast);
}

}
