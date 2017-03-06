module TypedRuby
  class Type::SpecialSelf < Type
    def to_type_notation
      ":self"
    end
  end
end
