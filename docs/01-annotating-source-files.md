# Annotating Source Files

Now that we've installed TypedRuby, it's time to annotate some source files!

Let's look over some example code:

```ruby
# @typedruby

class Greeter
  def @name : String

  def initialize(String name) => nil
    @name = name
    nil
  end

  def greet! => nil
    puts "Hello #{@name}!"
  end
end

def main => nil
  greeter = Greeter.new("Vicent")
  greeter.greet!
end

main
```

If we run TypedRuby over that code with `bundle exec typedruby example.rb`, it should exit cleanly without reporting any errors. Yay!

Let's try tweaking our `main` method:

```ruby
def main => nil
  greeter = Greeter.new(123)
  greeter.greet!
end
```

If we run TypedRuby again, we should get an error:

```
error: Could not match types:

        @ x.rb:6
      6 |    def initialize(String name) => nil
                            ^^^^^^ String, with:
        @ x.rb:17
     17 |    greeter = Greeter.new(123)
                                   ^^^ Integer
        @ x.rb:17
     17 |    greeter = Greeter.new(123)
                       ^^^^^^^^^^^^^^^^ in this expression
```

If we were to execute that code normally through Ruby, it would work just fine and print `Hello 123!` to the terminal, even though the idea of greeting a number doesn't really make any sense.

This example shows how TypedRuby can double check our assumptions at the boundaries of our code - rather than letting errors or nonsensical data slip through silently.

Let's try tweaking our code again. This time, we'd like to ask the user their name and greet them personally.

Here's a first attempt:

```ruby
def main => nil
  puts "What's your name?"
  name = gets.chomp
  greeter = Greeter.new(name)
  greeter.greet!
end
```

If we try to type check that code, we'll get an error:

```
error: Union member NilClass does not respond to #chomp

        @ x.rb:18
     18 |    name = gets.chomp
                    ^^^^ NilClass | String
```

Whoops! In our first attempt, we forgot that `gets` can return `nil` if there's no more input to read. Calling `chomp` straight away on the return value of `gets` would cause a runtime error in this case:

```
$ ruby x.rb
What's your name?
^D
x.rb:18:in `main': undefined method `chomp' for nil:NilClass (NoMethodError)
    from x.rb:23:in `<main>'
```

Let's change our code to check if `name` is nil before calling `chomp`:

```ruby
def main => nil
  puts "What's your name?"
  name = gets
  if name
    name = name.chomp
    greeter = Greeter.new(name)
    greeter.greet!
  end
end
```

Now when we run TypedRuby, it will once again exit cleanly without error. TypedRuby's flow-sensitive typing understands that inside the body of the `if`, the `name` variable must not be nil.
