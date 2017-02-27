module TypedRuby
  class Type::Tuple < Type
    attr_reader :types

    def initialize(types:)
      @type = type
    end

    def to_type_notation
      "[#{types.map(&:to_type_notation).join(", ")}]"
    end

    def subtype_of?(other)
      raise NotImplementedError
    end

    def supertype_of?(other)
      raise NotImplementedError
    end

    def ==(other)
      other.is_a?(Type::Tuple) && types == other.types
    end
  end
end
