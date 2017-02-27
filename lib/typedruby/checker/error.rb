module TypedRuby::Checker
  class Error
    attr_reader :message, :node

    def initialize(message:, node:)
      @message = message
      @node = node
    end
  end
end
