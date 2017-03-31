#include <ruby_parser/parser.hh>
#include <iostream>
#include <fstream>

int main(int argc, const char** argv)
{
  if (argc < 2) {
    std::cerr << "usage: " << argv[0] << " <source file>" << std::endl;
    return 1;
  }

  std::ifstream file { argv[1], std::ios_base::in };

  std::string source;

  file >> source;

  ruby_parser::parser::typedruby24 p { source };

  p.parse();
}
