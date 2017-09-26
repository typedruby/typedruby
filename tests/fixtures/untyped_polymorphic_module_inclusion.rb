# intentionally no typedruby annotation here

class Foo
  include Enumerable
end

def main => nil
  Foo.new.map { |x| x }
end
