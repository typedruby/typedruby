module TypedRuby
  class Error < StandardError
    attr_reader :node_backtrace

    def initialize(*)
      @node_backtrace = []
      super
    end
  end
end
