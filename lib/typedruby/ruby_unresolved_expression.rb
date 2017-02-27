module TypedRuby
  class RubyUnresolvedExpression < RubyObject
    attr_reader :node

    def initialize(env:, node:)
      super(klass: env.Object) # TODO - we need a special 'any' type here
      @node = node
    end
  end
end
