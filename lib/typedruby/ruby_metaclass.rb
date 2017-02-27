module TypedRuby
  class RubyMetaclass < RubyClass
    attr_reader :of

    def initialize(of:, klass:, name:, superklass:, type_parameters:)
      @of = of
      super(klass: klass, name: name, superklass: superklass, type_parameters: type_parameters)
    end
  end
end
