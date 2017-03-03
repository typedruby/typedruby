module TypedRuby
  class Type::Tuple < Type
    attr_reader :types

    def initialize(types:)
      @type = type
    end

    def to_type_notation
      "[#{types.map(&:to_type_notation).join(", ")}]"
    end

    def ==(other)
      other.is_a?(Type::Tuple) && types == other.types
    end
  end
end
