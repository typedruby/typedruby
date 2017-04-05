#ifndef RUBY_PARSER_CAPI_HH
#define RUBY_PARSER_CAPI_HH

#include "token.hh"
#include "node.hh"
#include "builder.hh"
#include "parser.hh"

extern "C" {

void*
ruby_parser_typedruby24_parse(const char* source, size_t source_length, ruby_parser::builder* builder);

bool
ruby_parser_static_env_is_declared(ruby_parser::parser::base* p, ruby_parser::token* tok);

void
ruby_parser_static_env_declare(ruby_parser::parser::base* p, ruby_parser::token* tok);

size_t
ruby_parser_token_get_start(ruby_parser::token* tok);

size_t
ruby_parser_token_get_end(ruby_parser::token* tok);

size_t
ruby_parser_token_get_string(ruby_parser::token* tok, const char** out_ptr);

size_t
ruby_parser_node_list_get_length(ruby_parser::node_list* list);

void*
ruby_parser_node_list_index(ruby_parser::node_list* list, size_t index);

}

#endif
