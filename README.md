# TypedRuby

TypedRuby is a gradual static type checker for Ruby. TypedRuby statically analyses your code and, with the help of type annotations, helps you find bugs sooner rather than later.

TypedRuby is designed to be gradually introduced into an existing code base, one method at a time. TypedRuby will ensure the type safety of methods that carry type annotations and will ignore any methods that don't.

Here's a quick example:

```ruby
def add(Integer a, Integer b) => Integer
  a + b
end

def greet(String name) => nil
  puts "Hello #{name}!"
end
```

Check out the [documentation](/docs) for an introductory guide and more details!

## Syntax

TypedRuby extends the Ruby grammar to allow argument and return type annotations in method definitions and blocks.

Type annotations for arguments come before the argument (and also before any sigils, eg. type annotations for rest arguments are written as `MyType *args`), and return type annotations come after a `=>` token after the argument list.

Here's an [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form) description of TypedRuby's type annotation syntax:

```ebnf
type                    = instance_type
                        | generic_instance_type
                        | array_type
                        | tuple_type
                        | hash_type
                        | proc_type
                        | nillable_type
                        | "nil"
                        | ":any"
                        | ":self"
                        | ":instance"
                        | ":class"
                        | "(", paren_inner, ")"
                        ;

types                   = type, { ",", type } ;

instance_type           = constant_path ;

generic_instance_type   = constant_path, "::", "[", types, "]" ;

constant_path           = (* same as Ruby *) ;

array_type              = "[", type, "]" ;

tuple_type              = "[", type, ",", types, "]" ;

hash_type               = "{", type, "=>", type, "}" ;

proc_type               = "{", "|", proc_args, "|", "=>", type, "}" ;

proc_args               = (* omitted for brevity, same as Ruby's argument syntax *) ;

nillable_type           = "~", type ;

paren_inner             = union_type
                        | type
                        ;

union_type              = type, { "|", type } ;
```

TypedRuby also extends the Ruby grammar with a few other extra syntax items:

* **Type casts**

  Valid anywhere an expression is, causes TypedRuby to ignore the inferred type for the expression and treat it as the specified type instead. Type casts should be avoided where possible, but can be a useful escape hatch.

  Grammar:

  ```ebnf
  type_cast = "(", expression, ":", type, ")" ;
  ```

  Example:

  ```ruby
  x = (123 : String)
  # x is now typed String
  ```

* **Instance variable type declarations**

  Valid in class/module bodies.

  Grammar:

  ```ebnf
  ivar_decl = "def", ivar, ":", type ;
  ```

  Example:

  ```ruby
  class Foo
    def @bar : String
  end
  ```

## Building

TypedRuby is written in Rust, and uses Ragel and Bison in its parser.

### OS X

You'll need to install Bison through Homebrew - TypedRuby requires a newer version than what OS X ships with:

```bash
brew install bison
```

You'll also need to install Ragel and Rust if you don't have them already:

```bash
brew install ragel rust
```

Then:

```bash
PATH="$(brew --prefix bison)/bin:$PATH" cargo build
```
