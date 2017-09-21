# @typedruby

class MySet::[T]
  def @elements : { T => nil }

  def initialize(Enumerable::[T] elements) => nil
    @elements = elements.map { |x| [x, nil] }.to_h
    nil
  end

  def concat(Enumerable::[T] other) => nil
    other.each do |x|
      @elements[x] = nil
    end
    nil
  end

  def elements => [T]
    @elements.keys
  end
end

def main => nil
  set = MySet.new([1, 2, 3])

  set.concat(["foo", "bar"])

  set.concat({ 4 => 5 })

  set
end
