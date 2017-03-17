module TypedRuby
  class Environment
    def @resolver : TypedRuby::Resolver
    def @root_scope : TypedRuby::Scope

    attr_reader :resolver
    attr_reader :root_scope

    attr_reader \
      :BasicObject,
      :Object,
      :Module,
      :Class,
      :Kernel,
      :Boolean,
      :TrueClass,
      :FalseClass,
      :NilClass,
      :Symbol,
      :String,
      :Numeric,
      :Integer,
      :Array,
      :Hash,
      :Float,
      :Regexp,
      :Exception,
      :StandardError,
      :Range,
      :Proc

    def @BasicObject : RubyClass
    def @Object : RubyClass
    def @Module : RubyClass
    def @Class : RubyClass
    def @Kernel : RubyModule
    def @Boolean : RubyClass
    def @TrueClass : RubyClass
    def @FalseClass : RubyClass
    def @NilClass : RubyClass
    def @Symbol : RubyClass
    def @String : RubyClass
    def @Numeric : RubyClass
    def @Integer : RubyClass
    def @Array : RubyClass
    def @Hash : RubyClass
    def @Float : RubyClass
    def @Regexp : RubyClass
    def @Exception : RubyClass
    def @StandardError : RubyClass
    def @Range : RubyClass
    def @Proc : RubyClass

    def initialize(Resolver resolver:) => nil
      @resolver = resolver

      bootstrap_class_system

      @root_scope = Scope.new(nil, nil, self.Object)

      nil
    end

    def bootstrap_class_system => nil
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

      @Kernel = RubyModule.new(klass: @Module, name: "Kernel")
      @Object.include_module(@Kernel)

      @Boolean = define_class("Boolean", @Object)
      @TrueClass = define_class("TrueClass", @Boolean)
      @FalseClass = define_class("FalseClass", @Boolean)
      @NilClass = define_class("NilClass", @Object)
      @Symbol = define_class("Symbol", @Object)
      @String = define_class("String", @Object)
      @Numeric = define_class("Numeric", @Object)
      @Integer = define_class("Integer", @Numeric)
      @Float = define_class("Float", @Numeric)
      @Array = define_class("Array", @Object, [:ElementType])
      @Hash = define_class("Hash", @Object, [:KeyType, :ValueType])
      @Regexp = define_class("Regexp", @Object)
      @Exception = define_class("Exception", @Object)
      @StandardError = define_class("StandardError", @Exception)
      @Range = define_class("Range", @Object, [:BeginType, :EndType])
      @Proc = define_class("Proc", @Object)

      @Class.define_method(id: :new, method:
        RubySpecialMethod.new(id: :new, klass: @Class, special_type: :class_new))

      @Proc.define_method(id: :call, method:
        RubySpecialMethod.new(id: :call, klass: @Proc, special_type: :proc_call))

      @Proc.define_method(id: :[], method:
        RubySpecialMethod.new(id: :[], klass: @Proc, special_type: :proc_call))

      nil
    end

    def define_class(String name, RubyClass superklass, [Symbol] type_parameters = []) => RubyClass
      klass = RubyClass.new(klass: @Class, name: name, superklass: superklass, type_parameters: type_parameters)
      @Object.constants[name.to_sym] = klass
      klass
    end

    def resolve_type(node:, scope:, genargs:)
      case node.type
      when :tr_cpath
        cpath, = *(node : [Node, nil])

        if cpath.type == :const
          cbase, id = *(cpath : [~Node, Symbol])

          if !cbase
            if (scope.mod.is_a?(RubyClass) && scope.mod.type_parameters.include?(id)) || genargs.include?(id)
              return Type::GenericTypeParameter.new(name: id)
            end
          end
        end

        Type::Instance.new(
          mod: resolve_cpath(node: cpath, scope: scope),
          type_parameters: [],
        )
      when :tr_geninst
        cpath, *parameters = *(node : [Node])

        Type::Instance.new(
          mod: resolve_cpath(node: cpath, scope: scope),
          type_parameters: parameters.map { |p| resolve_type(node: p, scope: scope, genargs: genargs) },
        )
      when :tr_nillable
        type_node, = *(node : [Node, nil])
        Type::Union.new(types: [
          nil_type,
          resolve_type(node: type_node, scope: scope, genargs: genargs),
        ])
      when :tr_array
        element_type_node, = *(node : [Node, nil])
        Type::Array.new(type: resolve_type(node: element_type_node, scope: scope, genargs: genargs))
      when :tr_hash
        key_type_node, value_type_node = *(node : [Node, Node])
        Type::Hash.new(
          key_type: resolve_type(node: key_type_node, scope: scope, genargs: genargs),
          value_type: resolve_type(node: value_type_node, scope: scope, genargs: genargs),
        )
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
      when :tr_proc
        prototype, = *(node : [Node, nil])
        Type::Proc.new(prototype_node: prototype, scope: scope)
      when :tr_tuple
        Type::Tuple.new(types: node.children.map { |n| resolve_type(node: n, scope: scope, genargs: genargs) })
      when :tr_or
        Type::Union.new(types: node.children.map { |n| resolve_type(node: n, scope: scope, genargs: genargs) })
      else
        raise Error, "unexpected type node: #{node.type}"
      end
    end

    def nil_type
      @nil_type ||= Type::Instance.new(mod: self.NilClass, type_parameters: [])
    end

    def resolve_cpath(node:, scope:)
      if node.type == :cbase
        return root_scope.mod
      end

      if node.type != :const
        raise Error, "not a static cpath: #{node}"
      end

      cbase, id = *(node : [Node, Symbol])

      if cbase
        mod = resolve_cpath(node: cbase, scope: scope)

        if !mod.is_a?(RubyModule)
          raise Error, "expected namespace"
        end

        if mod.has_const?(id)
          return mod.get_const(env: self, id: id, node: node)
        end

        resolver.autoload_const(mod: mod, id: id) or begin
          raise NoConstantError, "Could not resolve reference to constant #{mod.constant_path(id)}"
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

        raise NoConstantError, "Could not resolve reference to constant #{id} in lexical scope"
      end
    end
  end
end
