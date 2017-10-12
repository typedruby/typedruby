class A
end

class ::B
  class C
  end
end

class ::B::C::D
end

def test1 => nil
  A.new
end

def test2 => nil
  B::C.new
end

def test3 => nil
  B::C::D.new
end
