#include <ruby_parser/parser.hh>
#include <iostream>

int main()
{
  std::string source = "def foo; nil; end";
  ruby_parser::parser::typedruby24 p { source };

  while (true) {
    ruby_parser::token_ptr tok = p.lexer->advance();

    std::cout << *tok << std::endl;

    if (tok->type() == ruby_parser::token_type::eof
        || tok->type() == ruby_parser::token_type::error) {
      break;
    }
  }
}
