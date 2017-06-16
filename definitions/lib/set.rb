class Set::[ElementType]
  include Enumerable

  def initialize(Enumerable enum) => nil; end

  def self.[][ElementType](ElementType *elements) => Set::[ElementType]; end

  def <<(ElementType element) => :self; end

  def +((Set::[ElementType] | Array::[ElementType]) other) => :self; end

  def include?(ElementType element) => Boolean; end
end

# TODO to_set is actually defined on Enumerable:
class Array::[ElementType]
  def to_set => Set::[ElementType]; end
end
