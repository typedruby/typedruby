module TypedRuby
  class Location
    attr_reader :ast_location

    def initialize(ast_location)
      @ast_location = ast_location
    end

    def file
      ast_location.expression.source_buffer.name
    end

    def line
      ast_location.expression.line
    end

    def file_and_line
      "#{file}:#{line}"
    end

    def to_s
      "#<#{self.class} #{file}:#{line}>"
    end
  end
end
