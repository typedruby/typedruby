CXXFLAGS += -Wall -Wextra -pedantic -std=c++1y -I inc

ifeq ($(PROFILE),release)
	CXXFLAGS += -O3
else
	CXXFLAGS += -g
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

src/builder.o: CXXFLAGS += -Wno-unused-parameter
src/lexer.o: CXXFLAGS += -Wno-unused-const-variable

.SUFFIXES:

.PHONY: all clean

all: $(LIB_PATH)

main: main.o $(LIB_PATH)
	$(CXX) -o $@ $(CXXFLAGS) $(LDFLAGS) $^

clean:
	rm -f librubyparser.a $(OBJECTS)

$(LIB_PATH): $(OBJECTS)
	$(AR) rcs $@ $^

%.cc: %.rl
	$(RAGEL) -o $@ -C $<

%.cc: %.y
	$(BISON) -o $@ $<

%.o: %.cc inc/ruby_parser/*.hh
	$(CXX) -o $@ $(CXXFLAGS) -c $<
