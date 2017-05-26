#include <ruby_parser/parser.hh>
#include <ruby_parser/lexer.hh>
#include "grammars/typedruby24.hh"

using namespace ruby_parser;

parser::base::base(ruby_version version, const std::string& source, const struct builder& builder)
    : lexer_(std::make_unique<class lexer>(*this, version, source))
    , def_level(0)
    , builder(builder)
    , ast(0)
{
}

parser::base::~base()
{}

parser::typedruby24::typedruby24(const std::string& source, const struct builder& builder)
    : base(ruby_version::RUBY_24, source, builder)
{}

void parser::base::check_kwarg_name(const token *name) {
  char c = name->string().at(0);

  if (c >= 'A' && c <= 'Z') {
	  // XXX: todo
    // diagnostic :error, :argument_const, nil, name_t
  }
}

foreign_ptr parser::typedruby24::parse() {
	yy::parser p(*this);
	//p.set_debug_level(1);
	p.parse();
	return std::move(ast);
}
