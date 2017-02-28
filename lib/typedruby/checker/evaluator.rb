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
        attr_reader :node, :types

        def initialize(node:, types:)
          @node = node
          @types = types
        end

        def describe
          "[#{type.map(&:describe).join(", ")}]"
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

      class ProcType
        attr_reader :node, :lead, :opt, :rest, :post, :kwreq, :kwopt, :block, :return_type

        def initialize(node:, lead:, opt:, rest:, post:, kwreq:, kwopt:, block:, return_type:)
          @node = node
          @lead = lead
          @opt = opt
          @rest = rest
          @post = post
          @kwreq = kwreq
          @kwopt = kwopt
          @block = block
          @return_type = return_type
        end

        def describe
          args = [
            *lead.map { |l| l.describe },
            *opt.map { |o| "opt #{o.describe}" },
            *(rest && "rest #{rest.describe}"),
            *post.map { |p| "post #{p.describe}" },
            *kwreq.map { |k, t| "kwreq #{k} #{t.describe}" },
            *kwopt.map { |k, t| "kwopt #{k} #{t.describe}" },
            *(block && "block #{block.describe}"),
          ]

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

      attr_reader :env, :method, :scope, :locals, :errors

      def initialize(env:, method:)
        @env = env
        @method = method
        @scope = method.scope
        @errors = []
        @type_var_count = 0
      end

      def process_method_body
        method_prototype = method.prototype(env: env)

        locals = method_prototype.locals.reduce(NullLocal.new) { |locals, (name, type)|
          locals.assign(
            name: name,
            type: new_type_from_concrete(type, node: method.definition_node, self_type: self_type))
        }

        type, locals = process(method.body_node, locals)

        # TODO - unify type and method return type

        unify!(type, new_type_from_concrete(method_prototype.return_type, node: method.definition_node, self_type: self_type))
      end

      def new_type_var(node:)
        TypeVar.new(node: node, description: "t#{@type_var_count += 1}")
      end

      def new_type_from_concrete(concrete_type, node:, self_type:)
        case concrete_type
        when Type::Instance
          InstanceType.new(node: node, klass: concrete_type.mod, type_parameters: [])
        when Type::Array
          InstanceType.new(node: node, klass: env.Array,
            type_parameters: [new_type_from_concrete(concrete_type.type, node: node, self_type: self_type)])
        when Type::Hash
          InstanceType.new(node: node, klass: env.Hash,
            type_parameters: [
              new_type_from_concrete(concrete_type.key_type, node: node, self_type: self_type),
              new_type_from_concrete(concrete_type.value_type, node: node, self_type: self_type),
            ])
        when Type::Tuple
          TupleType.new(node: node, types: concrete_type.types.map { |t| new_type_from_concrete(t, node: node, self_type: self_type) })
        when Type::SpecialSelf
          self_type
        when Type::SpecialClass
          case self_type
          when InstanceType
            # TODO - return a generic instance of the class rather than the class with type parameters erased:
            InstanceType.new(node: node, klass: self_type.klass.metaklass(env: env), type_parameters: [])
          else
            raise "unknown self_type in Type::SpecialClass in new_type_from_concrete: #{self_type.describe}"
          end
        when Type::SpecialInstance
          case self_type
          when InstanceType
            if self_type.klass.is_a?(RubyMetaclass)
              InstanceType.new(node: node, klass: self_type.klass.of, type_parameters: [])
            else
              # only encountered when type checking the Class#new definition
              # in that case, rather than the receiver being a metaclass of a
              # real class (as is the case in an instantiation), it's just Class
              AnyType.new(node: node)
            end
          else
            raise "unknown self_type in Type::SpecialInstance in new_type_from_concrete: #{self_type.describe}"
          end
        when Type::GenericTypeParameter
          GenericTypeParameter.new(name: concrete_type.name)
        when Type::Union
          UnionType.new(node: node, types: concrete_type.types.map { |t| new_type_from_concrete(t, node: node, self_type: self_type) })
        when Type::Any
          AnyType.new(node: node)
        when Prototype
          ProcType.new(node: node,
            lead: concrete_type.lead.map { |arg| new_type_from_concrete(arg.type, node: node, self_type: self_type) },
            opt: concrete_type.opt.map { |arg| new_type_from_concrete(arg.type, node: node, self_type: self_type) },
            rest: concrete_type.rest && new_type_from_concrete(concrete_type.rest.type, node: node, self_type: self_type),
            post: concrete_type.post.map { |arg| new_type_from_concrete(arg.type, node: node, self_type: self_type) },
            kwreq: concrete_type.kwreq.map { |arg| [k, new_type_from_concrete(arg.type, node: node, self_type: self_type)] }.to_h,
            kwopt: concrete_type.kwopt.map { |arg| [k, new_type_from_concrete(arg.type, node: node, self_type: self_type)] }.to_h,
            block: concrete_type.block && new_type_from_concrete(concrete_type.block.type, node: node, self_type: self_type),
            return_type: new_type_from_concrete(concrete_type.return_type, node: node, self_type: self_type),
          )
        else
          raise "unknown concrete type #{concrete_type} in #{node}"
        end
      end

      def untyped_prototype
        @untyped_prototype ||= ProcType.new(node: nil,
          lead: [],
          opt: [],
          rest: InstanceType.new(node: nil, klass: env.Array, type_parameters: [AnyType.new(node: nil)]),
          post: [],
          kwreq: {},
          kwopt: {},
          block: AnyType.new(node: nil),
          return_type: AnyType.new(node: nil),
        )
      end

      def nil_type(node:)
        InstanceType.new(node: node, klass: env.NilClass, type_parameters: [])
      end

      def self_type
        @self_type ||= begin
          type_parameters =
            if method.klass.is_a?(RubyClass)
              method.klass.type_parameters.map { |name| GenericTypeParameter.new(name: name.to_s) }
            else
              []
            end

          InstanceType.new(node: nil, klass: method.klass, type_parameters: type_parameters)
        end
      end

      def unify!(t1, t2)
        unless unify(t1, t2)
          errors << Error.new(
            node: t1.node || t2.node,
            message: "Could not match #{t1.describe} with #{t2.describe}",
          )
        end
      end

      def unify(t1, t2)
        t1 = prune(t1)
        t2 = prune(t2)

        if t1.is_a?(TypeVar)
          if occurs_in_type?(t1, t2)
            false
          else
            t1.instance = t2
            true
          end
        elsif t2.is_a?(TypeVar)
          unify(t2, t1)
        elsif t1.is_a?(InstanceType) && t2.is_a?(InstanceType)
          if t1.klass == t2.klass
            t1.type_parameters.zip(t2.type_parameters) do |tp1, tp2|
              return false unless unify(tp1, tp2)
            end
            true
          else
            false
          end
        elsif t1.is_a?(AnyType)
          t2
        elsif t2.is_a?(AnyType)
          t1
        elsif t1.is_a?(GenericTypeParameter) && t2.is_a?(GenericTypeParameter)
          t1.name == t2.name
        else
          false
        end
      end

      def unify_or_make_union(t1, t2, node:)
        unified = new_type_var(node: node)

        if unify(unified, t1) && unify(unified, t2)
          unified
        else
          UnionType.new(node: node, types: [t1, t2])
        end
      end

      def prune(type)
        if type.is_a?(TypeVar) && type.instance
          type.instance = prune(type.instance)
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

          defs[key] = unify_or_make_union(l1_type, l2_tye)
        end

        defs.reduce(ancestor) { |p, (n, t)|
          p.assign(name: n, type: t)
        }
      end

      def process(node, locals)
        if node
          send("on_#{node.type}", node, locals)
        else
          [nil_type(node: node), locals]
        end
      end

      def process_all(nodes, locals)
        type = nil_type(node: nil)

        nodes.each do |node|
          type, locals = process(node, locals)
        end

        [type, locals]
      end

      def on_begin(node, locals)
        process_all(node.children, locals)
      end

      def on_lvasgn(node, locals)
        name, expr = *node

        expr_type, locals = process(expr, locals)

        [expr_type, locals.assign(name: name, type: expr_type)]
      end

      def on_ivasgn(node, locals)
        # TODO - we need to perform some sort of class-wide type inference of instance variable types
        # for now we'll just type them as any
        name, expr = *node

        process(expr, locals)
      end

      def on_lvar(node, locals)
        name, = *node

        type = new_type_var(node: node)

        unless locals[name]
          raise "No such local #{name} - this should not happen"
        end

        unify!(type, locals[name])

        [type, locals]
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

        block_locals = block_args.children.reduce(locals) { |l, arg_node|
          # TODO - we need proper prototype parsing in here

          case arg_node.type
          when :arg
            arg_name, = *arg_node

            l.assign(name: arg_name, type: new_type_var(node: arg_node))
          when :procarg0
            # to handle procarg0 correctly we need to look at the block
            # argument in the method prototype... TODO
            arg_name, = *arg_node

            l.assign(name: arg_name, type: AnyType.new(node: arg_node))
          end
        }

        # TODO - match block arguments against method

        block_return_type, _ = process(block_body, block_locals)

        [method_prototype.return_type, locals]
      end

      def process_send(send_node, locals)
        recv, mid, *args = *send_node

        if recv
          recv_type, locals = process(recv, locals)
        else
          # TODO - handle case where self has generic type parameters
          recv_type = InstanceType.new(node: send_node, klass: method.klass, type_parameters: [])
        end

        arg_types = args.map { |arg|
          t, locals = process(arg, locals)
          t
        }

        recv_type = prune(recv_type)

        case recv_type
        when InstanceType
          if method_entry = recv_type.klass.lookup_method_entry(mid)
            method = method_entry.definitions.last
            prototype = new_type_from_concrete(method.prototype(env: env), node: method.definition_node, self_type: recv_type)
          else
            errors << Error.new(
              message: "Could not resolve method ##{mid} on #{recv_type.describe}",
              node: send_node,
            )
            prototype = untyped_prototype
          end
        when AnyType
          prototype = untyped_prototype
        when TypeVar
          errors << Error.new(
            message: "Unknown receiver type in invocation of ##{mid}",
            node: send_node,
          )
          prototype = untyped_prototype
        else
          raise "unknown type #{recv_type.describe} as receiver to send"
        end

        match_prototype_with_arguments(prototype, arg_types, node: send_node)

        [prototype, locals]
      end

      def match_prototype_with_arguments(prototype, arg_types, node:)
        arg_types = arg_types.dup

        required_argc = prototype.lead.count + prototype.post.count

        if arg_types.count < required_argc
          errors << Error.new(
            message: "Too few arguments supplied in method invocation",
            node: node,
          )
        end

        if arg_types.count > required_argc && (prototype.kwreq.any? || prototype.kwopt.any?)
          last_arg = prune(arg_types.last)

          if last_arg.is_a?(KeywordHashType)
            arg_types.pop
            kw_arg = last_arg
            # TODO - type check
          end
        end

        prototype.lead.each do |lead_arg|
          arg_type = arg_types.shift or return
          unify!(arg_type, lead_arg)
        end

        prototype.post.each do |post_arg|
          arg_type = arg_types.pop or return
          unify!(arg_type, post_arg)
        end

        prototype.opt.each do |opt_arg|
          arg_type = arg_types.shift or break
          unify!(arg_type, opt_arg)
        end

        if arg_types.count > 0
          if rest_type = prune(prototype.rest)
            unless rest_type.is_a?(InstanceType) && rest_type.klass == env.Array
              # TODO sketchy
              raise "expected rest arg to be an array..."
            end

            rest_element_type = rest_type.type_parameters[0]

            arg_types.each do |arg_type|
              unify!(arg_type, rest_element_type)
            end
          else
            errors << Error.new(
              message: "Too many arguments supplied in method invocation",
              node: node,
            )
          end
        end
      end

      def on_const(node, locals)
        if validate_static_cpath(node)
          begin
            const = env.resolve_cpath(node: node, scope: scope)

            [InstanceType.new(node: node, klass: const.metaklass(env: env), type_parameters: []), locals]
          rescue NoConstantError => e
            errors << Error.new(
              message: e.message,
              node: node,
            )

            [new_type_var(node: node), locals]
          end
        else
          errors << Error.new(
            message: "Dynamic constant lookup",
            node: node
          )

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


      end

      def on_ivar(node, locals)
        # TODO - need to figure out a way to type instance variables
        [new_type_var(node: node), locals]
      end

      def on_if(node, locals)
        cond, then_expr, else_expr = *node

        # TODO - need flow sensitive typing here:
        cond_type, locals = process(cond, locals)

        # TODO - emit warning if cond_type is always truthy or always falsy

        then_type, then_locals = process(then_expr, locals)
        else_type, else_locals = process(else_expr, locals)

        type = unify_or_make_union(then_type, else_type, node: node)
        locals = merge_locals(then_locals, else_locals, node: node)

        [type, locals]
      end

      def on_false(node)
        Type::Instance.new(mod: env.FalseClass)
      end

      def on_true(node)
        Type::Instance.new(mod: env.TrueClass)
      end

      def on_nil(node, locals)
        [nil_type(node: node), locals]
      end

      def on_super(node, locals)
        # TODO -
        errors << Error.new(
          node: node,
          message: "I haven't implemented super calls yet",
        )

        [new_type_var(node: node), locals]
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
        [self_type, locals]
      end

      def on_hash(node, locals)
        key_type = new_type_var(node: node)
        value_type = new_type_var(node: node)

        keywords = {}

        node.children.each do |n|
          case n.type
          when :pair
            key, value = *n

            if key.type == :sym && keywords
              vt, locals = process(value, locals)

              keywords[key.children[0]] = vt
            else
              keywords = nil
            end

            kt, locals = process(key, locals)
            vt, locals = process(value, locals)

            unify!(key_type, kt)
            unify!(value_type, vt)
          else
            raise "unknown node type in hash literal: #{n}"
          end
        end

        if keywords
          [KeywordHashType.new(node: node, keywords: keywords), locals]
        else
          [InstanceType.new(node: node, klass: env.Hash, type_parameters: [key_type, value_type]), locals]
        end
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
            errors << Error.new(
              message: "Left-hand side of constant lookup (:: operator) is not a constant. Dynamic constant references are not permitted in TypedRuby code.",
              node: node,
            )
            return false
          end
        end
      end

      def capture_local_defs
        current_locals = locals

        yield

        locals
      ensure
        @locals = current_locals
      end
    end
  end
end
