module TypedRuby
  module Checker
    class Typecheck
      attr_reader :method

      def initialize(method:)
        @method = method
      end

      def perform(env:)
        # only typecheck methods with prototypes:
        unless method.prototype(env: env)
          return
        end

        evaluator = Evaluator.new(env: env, method: method)

        evaluator.process_method_body

        evaluator.errors.each do |error|
          UI.error(error.message, node: error.node)
        end
      end
    end
  end
end
