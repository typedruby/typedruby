#ifndef LEXER_H
#define LEXER_H

#include <stddef.h>

typedef enum {
  RUBY_18,
  RUBY_19,
  RUBY_20,
  RUBY_21,
  RUBY_22,
  RUBY_23,
  RUBY_24,
}
ruby_version_t;

typedef struct ruby_lexer_state_t ruby_lexer_state_t;

ruby_lexer_state_t*
ruby_lexer_init(ruby_version_t version, const char* source, size_t len);

void
ruby_lexer_free(ruby_lexer_state_t* lexer);

void
ruby_lexer_env_extend_static(ruby_lexer_state_t* lexer);

void
ruby_lexer_env_extend_dynamic(ruby_lexer_state_t* lexer);

void
ruby_lexer_env_unextend(ruby_lexer_state_t* lexer);

void
ruby_lexer_env_declare(ruby_lexer_state_t* lexer, const char* name, size_t len);

typedef enum {
  T_ERROR,
  T_EOF,
  T_NTH_REF,
}
ruby_token_type_t;

ruby_token_type_t
ruby_lexer_advance(ruby_lexer_state_t* lexer, const char** ptr, size_t* len);

#endif
