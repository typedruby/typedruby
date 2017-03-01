module TypedRuby
  class RubyClass < RubyModule
    attr_reader :type_parameters

    def initialize(klass:, name:, superklass:, type_parameters:)
      super(klass: klass, name: name)
      @superklass = superklass

      # TODO - enforce that when subclassing generic classes then type arguments are supplied.
      # something like:
      #
      #   class MyArray::[A, B] < Array::[A]; end
      #
      # this will cause issues for existing code that subclasses built-in
      # 'generic' classes like Array or Hash. let's deal with that later.
      @type_parameters = type_parameters
    end

    # overrides RubyObject#metaklass:
    def metaklass(env:)
      if @klass.is_a?(RubyMetaclass)
        @klass
      else
        super_for_metaclass = superklass

        while super_for_metaclass.is_a?(RubyIClass)
          super_for_metaclass = super_for_metaclass.superklass
        end

        @klass = RubyMetaclass.new(
          of: self,
          klass: @klass,
          name: "Class[#{name}]",
          # no need to check for nil superklass here because BasicObject's metaklass
          # is set during class system bootstrap
          superklass: super_for_metaclass.metaklass(env: env),
          type_parameters: type_parameters,
        )
      end
    end
  end
end
