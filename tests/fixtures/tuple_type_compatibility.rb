# @typedruby

def f([Integer, String] x) => nil; end

def test1 => nil
  f [1]
end

def test2 => nil
  f [1, "foo"]
end

def test3 => nil
  f [1, "foo", :bar]
end

def test4 => nil
  f ["foo", :bar, 1]
end
