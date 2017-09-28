# @typedruby
#
# Demonstrate that constants can be typed via .rbi files
class A
  STRS = ['a', 'b'].freeze

  def use_strs
    STRS
  end
end
