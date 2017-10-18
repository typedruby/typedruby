# Getting Started

TypedRuby is a gradual static type checker for Ruby. TypedRuby statically analyses your code and, with the help of type annotations, helps you find bugs sooner rather than later.

TypedRuby is designed to be gradually introduced into an existing code base, one method at a time. TypedRuby will ensure the type safety of methods that carry type annotations and will ignore any methods that don't.

Because TypedRuby operates as an external tool that checks your code without running it (similar to a linter like RuboCop), there's no overhead or performance hit at runtime.

Here's a quick example of some TypedRuby code:

```ruby
# @typedruby

def add(Integer a, Integer b) => Integer
  a + b
end

def greet(String name) => nil
  puts "Hello #{name}!"
end
```

TypedRuby extends standard Ruby syntax to allow type annotations in method definitions. There's a few other syntax extensions that TypedRuby supports - we'll cover those later.

In the `add` method in the code example above, we've declared that the `a` and `b` parameters should be be integers, and that the method also returns an integer.

Similarly, the `greet` method declares that it takes a `String` argument and returns `nil`. Writing `nil` here is exactly the same as writing `NilClass` - returning nothing from a method is common enough that TypedRuby supports the shorthand.

Also note the `# @typedruby` comment at the beginning of the file. This comment tells TypedRuby that this file is a TypedRuby source file and should be more strictly checked. This magic comment is recommended, but not required. You can still annotate methods and have TypedRuby check them without the comment.

### Installing

Installing TypedRuby is as easy as adding this line to your application's `Gemfile` and running `bundle install`:

```ruby
gem "typedruby"
```

To use the TypedRuby syntax extensions, you'll also need to install our fork of the Ruby VM, available at [`github/ruby@2.4+typedruby`](https://github.com/github/ruby/tree/2.4+typedruby).

Our Ruby fork patches the Ruby parser to accept the TypedRuby syntax extensions and ignore them. You can use this Ruby fork as your primary Ruby version - all existing Ruby code will still work just the same under it.

### Using

Invoke the `typedruby check` command and pass the paths to the source files that should be checked:

```
$ bundle exec typedruby check lib/*.rb
```

TypedRuby will load the files you pass and any other source files required by your application.

The `typedruby check` command also supports a number of command line options that allow you to manipulate the load path, enable Rails-style autoloading, and more. Pass `--help` to `typedruby check` for more details on these options.
