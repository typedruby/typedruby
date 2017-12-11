# @typedruby

class Invariant::[T]
  def initialize(T v) => nil; end
end

class Covariant::[+T]
  def initialize(T v) => nil; end
end

class Contravariant::[-T]
  def initialize(T v) => nil; end
end

class A
end

class B < A
end

class C < B
end

def inv(Invariant::[B] x) => nil; end

def test_inv => nil
  inv(Invariant.new(A.new))
  inv(Invariant.new(B.new))
  inv(Invariant.new(C.new))
end

def cov(Covariant::[B] x) => nil; end

def test_cov => nil
  cov(Covariant.new(A.new))
  cov(Covariant.new(B.new))
  cov(Covariant.new(C.new))
end

def contra(Contravariant::[B] x) => nil; end

def test_contra => nil
  contra(Contravariant.new(A.new))
  contra(Contravariant.new(B.new))
  contra(Contravariant.new(C.new))
end
