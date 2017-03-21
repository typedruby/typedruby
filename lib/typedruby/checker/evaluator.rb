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
          @lead_types = lead_types
          @splat_type = splat_type
          @post_types = post_types
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
        attr_reader :node, :name

        def initialize(node:, name:)
          @node = node
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
          "#{type.describe} #{name}:"
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
          "#{type.describe} #{name}: ..."
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

      class Splat
        attr_reader :node, :type

        def initialize(node:, type:)
          @node = node
          @type = type
        end
      end

      class BlockPass
        attr_reader :node, :type

        def initialize(node:, type:)
          @node = node
          @type = type
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

        def type_parameter?(name)
          klass = self_type.klass

          if klass.is_a?(RubyClass) && klass.type_parameters.include?(name)
            true
          elsif method_type_parameters.key?(name)
            method_type_parameters[name]
          else
            false
          end
        end

        def fetch_type_parameter(name)
          klass = self_type.klass

          if klass.is_a?(RubyClass) and index = klass.type_parameters.index(name)
            self_type.type_parameters[index]
          elsif method_type_parameters.key?(name)
            method_type_parameters[name]
          else
            raise "should not happen: couldn't find type parameter #{name}"
          end
        end
      end

      attr_reader :env, :method, :scope, :errors, :method_proc_type, :type_context

      def initialize(env:, method:)
        @env = env
        @method = method
        @scope = method.scope
        @errors = []
        @type_var_count = 0

        type_parameters =
          if method.klass.is_a?(RubyClass)
            method.klass.type_parameters.map { |param| GenericTypeParameter.new(node: nil, name: param) }
          else
            []
          end

        @type_context = TypeContext.new(
          self_type: new_instance_type(node: method.definition_node, klass: method.klass, type_parameters: type_parameters),
          method_type_parameters: {},
        )
      end

      def process_method_body
        @method_proc_type, @type_context, locals = parse_prototype(method.prototype_node, NullLocal.new,
          type_context: type_context,
          scope: method.scope,
        )

        @type_context.method_type_parameters.each do |name, type_var|
          unify!(type_var, GenericTypeParameter.new(node: type_var.node, name: name.to_s))
        end

        method_proc_type.args.each do |arg|
          if prune(arg.type).is_a?(TypeVar)
            errors << Error.new("Missing method argument type annotation", [
              Error::MessageWithLocation.new(
                message: "here",
                location: arg.node.location.expression,
              )
            ])
            unify!(arg.type, AnyType.new(node: arg.node))
          end
        end

        if prune(method_proc_type.return_type).is_a?(TypeVar)
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

      def new_instance_type(node:, klass:, type_parameters:)
        expected_type_parameters =
          if klass.is_a?(RubyClass)
            klass.type_parameters.count
          else
            0
          end

        if type_parameters.count < expected_type_parameters
          errors << Error.new("Too few type parameters supplied in instantiation of #{klass.name}", [
            Error::MessageWithLocation.new(
              message: "here",
              location: node.location.expression,
            )
          ])

          type_parameters.concat([AnyType.new(node: node)] * (expected_type_parameters - type_parameters.count))
        elsif type_parameters.count > expected_type_parameters
          errors << Error.new("Too many type parameters supplied in instantiation of #{klass.name}", [
            Error::MessageWithLocation.new(
              message: "here",
              location: node.location.expression,
            )
          ])

          type_parameters = type_parameters[0, expected_type_parameters]
        end

        InstanceType.new(node: node, klass: klass, type_parameters: type_parameters)
      end

      def new_array_type(node:, element_type:)
        new_instance_type(node: node, klass: env.Array, type_parameters: [element_type])
      end

      def new_hash_type(node:, key_type:, value_type:)
        new_instance_type(node: node, klass: env.Hash, type_parameters: [key_type, value_type])
      end

      def resolve_type(node:, scope:, type_context:)
        case node.type
        when :tr_cpath
          cpath, = *node
          cbase, id = *cpath

          if !cbase && type_context.type_parameter?(id)
            type_context.fetch_type_parameter(id)
          else
            mod = env.resolve_cpath(node: cpath, scope: scope)

            if !mod.is_a?(RubyModule)
              errors << Error.new("Constant used in type does not reference class/module", [
                Error::MessageWithLocation.new(
                  message: "here",
                  location: cpath.location.expression,
                )
              ])

              return AnyType.new(node: node)
            end

            new_instance_type(node: node, klass: mod, type_parameters: [])
          end
        when :tr_geninst
          cpath, *parameters = *node

          mod = env.resolve_cpath(node: cpath, scope: scope)

          if !mod.is_a?(RubyModule)
            errors << Error.new("Constant used in type does not reference class/module", [
              Error::MessageWithLocation.new(
                message: "here",
                location: cpath.location.expression,
              )
            ])

            return AnyType.new(node: node)
          end

          new_instance_type(node: node, klass: mod, type_parameters: parameters.map { |parameter|
            resolve_type(node: parameter, scope: scope, type_context: type_context)
          })
        when :tr_nillable
          type_node, = *node

          make_union(nil_type(node: node), resolve_type(node: type_node, scope: scope, type_context: type_context), node: node)
        when :tr_array
          element_type_node, = *node

          new_array_type(node: node,
            element_type: resolve_type(node: element_type_node, scope: scope, type_context: type_context),
          )
        when :tr_hash
          key_type_node, value_type_node = *node

          new_hash_type(node: node,
            key_type: resolve_type(node: key_type_node, scope: scope, type_context: type_context),
            value_type: resolve_type(node: value_type_node, scope: scope, type_context: type_context),
          )
        when :tr_nil
          nil_type(node: node)
        when :tr_special
          special_type, = *node

          case special_type
          when :any
            AnyType.new(node: node)
          when :class
            new_instance_type(node: node, klass: type_context.self_type.klass.metaklass(env: env), type_parameters: [])
          when :instance
            if type_context.self_type.klass.is_a?(RubyMetaclass)
              new_instance_type(node: node, klass: type_context.self_type.klass.of, type_parameters: [])
            else
              # only encountered when type checking the Class#new definition
              # in that case, rather than the receiver being a metaclass of a
              # real class (as is the case in an instantiation), it's just Class
              AnyType.new(node: node)
            end
          when :self
            type_context.self_type
          end
        when :tr_proc
          prototype_node, = *node
          proc_type, proc_type_context, _ = parse_prototype(prototype_node, NullLocal.new, scope: scope, type_context: type_context)
          proc_type
        when :tr_tuple
          TupleType.new(
            node: node,
            lead_types: node.children.map { |n|
              resolve_type(node: n, scope: scope, type_context: type_context)
            },
            splat_type: nil,
            post_types: [],
          )
        when :tr_or
          a_node, b_node = *node

          make_union(
            resolve_type(node: a_node, scope: scope, type_context: type_context),
            resolve_type(node: b_node, scope: scope, type_context: type_context),
            node: node,
          )
        else
          raise "unknown type node: #{node.type}"
        end
      end

      def new_type_from_concrete(concrete_type, node:, type_context:)
        case concrete_type
        when Type::Instance
          new_instance_type(
            node: node,
            klass: concrete_type.mod,
            type_parameters: concrete_type.type_parameters.map { |param|
              new_type_from_concrete(param, node: node, type_context: type_context)
            },
          )
        when Type::Array
          new_array_type(node: node, element_type: new_type_from_concrete(concrete_type.type, node: node, type_context: type_context))
        when Type::Hash
          new_instance_type(node: node, klass: env.Hash,
            type_parameters: [
              new_type_from_concrete(concrete_type.key_type, node: node, type_context: type_context),
              new_type_from_concrete(concrete_type.value_type, node: node, type_context: type_context),
            ])
        when Type::Tuple
          TupleType.new(
            node: node,
            lead_types: concrete_type.types.map { |t| new_type_from_concrete(t, node: node, type_context: type_context) },
            splat_type: nil,
            post_types: [],
          )
        when Type::SpecialSelf
          type_context.self_type
        when Type::SpecialClass
          case type_context.self_type
          when InstanceType
            # TODO - return a generic instance of the class rather than the class with type parameters erased:
            new_instance_type(node: node, klass: type_context.self_type.klass.metaklass(env: env), type_parameters: [])
          else
            raise "unknown self_type in Type::SpecialClass in new_type_from_concrete: #{type_context.self_type.describe}"
          end
        when Type::SpecialInstance
          case type_context.self_type
          when InstanceType
            if type_context.self_type.klass.is_a?(RubyMetaclass)
              new_instance_type(node: node, klass: type_context.self_type.klass.of, type_parameters: [])
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
          proc_type, proc_type_context, _ = parse_prototype(concrete_type.prototype_node, NullLocal.new, type_context: type_context, scope: concrete_type.scope)
          proc_type
        when Prototype
          args =
            concrete_type.lead.map { |arg| RequiredArg.new(type: new_type_from_concrete(arg.type, node: node, type_context: type_context), node: node) } +
            concrete_type.opt.map { |arg| OptionalArg.new(type: new_type_from_concrete(arg.type, node: node, type_context: type_context), node: node, expr: nil) } +
            (concrete_type.rest ? [RestArg.new(type: concrete_type.rest.type, node: node)] : []) +
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
              type: AnyType.new(node: nil),
            )
          ],
          block: AnyType.new(node: nil),
          return_type: AnyType.new(node: nil),
        )
      end

      def nil_type(node:)
        new_instance_type(node: node, klass: env.NilClass, type_parameters: [])
      end

      def unify!(t1, t2, node: nil)
        t1 = prune(t1)
        t2 = prune(t2)

        if t1.is_a?(TypeVar)
          if t1 == t2
            # already unified
            t1
          elsif occurs_in_type?(t1, t2)
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
          if t1.lead_types.count == t2.lead_types.count
            t1.lead_types.zip(t2.lead_types).each do |ty1, ty2|
              unify!(ty1, ty2, node: node)
            end
          else
            fail_unification!(t1, t2, node: node)
            return
          end

          if !!t1.splat_type ^ !!t2.splat_type
            fail_unification!(t1, t2, node: node)
            return
          end

          if t1.splat_type
            unify!(t1.splat_type, t2.splat_type, node: node)
          end

          if t1.post_types.count == t2.post_types.count
            t1.post_types.zip(t2.post_types).each do |ty1, ty2|
              unify!(ty1, ty2, node: node)
            end
          else
            fail_unification!(t1, t2, node: node)
            return
          end
        elsif t1.is_a?(TupleType)
          if t2.is_a?(InstanceType) && t2.klass == env.Array
            array_element_type = t2.type_parameters[0]

            t1.lead_types.each do |lead_type|
              unify!(lead_type, array_element_type, node: node)
            end

            if splat_type = t1.splat_type
              unify!(splat_type, array_element_type, node: node)
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
            (!!t1.splat_type == !!t2.splat_type && (!t1.splat_type || same_type?(t1.splat_type, t2.splat_type))) &&
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
        elsif t1.is_a?(ProcType) && t2.is_a?(ProcType)
          if t1.args.count == 1 && t1.args[0].is_a?(ProcArg0)
            if t2.args.count == 1 && t2.args[0].is_a?(ProcArg0)
              unless same_type?(t1.args[0].type, t2.args[0].type)
                return false
              end
            else
              # handle procarg0 expansion
              raise "nope"
            end
          elsif t2.args.count == 1 && t2.args[0].is_a?(ProcArg0)
            # handle procarg0 expansion
            raise "nope"
          elsif t1.args.count == t2.args.count
            t1.args.zip(t2.args).each do |arg1, arg2|
              unless arg1.class == arg2.class && same_type?(arg1.type, arg2.type)
                return false
              end
            end
          else
            return false
          end

          if t1.block && t2.block
            unless same_type?(t1.block, t2.block)
              return false
            end
          elsif !!t1.block ^ !!t2.block
            return false
          end

          same_type?(t1.return_type, t2.return_type)
        else
          false
        end
      end

      def compatible_type?(source:, target:)
        source = prune(source)
        target = prune(target)

        if source.is_a?(TypeVar) || target.is_a?(TypeVar)
          unify!(source, target, node: source.node || target.node)
        elsif source.is_a?(InstanceType) && target.is_a?(InstanceType)
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
        elsif source.is_a?(KeywordHashType) && target.is_a?(InstanceType) && target.klass == env.Hash
          key_type, value_type = target.type_parameters
          if key_type.is_a?(InstanceType) && key_type.klass == env.Symbol
            source.keywords.values.all? { |v|
              same_type?(v, value_type)
            }
          else
            false
          end
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

      def make_union_from_types(types, node:)
        types.reduce { |a, b| make_union(a, b, node: node) }
      end

      def truthy_type(type, node:)
        type = prune(type)

        if type.is_a?(UnionType)
          make_union_from_types(type.types.reject { |t| always_falsy?(t) }, node: node)
        elsif type.is_a?(InstanceType)
          if type.klass == env.Boolean
            new_instance_type(node: node, klass: env.TrueClass, type_parameters: [])
          elsif type.klass == env.FalseClass || type.klass == env.NilClass
            nil
          else
            type
          end
        elsif type.is_a?(AnyType)
          type
        elsif type.is_a?(TypeVar)
          type
        else
          type
        end
      end

      def falsy_type(type, node:)
        type = prune(type)

        if type.is_a?(UnionType)
          make_union_from_types(type.types.reject { |t| always_truthy?(t) }, node: node)
        elsif type.is_a?(InstanceType)
          if type.klass == env.Boolean
            new_instance_type(node: node, klass: env.FalseClass, type_parameters: [])
          elsif type.klass == env.FalseClass || type.klass == env.NilClass
            type
          elsif type.klass == env.Object || type.klass == env.BasicObject # superclasses of falsy classes
            make_union(
              new_instance_type(node: node, klass: env.FalseClass, type_parameters: []),
              new_instance_type(node: node, klass: env.NilClass, type_parameters: []),
              node: node,
            )
          else
            nil
          end
        elsif type.is_a?(AnyType)
          type
        elsif type.is_a?(TypeVar)
          type
        else
          nil
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

          if other_type.splat_type
            occurs_in_type?(type_var, other_type.splat_type)
          end

          other_type.post_types.any? { |t| occurs_in_type?(type_var, t) }
        when GenericTypeParameter
          false
        when UnionType
          other_type.types.any? { |t| occurs_in_type?(type_var, t) }
        when KeywordHashType
          other_type.keywords.any? { |n, t| occurs_in_type?(type_var, t) }
        when ProcType
          other_type.args.any? { |arg| occurs_in_type?(type_var, arg.type) } ||
            (other_type.block && occurs_in_type?(type_var, other_type.block)) ||
            occurs_in_type?(type_var, other_type.return_type)
        else
          pry binding
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
        id, expr = *node

        process_ivasgn(id: id, expr: expr, locals: locals, node: node)
      end

      def process_ivasgn(id:, expr:, locals:, node:)
        expr_type, locals = process(expr, locals)

        unless ivar = method.klass.lookup_ivar(id)
          errors << Error.new("Assignment to undeclared instance variable", [
            Error::MessageWithLocation.new(
              message: "here",
              location: node.location.expression,
            )
          ])

          return expr_type, locals
        end

        ivar_type = resolve_type(node: ivar.type_node, scope: ivar.scope, type_context: type_context)

        assert_compatible!(source: expr_type, target: ivar_type, node: node)

        truthy_ivar_type = truthy_type(ivar_type, node: node)
        falsy_ivar_type = falsy_type(ivar_type, node: node)

        case node.type
        when :or_asgn
          if falsy_ivar_type == nil
            errors << Error.new("Instance variable on left hand side of ||= is always truthy", [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.children[0].location.expression,
              )
            ])

            return ivar_type, locals
          end

          if truthy_ivar_type == nil
            errors << Error.new("Instance variable on left hand side of ||= is always falsy", [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.children[0].location.expression,
              )
            ])

            return expr_type, locals
          end

          return make_union(truthy_ivar_type, expr_type, node: node), locals
        when :and_asgn
          if truthy_ivar_type == nil
            errors << Error.new("Instance variable on left hand side of &&= is always falsy", [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.children[0].location.expression,
              )
            ])

            return ivar_type, locals
          end

          if falsy_ivar_type == nil
            errors << Error.new("Instance variable on left hand side of &&= is always truthy", [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.children[0].location.expression,
              )
            ])

            return expr_type, locals
          end

          return make_union(falsy_ivar_type, expr_type, node: node), locals
        when :ivasgn
          return expr_type, locals
        else
          raise "unknown node type #{node.type} in process_ivasgn"
        end
      end

      def process_lvasgn(id:, expr:, locals:, node:)
        expr_type, locals = process(expr, locals)

        local_type = locals[id]

        if !local_type
          local_type = nil_type(node: node)
          locals = locals.assign(name: id, type: local_type)
        end

        truthy_local_type = truthy_type(local_type, node: node)
        falsy_local_type = falsy_type(local_type, node: node)

        case node.type
        when :or_asgn
          if falsy_local_type == nil
            errors << Error.new("Local variable on left hand side of ||= is always truthy", [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.children[0].location.expression,
              )
            ])

            return local_type, locals
          end

          if truthy_local_type == nil
            errors << Error.new("Local variable on left hand side of ||= is always falsy", [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.children[0].location.expression,
              )
            ])

            return expr_type, locals.assign(name: id, type: expr_type)
          end

          local_type = make_union(truthy_local_type, expr_type, node: node)
          return local_type, locals.assign(name: id, type: local_type)
        when :and_asgn
          if truthy_local_type == nil
            errors << Error.new("Local variable on left hand side of &&= is always falsy", [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.children[0].location.expression,
              )
            ])

            return local_type, locals
          end

          if falsy_local_type == nil
            errors << Error.new("Local variable on left hand side of &&= is always truthy", [
              Error::MessageWithLocation.new(
                message: "here",
                location: node.children[0].location.expression,
              )
            ])

            return expr_type, locals.assign(name: id, type: expr_type)
          end

          local_type = make_union(falsy_ivar_type, expr_type, node: node)
          return local_type, locals.assign(name: id, type: local_type)
        when :lvasgn
          return expr_type, locals
        else
          raise "unknown node type #{node.type} in process_lvasgn"
        end
      end

      def on_masgn(node, locals)
        lhs, rhs = *node

        lhs_type, locals = process(lhs, locals)

        if rhs.type == :array
          rhs_type, locals = process_array_tuple(rhs, locals)
        else
          rhs_type, locals = process(rhs, locals)
        end

        lhs_type.lead_types.zip(rhs_type.lead_types) do |lhs, rhs|
          lhs ||= lhs_type.splat_type
          rhs ||= rhs_type.splat_type

          break unless lhs

          if rhs
            assert_compatible!(source: rhs, target: lhs, node: node)
          else
            errors << Error.new("Too many items on left-hand side of multiple assignment", [
              Error::MessageWithLocation.new(
                message: "here",
                location: lhs.node.location.expression,
              )
            ])
          end
        end

        if lhs_type.splat_type
          if rhs_type.splat_type
            assert_compatible!(source: rhs_type.splat_type, target: rhs_type.splat_type, node: node)

            lhs_type.post_types.reverse.zip(rhs_type.post_types.reverse) do |lhs, rhs|
              lhs ||= lhs_type.splat_type
              rhs ||= rhs_type.splat_type
              assert_compatible!(source: rhs, target: lhs, node: node)
            end
          else
            errors << Error.new("Useless splat, will always be empty", [
              Error::MessageWithLocation.new(
                message: "here",
                location: lhs_type.splat_type.node.location.expression,
              )
            ])
          end
        end

        [rhs_type, locals]
      end

      def on_mlhs(node, locals)
        lead_types = []
        splat_type = nil
        post_types = []

        node.children.each do |n|
          case n.type
          when :lvasgn
            name, = *n
            type = new_type_var(node: n)
            locals = locals.assign(name: name, type: type)

            if splat_type
              post_types << type
            else
              lead_types << type
            end
          when :splat
            splat_lhs, = *n
            raise "unexpected node in lhs splat: #{splat_lhs}" unless splat_lhs.type == :lvasgn
            name, = *splat_lhs
            type = new_type_var(node: n)
            locals = locals.assign(name: name, type: new_array_type(node: n, element_type: type))
            splat_type = type
          else
            raise "unexpected lhs node: #{n}"
          end
        end

        [TupleType.new(node: node, lead_types: lead_types, splat_type: splat_type, post_types: post_types), locals]
      end

      def process_array_tuple(node, locals)
        lead_types = []
        splat_type = nil
        post_types = []

        rhs_tuples = []

        node.children.each do |rhs_node|
          if rhs_node.type == :splat
            rhs_splat, = *rhs_node
            type, locals = process(rhs_splat, locals)

            if type.is_a?(TupleType)
              type.lead_types.each do |lead_type|
                rhs_tuples << TupleType.new(node: lead_type.node, lead_types: [lead_type], splat_type: nil, post_types: [])
              end

              if type.splat_type
                rhs_tuples << TupleType.new(node: type.splat_type.node, lead_types: [], splat_type: type.splat_type, post_types: [])
              end

              type.post_types.each do |post_type|
                rhs_tuples << TupleType.new(node: post_type.node, lead_types: [post_type], splat_type: nil, post_types: [])
              end
            elsif type.is_a?(InstanceType) && type.klass == env.Array
              type = type.type_parameters[0]
              rhs_tuples << TupleType.new(node: rhs_node, lead_types: [], splat_type: type, post_types: [])
            else
              # attempt to call #to_a
              errors << Error.new("Cannot splat non-array (well, you can actually. I just haven't implemented it yet...)", [
                Error::MessageWithLocation.new(
                  message: "here",
                  location: rhs_node.location.expression,
                )
              ])
            end
          else
            type, locals = process(rhs_node, locals)

            rhs_tuples << TupleType.new(node: rhs_node, lead_types: [type], splat_type: nil, post_types: [])
          end
        end

        while rhs_tuples.any? && !rhs_tuples.first.splat_type
          lead_types << rhs_tuples.shift.lead_types.first
        end

        while rhs_tuples.any? && !rhs_tuples.last.splat_type
          post_types.unshift(rhs_tuples.pop.lead_types.first)
        end

        if rhs_tuples.any?
          # first tuple remaining at this point must be a splat:
          splat_type = rhs_tuples.shift.splat_type

          rhs_tuples.each do |rhs_tuple|
            if rhs_tuple.splat_type
              unify!(splat_type, rhs_tuple.splat_type, node: node)
            else
              unify!(splat_type, rhs_tuple.lead_types.first, node: node)
            end
          end
        end

        [TupleType.new(node: node, lead_types: lead_types, splat_type: splat_type, post_types: post_types), locals]
      end

      def on_lvar(node, locals)
        name, = *node

        unless locals[name]
          raise "No such local #{name} - this should not happen"
        end

        [LocalVariableType.new(node: node, local: name, type: locals[name]), locals]
      end

      def on_send(node, locals)
        possible_method_prototypes, locals = process_send(node, locals, block_node: nil)

        return_type = make_union_from_types(possible_method_prototypes.map(&:return_type), node: node)

        [return_type, locals]
      end

      def on_block(node, locals)
        call_node, block_args, block_body = *node

        case call_node.type
        when :send
          possible_method_prototypes, locals = process_send(call_node, locals, block_node: node)

          return_type = make_union_from_types(possible_method_prototypes.map(&:return_type), node: node)
        else
          raise "unknown call node type #{call_node.type} in on_block"
        end

        [return_type, locals]
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

        prototype = prototype_from_method_entry(method_entry, self_type: method.klass, node: node)

        match_prototype_with_arguments(prototype, arg_types, node: node)

        type = new_type_var(node: node)
        unify!(type, prototype.return_type)

        [type, locals]
      end

      def parse_prototype(prototype_node, locals, type_context:, scope:)
        if prototype_node.type == :prototype
          genargs_node, args_node, return_type_node = *prototype_node

          if genargs_node
            duplicate_type_parameters = type_context.method_type_parameters.keys & genargs_node.children

            if duplicate_type_parameters.any?
              errors << Error.new("Duplicate type parameter names", [
                Error::MessageWithLocation.new(
                  message: duplicate_type_parameters.join(", "),
                  location: genargs_node.location.expression,
                )
              ])
            end

            type_context = TypeContext.new(
              self_type: type_context.self_type,
              method_type_parameters: type_context.method_type_parameters.merge(genargs_node.children.map { |name|
                [name, new_type_var(node: genargs_node)]
              }.to_h),
            )
          end

          return_type = resolve_type(node: return_type_node, scope: scope, type_context: type_context)
        else
          args_node = prototype_node
          return_type = new_type_var(node: args_node)
        end

        args_node.children.each do |arg_node|
          argument, locals = parse_argument(arg_node, locals, type_context: type_context, scope: scope)
        end

        arguments = args_node.children.map { |arg_node|
          argument, locals = parse_argument(arg_node, locals, type_context: type_context, scope: scope)
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

        [prototype, type_context, locals]
      end

      def parse_argument(typed_arg_node, locals, type_context:, scope:)
        if typed_arg_node.type == :typed_arg
          type_node, arg_node = *typed_arg_node
          type = resolve_type(node: type_node, scope: scope, type_context: type_context)
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
              arg, locals = parse_argument(n, locals, type_context: type_context, scope: scope)
              arg
            }
            unify!(type, TupleType.new(node: arg_node, lead_types: args.map(&:type), splat_type: nil, post_types: []))
            argument = ProcArg0.new(node: arg_node, type: type)
          end
        when :restarg
          arg_name, = *arg_node

          if arg_name
            locals = locals.assign(
              name: arg_name,
              type: new_array_type(
                node: type_node || arg_node,
                element_type: type))
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
        when :mlhs
          args = arg_node.children.map { |n|
            arg, locals = parse_argument(n, locals, type_context: type_context, scope: scope)
            arg
          }

          unless args.all? { |arg| arg.is_a?(RequiredArg) }
            raise "implement other argument types in mlhs"
          end

          unify!(type, TupleType.new(node: arg_node, lead_types: args.map(&:type), splat_type: nil, post_types: []))
          argument = RequiredArg.new(node: arg_node, type: type)
        else
          raise "unknown arg type: #{arg_node.type}"
        end

        [argument, locals]
      end

      def map_process(nodes, locals)
        types = nodes.map { |node|
          type, locals = process(node, locals)
          type
        }

        [types, locals]
      end

      def process_send(send_node, locals, block_node:)
        recv, mid, *args = *send_node

        if recv
          recv_type, locals = process(recv, locals)
        else
          recv_type = type_context.self_type
        end

        arg_types, locals = map_process(args, locals)

        possible_prototypes = possible_prototypes_for_invocation(recv_type: recv_type, mid: mid, send_node: send_node, recv_node: recv)

        unless possible_prototypes
          errors << Error.new("Could not resolve method ##{mid} on #{recv_type.describe}", [
            Error::MessageWithLocation.new(
              message: "in this invocation",
              location: send_node.location.selector,
            ),
          ])
          possible_prototypes = [untyped_prototype]
        end

        if block_node
          _, block_args, block_body = *block_node

          block_prototype, block_type_context, block_locals = parse_prototype(block_args, locals, type_context: type_context, scope: scope)

          assert_compatible!(
            source: block_prototype,
            target: make_union_from_types(possible_prototypes.map(&:block), node: block_node),
            node: block_node,
          )

          if block_body
            block_return_type, _ = process(block_body, block_locals)
          else
            block_return_type = nil_type(node: block_args)
          end

          assert_compatible!(source: block_return_type, target: block_prototype.return_type, node: block_node)

          arg_types << BlockPass.new(node: block_node, type: block_prototype)
        end

        possible_prototypes.each do |prototype|
          match_prototype_with_arguments(prototype, arg_types, node: send_node)
        end

        [possible_prototypes, locals]
      end

      def possible_prototypes_for_invocation(recv_type:, mid:, send_node:, recv_node:)
        recv_type = prune(recv_type)

        case recv_type
        when InstanceType
          if method_entry = recv_type.klass.lookup_method_entry(mid)
            return [prototype_from_method_entry(method_entry, self_type: recv_type, node: send_node)]
          end
        when KeywordHashType
          if method_entry = env.Hash.lookup_method_entry(mid)
            return [prototype_from_method_entry(
              method_entry,
              self_type: new_instance_type(
                node: recv_type.node,
                klass: env.Hash,
                type_parameters: [
                  new_instance_type(
                    node: recv_type.node,
                    klass: env.Symbol,
                    type_parameters: [],
                  ),
                  AnyType.new(node: recv_type.node),
                ]
              ),
              node: send_node,
            )]
          end
        when ProcType
          if method_entry = env.Proc.lookup_method_entry(mid)
            return [prototype_from_method_entry(
              method_entry,
              self_type: recv_type,
              node: send_node,
            )]
          end
        when UnionType
          recv_type.types.flat_map { |type|
            if prototypes = possible_prototypes_for_invocation(recv_type: type, mid: mid, send_node: send_node, recv_node: recv_node)
              prototypes
            else
              errors << Error.new("Union member type #{type.describe} does not respond to ##{mid}:", [
                Error::MessageWithLocation.new(
                  message: recv_type.describe,
                  location: recv_node.location.expression,
                )
              ])
              [untyped_prototype]
            end
          }
        when AnyType
          return [untyped_prototype]
        when TypeVar
          errors << Error.new("Unknown receiver type in invocation of ##{mid}", [
            Error::MessageWithLocation.new(
              message: "here",
              location: recv_node.location.expression,
            ),
          ])
          return [untyped_prototype]
        else
          errors << Error.new("Internal error: don't know how to send messages to:", [
            Error::MessageWithLocation.new(
              message: recv_type.describe,
              location: recv_node.location.expression,
            ),
          ])
          return [untyped_prototype]
        end
      end

      def prototype_from_method_entry(method_entry, self_type:, node:)
        method = method_entry.definitions.last

        type_context = TypeContext.new(
          self_type: self_type,
          method_type_parameters: {},
        )

        case method
        when RubyMethod
          prototype, type_context, locals = parse_prototype(method.prototype_node, NullLocal.new,
            type_context: type_context,
            scope: method.scope,
          )

          prototype
        when RubyAttrReader, RubyAttrWriter
          if ivar = method.klass.lookup_ivar(:"@#{method.name}")
            ivar_type = resolve_type(node: ivar.type_node, scope: ivar.scope, type_context: type_context)
          else
            ivar_type = AnyType.new(node: method.definition_node)

            errors << Error.new("Accessing undeclared instance variable", [
              Error::MessageWithLocation.new(
                message: "through this attribute",
                location: method.definition_node.location.expression,
              ),
              Error::MessageWithLocation.new(
                message: "here",
                location: node.location.expression,
              )
            ])
          end

          if method.is_a?(RubyAttrReader)
            ProcType.new(
              node: method.definition_node,
              args: [],
              block: nil,
              return_type: ivar_type,
            )
          else
            ProcType.new(
              node: method.definition_node,
              args: [RequiredArg.new(node: method.definition_node, type: ivar_type)],
              block: nil,
              return_type: ivar_type,
            )
          end
        when RubySpecialMethod
          case method.special_type
          when :class_new
            if method_entry = self_type.klass.of.lookup_method_entry(:initialize)
              new_instance_type = new_instance_type(
                node: node,
                klass: self_type.klass.of,
                type_parameters: self_type.klass.of.type_parameters.map { |t| new_type_var(node: node) },
              )

              initialize_prototype = prototype_from_method_entry(method_entry,
                self_type: new_instance_type,
                node: node,
              )

              ProcType.new(
                node: initialize_prototype.node,
                args: initialize_prototype.args,
                block: initialize_prototype.block,
                return_type: new_instance_type,
              )
            else
              raise "couldn't find #initialize method on class? what do"
            end
          when :proc_call
            if self_type.is_a?(ProcType)
              self_type
            else
              untyped_prototype
            end
          else
            raise "unknown special method type: #{method.special_type}"
          end
        else
          raise "unknown method type: #{method}"
        end
      end

      def infer_symbol_as_proc_type(symbol_node, prototype:)
        method_id, = *symbol_node

        prototype_block_type = prototype.block

        if prototype_block_type.is_a?(UnionType)
          non_nil_types = prototype_block_type.types.reject { |ty| ty.is_a?(InstanceType) && ty.klass == env.NilClass }

          if non_nil_types.count != 1
            errors << Error.new("I can't infer the type for the symbol-as-proc", [
              Error::MessageWithLocation.new(
                message: "passed here",
                location: symbol_node.location.expression,
              ),
              Error::MessageWithLocation.new(
                message: "because the block type defined in the method prototype is too complex",
                location: prototype_block_type.node.location.expression,
              ),
            ])

            return new_type_var(node: symbol_node)
          end

          prototype_block_type = non_nil_types.first
        end

        if prototype_block_type.is_a?(AnyType)
          return AnyType.new(node: symbol_node)
        end

        if !prototype_block_type.is_a?(ProcType)
          errors << Error.new("I can't infer the type for the symbol-as-proc", [
            Error::MessageWithLocation.new(
              message: "passed here",
              location: symbol_node.location.expression,
            ),
            Error::MessageWithLocation.new(
              message: "because the block type defined in the method prototype is not a proc type",
              location: prototype_block_type.node.location.expression,
            ),
          ])

          return new_type_var(node: symbol_node)
        end

        # prototype.block is checked for nil in caller
        # (match_prototype_with_arguments)
        if prototype_block_type.args.count == 0
          errors << Error.new("I can't infer the type for the symbol-as-proc", [
            Error::MessageWithLocation.new(
              message: "passed here",
              location: symbol_node.location.expression,
            ),
            Error::MessageWithLocation.new(
              message: "because the block type defined in the method prototype takes no arguments",
              location: prototype_block_type.node.location.expression,
            ),
          ])

          return new_type_var(node: symbol_node)
        end

        recv_arg, *rest_args = prototype_block_type.args

        invokee_proc_type = ProcType.new(
          node: symbol_node,
          args: rest_args,
          block: prototype_block_type.block,
          return_type: prototype_block_type.return_type,
        )

        prototypes = possible_prototypes_for_invocation(recv_type: recv_arg.type, mid: method_id, send_node: symbol_node, recv_node: recv_arg.node)

        if prototypes
          prototypes.each do |prototype|
            assert_compatible!(source: prototype, target: invokee_proc_type, node: symbol_node)
          end
        else
          errors << Error.new("Could not resolve method ##{method_id} on #{recv_arg.type.describe}", [
            Error::MessageWithLocation.new(
              message: "in this symbol-as-proc",
              location: symbol_node.location.expression,
            ),
          ])
        end

        prototype_block_type
      end

      def block_type_from_block_pass(block_pass, prototype:)
        block_type = prune(block_pass.type)

        if block_type.is_a?(ProcType)
          return block_type
        end

        if block_type.is_a?(InstanceType)
          if block_type.klass == env.Proc || block_type.klass == env.NilClass
            return block_type
          end

          if block_type.klass == env.Symbol
            passed_node, = *block_pass.node

            if passed_node.type == :sym
              return infer_symbol_as_proc_type(passed_node, prototype: prototype)
            else
              errors << Error.new("Expected symbol literal in block pass", [
                Error::MessageWithLocation.new(
                  message: "but an expression evaluating to a Symbol instance was passed instead",
                  location: passed_node.location.expression,
                )
              ])
              return new_type_var(node: passed_node)
            end
          end
        end

        # TODO - call to_proc on the type and see what we get back
        raise "TODO"
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

        if arg_types.last.is_a?(BlockPass)
          block_pass = arg_types.pop

          if prototype.block
            block_type = block_type_from_block_pass(block_pass, prototype: prototype)

            assert_compatible!(source: block_type, target: prototype.block, node: node)
          else
            errors << Error.new("Block passed to method not accepting one", [
              Error::MessageWithLocation.new(
                message: "here",
                location: block_pass.node.location.expression,
              )
            ])
          end
        elsif prototype.block
          # TODO blow up
        end

        if arg_types.count > required_argc && prototype_args.last.is_a?(KeywordHashArg)
          last_arg = prune(arg_types.last)

          if last_arg.is_a?(KeywordHashType)
            prototype_args.pop
            arg_types.pop
            # TODO - type check
          end
        end

        while arg_types.any? && prototype_args.first.is_a?(RequiredArg)
          arg_type = arg_types.shift
          assert_compatible!(source: arg_type, target: prototype_args.shift.type, node: node)
        end

        while arg_types.any? && prototype_args.last.is_a?(RequiredArg)
          arg_type = arg_types.pop
          assert_compatible!(source: arg_type, target: prototype_args.pop.type, node: node)
        end

        while arg_types.any? && prototype_args.first.is_a?(OptionalArg)
          arg_type = arg_types.shift
          assert_compatible!(source: arg_type, target: prototype_args.shift.type, node: node)
        end

        if prototype_args.first.is_a?(RestArg)
          rest_arg_type = prune(prototype_args.first.type)

          arg_types.each do |arg_type|
            assert_compatible!(source: arg_type, target: rest_arg_type, node: node)
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
                  self_type: new_instance_type(
                    node: const.node,
                    klass: const.scope.mod,
                    type_parameters: [],
                  ),
                  method_type_parameters: [],
                )
              )
            elsif const.is_a?(RubyObject)
              type = new_instance_type(node: node, klass: const.metaklass(env: env), type_parameters: [])
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

        [new_instance_type(node: node, klass: env.String, type_parameters: []), locals]
      end

      def on_str(node, locals)
        [new_instance_type(node: node, klass: env.String, type_parameters: []), locals]
      end

      def on_regexp(node, locals)
        *parts, regopt = *node

        # TODO - ensure that all parts are to_s-able
        _, locals = process_all(parts, locals)

        [new_instance_type(node: node, klass: env.Regexp, type_parameters: []), locals]
      end

      def on_ivar(node, locals)
        id, = *node

        if ivar = method.klass.lookup_ivar(id)
          [resolve_type(node: ivar.type_node, scope: ivar.scope, type_context: type_context), locals]
        else
          errors << Error.new("Reference to undeclared instance variable", [
            Error::MessageWithLocation.new(
              message: "here",
              location: node.location.expression,
            )
          ])
          [AnyType.new(node: node), locals]
        end
      end

      def on_if(node, locals)
        process_conditional(node, locals)
      end

      # this method has a very over-simplified way of doing some sort of
      # flow-sensitive typing. we probably need fancier control flow analysis
      # to make this more robust, but let's see how this goes for now.
      def process_conditional(node, locals)
        case node.type
        when :if
          cond, then_expr, else_expr = *node
        when :or
          cond, else_expr, _ = *node
        when :and
          cond, then_expr, _ = *node
        end

        cond_type, locals = process(cond, locals)

        then_locals = locals
        else_locals = locals

        truthy_type = truthy_type(cond_type, node: node)
        falsy_type = falsy_type(cond_type, node: node)
        useless_conditional_warning(truthy_type, falsy_type, node: node)

        if cond_type.is_a?(LocalVariableType)
          # TODO this is very simple specialisation of local variable types
          if truthy_type
            then_locals = locals.assign(name: cond_type.local, type: truthy_type)
          end

          if falsy_type
            else_locals = locals.assign(name: cond_type.local, type: falsy_type)
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

        case node.type
        when :if
          type = make_union(then_type, else_type, node: node)
          locals = merge_locals(then_locals, else_locals, node: node)
        when :or
          type = make_union(cond_type, else_type, node: node)
          locals = merge_locals(locals, else_locals, node: node)
        when :and
          type = make_union(cond_type, then_type, node: node)
          locals = merge_locals(locals, then_locals, node: node)
        else
          raise "unknown node type in process_conditional: #{node.type}"
        end

        [type, locals]
      end

      def on_or(node, locals)
        process_conditional(node, locals)
      end

      def on_and(node, locals)
        process_conditional(node, locals)
      end

      def process_conditional_asgn(node, locals)
        lhs, rhs = *node

        case lhs.type
        when :ivasgn
          id, = *lhs
          process_ivasgn(id: id, expr: rhs, locals: locals, node: node)
        when :lvasgn
          id, = *lhs
          process_lvasgn(id: id, expr: rhs, locals: locals, node: node)
        else
          raise "unknown lhs type #{lhs.type} in process_conditional_asgn"
        end
      end

      def on_or_asgn(node, locals)
        process_conditional_asgn(node, locals)
      end

      def on_and_asgn(node, locals)
        process_conditional_asgn(node, locals)
      end

      def on_while(node, locals)
        cond, body = *node

        # TODO - need to push something into the environment for 'break'

        cond_type, first_iteration_locals = process(cond, locals)

        truthy_cond_type = truthy_type(cond_type, node: cond)
        falsy_cond_type = falsy_type(cond_type, node: cond)

        if cond_type.is_a?(LocalVariableType) && truthy_cond_type
          first_iteration_locals = first_iteration_locals.assign(
            name: cond_type.local,
            type: truthy_cond_type,
          )
        end

        _, first_iteration_locals = process(body, first_iteration_locals)

        generalised_iteration_locals = merge_locals(locals, first_iteration_locals, node: node)

        generalised_cond_type, generalised_iteration_locals = process(cond, generalised_iteration_locals)

        truthy_generalised_cond_type = truthy_type(generalised_cond_type, node: cond)
        falsy_generalised_cond_type = falsy_type(generalised_cond_type, node: cond)

        useless_conditional_warning(truthy_generalised_cond_type, falsy_generalised_cond_type, node: node)

        if generalised_cond_type.is_a?(LocalVariableType) && truthy_generalised_cond_type
          generalised_iteration_locals = generalised_iteration_locals.assign(
            name: generalised_cond_type.local,
            type: truthy_generalised_cond_type,
          )
        end

        _, generalised_iteration_locals = process(body, generalised_iteration_locals)

        [nil_type(node: node), generalised_iteration_locals]
      end

      def useless_conditional_warning(truthy_type, falsy_type, node:)
        if !truthy_type || !falsy_type
          errors << Error.new("Condition expression in #{node.type} is always #{truthy_type ? "truthy" : "falsy"}", [
            Error::MessageWithLocation.new(
              message: "here",
              location: node.location.expression,
            )
          ])
        end
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
          return new_instance_type(node: node, klass: env.StandardError, type_parameters: [])
        end

        if classes_node.type != :array
          raise "expected klasses to be an array"
        end

        types = classes_node.children.map { |c|
          t, locals = process(c, locals)

          t = prune(t)

          if t.is_a?(InstanceType) && t.klass.is_a?(RubyMetaclass) && t.klass.of.is_a?(RubyModule)
            new_instance_type(klass: t.klass.of, type_parameters: [], node: c)
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

      def on_true(node, locals)
        [new_instance_type(node: node, klass: env.TrueClass, type_parameters: []), locals]
      end

      def on_false(node, locals)
        [new_instance_type(node: node, klass: env.FalseClass, type_parameters: []), locals]
      end

      def on_array(node, locals)
        element_type = new_type_var(node: node)

        node.children.each do |element_node|
          type, locals = process(element_node, locals)

          unify!(element_type, type)
        end

        [new_array_type(node: node, element_type: element_type), locals]
      end

      def on_self(node, locals)
        type = new_instance_type(
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

          [new_instance_type(node: node, klass: env.Hash, type_parameters: [key_type, value_type]), locals]
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
        [new_instance_type(node: node, klass: env.Integer, type_parameters: []), locals]
      end

      def on_sym(node, locals)
        [new_instance_type(node: node, klass: env.Symbol, type_parameters: []), locals]
      end

      def on_float(node, locals)
        [new_instance_type(node: node, klass: env.Float, type_parameters: []), locals]
      end

      def on_return(node, locals)
        if node.children.count > 1
          return_type, locals = process_array_tuple(node, locals)
        elsif node.children.count == 1
          return_type, locals = process(node.children.first, locals)
        else
          return_type = nil_type(node: node)
        end

        assert_compatible!(source: return_type, target: method_proc_type.return_type, node: node)

        # TODO - we need a void type
        [nil_type(node: node), locals]
      end

      def on_tr_cast(node, locals)
        expr, type_node = *node

        _, locals = process(expr, locals)

        type = resolve_type(node: type_node, scope: scope, type_context: type_context)

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

      def on_irange(node, locals)
        begin_node, end_node = *node

        begin_type, locals = process(begin_node, locals)
        end_type, locals = process(end_node, locals)

        # TODO - check (hopefully with type constraints in the Range class)
        # that the range bound types are comparable

        [new_instance_type(node: node, klass: env.Range, type_parameters: [begin_type, end_type]), locals]
      end

      def on_erange(node, locals)
        begin_node, end_node = *node

        begin_type, locals = process(begin_node, locals)
        end_type, locals = process(end_node, locals)

        # TODO - check (hopefully with type constraints in the Range class)
        # that the range bound types are comparable

        [new_instance_type(node: node, klass: env.Range, type_parameters: [begin_type, end_type]), locals]
      end

      def on_redo(node, locals)
        # TODO - need a void type
        [new_type_var(node: node), locals]
      end

      def on_retry(node, locals)
        # TODO - need a void type
        [new_type_var(node: node), locals]
      end

      def on_block_pass(node, locals)
        block, = *node

        block_type, locals = process(block, locals)

        [BlockPass.new(node: node, type: block_type), locals]
      end

      def on_defined?(node, locals)
        [make_union(nil_type(node: node), new_instance_type(klass: env.String, type_parameters: [], node: node), node: node), locals]
      end

      def on_nth_ref(node, locals)
        type = make_union(
          nil_type(node: node),
          new_instance_type(
            klass: env.String,
            type_parameters: [],
            node: node,
          ),
          node: node,
        )

        [type, locals]
      end

      def validate_static_cpath(node)
        loop do
          left, _ = *node

          if left.nil?
            return true
          elsif left.type == :cbase
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
