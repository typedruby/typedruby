module TypedRuby
  class Prototype
    class Argument
      attr_reader :type, :name

      def initialize(type:, name:)
        @type = type
        @name = name
      end
    end

    attr_reader \
      :lead,
      :opt,
      :rest,
      :post,
      :kwreq,
      :kwopt,
      :block,
      :return_type

    attr_reader :locals

    def self.from_node(env:, scope:, node:)
      if node.type == :args
        # a bare args node not wrapped by a prototype node should not contain
        # any type annotations

        if node.children.any? { |arg| arg.type == :typed_arg }
          raise Error, "partial type annotations are not permitted at #{node}"
        end

        return nil
      end

      if node.type != :prototype
        raise Error, "unexpected node type: #{node.type}"
      end

      args, return_type_node = *node

      lead = []
      opt = []
      rest = nil
      post = []
      kwreq = []
      kwopt = []
      block = nil

      bare_arg_is_lead = true

      args.children.each do |typed_arg|
        if typed_arg.type != :typed_arg
          raise Error, "partial type annotations are not permitted at #{node}"
        end

        arg_type_node, arg_node = *typed_arg

        arg_type = env.resolve_type(node: arg_type_node, scope: scope)

        case arg_node.type
        when :arg
          arg_name, = *arg_node

          arg = Prototype::Argument.new(type: arg_type, name: arg_name)

          if bare_arg_is_lead
            lead << arg
          else
            post << arg
          end
        when :optarg
          arg_name, arg_default_expr = *arg_node

          # TODO - type check arg_default_expr against arg_type

          opt << Prototype::Argument.new(type: arg_type, name: arg_name)

          bare_arg_is_lead = false
        when :restarg
          arg_name, = *arg_node

          rest = Prototype::Argument.new(type: Type::Array.new(type: arg_type), name: arg_name)

          bare_arg_is_lead = false
        when :kwarg
          arg_name, = *arg_node

          kwreq << Prototype::Argument.new(type: arg_type, name: arg_name)
        when :kwoptarg
          arg_name, arg_default_expr = *arg_node

          kwopt << Prototype::Argument.new(type: arg_type, name: arg_name)
        when :kwrestarg
          # TODO
          raise Error, "kwrestarg not implemented yet"
        when :blockarg
          arg_name, = *arg_node

          # TODO - check that arg_type is an appropriate type: either a proc or nillable proc

          block = Prototype::Argument.new(type: arg_type, name: arg_name)
        else
          raise Error, "unexpected node type: #{node.type}"
        end
      end

      return_type = env.resolve_type(node: return_type_node, scope: scope)

      new(
        lead: lead,
        opt: opt,
        rest: rest,
        post: post,
        kwreq: kwreq,
        kwopt: kwopt,
        block: block,
        return_type: return_type,
      )
    end

    def initialize(lead:, opt:, rest:, post:, kwreq:, kwopt:, block:, return_type:)
      @lead = lead
      @opt = opt
      @rest = rest
      @post = post
      @kwreq = kwreq
      @kwopt = kwopt
      @block = block
      @return_type = return_type

      @locals = [*lead, *opt, rest, *post, *kwreq, *kwopt, block].compact.map { |arg| [arg.name, arg.type] }.to_h.freeze
    end

    def compatible?(other)
      return false unless compatible_arguments?(lead, other.lead)
      return false unless compatible_arguments?(opt, other.opt)
      return false unless rest&.type == other.rest&.type
      return false unless compatible_arguments?(post, other.post)
      return false unless compatible_arguments?(kwreq, other.kwreq)
      return false unless compatible_arguments?(kwopt, other.kwopt)
      return false unless block&.type == other.block&.type
      return false unless return_type == other.return_type

      true
    end

    def to_type_notation
      args = []

      lead.each do |arg|
        args << "#{arg.type.to_type_notation} #{arg.name}"
      end

      opt.each do |arg|
        args << "#{arg.type.to_type_notation} #{arg.name} = ..."
      end

      if rest
        args << "#{rest.type.to_type_notation} *#{rest.name}"
      end

      post.each do |arg|
        args << "#{arg.type.to_type_notation} #{arg.name}"
      end

      kwreq.each do |arg|
        args << "#{arg.type.to_type_notation} #{arg.name}:"
      end

      kwopt.each do |arg|
        args << "#{arg.type.to_type_notation} #{arg.name}: ..."
      end

      if block
        args << "#{block.type.to_type_notation} &#{block.name}"
      end

      "(#{args.join(", ")}) => #{return_type.to_type_notation}"
    end

  private
    def compatible_arguments?(args1, args2)
      args1.map(&:type) == args2.map(&:type)
    end
  end
end
