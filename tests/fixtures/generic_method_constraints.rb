# @typedruby

def f[T : Numeric](T x, T y) => T
  x < x
end

def test1 => Float
  f(2.0, 3)
end

def test2 => String
  f("foo", "bar")
end

def test3 => Numeric
  f(2.0, 3.0)
end
