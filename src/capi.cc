#include <ruby_parser/capi.hh>
#include <cstdio>

ruby_parser::parser::typedruby24*
ruby_parser_typedruby24_new(const char* source_ptr, size_t source_length, const ruby_parser::builder* builder)
{
  std::string source { source_ptr, source_length };
  return new ruby_parser::parser::typedruby24(source, *builder);
}

void
ruby_parser_typedruby24_free(ruby_parser::parser::typedruby24* parser)
{
  delete parser;
}

void*
ruby_parser_parse(ruby_parser::parser::base* parser)
{
  return parser->parse().release();
}

bool
ruby_parser_static_env_is_declared(const ruby_parser::parser::base* p, const char* name, size_t length)
{
  std::string id { name, length };
  return p->lexer_->is_declared(id);
}

void
ruby_parser_static_env_declare(ruby_parser::parser::base* p, const char* name, size_t length)
{
  std::string id { name, length };
  p->lexer_->declare(id);
}

size_t
ruby_parser_token_get_start(const ruby_parser::token* tok)
{
  return tok->start();
}

size_t
ruby_parser_token_get_end(const ruby_parser::token* tok)
{
  return tok->end();
}

size_t
ruby_parser_token_get_string(const ruby_parser::token* tok, const char** out_ptr)
{
  *out_ptr = tok->string().data();
  return tok->string().size();
}

size_t
ruby_parser_node_list_get_length(const ruby_parser::node_list* list)
{
  return list->nodes.size();
}

void*
ruby_parser_node_list_index(ruby_parser::node_list* list, size_t index)
{
  return list->nodes.at(index).release();
}

size_t
ruby_parser_diagnostics_get_length(const ruby_parser::parser::base* parser)
{
  return parser->diagnostics.size();
}

ruby_parser::diagnostic_level
ruby_parser_diagnostic_get_level(const ruby_parser::parser::base* parser, size_t index)
{
  return parser->diagnostics.at(index).level();
}

size_t
ruby_parser_diagnostic_get_message(const ruby_parser::parser::base* parser, size_t index, const char** out_ptr)
{
  auto& message = parser->diagnostics.at(index).message();
  *out_ptr = message.data();
  return message.size();
}

size_t
ruby_parser_diagnostic_get_begin(const ruby_parser::parser::base* parser, size_t index)
{
  return parser->diagnostics.at(index).location().begin_pos;
}

size_t
ruby_parser_diagnostic_get_end(const ruby_parser::parser::base* parser, size_t index)
{
  return parser->diagnostics.at(index).location().end_pos;
}
