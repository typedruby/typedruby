module Foo::[T]
  def get_t => T; end

  def get_self => :self; end
end

class Bar
  extend Foo::[Integer]
end

def main => nil
  reveal_type(Bar.get_t)
  reveal_type(Bar.get_self)
  nil
end
