def untyped
  123
end

# @typedruby: (Integer x, String y) => ~[String]
def comment_anno
  nil
end

def typed(String x, Integer y) => ~[Integer]
  nil
end
