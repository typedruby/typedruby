module TypedRuby
  class Type::Instance < Type
    attr_reader :mod, :type_parameters

    def initialize(mod:, type_parameters:)
      @mod = mod
      @type_parameters = type_parameters
    end

    def to_type_notation
      @mod.name +
        if type_parameters.any?
          "::[#{type_parameters.map(&:to_type_notation).join(", ")}]"
        else
          ""
        end
    end

    def ==(other)
      other.is_a?(Type::Instance) && mod == other.mod && type_parameters == other.type_parameters
    end
  end
end
