CXXFLAGS += -Wall -Wextra -pedantic -std=c++1y -I inc -g

OBJECTS = \
	src/lexer.o \
	src/literal.o \
	src/parser.o \
	src/state_stack.o \
	src/token.o \
	src/grammars/typedruby24.o \

RAGEL ?= ragel
BISON ?= bison

.SUFFIXES:

.PHONY: all clean

all: main librubyparser.a

main: main.o librubyparser.a
	$(CXX) -o $@ $(CXXFLAGS) $(LDFLAGS) $^

clean:
	rm -f librubyparser.a $(OBJECTS)

librubyparser.a: $(OBJECTS)
	$(AR) rcs $@ $^

%.cc: %.rl
	$(RAGEL) -o $@ -C $<

%.cc: %.y
	$(BISON) -o $@ $<

%.o: %.cc inc/ruby_parser/*.hh
	$(CXX) -o $@ $(CXXFLAGS) -c $<
