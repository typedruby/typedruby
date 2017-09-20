# @typedruby

class Foo < Array
  include Enumerable
end

class Bar
  include Enumerable
  include Enumerable
end

module MonomorphicModule
end

class Bar
  include MonomorphicModule
  include MonomorphicModule
end
