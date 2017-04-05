#ifndef RUBY_PARSER_CAPI_HH
#define RUBY_PARSER_CAPI_HH

#include "token.hh"
#include "node.hh"
#include "builder.hh"
#include "parser.hh"

extern "C" {

void*
ruby_parser_typedruby24_parse(const char* source, size_t source_length, const ruby_parser::builder* builder);

bool
ruby_parser_static_env_is_declared(const ruby_parser::parser::base* p, const char* name, size_t length);

void
ruby_parser_static_env_declare(ruby_parser::parser::base* p, const char* name, size_t length);

size_t
ruby_parser_token_get_start(const ruby_parser::token* tok);

size_t
ruby_parser_token_get_end(const ruby_parser::token* tok);

size_t
ruby_parser_token_get_string(const ruby_parser::token* tok, const char** out_ptr);

size_t
ruby_parser_node_list_get_length(const ruby_parser::node_list* list);

void*
ruby_parser_node_list_index(ruby_parser::node_list* list, size_t index);

}

#endif
