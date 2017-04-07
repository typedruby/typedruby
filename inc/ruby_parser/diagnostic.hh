#ifndef DIAGNOSTIC_HH
#define DIAGNOSTIC_HH

#include <cstddef>
#include <string>

#include "token.hh"

namespace ruby_parser {
  class diagnostic {
  public:
    enum class level {
      NOTE,
      WARNING,
      ERROR,
      FATAL,
    };

    struct range {
      const size_t begin_pos;
      const size_t end_pos;

      range(size_t begin_pos, size_t end_pos)
        : begin_pos(begin_pos)
        , end_pos(end_pos)
      {}
    };

  private:
    level level_;
    std::string message_;
    range location_;

  public:
    diagnostic(level level, std::string message, range location)
      : level_(level)
      , message_(message)
      , location_(location)
    {}

    diagnostic(level level, std::string message, token& token)
      : level_(level)
      , message_(message)
      , location_(token.start(), token.end())
    {}

    level level() const {
      return level_;
    }

    const std::string& message() const {
      return message_;
    }

    const range& range() const {
      return location_;
    }
  };
}

#endif
