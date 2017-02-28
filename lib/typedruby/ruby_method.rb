module TypedRuby
  class RubyMethod
    attr_reader :klass, :definition_node, :prototype_node, :body_node, :scope

    def initialize(klass:, definition_node:, scope:)
      @klass = klass
      @definition_node = definition_node
      @scope = scope

      case definition_node.type
      when :def
        _, @prototype_node, @body_node = *definition_node
      when :defs
        _, _, @prototype_node, @body_node = *definition_node
      else
        raise "unknown definition node type: #{definition_node.type}"
      end
    end

    def prototype(env:)
      return @prototype if defined?(@prototype)

      @prototype = Prototype.from_node(env: env, scope: scope, node: prototype_node)
    end

    def source_location
      Location.new(node.location)
    end
  end
end
