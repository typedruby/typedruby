#ifndef LEXER_H
#define LEXER_H

#include <stddef.h>
#include <stdint.h>

#include "tokens_gen.h"

typedef enum {
  RUBY_18 = 18,
  RUBY_19 = 19,
  RUBY_20 = 20,
  RUBY_21 = 21,
  RUBY_22 = 22,
  RUBY_23 = 23,
  RUBY_24 = 24,
}
ruby_version_t;

typedef struct ruby_lexer_state_t ruby_lexer_state_t;

ruby_lexer_state_t*
ruby_lexer_init(ruby_version_t version, void* context, const char* source, size_t len);

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

struct ruby_token_t {
  int type;
  size_t offset_start;
  size_t offset_end;
  const char* value_ptr;
  size_t value_len;
};

void
ruby_lexer_advance(ruby_lexer_state_t* lexer);

void
ruby_lexer_foo();

#endif
