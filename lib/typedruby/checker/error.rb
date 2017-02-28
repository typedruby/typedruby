module TypedRuby::Checker
  class Error
    attr_reader :message, :node

    def initialize(message:, node:)
      @message = message
      @node = node
    end

    def file
      node && node.location.expression.source_buffer.name
    end

    def line
      node && node.location.expression.first_line
    end
  end
end
