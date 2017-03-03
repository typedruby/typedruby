module TypedRuby
  class Type::Any < Type
    INSTANCE = new.freeze

    def to_type_notation
      "any"
    end

    def ==(other)
      other.is_a?(Type::Any)
    end
  end
end
