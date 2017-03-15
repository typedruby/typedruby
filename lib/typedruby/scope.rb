module TypedRuby
  class Scope
    attr_reader :parent, :node, :mod

    attr_accessor :method_visibility, :module_func

    alias_method :module_func?, :module_func

    def initialize(parent, node, mod)
      @parent = parent
      @node = node
      @mod = mod

      @method_visibility = :public
      @module_func = false
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
