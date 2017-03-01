module TypedRuby
  class RubyUnresolvedExpression
    attr_reader :scope, :node, :type

    def initialize(scope:, node:, type:)
      @scope = scope
      @node = node
      @type = type
    end
  end
end
