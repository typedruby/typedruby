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
          puts "\e[31;1merror:\e[0;1m #{error.message}\e[0m"
          puts

          last_file = nil

          error.context.each do |context|
            loc = context.location

            if last_file != loc.source_buffer.name
              last_file = loc.source_buffer.name
              puts "       \e[34;4;1m#{last_file}\e[0m"
            end

            lines = File.readlines(last_file)
            line_number_padding = lines.count.to_s.size

            line = lines[loc.line - 1].chomp

            printf "       \e[33;1m%#{line_number_padding}d\e[0m ", loc.line
            print line[0, loc.column]
            print "\e[31;1m"
            print line[loc.column...loc.last_column]
            print "\e[0m"
            print line[loc.last_column..-1]
            puts
            puts "#{" " * (line_number_padding + 8)}#{" " * loc.column}\e[31;1m#{"^" * (loc.last_column - loc.column)}\e[0;1m #{context.message}\e[0m"
            puts
          end
        end
      end
    end
  end
end
