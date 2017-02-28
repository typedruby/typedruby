module TypedRuby::Checker
  class Error
    class MessageWithNode
      attr_reader :message, :node

      def initialize(message:, node:)
        @message = message
        @node = node
      end
    end

    attr_reader :message, :context

    def initialize(message, context = [])
      @message = message
      @context = context
    end
  end
end
