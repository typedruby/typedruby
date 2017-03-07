module TypedRuby
  class RubyAttrWriter
    attr_reader :klass, :name, :definition_node

    def initialize(klass:, name:, definition_node:)
      @klass = klass
      @name = name
      @definition_node = definition_node
    end

    def source_location
      Location.new(node.location)
    end
  end
end
