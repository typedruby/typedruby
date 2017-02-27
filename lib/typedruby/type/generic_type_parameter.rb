module TypedRuby
  class Type::GenericTypeParameter
    attr_reader :klass, :name

    def initialize(klass:, name:)
      @klass = klass
      @name = name
    end

    def to_type_notation
      "#{name} of #{klass.to_type_notation}"
    end

    def ==(other)
      other.is_a?(GenericTypeParameter) && klass == other.klass && name == other.name
    end
  end
end
