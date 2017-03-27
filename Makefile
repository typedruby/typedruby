CXXFLAGS += -Wall -Wextra -pedantic -std=c++1y -I inc

OBJECTS = src/Token.o src/Literal.o src/Lexer.o

RAGEL ?= ragel

librubyparser.a: $(OBJECTS)
	$(AR) rcs $@ $^

%.cc: %.rl
	$(RAGEL) -o $@ -C $<

%.o: %.cc inc/ruby_parser/*.hh
	$(CXX) -o $@ $(CXXFLAGS) -c $<
