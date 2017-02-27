module TypedRuby
  class Type::Instance < Type
    attr_reader :mod

    def initialize(mod:)
      @mod = mod
    end

    def to_type_notation
      @mod.name
    end

    def subtype_of?(other)
      raise NotImplementedError
    end

    def supertype_of?(other)
      raise NotImplementedError
    end

    def ==(other)
      other.is_a?(Type::Instance) && mod == other.mod
    end
  end
end
