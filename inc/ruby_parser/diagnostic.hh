#ifndef RUBY_PARSER_DIAGNOSTIC_HH
#define RUBY_PARSER_DIAGNOSTIC_HH

#include <cstddef>
#include <string>

#include "token.hh"

namespace ruby_parser {
  enum class diagnostic_level {
    NOTE    = 1,
    WARNING = 2,
    ERROR   = 3,
    FATAL   = 4,
  };

  class diagnostic {
  public:
    struct range {
      const size_t begin_pos;
      const size_t end_pos;

      range(size_t begin_pos, size_t end_pos)
        : begin_pos(begin_pos)
        , end_pos(end_pos)
      {}
    };

  private:
    diagnostic_level level_;
    std::string message_;
    range location_;

  public:
    diagnostic(diagnostic_level level, const std::string& message, range location)
      : level_(level)
      , message_(message)
      , location_(location)
    {}

    diagnostic(diagnostic_level level, const std::string& message, const token *token)
      : level_(level)
      , message_(message)
      , location_(token->start(), token->end())
    {}

    diagnostic_level level() const {
      return level_;
    }

    const std::string& message() const {
      return message_;
    }

    const range& location() const {
      return location_;
    }
  };
}

#endif
