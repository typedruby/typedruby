#ifndef RUBY_PARSER_OPTIONAL_SIZE_T
#define RUBY_PARSER_OPTIONAL_SIZE_T

#include <limits>
#include <cassert>

namespace ruby_parser {
  class optional_size {
    const size_t none_value = std::numeric_limits<size_t>::max();

    size_t value;

  public:
    constexpr optional_size() : value(none_value) {}
    constexpr optional_size(size_t value) : value(value) {}

    static constexpr optional_size none() {
      optional_size sz;
      return sz;
    }

    operator bool() const {
      return value != none_value;
    }

    optional_size& operator=(const optional_size& other) {
      value = other.value;
      return *this;
    }

    optional_size& operator=(size_t other) {
      value = other;
      return *this;
    }

    size_t operator*() const {
      assert(value != none_value);
      return value;
    }

    size_t operator||(size_t default_) const {
      if (*this) {
        return value;
      } else {
        return default_;
      }
    }
  };
}

#endif
