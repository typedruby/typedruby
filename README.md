**Note:** TypedRuby is not currently under active development.

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

## Installing

Add `typedruby` to your `Gemfile` and you're ready to go!

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
