CXXFLAGS += -Wall -Wextra -pedantic -std=c++1y -I inc

OBJECTS = \
	src/Lexer.o \
	src/Literal.o \
	src/Token.o \
	src/TypedRuby24.o \

RAGEL ?= ragel
BISON ?= bison

.SUFFIXES:

.PHONY: all clean

all: librubyparser.a

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
