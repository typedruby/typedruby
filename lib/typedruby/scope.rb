module TypedRuby
  class Scope
    attr_reader :parent, :node, :mod

    def initialize(parent, node, mod)
      @parent = parent
      @node = node
      @mod = mod
    end

    def top?
      parent == nil
    end

    def each_scope
      scope = self

      while scope
        yield scope.mod
        scope = scope.parent
      end

      nil
    end
  end
end
