module TypedRuby
  class RubyIVar
    attr_reader :klass, :id, :type_node, :scope

    def initialize(klass:, id:, type_node:, scope:)
      @klass = klass
      @id = id
      @type_node = type_node
      @scope = scope
    end
  end
end
