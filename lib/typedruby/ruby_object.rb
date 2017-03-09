module TypedRuby
  class RubyObject
    def @klass : RubyClass

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
        (@klass : RubyMetaclass)
      else
        @klass = RubyMetaclass.new(
          of: self,
          name: "Class[#{name}]",
          klass: env.Class,
          superklass: @klass,
          type_parameters: @klass.type_parameters,
        )
      end
    end
  end
end
