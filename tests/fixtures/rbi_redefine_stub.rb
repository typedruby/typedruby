# @typedruby
require_relative "_lib/has_stub.rb"

# Redefining a method already defined in a .rbi file is illegal.
class HasStub
  def hi
  end
end
