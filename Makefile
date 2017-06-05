CXXFLAGS += -Wall -Wextra -pedantic -std=c++14 -I include -fPIC

ifeq ($(PROFILE),release)
	CXXFLAGS += -O3
else
	CXXFLAGS += -ggdb3 -O0
endif

OBJECTS = \
	src/capi.o \
	src/lexer.o \
	src/literal.o \
	src/driver.o \
	src/state_stack.o \
	src/token.o \
	src/grammars/typedruby24.o \

RAGEL ?= ragel
BISON ?= bison

LIB_PATH ?= librubyparser.a

.SUFFIXES:
.PHONY: all clean

all: $(LIB_PATH) src/ffi_builder.rsinc

clean:
	rm -f librubyparser.a $(OBJECTS) src/grammars/*.cc src/grammars/*.hh

$(LIB_PATH): $(OBJECTS)
	$(AR) rcs $@ $^

%.o: %.cc include/ruby_parser/*.hh src/grammars/typedruby24.hh
	$(CXX) -o $@ $(CXXFLAGS) -c $<

%.cc: %.rl
	$(RAGEL) -o $@ -C $<

%.cc %.hh: %.ypp
	$(BISON) --defines=$*.hh -o $*.cc $*.ypp

src/ffi_builder.rsinc: include/ruby_parser/builder.hh
	script/mkbuilder $< > $@

.clang_complete: Makefile
	echo $(CXXFLAGS) > $@

docker-test:
	docker run -it --rm -v $(CURDIR):/source github/ruby_parser-test

# Do not remove generated Bison output
.PRECIOUS: %.cc %.hh
