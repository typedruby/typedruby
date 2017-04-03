#include <ruby_parser/parser.hh>
#include <ruby_parser/lexer.hh>

using namespace ruby_parser;

parser::base::base(ruby_version version, const std::string& source, const struct builder& builder)
    : lexer(std::make_unique<class lexer>(version, source))
    , def_level(0)
    , builder(builder)
{
}

parser::typedruby24::typedruby24(const std::string& source, const struct builder& builder)
    : base(ruby_version::RUBY_24, source, builder)
{}

void parser::base::check_kwarg_name(const token_ptr& name) {
  char c = name->string().at(0);

  if (c >= 'A' && c <= 'Z') {
    // diagnostic :error, :argument_const, nil, name_t
  }
}
extern "C" {
  int ruby_parser_typedruby24_yyparse(parser::typedruby24&);
}

node_ptr parser::typedruby24::parse() {
  ruby_parser_typedruby24_yyparse(*this);
  return std::move(ast);
}
