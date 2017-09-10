# Test that unification won't generate recursive types
def recursive => nil
  a = []
  a << a
  a << 1
  nil
end
