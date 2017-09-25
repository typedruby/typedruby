# @typedruby

class Foo < Array
  include Enumerable::[Integer]
end

class Bar
  include Enumerable::[Integer]
  include Enumerable::[String]
end

module MonomorphicModule
end

class Bar
  include MonomorphicModule
  include MonomorphicModule
end
