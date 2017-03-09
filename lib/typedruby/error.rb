module TypedRuby
  class Error < StandardError
    def @node_backtrace : [Node]

    attr_reader :node_backtrace

    def initialize(*)
      @node_backtrace = []
      super
    end
  end
end
