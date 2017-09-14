# @typedruby

def untyped(x)
  x.to_s
end

def typed(Integer x) => String
  untyped(x)
end
