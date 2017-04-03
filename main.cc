#include <ruby_parser/parser.hh>
#include <iostream>
#include <fstream>

std::string read_entire(std::istream& is)
{
    std::string str;
    char buff[4096];

    while (!is.eof()) {
        is.read(buff, sizeof(buff));
        str.append(buff, is.gcount());
    }

    return str;
}

int main(int argc, const char** argv)
{
  // if (argc < 2) {
  //   std::cerr << "usage: " << argv[0] << " <source file>" << std::endl;
  //   return 1;
  // }

  // std::ifstream file { argv[1], std::ios_base::in };

  // std::string source = read_entire(file);

  // ruby_parser::parser::typedruby24 p { source };

  // p.parse();
}
