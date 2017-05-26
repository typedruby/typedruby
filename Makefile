CXXFLAGS += -Wall -Wextra -pedantic -std=c++14 -I inc -fPIC

ifeq ($(PROFILE),release)
	CXXFLAGS += -O3
else
	CXXFLAGS += -ggdb3 -O0
endif

OBJECTS = \
	src/capi.o \
	src/lexer.o \
	src/literal.o \
	src/parser.o \
	src/state_stack.o \
	src/token.o \
	src/grammars/typedruby24.o \

RAGEL ?= ragel
BISON ?= bison

LIB_PATH ?= librubyparser.a

.SUFFIXES:
.PHONY: all clean

all: $(LIB_PATH)

clean:
	rm -f librubyparser.a $(OBJECTS) src/grammars/*.cc src/grammars/*.hh

$(LIB_PATH): $(OBJECTS)
	$(AR) rcs $@ $^

%.o: %.cc inc/ruby_parser/*.hh src/grammars/typedruby24.hh
	$(CXX) -o $@ $(CXXFLAGS) -c $<

%.cc: %.rl
	$(RAGEL) -o $@ -C $<

%.cc %.hh: %.ypp
	$(BISON) --defines=$*.hh -o $*.cc $*.ypp

# Do not remove generated Bison output
.PRECIOUS: %.cc %.hh
