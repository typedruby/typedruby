module TypedRuby
  class Type
    autoload :Any, "typedruby/type/any"
    autoload :Array, "typedruby/type/array"
    autoload :Instance, "typedruby/type/instance"
    autoload :Hash, "typedruby/type/hash"
    autoload :Proc, "typedruby/type/proc"
    autoload :SpecialClass, "typedruby/type/special_class"
    autoload :SpecialInstance, "typedruby/type/special_instance"
    autoload :SpecialSelf, "typedruby/type/special_self"
    autoload :Splat, "typedruby/type/splat"
    autoload :Tuple, "typedruby/type/tuple"
    autoload :Union, "typedruby/type/union"
    autoload :GenericTypeParameter, "typedruby/type/generic_type_parameter"

    def to_type_notation
      raise NotImplementedError
    end

    def subtype_of?(other)
      raise NotImplementedError
    end

    def supertype_of?(other)
      raise NotImplementedError
    end

    def ==(other)
      raise NotImplementedError
    end

    def compatible_with?(other)
      self == other || subtype_of?(other)
    end
  end
end
