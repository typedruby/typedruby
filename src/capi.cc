#include <ruby_parser/capi.hh>
#include <cstdio>

void*
ruby_parser_typedruby24_parse(const char* source_, size_t source_length, ruby_parser::builder* builder)
{
    std::string source { source_, source_length };
    ruby_parser::parser::typedruby24 parser { source, *builder };
    auto ast = parser.parse().release();
    return ast;
}

size_t
ruby_parser_token_get_start(ruby_parser::token* tok)
{
  return tok->start();
}

size_t
ruby_parser_token_get_end(ruby_parser::token* tok)
{
  return tok->end();
}

size_t
ruby_parser_token_get_string(ruby_parser::token* tok, const char** out_ptr)
{
  *out_ptr = tok->string().data();
  return tok->string().size();
}

size_t
ruby_parser_node_list_get_length(ruby_parser::node_list* list)
{
  return list->nodes.size();
}

void*
ruby_parser_node_list_index(ruby_parser::node_list* list, size_t index)
{
  return list->nodes.at(index).release();
}
