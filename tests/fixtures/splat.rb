def splat1 => [Integer]
end

def test1 => nil
  [*splat1]
end

def splat2 => [Integer, String]
end

def test2 => nil
  [*splat2]
end

def splat3 => ([Integer] | [String])
end

def test3 => nil
  [*splat3]
end

def splat4 => ([Integer] | String)
end

def test4 => nil
  [*splat4]
end
