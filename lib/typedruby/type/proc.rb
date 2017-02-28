module TypedRuby
  class Type
    class Proc < Type
      attr_reader :prototype_node, :scope

      def initialize(prototype_node:, scope:)
        @prototype_node = prototype_node
        @scope = scope
      end
    end
  end
end
