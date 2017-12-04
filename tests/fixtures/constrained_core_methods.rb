# @typedruby

def array_compact1 => nil
  reveal_type [1, nil, 3].compact
  nil
end

def array_compact2 => nil
  reveal_type [nil, 2, 3].compact
  nil
end

def array_compact_no_nil => nil
  reveal_type [1, 2, 3].compact
  nil
end

def array_to_h => nil
  reveal_type [["one", 1], ["two", 2]].to_h
  nil
end

def array_to_h_err => :any
  [["one", 1], 2].to_h
  nil
end
