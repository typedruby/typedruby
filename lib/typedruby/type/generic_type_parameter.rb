module TypedRuby
  class Type::GenericTypeParameter < Type
    attr_reader :name

    def initialize(name:)
      @name = name
    end

    def to_type_notation
      name
    end

    def ==(other)
      other.is_a?(GenericTypeParameter) && klass == other.klass && name == other.name
    end
  end
end
