def array_compact => nil
  [1, nil, 3].compact
end

def array_compact_err => :any
  [1, 2, 3].compact
end

def array_to_h => nil
  [["one", 1], ["two", 2]].to_h
end

def array_to_h_err => :any
  [["one", 1], 2].to_h
end
