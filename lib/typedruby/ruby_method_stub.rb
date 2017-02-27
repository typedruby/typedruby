module TypedRuby
  class RubyMethodStub
    attr_reader :klass, :prototype, :definition_node

    def initialize(klass:, prototype:, definition_node:)
      @klass = klass
      @prototype = prototype
      @definition_node = definition_node
    end

    def prototype(env:)
      @prototype
    end

    def source_location
      Location.new(node.location)
    end
  end
end
