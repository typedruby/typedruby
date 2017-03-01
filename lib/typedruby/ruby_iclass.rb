module TypedRuby
  class RubyIClass < RubyModule
    attr_reader :mod

    def initialize(name:, superklass:, mod:)
      super(name: name, klass: nil)
      @superklass = superklass
      @mod = mod
      @constants = mod.constants
      @methods = mod.methods
    end

    def delegate
      mod
    end
  end
end
