#ifndef RUBY_PARSER_CAPI_HH
#define RUBY_PARSER_CAPI_HH

#include "token.hh"
#include "node.hh"
#include "builder.hh"
#include "parser.hh"

extern "C" {

typedef void ruby_parser_typedruby_t;

ruby_parser::typedruby24*
ruby_parser_typedruby24_new(const char* source, size_t source_length, const ruby_parser::builder* builder);

void
ruby_parser_typedruby24_free(ruby_parser::typedruby24* parser);

void*
ruby_parser_parse(ruby_parser::base_driver* parser);

bool
ruby_parser_static_env_is_declared(const ruby_parser::base_driver *p, const char* name, size_t length);

void
ruby_parser_static_env_declare(ruby_parser::base_driver *p, const char* name, size_t length);

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

size_t
ruby_parser_diagnostics_get_length(const ruby_parser::base_driver* parser);

ruby_parser::diagnostic_level
ruby_parser_diagnostic_get_level(const ruby_parser::base_driver* parser, size_t index);

size_t
ruby_parser_diagnostic_get_message(const ruby_parser::base_driver* parser, size_t index, const char** out_ptr);

size_t
ruby_parser_diagnostic_get_begin(const ruby_parser::base_driver* parser, size_t index);

size_t
ruby_parser_diagnostic_get_end(const ruby_parser::base_driver* parser, size_t index);

}

#endif
