module TypedRuby
  module Type
    class Splat
      attr_reader :type

      def initialize(type: type)
        @type = type
      end
    end
  end
end
