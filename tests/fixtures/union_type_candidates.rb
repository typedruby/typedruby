def get_length[T]((String | Array::[T]) x) => Integer; end

def test1 => nil
  get_length("foo")
  get_length([1, 2, 3])
  nil
end

class P::[T]
  def initialize(T x) => nil; end
end

def wrap[T]((T | P::[T]) p) => P::[T]; end

def test2 => nil
  reveal_type(wrap(123))
  reveal_type(wrap(P.new(123)))
  nil
end

def test3 => nil
  reveal_type(wrap((nil : :any)))
  nil
end

def wrap2[T]((Array::[T] | P::[T]) x) => P::[T]
end

def test4 => nil
  reveal_type(wrap2([123]))
  reveal_type(wrap2(P.new("foo")))
  reveal_type(wrap2((nil : :any)))
  nil
end

def ambiguous_var[T, U]((T | U) x) => nil; end

def test5 => nil
  ambiguous_var(123)
  nil
end

module A
end

module B
end

class C
  include A
  include B
end

def ambiguous_module((A | B) x) => nil; end

def test6 => nil
  ambiguous_module(C.new)
end
