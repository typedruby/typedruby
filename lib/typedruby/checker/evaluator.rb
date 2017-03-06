module TypedRuby
  module Checker
    class Evaluator
      class NullLocal
        def ancestors
          [self]
        end

        def [](name)
          nil
        end

        def assign(name:, type:)
          Local.new(parent: self, name: name, type: type)
        end

        def lca(other)
          self
        end

        def collapse(ancestor)
          {}
        end
      end

      class Local
        attr_reader :parent, :name, :type

        def initialize(parent:, name:, type:)
          @parent = parent
          @name = name
          @type = type
        end

        def [](name)
          if self.name == name
            type
          else
            parent[name]
          end
        end

        def assign(name:, type:)
          Local.new(parent: self, name: name, type: type)
        end

        def ancestors
          parent.ancestors << self
        end

        def lca(other)
          common_ancestor = nil

          ancestors.zip(other.ancestors) do |a, b|
            if a == b
              common_ancestor = a
            else
              break
            end
          end

          common_ancestor
        end

        def collapse(ancestor)
          if self == ancestor
            {}
          else
            defs = parent.collapse(ancestor)
            defs[name] = type
            defs
          end
        end
      end

      class InstanceType
        # klass must be a concrete TypedRuby::RubyClass
        # type_parameters are Checker-level type vars or type operators
        attr_reader :node, :klass, :type_parameters

        def initialize(node:, klass:, type_parameters:)
          @node = node
          @klass = klass
          @type_parameters = type_parameters
        end

        def describe
          type_parameter_suffix =
            if type_parameters.any?
              "::[#{type_parameters.map(&:describe).join(", ")}]"
            else
              ""
            end

          "#{klass.name}#{type_parameter_suffix}"
        end
      end

      class TupleType
        attr_reader :node, :lead_types, :splat_type, :post_types

        def initialize(node:, lead_types:, splat_type:, post_types:)
          @node = node
          @lead_types = post_types
          @splat_type = splat_type
          @post_types = post_type
        end

        def describe
          types = []

          types.concat(lead_types.map(&:describe))

          if splat_type
            types << "*#{splat_type.describe}"
          end

          types.concat(post_types.map(&:describe))

          "[#{types.join(", ")}]"
        end
      end

      class UnionType
        attr_reader :node, :types

        def initialize(node:, types:)
          @node = node
          @types = types
        end

        def describe
          "#{types.map(&:describe).join(" or ")}"
        end
      end

      class AnyType
        attr_reader :node

        def initialize(node:)
          @node = node
        end

        def describe
          "any"
        end
      end

      class GenericTypeParameter
        attr_reader :name

        def initialize(name:)
          @name = name
        end

        def describe
          name
        end
      end

      class KeywordHashType
        attr_reader :node, :keywords

        def initialize(node:, keywords:)
          @node = node
          @keywords = keywords
        end

        def describe
          "{" + keywords.map { |k, t| "#{k}: #{t.describe}" }.join(", ") + "}"
        end
      end

      class RequiredArg
        attr_reader :node, :type

        def initialize(node:, type:)
          @node = node
          @type = type
        end

        def describe
          type.describe
        end
      end

      class ProcArg0
        attr_reader :node, :type

        def initialize(node:, type:)
          @node = node
          @type = type
        end

        def describe
          "#{type.describe}"
        end
      end

      class OptionalArg
        attr_reader :node, :type, :expr

        def initialize(node:, type:, expr:)
          @node = node
          @type = type
          @expr = expr
        end

        def describe
          "#{type.describe} = ..."
        end
      end

      class RestArg
        attr_reader :node, :type

        def initialize(node:, type:)
          @node = node
          @type = type
        end

        def describe
          "#{type.describe} *"
        end
      end

      class KeywordHashArg
        attr_reader :type, :keywords

        def initialize(type:, keywords:)
          @type = type
          @keywords = keywords
        end

        def describe
          "{#{keywords.map(&:describe).join(", ")}}"
        end
      end

      class KeywordArg
        attr_reader :node, :name, :type

        def initialize(node:, name:, type:)
          @node = node
          @name = name
          @type = type
        end

        def describe
          "#{type.describe} #{kw}:"
        end
      end

      class OptionalKeywordArg
        attr_reader :node, :name, :type, :expr

        def initialize(node:, name:, type:, expr:)
          @node = node
          @name = name
          @type = type
          @expr = expr
        end

        def describe
          "#{type.describe} #{kw}: ..."
        end
      end

      class BlockArg
        attr_reader :node, :type

        def initialize(node:, type:)
          @node = node
          @type = type
        end

        def describe
          "#{type.describe} &"
        end
      end

      class ProcType
        attr_reader :node, :args, :block, :return_type

        def initialize(node:, args:, block:, return_type:)
          @node = node
          @args = args
          @block = block
          @return_type = return_type
        end

        def describe
          args = self.args.map(&:describe)

          args << block.describe if block

          "(#{args.join(", ")}) => #{return_type.describe}"
        end
      end

      class TypeVar
        attr_accessor :node, :description, :instance

        def initialize(node:, description:)
          @node = node
          @description = description
          @instance = nil
        end

        def describe
          if instance
            instance.describe
          else
            description
          end
        end
      end

      class LocalVariableType
        attr_accessor :node, :local, :type

        def initialize(node:, local:, type:)
          @node = node
          @local = local
          @type = type
        end

        def describe
          type.describe
        end
      end

      class TypeContext
        attr_reader :self_type, :method_type_parameters

        def initialize(self_type:, method_type_parameters:)
          @self_type = self_type
          @method_type_parameters = method_type_parameters
        end

        def fetch_type_parameter(name)
          if index = self_type.klass.type_parameters.index(name)
            self_type.type_parameters[index]
          elsif method_type_parameters.key?(name)
            method_type_parameters[name]
          else
            raise "should not happen: couldn't find type parameter #{name}"
          end
        end
      end

      attr_reader :env, :method, :scope, :locals, :errors, :method_proc_type, :type_context

      def initialize(env:, method:)
        @env = env
        @method = method
        @scope = method.scope
        @errors = []
        @type_var_count = 0

        type_parameters =
          if method.klass.is_a?(RubyClass)
            method.klass.type_parameters.map { |name| GenericTypeParameter.new(name: name.to_s) }
          else
            []
          end

        @type_context = TypeContext.new(
          self_type: InstanceType.new(node: method.definition_node, klass: method.klass, type_parameters: type_parameters),
          method_type_parameters: method.prototype(env: env).type_parameters.map { |param_name|
            [param_name, GenericTypeParameter.new(name: param_name)]
          }.to_h,
        )
      end

      def process_method_body
        @method_proc_type, locals = parse_prototype(method.prototype_node, NullLocal.new,
          type_context: type_context,
          scope: scope,
        )

        method_proc_type.args.each do |arg|
          if arg.type.is_a?(TypeVar)
            errors << Error.new("Missing method argument type annotation", [
              Error::MessageWithLocation.new(
                message: "here",
                location: arg.node.location.expression,
              )
            ])
            unify!(arg.type, AnyType.new(node: arg.node))
          end
        end

        if method_proc_type.return_type.is_a?(TypeVar)
          errors << Error.new("Missing method return type annotation", [
            Error::MessageWithLocation.new(
              message: "expected '=> SomeType' here",
              location: method.prototype_node.location.expression,
            )
          ])
          unify!(method_proc_type.return_type, AnyType.new(node: method.prototype_node))
        end

        if method.body_node
          type, locals = process(method.body_node, locals)

          assert_compatible!(source: type, target: method_proc_type.return_type, node: nil)
        else
          # if method body is missing, just ignore any type error (stub definitions would rely on this for instance)
          # TODO - perhaps revisit this decision later?
        end
      end

      def new_type_var(node:)
        TypeVar.new(node: node, description: "t#{@type_var_count += 1}")
      end

      def new_type_from_concrete(concrete_type, node:, type_context:)
        case concrete_type
        when Type::Instance
          type_parameters = concrete_type.type_parameters.map { |param|
            new_type_from_concrete(param, node: node, type_context: type_context)
          }

          expected_type_parameters =
            if concrete_type.mod.is_a?(RubyClass)
              concrete_type.mod.type_parameters.count
            else
              0
            end

          if type_parameters.count < expected_type_parameters
            errors << Error.new("Too few type parameters supplied in instantiation of #{concrete_type.to_type_notation}", [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.location.expression,
              ),
            ])

            type_parameters.concat([AnyType.new(node: node)] * (expected_type_parameters - type_parameters.count))
          elsif type_parameters.count > expected_type_parameters
            errors << Error.new("Too many type parameters supplied in instantiation of #{concrete_type.to_type_notation}", [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.location.expression,
              ),
            ])

            type_parameters = type_parameters[0, expected_type_parameters]
          end

          InstanceType.new(node: node, klass: concrete_type.mod, type_parameters: type_parameters)
        when Type::Array
          InstanceType.new(node: node, klass: env.Array,
            type_parameters: [new_type_from_concrete(concrete_type.type, node: node, type_context: type_context)])
        when Type::Hash
          InstanceType.new(node: node, klass: env.Hash,
            type_parameters: [
              new_type_from_concrete(concrete_type.key_type, node: node, type_context: type_context),
              new_type_from_concrete(concrete_type.value_type, node: node, type_context: type_context),
            ])
        when Type::Tuple
          TupleType.new(
            node: node,
            lead_types: concrete_type.types.map { |t| new_type_from_concrete(t, node: node, type_context: type_context) },
            rest_type: nil,
            post_types: [],
          )
        when Type::SpecialSelf
          type_context.self_type
        when Type::SpecialClass
          case type_context.self_type
          when InstanceType
            # TODO - return a generic instance of the class rather than the class with type parameters erased:
            InstanceType.new(node: node, klass: type_context.self_type.klass.metaklass(env: env), type_parameters: [])
          else
            raise "unknown self_type in Type::SpecialClass in new_type_from_concrete: #{type_context.self_type.describe}"
          end
        when Type::SpecialInstance
          case type_context.self_type
          when InstanceType
            if type_context.self_type.klass.is_a?(RubyMetaclass)
              InstanceType.new(node: node, klass: type_context.self_type.klass.of, type_parameters: [])
            else
              # only encountered when type checking the Class#new definition
              # in that case, rather than the receiver being a metaclass of a
              # real class (as is the case in an instantiation), it's just Class
              AnyType.new(node: node)
            end
          else
            raise "unknown self_type in Type::SpecialInstance in new_type_from_concrete: #{type_context.self_type.describe}"
          end
        when Type::GenericTypeParameter
          type_context.fetch_type_parameter(concrete_type.name)
        when Type::Union
          concrete_type.types.map { |t|
            new_type_from_concrete(t, node: node, type_context: type_context)
          }.reduce { |a, b|
            make_union(a, b, node: node)
          }
        when Type::Any
          AnyType.new(node: node)
        when Type::Proc
          proc_type, _ = parse_prototype(concrete_type.prototype_node, NullLocal.new, type_context: type_context, scope: concrete_type.scope)
          proc_type
        when Prototype
          args =
            concrete_type.lead.map { |arg| RequiredArg.new(type: new_type_from_concrete(arg.type, node: node, type_context: type_context), node: node) } +
            concrete_type.opt.map { |arg| OptionalArg.new(type: new_type_from_concrete(arg.type, node: node, type_context: type_context), node: node, expr: nil) } +
            (concrete_type.rest ? [RestArg.new(type: new_type_from_concrete(concrete_type.rest.type, node: node, type_context: type_context), node: node)] : []) +
            concrete_type.post.map { |arg| RequiredArg.new(type: new_type_from_concrete(arg.type, node: node, type_context: type_context), node: node) }

          if concrete_type.kwreq.any? || concrete_type.kwopt.any?
            keywords = concrete_type.kwreq.map { |arg|
                KeywordArg.new(node: node, name: arg.name.to_sym, type: arg.type)
              } + concrete_type.kwopt.map { |arg|
                OptionalKeywordArg.new(node: node, name: arg.name.to_sym, type: arg.type, expr: nil)
              }

            type = KeywordHashType.new(
              node: node,
              keywords: keywords.grep(KeywordArg).map { |kw|
                [kw.name, kw.type]
              }.to_h,
            )

            args << KeywordHashArg.new(
              type: type,
              keywords: keywords,
            )
          end

          ProcType.new(node: node,
            args: args,
            block: concrete_type.block && new_type_from_concrete(concrete_type.block.type, node: node, type_context: type_context),
            return_type: new_type_from_concrete(concrete_type.return_type, node: node, type_context: type_context),
          )
        else
          raise "unknown concrete type #{concrete_type} in #{node}"
        end
      end

      def untyped_prototype
        @untyped_prototype ||= ProcType.new(
          node: nil,
          args: [
            RestArg.new(
              node: nil,
              type: InstanceType.new(node: nil, klass: env.Array, type_parameters: [AnyType.new(node: nil)]),
            )
          ],
          block: AnyType.new(node: nil),
          return_type: AnyType.new(node: nil),
        )
      end

      def nil_type(node:)
        InstanceType.new(node: node, klass: env.NilClass, type_parameters: [])
      end

      def unify!(t1, t2, node: nil)
        t1 = prune(t1)
        t2 = prune(t2)

        if t1.is_a?(TypeVar)
          if occurs_in_type?(t1, t2)
            fail_unification!(t1, t2, node: node)
          else
            t1.instance = t2
          end
        elsif t2.is_a?(TypeVar)
          unify!(t2, t1, node: node)
        elsif t1.is_a?(InstanceType) && t2.is_a?(InstanceType)
          if t1.klass == t2.klass
            t1.type_parameters.zip(t2.type_parameters) do |tp1, tp2|
              unify!(tp1, tp2, node: node)
            end
            t2
          else
            fail_unification!(t1, t2, node: node)
          end
        elsif t1.is_a?(TupleType) && t2.is_a?(TupleType)
          if t1.lead_types.count != t2.lead_types.count
            fail_unification!(t1, t2, node: node)
          end

          t1.lead_types.zip(t2.lead_types).each do |ty1, ty2|
            unify!(ty1, ty2, node: node)
          end

          if !!t1.rest_type ^ !!t2.rest_type
            fail_unification!(t1, t2, node: node)
          end

          if t1.rest_type
            unify!(t1, t2, node: node)
          end

          if t1.post_types.count != t2.post_types.count
            fail_unification!(t1, t2, node: node)
          end

          t1.post_types.zip(t2.post_types).each do |ty1, ty2|
            unify!(ty1, ty2, node: node)
          end
        elsif t1.is_a?(TupleType)
          if t2.is_a?(InstanceType) && t2.klass == env.Array
            array_element_type = t2.type_parameters[0]

            t1.lead_types.each do |lead_type|
              unify!(lead_type, array_element_type, node: node)
            end

            if rest_type = t1.rest_type
              unify!(rest_type, array_element_type, node: node)
            end

            t1.post_types.each do |post_type|
              unify!(post_type, array_element_type, node: node)
            end
          else
            fail_unification!(t1, t2, node: node)
          end
        elsif t2.is_a?(TupleType)
          unify!(t2, t1, node: node)
        elsif t1.is_a?(AnyType)
          t2
        elsif t2.is_a?(AnyType)
          t1
        elsif t1.is_a?(GenericTypeParameter) && t2.is_a?(GenericTypeParameter)
          if t1.name == t2.name
            t1
          else
            fail_unification!(t1, t2)
          end
        elsif t1.is_a?(ProcType) && t2.is_a?(ProcType)
          if t1.args.count == 1 && t1.args[0].is_a?(ProcArg0)
            if t2.args.count == 1 && t2.args[0].is_a?(ProcArg0)
              unify!(t1.args[0].type, t2.args[0].type, node: node)
            else
              # handle procarg0 expansion
              raise "nope"
            end
          elsif t2.args.count == 1 && t2.args[0].is_a?(ProcArg0)
            # handle procarg0 expansion
            raise "nope"
          elsif t1.args.count == t2.args.count
            t1.args.zip(t2.args).each do |arg1, arg2|
              if arg1.class != arg2.class
                fail_unification!(t1, t2, node: node)
              end

              unify!(arg1.type, arg2.type, node: node)
            end
          else
            fail_unification!(t1, t2, node: node)
          end

          if t1.block && t2.block
            unify!(t1.block, t2.block, node: node)
          elsif !!t1.block ^ !!t2.block
            fail_unification!(t1.block, t2.block, node: node)
          end

          unify!(t1.return_type, t2.return_type, node: node)
        else
          raise "unknown case in unify!\n#{t1.describe}\n#{t2.describe}"
          fail_unification!(t1, t2, node: node)
        end
      end

      def same_type?(t1, t2)
        t1 = prune(t1)
        t2 = prune(t2)

        if t1.is_a?(TypeVar) || t2.is_a?(TypeVar)
          unify!(t1, t2, node: t1.node || t2.node)
          true
        elsif t1.is_a?(InstanceType) && t2.is_a?(InstanceType)
          t1.klass == t2.klass &&
            same_ordered_types?(t1.type_parameters, t2.type_parameters)
        elsif t1.is_a?(TupleType) && t2.is_a?(TupleType)
          same_ordered_types?(t1.lead_types, t2.lead_types) &&
            (!!t1.rest_type == !!t2.rest_type && (!t1.rest_type || same_type?(t1.rest_type, t2.rest_type))) &&
            t1.post_types.count == t2.post_types.count &&
            same_ordered_types?(t1.post_types, t2.post_types)
        elsif t1.is_a?(AnyType) && t2.is_a?(AnyType)
          true
        elsif t1.is_a?(GenericTypeParameter) && t2.is_a?(GenericTypeParameter)
          t1.name == t2.name
        elsif t1.is_a?(KeywordHashType) && t2.is_a?(KeywordHashType)
          t1.keywords.keys.sort == t2.keywords.keys.sort &&
            t1.keywords.keys.map { |k|
              same_type?(t1.keywords[k], t2.keywords[k])
            }
        elsif t1.is_a?(UnionType) && t2.is_a?(UnionType)
          same_unordered_types?(t1.types, t2.types)
        else
          false
        end
      end

      def compatible_type?(source:, target:)
        source = prune(source)
        target = prune(target)

        if source.is_a?(InstanceType) && target.is_a?(InstanceType)
          source.klass.has_ancestor?(target.klass) &&
            (target.type_parameters.empty? ||
              same_ordered_types?(source.type_parameters, target.type_parameters))
        elsif source.is_a?(UnionType)
          source.types.all? { |source_type|
            compatible_type?(source: source_type, target: target)
          }
        elsif target.is_a?(UnionType)
          target.types.any? { |target_type|
            compatible_type?(source: source, target: target_type)
          }
        elsif source.is_a?(AnyType)
          true
        elsif target.is_a?(AnyType)
          true
        else
          same_type?(source, target)
        end
      end

      def assert_compatible!(source:, target:, node:)
        unless compatible_type?(source: source, target: target)
          add_type_error(source, target, node: node)
        end
      end

      def same_ordered_types?(types1, types2)
        return false unless types1.count == types2.count

        types1.zip(types2) do |t1, t2|
          return false unless same_type?(t1, t2)
        end

        true
      end

      def same_unordered_types?(types1, types2)
        return false unless types1.count == types2.count

        types1 = types1.dup
        types2 = types2.dup

        types1.each do |t1|
          if t2_index = types2.find_index { |t2| same_type?(t1, t2) }
            types2.delete_at(t2_index)
          else
            return false
          end
        end

        true
      end

      def always_truthy?(type)
        type = prune(type)

        if type.is_a?(InstanceType)
          return false if type.klass == env.Boolean

          type.klass != env.FalseClass && type.klass != env.NilClass
        elsif type.is_a?(UnionType)
          type.types.all? { |t| always_truthy?(t) }
        elsif type.is_a?(AnyType)
          false
        else
          true
        end
      end

      def always_falsy?(type)
        type = prune(type)

        if type.is_a?(InstanceType)
          return false if type.klass == env.Boolean

          type.klass == env.FalseClass || type.klass == env.NilClass
        elsif type.is_a?(UnionType)
          type.types.all? { |t| always_falsy?(t) }
        elsif type.is_a?(AnyType)
          false
        else
          false
        end
      end

      def make_union(t1, t2, node:)
        t1 = prune(t1)
        t2 = prune(t2)

        if t1.is_a?(UnionType) && t2.is_a?(UnionType)
          t2.types.reduce(t1) { |a, b| make_union(a, b, node: node) }
        elsif t1.is_a?(UnionType)
          if t1.types.any? { |t| compatible_type?(source: t2, target: t) }
            t1
          else
            UnionType.new(node: node, types: [*t1.types, t2])
          end
        elsif t2.is_a?(UnionType)
          make_union(t2, t1, node: node)
        elsif compatible_type?(source: t1, target: t2)
          t2
        elsif compatible_type?(source: t2, target: t1)
          t1
        else
          UnionType.new(node: node, types: [t1, t2])
        end
      end

      def fail_unification!(t1, t2, node:)
        add_type_error(t1, t2, node: node)
        t2
      end

      def add_type_error(t1, t2, node:)
        context = [
          Error::MessageWithLocation.new(message: "#{t1.describe}, with:", location: t1.node.location.expression),
          Error::MessageWithLocation.new(message: "#{t2.describe}", location: t2.node.location.expression),
        ]

        if node
          context << Error::MessageWithLocation.new(message: "in this expression", location: node.location.expression)
        end

        errors << Error.new("Could not match types:", context)
      end

      def prune(type)
        if type.is_a?(TypeVar) && type.instance
          type.instance = prune(type.instance)
        elsif type.is_a?(LocalVariableType)
          prune(type.type)
        else
          type
        end
      end

      def occurs_in_type?(type_var, other_type)
        case other_type = prune(other_type)
        when TypeVar
          type_var == other_type
        when InstanceType
          other_type.type_parameters.any? { |t| occurs_in_type?(type_var, t) }
        when AnyType
          false
        when TupleType
          other_type.lead_types.any? { |t| occurs_in_type?(type_var, t) }

          if other_type.rest_type
            occurs_in_type?(type_var, other_type.rest_type)
          end

          other_type.post_types.any? { |t| occurs_in_type?(type_var, t) }
        when GenericTypeParameter
          false
        when UnionType
          other_type.types.any? { |t| occurs_in_type?(type_var, t) }
        when KeywordHashType
          other_type.keywords.any? { |n, t| occurs_in_type?(type_var, t) }
        else
          raise "unknown type in occurs_in_type?: #{other_type}"
        end
      end

      def merge_locals(l1, l2, node:)
        ancestor = l1.lca(l2)

        l1_defs = l1.collapse(ancestor)
        l2_defs = l2.collapse(ancestor)

        defs = {}

        (l1_defs.keys | l2_defs.keys).each do |key|
          l1_type = l1_defs[key] || nil_type(node: node)
          l2_type = l2_defs[key] || nil_type(node: node)

          defs[key] = make_union(l1_type, l2_type, node: node)
        end

        defs.reduce(ancestor) { |p, (n, t)|
          p.assign(name: n, type: t)
        }
      end

      def process(node, locals)
        send("on_#{node.type}", node, locals)
      end

      def process_all(nodes, locals)
        type = nil

        nodes.each do |node|
          type, locals = process(node, locals)
        end

        [type, locals]
      end

      def on_begin(node, locals)
        if node.children.any?
          process_all(node.children, locals)
        else
          nil_type(node: node)
        end
      end

      def on_kwbegin(node, locals)
        on_begin(node, locals)
      end

      def on_lvasgn(node, locals)
        name, expr = *node

        expr_type, locals = process(expr, locals)

        local_type = LocalVariableType.new(node: node, local: name, type: expr_type)

        [local_type, locals.assign(name: name, type: expr_type)]
      end

      def on_ivasgn(node, locals)
        name, expr = *node

        expr_type, locals = process(expr, locals)

        ivar_type = method.klass.type_for_ivar(name: name, node: node)

        assert_compatible!(source: expr_type, target: ivar_type, node: node)

        [expr_type, locals]
      end

      def on_lvar(node, locals)
        name, = *node

        unless locals[name]
          raise "No such local #{name} - this should not happen"
        end

        [LocalVariableType.new(node: node, local: name, type: locals[name]), locals]
      end

      def on_splat(node)
        Type::Splat.new(type: process(node))
      end

      def on_send(node, locals)
        method_prototype, locals = process_send(node, locals)

        [method_prototype.return_type, locals]
      end

      def on_block(node, locals)
        send, block_args, block_body = *node

        method_prototype, locals = process_send(send, locals)

        block_prototype, block_locals = parse_prototype(block_args, locals, type_context: type_context, scope: scope)
        unify!(block_prototype, method_prototype.block)

        block_return_type, _ = process(block_body, block_locals)
        unify!(block_return_type, block_prototype.return_type, node: block_body)

        [method_prototype.return_type, locals]
      end

      def on_super(node, locals)
        args = node.children

        arg_types, locals = map_process(args, locals)

        unless klass = self.method.klass.superklass
          errors << Error.new("Can't invoke super where no superclass exists", [
            Error::MessageWithLocation.new(
              message: "here",
              location: node.location.keyword,
            )
          ])
          return AnyType.new(node: node), locals
        end

        unless method_entry = klass.lookup_method_entry(self.method.id)
          errors << Error.new("Could not resolve superclass implementation of ##{self.method.id}", [
            Error::MessageWithLocation.new(
              message: "here",
              location: node.location.keyword,
            )
          ])
          return AnyType.new(node: node), locals
        end

        prototype = prototype_from_method_entry(method_entry, self_type: method.klass)

        match_prototype_with_arguments(prototype, arg_types, node: node)

        type = new_type_var(node: node)
        unify!(type, prototype.return_type)

        [type, locals]
      end

      def parse_prototype(prototype_node, locals, type_context:, scope:)
        genargs = type_context.method_type_parameters.keys

        if prototype_node.type == :prototype
          genargs_node, args_node, return_type_node = *prototype_node

          if genargs_node
            genargs.concat(genargs_node.children)
          end

          concrete_return_type = env.resolve_type(node: return_type_node, scope: scope, genargs: genargs)
          return_type = new_type_from_concrete(concrete_return_type, node: return_type_node, type_context: type_context)
        else
          args_node = prototype_node
          return_type = new_type_var(node: args_node)
        end

        args_node.children.each do |arg_node|
          argument, locals = parse_argument(arg_node, locals, type_context: type_context, scope: scope, genargs: genargs)
        end

        arguments = args_node.children.map { |arg_node|
          argument, locals = parse_argument(arg_node, locals, type_context: type_context, scope: scope, genargs: genargs)
          argument
        }

        if arguments.last.is_a?(KeywordArg) || arguments.last.is_a?(OptionalKeywordArg)
          # pop individual keyword args off the arguments array and combine into KeywordHashArg
          keywords = []

          while arguments.last.is_a?(KeywordArg) || arguments.last.is_a?(OptionalKeywordArg)
            keywords << arguments.pop
          end

          arguments << KeywordHashArg.new(
            keywords: keywords,
            type: KeywordHashType.new(
              node: args_node,
              keywords: keywords.grep(KeywordArg).map { |kw|
                [kw.name, kw.type]
              }.to_h,
            ),
          )
        end

        if arguments.last.is_a?(BlockArg)
          block_type = arguments.pop.type
        end

        prototype = ProcType.new(
          node: prototype_node,
          args: arguments,
          block: block_type,
          return_type: return_type,
        )

        [prototype, locals]
      end

      def parse_argument(typed_arg_node, locals, type_context:, scope:, genargs:)
        if typed_arg_node.type == :typed_arg
          type_node, arg_node = *typed_arg_node
          concrete_type = env.resolve_type(node: type_node, scope: scope, genargs: genargs)
          type = new_type_from_concrete(concrete_type, node: type_node, type_context: type_context)
        else
          arg_node = typed_arg_node
          type = new_type_var(node: typed_arg_node)
        end

        case arg_node.type
        when :arg
          arg_name, = *arg_node
          locals = locals.assign(name: arg_name, type: type)
          argument = RequiredArg.new(node: arg_node, type: type)
        when :procarg0
          if arg_node.children.count == 1 && arg_node.children.first.is_a?(Symbol)
            arg_name = arg_node.children.first
            locals = locals.assign(name: arg_name, type: type)
            argument = ProcArg0.new(node: arg_node, type: type)
          else
            args = arg_node.children.map { |n|
              arg, locals = parse_argument(n, locals, type_context: type_context, scope: scope, genargs: genargs)
              arg
            }
            unify!(type, TupleType.new(node: arg_node, lead_types: args.map(&:type), rest_type: nil, post_types: []))
            argument = ProcArg0.new(node: arg_node, type: type)
          end
        when :restarg
          arg_name, = *arg_node

          if arg_name
            locals = locals.assign(
              name: arg_name,
              type: InstanceType.new(
                node: type_node || arg_node,
                klass: env.Array,
                type_parameters: [type]))
          end

          argument = RestArg.new(node: arg_node, type: type)
        when :blockarg
          arg_name, = *arg_node

          if arg_name
            locals = locals.assign(name: arg_name, type: type)
          end

          argument = BlockArg.new(node: arg_node, type: type)
        when :optarg
          arg_name, expr = *arg_node
          locals = locals.assign(name: arg_name, type: type)
          argument = OptionalArg.new(node: arg_node, type: type, expr: expr)
        when :kwarg
          arg_name, = *arg_node
          locals = locals.assign(name: arg_name, type: type)
          argument = KeywordArg.new(node: arg_node, name: arg_name, type: type)
        when :kwoptarg
          arg_name, expr = *arg_node
          locals = locals.assign(name: arg_name, type: type)
          argument = OptionalKeywordArg.new(node: arg_node, name: arg_name, type: type, expr: expr)
        else
          raise "unknown arg type: #{arg_node.type}"
        end

        [argument, locals]
      end

      def map_process(nodes, locals)
        types = nodes.map { |node|
          type, locals = process(node, locals)
          tvar = new_type_var(node: node)
          unify!(tvar, type)
          tvar
        }

        [types, locals]
      end

      def process_send(send_node, locals)
        recv, mid, *args = *send_node

        if recv
          recv_type, locals = process(recv, locals)
        else
          # TODO - handle case where self has generic type parameters
          recv_type = InstanceType.new(node: send_node, klass: method.klass, type_parameters: [])
        end

        arg_types, locals = map_process(args, locals)

        recv_type = prune(recv_type)

        case recv_type
        when InstanceType
          if method_entry = recv_type.klass.lookup_method_entry(mid)
            prototype = prototype_from_method_entry(method_entry, self_type: recv_type)
          end
        when KeywordHashType
          if method_entry = env.Hash.lookup_method_entry(mid)
            prototype = prototype_from_method_entry(method_entry, self_type: InstanceType.new(
              node: recv_type.node,
              klass: env.Hash,
              type_parameters: [
                InstanceType.new(
                  node: recv_type.node,
                  klass: env.Symbol,
                  type_parameters: [],
                ),
                AnyType.new(node: recv_type.node),
              ]
            ))
          end
        when AnyType
          prototype = untyped_prototype
        when TypeVar
          errors << Error.new("Unknown receiver type in invocation of ##{mid}", [
            Error::MessageWithLocation.new(
              message: "here",
              location: recv.location.expression,
            ),
          ])
          prototype = untyped_prototype
        else
          raise "unknown type #{recv_type.describe} as receiver to send"
        end

        unless prototype
          errors << Error.new("Could not resolve method ##{mid} on #{recv_type.describe}", [
            Error::MessageWithLocation.new(
              message: "in this invocation",
              location: send_node.location.selector,
            ),
          ])
          prototype = untyped_prototype
        end

        match_prototype_with_arguments(prototype, arg_types, node: send_node)

        [prototype, locals]
      end

      def prototype_from_method_entry(method_entry, self_type:)
        method = method_entry.definitions.last

        if concrete_prototype = method.prototype(env: env)
          new_type_from_concrete(concrete_prototype,
            node: method.definition_node,
            type_context: TypeContext.new(
              self_type: self_type,
              method_type_parameters: concrete_prototype.type_parameters.map { |type_parameter|
                [type_parameter, new_type_var(node: method.prototype_node)]
              }.to_h,
            ),
          )
        else
          untyped_prototype
        end
      end

      def match_prototype_with_arguments(prototype, arg_types, node:)
        arg_types = arg_types.dup
        prototype_args = prototype.args.dup

        required_argc = prototype_args.grep(RequiredArg).count

        if arg_types.count < required_argc
          errors << Error.new("Too few arguments", [
            Error::MessageWithLocation.new(
              message: "in this method invocation",
              location: node.location.expression,
            ),
          ])
        end

        if arg_types.count > required_argc && prototype_args.last.is_a?(KeywordHashArg)
          last_arg = prune(arg_types.last)

          if last_arg.is_a?(KeywordHashType)
            prototype_args.pop
            arg_types.pop
            # TODO - type check
          end
        end

        while prototype_args.first.is_a?(RequiredArg)
          arg_type = arg_types.shift
          assert_compatible!(source: arg_type, target: prototype_args.shift.type, node: nil)
        end

        while prototype_args.last.is_a?(RequiredArg)
          arg_type = arg_types.pop
          assert_compatible!(source: arg_type, target: prototype_args.pop.type, node: nil)
        end

        while arg_types.any? && prototype_args.first.is_a?(OptionalArg)
          arg_type = arg_types.shift
          assert_compatible!(source: arg_type, target: prototype_args.shift.type, node: nil)
        end

        if prototype_args.first.is_a?(RestArg)
          rest_arg_type = prune(prototype_args.first.type)

          unless rest_arg_type.is_a?(InstanceType) && rest_arg_type.klass == env.Array
            # TODO sketchy
            raise "wtf expected rest arg to be an array"
          end

          rest_arg_type = rest_arg_type.type_parameters[0]

          arg_types.each do |arg_type|
            assert_compatible!(source: arg_type, target: rest_arg_type, node: nil)
          end
        else
          if arg_types.any?
            errors << Error.new("Too many arguments", [
              Error::MessageWithLocation.new(
                message: "in this method invocation",
                location: node.location.expression,
              ),
            ])
          end
        end
      end

      def on_const(node, locals)
        if validate_static_cpath(node)
          begin
            const = env.resolve_cpath(node: node, scope: scope)

            if const.is_a?(RubyUnresolvedExpression)
              type = new_type_from_concrete(const.type, node: const.node, type_context:
                TypeContext.new(
                  self_type: InstanceType.new(
                    node: const.node,
                    klass: const.scope.mod,
                    type_parameters: [],
                  ),
                  method_type_parameters: [],
                )
              )
            elsif const.is_a?(RubyObject)
              type = InstanceType.new(node: node, klass: const.metaklass(env: env), type_parameters: [])
            end

            [type, locals]
          rescue NoConstantError => e
            errors << Error.new(e.message, [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.location.expression,
              ),
            ])

            [new_type_var(node: node), locals]
          end
        else
          errors << Error.new("Dynamic constant lookup", [
            Error::MessageWithLocation.new(
              message: "here",
              location: node.location.expression,
            ),
          ])

          [new_type_var(node: node), locals]
        end
      end

      def on_dstr(node, locals)
        node.children.each do |n|
          type, locals = process(n, locals)

          # TODO - unify type with something that says it should respond to to_s
        end

        [InstanceType.new(node: node, klass: env.String, type_parameters: []), locals]
      end

      def on_str(node, locals)
        [InstanceType.new(node: node, klass: env.String, type_parameters: []), locals]
      end

      def on_regexp(node, locals)
        *parts, regopt = *node

        # TODO - ensure that all parts are to_s-able
        _, locals = process_all(parts, locals)

        pry binding
      end

      def on_ivar(node, locals)
        name, = *node
        # TODO - if the instance variable does not yet exist, we want to defer
        # checking of the rest of this method until we process another method
        # where it does exist
        [method.klass.type_for_ivar(name: name, node: node), locals]
      end

      # this method has a very over-simplified way of doing some sort of
      # flow-sensitive typing. we probably need fancier control flow analysis
      # to make this more robust, but let's see how this goes for now.
      def on_if(node, locals)
        cond, then_expr, else_expr = *node

        cond_type, locals = process(cond, locals)

        then_locals = locals
        else_locals = locals

        if cond_type.is_a?(LocalVariableType)
          # TODO this is very simple specialisation of local variable types
          if cond_type.type.is_a?(UnionType)
            truthy_types = cond_type.type.types.reject { |t| always_falsy?(t) }
            falsy_types = cond_type.type.types.reject { |t| always_truthy?(t) }

            if truthy_types.empty?
              pry binding
            end

            if falsy_types.empty?
              pry binding
            end

            then_locals = locals.assign(
              name: cond_type.local,
              type: truthy_types.reduce { |a, b| make_union(a, b, node: node) },
            )
            else_locals = locals.assign(
              name: cond_type.local,
              type: falsy_types.reduce { |a, b| make_union(a, b, node: node) },
            )
          end
        end

        if then_expr
          then_type, then_locals = process(then_expr, then_locals)
        else
          then_type = nil_type(node: node)
        end

        if else_expr
          else_type, else_locals = process(else_expr, else_locals)
        else
          else_type = nil_type(node: node)
        end

        if then_expr && then_expr.type == :return
          return else_type, else_locals
        end

        if else_expr && else_expr.type == :return
          return then_type, then_locals
        end

        type = make_union(then_type, else_type, node: node)
        locals = merge_locals(then_locals, else_locals, node: node)

        [type, locals]
      end

      def on_rescue(node, locals)
        begin_expr, *resbodies, else_expr = *node

        begin_type, begin_locals = process(begin_expr, locals)

        # merge begin_locals with the locals we began with to ensure any local
        # defined in the begin body is nillable (if an exception was raised and
        # caught any variables defined might be left nil)
        locals = merge_locals(locals, begin_locals, node: begin_expr)

        expr_type = begin_type

        resbodies.each do |resbody|
          resbody_type, locals = on_resbody(resbody, locals)

          expr_type = make_union(expr_type, resbody_type, node: resbody)
        end

        if else_expr
          else_type, else_locals = process(else_expr, begin_locals)

          expr_type = make_union(expr_type, else_type, node: else_expr)

          locals = merge_locals(locals, else_locals)
        end

        [expr_type, locals]
      end

      def on_resbody(node, locals)
        classes_node, lvasgn, body = *node

        exception_type = type_for_rescue(node)

        if lvasgn
          local_name, = *lvasgn
          locals = locals.assign(name: local_name, type: exception_type)
        end

        process(body, locals)
      end

      def type_for_rescue(node)
        classes_node, _, _ = *node

        if !classes_node
          return InstanceType.new(node: node, klass: env.StandardError, type_parameters: [])
        end

        if classes_node.type != :array
          raise "expected klasses to be an array"
        end

        types = classes_node.children.map { |c|
          t, locals = process(c, locals)

          t = prune(t)

          if t.is_a?(InstanceType) && t.klass.is_a?(RubyMetaclass) && t.klass.of.is_a?(RubyModule)
            InstanceType.new(klass: t.klass.of, type_parameters: [], node: c)
          else
            errors << Error.new("Expected class/module in rescue clause", [
              Error::MessageWithLocation.new(
                message: "here",
                location: c.location.expression,
              )
            ])
            AnyType.new(node: node)
          end
        }

        types.reduce { |t1, t2|
          make_union(t1, t2)
        }
      end

      def on_nil(node, locals)
        [nil_type(node: node), locals]
      end

      def on_array(node, locals)
        element_type = new_type_var(node: node)

        node.children.each do |element_node|
          type, locals = process(element_node, locals)

          unify!(element_type, type)
        end

        [InstanceType.new(node: node, klass: env.Array, type_parameters: [element_type]), locals]
      end

      def on_self(node, locals)
        type = InstanceType.new(
          node: node,
          klass: type_context.self_type.klass,
          type_parameters: type_context.self_type.type_parameters,
        )

        [type, locals]
      end

      def on_hash(node, locals)
        if keyword_hash?(node)
          keywords = {}

          node.children.each do |n|
            case n.type
            when :pair
              key, value = *n
              key_sym, = *key
              value_type, locals = process(value, locals)
              keywords[key_sym] = value_type
            else
              raise "unknown node type in hash literal: #{n}"
            end
          end

          [KeywordHashType.new(node: node, keywords: keywords), locals]
        else
          key_type = new_type_var(node: node)
          value_type = new_type_var(node: node)

          node.children.each do |n|
            case n.type
            when :pair
              key, value = *n

              kt, locals = process(key, locals)
              vt, locals = process(value, locals)

              unify!(key_type, kt)
              unify!(value_type, vt)
            else
              raise "unknown node type in hash literal: #{n}"
            end
          end

          [InstanceType.new(node: node, klass: env.Hash, type_parameters: [key_type, value_type]), locals]
        end
      end

      def keyword_hash?(hash_node)
        return false if hash_node.children.empty?

        hash_node.children.all? { |pair_node|
          key, value = *pair_node
          key.type == :sym
        }
      end

      def on_int(node, locals)
        [InstanceType.new(node: node, klass: env.Integer, type_parameters: []), locals]
      end

      def on_sym(node, locals)
        [InstanceType.new(node: node, klass: env.Symbol, type_parameters: []), locals]
      end

      def on_float(node, locals)
        [InstanceType.new(node: node, klass: env.Float, type_parameters: []), locals]
      end

      def on_return(node, locals)
        expr, = *node

        expr_type, locals = process(expr, locals)

        type = new_type_var(node: node)
        unify!(type, expr_type)
        assert_compatible!(source: type, target: method_proc_type.return_type, node: expr)

        # TODO - we need a void type
        [nil_type(node: node), locals]
      end

      def on_tr_cast(node, locals)
        expr, type_node = *node

        _, locals = process(expr, locals)

        concrete_type = env.resolve_type(node: type_node, scope: scope, genargs: method.prototype(env: env).type_parameters)
        type = new_type_from_concrete(concrete_type, node: type_node, type_context: type_context)

        [type, locals]
      end

      def on_case(node, locals)
        # TODO - flow sensitive typing for case class structures

        case_expr, *whens, else_expr = *node

        case_type, locals = process(case_expr, locals)

        branch_types = []
        branch_locals = []

        whens.each do |when_branch|
          *conds, expr = *when_branch

          conds.each do |cond|
            cond_type, locals = process(cond, locals)
            # TODO - check that the condition at least responds to #===?
          end

          expr_type, expr_locals = process(expr, locals)

          branch_types << expr_type
          branch_locals << expr_locals
        end

        if else_expr
          else_type, else_locals = process(else_expr, locals)
        else
          else_type = nil_type(node: node)
        end

        branch_types << else_type

        result_type = branch_types.reduce { |a, b| make_union(a, b, node: node) }
        result_locals = branch_locals.reduce { |a, b| merge_locals(a, b, node: node) }

        [result_type, result_locals]
      end

      def validate_static_cpath(node)
        loop do
          left, _ = *node

          if left.nil?
            return true
          elsif node.type == :cbase
            return true
          elsif left.type == :const
            node = left
            next
          else
            errors << Error.new("Dynamic constant lookup", [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.location.expression,
              ),
            ])
            return false
          end
        end
      end
    end
  end
end
