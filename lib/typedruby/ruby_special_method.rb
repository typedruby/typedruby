module TypedRuby
  class RubySpecialMethod
    attr_reader :klass, :id, :special_type

    def initialize(klass:, id:, special_type:)
      @id = id
      @klass = klass
      @special_type = special_type
    end
  end
end
