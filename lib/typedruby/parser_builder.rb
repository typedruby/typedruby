module TypedRuby
  # subclass of the parser gem's default builder that lets us modernise
  # without setting global state:
  class ParserBuilder < ::Parser::Builders::Default
    def self.emit_procarg0
      true
    end

    def self.emit_lambda
      true
    end

    # The #string_value implementation in Parser::Builders::Default raises if
    # the string is invalid UTF-8. This behaviour diverges from stock Ruby and
    # causes issues for some gems our application uses, so we need to override
    # #string_value to allow invalid strings:
    def string_value(token)
      token[0]
    end
  end
end
