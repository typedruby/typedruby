module TypedRuby
  class Type::Hash < Type
    attr_reader :key_type, :value_type

    def initialize(key_type:, value_type:)
      @key_type = key_type
      @value_type = value_type
    end

    def to_type_notation
      "{ #{key_type.to_type_notation} => #{value_type.to_type_notation} }"
    end

    def ==(other)
      other.is_a?(Type::Hash) && key_type == other.key_type && value_type == other.value_type
    end
  end
end
