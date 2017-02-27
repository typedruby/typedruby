module TypedRuby
  class Environment
    attr_reader :resolver
    attr_reader :root_scope

    attr_reader \
      :BasicObject,
      :Object,
      :Module,
      :Class,
      :Kernel,
      :TrueClass,
      :FalseClass,
      :NilClass,
      :Symbol,
      :String,
      :Numeric,
      :Integer,
      :Array,
      :Hash,
      :Float

    def initialize(resolver:)
      @resolver = resolver

      bootstrap_class_system

      @root_scope = Scope.new(nil, nil, self.Object)
    end

    def bootstrap_class_system
      @BasicObject = RubyClass.allocate
      @Object = RubyClass.allocate
      @Module = RubyClass.allocate
      @Class = RubyClass.allocate

      @BasicObject.send(:initialize,
        # RubyClass#metaclass assumes that #superklass is non-nil.
        # BasicObject is the only class in Ruby without a superclass, so we need
        # to initialize its metaclass manually:
        klass: RubyMetaclass.new(
          of: @BasicObject,
          klass: @Class,
          name: "Class[BasicObject]",
          superklass: @Class,
          type_parameters: [],
        ),
        name: "BasicObject",
        superklass: nil,
        type_parameters: [],
      )
      @BasicObject.constants[:BasicObject] = @BasicObject

      @Object.send(:initialize,
        klass: @Class,
        name: "Object",
        superklass: @BasicObject,
        type_parameters: [],
      )
      @Object.constants[:Object] = @Object

      @Module.send(:initialize,
        klass: @Class,
        name: "Module",
        superklass: @Object,
        type_parameters: [],
      )
      @Object.constants[:Module] = @Module

      @Class.send(:initialize,
        klass: @Class,
        name: "Class",
        superklass: @Module,
        type_parameters: [],
      )
      @Object.constants[:Class] = @Class

      @TrueClass = RubyClass.new(klass: @Class, name: "TrueClass", superklass: @Object, type_parameters: [])
      @Object.constants[:TrueClass] = @TrueClass

      @FalseClass = RubyClass.new(klass: @Class, name: "FalseClass", superklass: @Object, type_parameters: [])
      @Object.constants[:FalseClass] = @FalseClass

      @NilClass = RubyClass.new(klass: @Class, name: "NilClass", superklass: @Object, type_parameters: [])
      @Object.constants[:NilClass] = @NilClass

      @Symbol = RubyClass.new(klass: @Class, name: "Symbol", superklass: @Object, type_parameters: [])
      @Object.constants[:Symbol] = @Symbol

      @String = RubyClass.new(klass: @Class, name: "String", superklass: @Object, type_parameters: [])
      @Object.constants[:String] = @String

      @Numeric = RubyClass.new(klass: @Class, name: "Numeric", superklass: @Object, type_parameters: [])
      @Object.constants[:Numeric] = @Numeric

      @Integer = RubyClass.new(klass: @Class, name: "Integer", superklass: @Numeric, type_parameters: [])
      @Object.constants[:Integer] = @Integer

      @Array = RubyClass.new(klass: @Class, name: "Array", superklass: @Object, type_parameters: [:ElementType])
      @Object.constants[:Array] = @Array

      @Hash = RubyClass.new(klass: @Class, name: "Hash", superklass: @Object, type_parameters: [:KeyType, :ValueType])
      @Object.constants[:Hash] = @Hash

      @Float = RubyClass.new(klass: @Class, name: "Float", superklass: @Numeric, type_parameters: [])
      @Object.constants[:Float] = @Float
    end

    def resolve_type(node:, scope:)
      case node.type
      when :tr_cpath
        cpath, = *node

        if cpath.type == :const
          cbase, id = *cpath

          if cbase == nil && scope.mod.is_a?(RubyClass) && scope.mod.type_parameters.include?(id)
            return Type::GenericTypeParameter.new(klass: scope.mod, name: id)
          end
        end

        Type::Instance.new(mod: resolve_cpath(node: cpath, scope: scope))
      when :tr_nillable
        type_node, = *node
        Type::Union.new(types: [
          nil_type,
          resolve_type(node: type_node, scope: scope),
        ])
      when :tr_array
        element_type_node, = *node
        Type::Array.new(type: resolve_type(node: element_type_node, scope: scope))
      when :tr_nil
        nil_type
      when :tr_special
        case node.children[0]
        when :any
          Type::Any.new
        when :class
          Type::SpecialClass.new
        when :instance
          Type::SpecialInstance.new
        when :self
          Type::SpecialSelf.new
        else
          raise Error, "unexpected special type: #{node.children[0]}"
        end
      else
        raise Error, "unexpected type node: #{node.type}"
      end
    end

    def nil_type
      @nil_type ||= Type::Instance.new(mod: self.NilClass)
    end

    def resolve_cpath(node:, scope:)
      if node.type == :cbase
        return root_scope.mod
      end

      if node.type != :const
        raise Error, "not a static cpath: #{node}"
      end

      cbase, id = *node

      if cbase
        mod = resolve_cpath(node: cbase, scope: scope)

        if !mod.is_a?(RubyModule)
          raise Error, "expected namespace"
        end

        if mod.has_const?(id)
          return mod.get_const(env: self, id: id, node: node)
        end

        resolver.autoload_const(mod: mod, id: id) or begin
          raise Error, "no such constant #{mod.constant_path(id)}"
        end
      else
        # look up in lexical scope
        scope.each_scope do |mod|
          if mod.has_const?(id)
            return mod.get_const(env: self, id: id, node: node)
          end
        end

        scope.each_scope do |mod|
          if autoloaded_mod = resolver.autoload_const(mod: mod, id: id)
            return autoloaded_mod
          end
        end

        raise Error, "no such constant #{id}"
      end
    end
  end
end
