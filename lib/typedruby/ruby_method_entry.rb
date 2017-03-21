module TypedRuby
  class RubyMethodEntry
    attr_reader :klass, :id, :definitions

    def initialize(klass, id)
      @klass = klass
      @id = id
      @definitions = []
    end

    def define(method:)
      if definitions.count > 0
        # TODO - figure out an appropriate place to do this prototype check
        # last_prototype = definitions.last.prototype

        # if method.prototype && last_prototype
        #   if !last_prototype.compatible?(method.prototype)
        #     UI.warn("defining method #{klass.name}##{id} with prototype #{method.prototype.to_type_notation}; incompatible with previously defined prototype #{last_prototype.to_type_notation}", node: method.body_node)
        #   end
        # end
      end

      definitions << method
    end
  end
end
