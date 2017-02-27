module TypedRuby
  class Type::Array < Type
    attr_reader :type

    def initialize(type:)
      @type = type
    end

    def to_type_notation
      "[#{type.to_type_notation}]"
    end

    def subtype_of?(other)
      raise NotImplementedError
    end

    def supertype_of?(other)
      raise NotImplementedError
    end

    def ==(other)
      other.is_a?(Type::Array) && type == other.type
    end
  end
end
