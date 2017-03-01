module TypedRuby::Checker
  class Error
    class MessageWithLocation
      attr_reader :message, :location

      def initialize(message:, location:)
        @message = message
        @location = location
      end
    end

    attr_reader :message, :context

    def initialize(message, context = [])
      @message = message
      @context = context
    end
  end
end
