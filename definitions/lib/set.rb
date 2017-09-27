class Set::[ElementType]
  include Enumerable::[ElementType]

  def initialize(Enumerable::[ElementType] enum = nil) => nil; end

  def self.[][ElementType](ElementType *elements) => Set::[ElementType]; end

  def <<(ElementType element) => :self; end

  def +(Enumerable::[ElementType] other) => :self; end

  def include?(ElementType element) => Boolean; end

  def |(Enumerable::[ElementType] other) => :self; end

  def &(Enumerable::[ElementType] other) => :self; end

  def to_a => [ElementType]; end
end

class SortedSet < Set
end

module Enumerable::[EnumType]
  def to_set => Set::[EnumType]; end
end
