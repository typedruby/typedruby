# @typedruby

def test1 => nil
  [1,2,3].each_with_object({}) do |num, hash|
    hash[num] = num
  end
end

def test2 => nil
  [1,2,3,4].group_by { |n| n < 3 }
end
