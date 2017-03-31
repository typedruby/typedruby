#include <ruby_parser/parser.hh>
#include <iostream>

int main()
{
  ruby_parser::parser::typedruby24 p {
    "a = 1 + 2"
  };

  p.parse();
}
