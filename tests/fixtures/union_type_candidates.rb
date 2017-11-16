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
  reveal_type(123)
  reveal_type(P.new(123))
  nil
end

def ambiguous[T, U]((T | U) x) => nil; end

def test3 => nil
  ambiguous(123)
end
