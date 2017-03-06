module TypedRuby
  class Type::SpecialInstance < Type
    def to_type_notation
      ":instance"
    end
  end
end
