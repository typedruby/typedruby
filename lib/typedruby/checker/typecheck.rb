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

        evaluator.errors.group_by(&:file).each do |file, errors|
          puts "\e[34;4;1m#{file}\e[0m"

          lines = File.readlines(file)

          line_number_padding = errors.map { |e| e.node.location.expression.last_line }.max.to_s.size

          errors.sort_by(&:line).each do |error|
            puts "  \e[31;1merror:\e[0;1m #{error.message}\e[0m"
            printf "    \e[33;1m%#{line_number_padding}d\e[0m %s\n", error.line, lines[error.line - 1].chomp
            puts
          end
        end
      end
    end
  end
end
