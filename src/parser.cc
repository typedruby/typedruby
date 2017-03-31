#include <ruby_parser/parser.hh>
#include <ruby_parser/lexer.hh>

using namespace ruby_parser;

parser::base::base(ruby_version version, const std::string& source)
    : lexer(std::make_unique<class lexer>(version, source))
    , def_level(0)
{
}

parser::typedruby24::typedruby24(const std::string& source)
    : base(ruby_version::RUBY_24, source)
{}

void parser::base::check_kwarg_name(const token_ptr& name) {
    char c = name->string().at(0);

    if (c >= 'A' && c <= 'Z') {
        // diagnostic :error, :argument_const, nil, name_t
    }
}
