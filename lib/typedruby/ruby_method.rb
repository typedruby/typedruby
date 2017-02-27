module TypedRuby
  class RubyMethod
    attr_reader :klass, :prototype_node, :body_node, :scope

    def initialize(klass:, prototype_node:, body_node:, scope:)
      @klass = klass
      @prototype_node = prototype_node
      @body_node = body_node
      @scope = scope
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
