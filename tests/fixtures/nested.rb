# @typedruby
# Tickle a one-time bug in the ragel stack handling

def too_nested
  "#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{"#{nil}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"}"
end

p too_nested
