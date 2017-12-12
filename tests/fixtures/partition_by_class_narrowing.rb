# @typedruby

module Enumerable::[EnumType]
  include Kernel
end

def test1(Enumerable::[[String, Integer]] enum) => nil
  if enum.is_a?(Hash)
    reveal_type(enum.keys.first)
  end

  nil
end
