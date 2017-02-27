module TypedRuby
  class RubyObject
    def initialize(RubyClass klass:) => nil
      @klass = klass
      nil
    end

    def name => String
      "#<#{klass.name}>"
    end

    def inspect => String
      "#<%s: %s>" % [self.class.name, name]
    end

    def klass => RubyClass
      @klass
    end

    def metaklass(Environment env:) => RubyMetaclass
      if @klass.is_a?(RubyMetaclass)
        @klass
      else
        @klass = RubyMetaclass.new(
          name: "Class[#{name}]",
          klass: env.Class,
          superklass: @klass,
          type_parameters: @klass.type_parameters,
        )
      end
    end
  end
end
