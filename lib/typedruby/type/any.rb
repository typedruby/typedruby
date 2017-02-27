module TypedRuby
  class Type::Any < Type
    INSTANCE = new.freeze

    def to_type_notation
      "any"
    end

    def subtype_of?(other)
      true
    end

    def supertype_of?(other)
      true
    end

    def ==(other)
      other.is_a?(Type::Any)
    end
  end
end
