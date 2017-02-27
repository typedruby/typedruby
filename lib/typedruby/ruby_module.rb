module TypedRuby
  class RubyModule < RubyObject
    attr_reader :name, :constants, :methods, :superklass

    def initialize(RubyClass klass:, String name:) => nil
      super(klass: klass)
      @name = name
      @constants = {}
      @methods = {}
      @superklass = nil
    end

    def has_const?(id)
      if constants.key?(id)
        true
      elsif superklass
        superklass.has_const?(id)
      else
        false
      end
    end

    def autoload_const(env:, id:, file:, node:)
      if !constants.key?(id)
        autoload_entry = AutoloadEntry.new(file: file, node: node)
        env.resolver.pending_work << autoload_entry
        constants[id] = autoload_entry
      elsif constants[id].is_a?(AutoloadEntry)
        constants[id].file = file
        constants[id].node = node
      else
        # pass
      end
    end

    def get_const(env:, id:, node:, autoload: true)
      if constants.key?(id)
        if constants[id].is_a?(AutoloadEntry)
          if autoload
            autoload_entry = constants[id]
            env.resolver.require_file(file: autoload_entry.file, node: node)
            return get_const(env: env, id: id, node: node, autoload: false)
          end
        else
          return constants[id]
        end
      elsif superklass
        return superklass.get_const(env: env, id: id, node: node)
      end

      raise Error, "no such constant #{constant_path(id)}"
    end

    def set_const(id:, value:)
      if constants.key?(id) && !constants[id].is_a?(AutoloadEntry)
        raise Error, "cannot redefine constant #{constant_path(id)}"
      else
        constants[id] = value
      end
    end

    def define_module(env:, id:, node:)
      autoload_load(env: env, id: id, node: node)

      if has_const_for_definition?(env: env, id: id)
        mod = get_const_for_definition(env: env, id: id, node: node)

        if mod.is_a?(RubyModule)
          mod
        else
          raise Error, "#{constant_path(id)} is not a module!"
        end
      else
        constants[id] = RubyModule.new(
          klass: env.Module,
          name: constant_path(id),
        )
      end
    end

    def define_class(env:, id:, superklass:, node:, type_parameters:)
      autoload_load(env: env, id: id, node: node)

      if has_const_for_definition?(env: env, id: id)
        klass = get_const_for_definition(env: env, id: id, node: node)

        if klass.is_a?(RubyClass)
          if superklass && klass.superklass != superklass
            raise Error, "superclass mismatch for #{klass.name} in declaration"
          end

          if type_parameters && klass.type_parameters != type_parameters
            raise Error, "type parameter mismatch for #{klass.name} in declaration"
          end

          klass
        else
          raise Error, "#{constant_path(id)} is not a class!"
        end
      else
        constants[id] = RubyClass.new(
          klass: env.Module,
          name: constant_path(id),
          superklass: superklass || env.Object,
          type_parameters: type_parameters || [],
        )
      end
    end

    def define_method(id:, method:)
      method_entry(id).define(method: method)
    end

    def undefine_method(id:)
      method_entry(id).undefine
    end

    def alias_method(to_id:, from_id:)
      if method_entry = lookup_method_entry(from_id)
        # TODO - don't just copy the most recent definition
        if method = method_entry.definitions.last
          define_method(
            id: to_id,
            method: method,
          )
          return
        end
      end

      raise Error, "no such method #{name}##{from_id} in alias"
    end

    def lookup_method_entry(id)
      if methods.key?(id)
        methods[id]
      elsif superklass
        superklass.lookup_method_entry(id)
      else
        nil
      end
    end

    def method_entry(id)
      methods[id] ||= RubyMethodEntry.new(self, id)
    end

    def autoload_load(env:, id:, node:)
      if constants[id].is_a?(AutoloadEntry)
        autoload_entry = constants[id]
        env.resolver.require_file(file: autoload_entry.file, node: node)
      end
    end

    def has_const_for_definition?(env:, id:)
      # vm_search_const_defined_class special cases constant lookups against
      # Object when used in a class/module definition context:
      if self == env.Object
        k = self
        while k
          return true if k.constants.key?(id)
          k = k.superklass
        end
        false
      else
        constants.key?(id) && !constants[id].is_a?(AutoloadEntry)
      end
    end

    def get_const_for_definition(env:, id:, node:)
      if self == env.Object
        get_const(env: env, id: id, node: node)
      else
        constants[id]
      end
    end

    def constant_path(id)
      if name == "Object"
        id
      else
        "#{name}::#{id}"
      end
    end
  end
end
