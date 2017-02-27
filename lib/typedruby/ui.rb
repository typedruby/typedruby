module TypedRuby
  module UI
    def self.print(msg)
      $stdout.write("\r\e[K#{msg}")
    end

    def self.puts(msg = "")
      print("#{msg}\n")
      print(@last_status)
    end

    def self.status(status)
      @last_status = status
      print(status)
    end

    def self.warn(msg, node:)
      file = node.location.expression.source_buffer.name
      line = node.location.expression.line

      puts("warning: #{file}:#{line}: #{msg}")
    end

    def self.error(msg, node:)
      file = node.location.expression.source_buffer.name
      line = node.location.expression.line

      puts("error: #{file}:#{line}: #{msg}")
    end
  end
end
