class Set::[ElementType]
  include Enumerable

  def initialize(Enumerable::[ElementType] enum = nil) => nil; end

  def self.[][ElementType](ElementType *elements) => Set::[ElementType]; end

  def <<(ElementType element) => :self; end

  def +((Set::[ElementType] | Array::[ElementType]) other) => :self; end

  def include?(ElementType element) => Boolean; end

  def |((Set::[ElementType] | Array::[ElementType]) other) => :self; end

  def &((Set::[ElementType] | Array::[ElementType]) other) => :self; end

  def to_a => [ElementType]; end
end

class SortedSet < Set
end

module Enumerable::[ElementType]
  def to_set => Set::[ElementType]; end
end
