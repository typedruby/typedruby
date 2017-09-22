# Types

## Instances

Instance types are the most common types used in TypedRuby code. Writing the name of a class indicates that only instances of that class (or subclasses) are permitted:

```ruby
def project => String
  "TypedRuby"
end

def appeared_in => Integer
  2017
end

def pi => Float
  3.141592653
end

def nothing => nil
  nil
end

def raining? => Boolean
  true
end
```

Of note is the `Boolean` type. This is one place where TypedRuby deviates from standard Ruby.

In Ruby, `true` and `false` are instances of `TrueClass` and `FalseClass` respectively. Both `TrueClass` and `FalseClass` inherit directly from `Object`.

In TypedRuby however, `TrueClass` and `FalseClass` are both subclasses of the `Boolean` class, which is itself a subclass of `Object`.

## Generic instances

Some classes in Ruby act as containers for other objects. Think of classes like `Array` and `Hash`. For instances of these classes, it's not good enough to just know that they're arrays or hashes. We need to know what's inside of them.

TypedRuby supports generics for this reason:

```ruby
def numbers => Array::[Integer]
  [1, 2, 3]
end

def programming_languages => Hash::[String, Integer]
  {
    "Ruby" => 1995,
    "JavaScript" => 1995,
    "Rust" => 2010,
  }
end
```

Arrays and hashes are so common in TypedRuby that they have their own special syntax. The above example can also be written as:

```ruby
def numbers => [Integer]
  [1, 2, 3]
end

def programming_languages => { String => Integer }
  {
    "Ruby" => 1995,
    "JavaScript" => 1995,
    "Rust" => 2010,
  }
end
```

There's no difference between the shorthand syntax for `Array` and `Hash` and the longer version - which syntax you use is a matter of preference.

## Procs

Just like TypedRuby supports generics so that we can properly type container classes like `Array` and `Hash`, TypedRuby also has a special proc type that lets us declare the argument and return types of a block, proc, or lambda.

```ruby
def foo({ |String s| => Integer } &my_block) => Integer
  my_block.call("Hello")
end
```

TypedRuby also allows block parameters to be unnamed, if you would prefer to use `yield` instead:

```ruby
def foo({ |String s| => Integer } &) => Integer
  yield "Hello"
end
```

The argument list syntax in proc types can accept any type of argument that Ruby supports, such as rest arguments:

```ruby
def foo({ |String s, Integer *nums| => nil } &) => nil
  yield "foo", 1, 2, 3
end
```

## Unions

It's very common in Ruby code to accept a value that may be any of a set of types. For instance, a method could take either a Symbol or a String as an argument:

```ruby
def render((Symbol | String) template) => nil
  # do stuff
end
```

TypedRuby will allow callers of this method to pass any object that's compatible with either `Symbol` _or_ `String`.

TypedRuby makes sure that you only perform operations on union types that are valid on _all_ possible values, for example:

```ruby
def legal((Symbol | String) s) => (Symbol | String)
  # this is valid - both Symbol and String respond to #upcase:
  s.upcase
end

def illegal((Integer | String) x) => Integer
  # this is an error - Integer responds to #-, but String does not:
  x - 123
end
```

A very common case of union types is the case where a method might return `nil` _or_ something else. This pattern is so common that TypedRuby has a special syntax for nillable types:

```ruby
def maybe => ~Integer
  if rand < 0.5
    123
  else
    nil
  end
end
```

The `~` in front of the type means "`nil` or". The shorthand syntax `~Integer` is the exact same as writing `(nil | Integer)`.

## Tuples

Another very common pattern in Ruby is the use of arrays as tuples, for example:

```ruby
def cities_with_population => [[String, Integer]]
  [
    ["Melbourne", 4_725_000],
    ["London", 8_788_000],
    ["Berlin", 3_671_000],
  ]
end
```

This is especially common when it comes to blocks, where a single tuple value can be unpacked as multiple arguments:

```ruby
my_hash.each do |k, v|
  # ...
end
```

Tuple types in TypedRuby will very eagerly degrade to normal `Array` instances as soon as TypedRuby cannot guarantee the types of specific elements:

```ruby
def foo([Integer, Integer, String] tuple) => nil
  tuple.push(123)
  # tuple is now typed as Array::[Integer | String]
end
```
