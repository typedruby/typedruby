require "set"

module TypedRuby
  class Type::Union < Type
    attr_reader :types

    def initialize(types:)
      @types = types.flat_map { |t|
        if t.is_a?(Type::Union)
          t.types
        else
          [t]
        end
      }.to_set
    end

    def to_type_notation
      types.map(&:to_type_notation).join("|")
    end

    def ==(other)
      other.is_a?(Type::Union) && types == other.types
    end
  end
end
