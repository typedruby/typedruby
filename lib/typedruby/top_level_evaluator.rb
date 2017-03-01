module TypedRuby
  class TopLevelEvaluator
    attr_reader :env, :resolver, :scope

    include AST::Processor::Mixin

    def initialize(env:, resolver:, scope:)
      @env = env
      @resolver = resolver
      @scope = scope
    end

    def process(node)
      super
    rescue Error => e
      e.node_backtrace << node
      raise e
    end

    def handler_missing(node)
      raise Error, "Missing handler for: #{node.type}!"
    end

    def on_begin(node)
      process_all(node.children)
    end

    def on_send(node)
      recv, mid, *args = *node

      # special case requires
      if recv == nil
        case mid
        when :require
          process_require(node)
          return
        when :require_relative
          process_require_relative(node)
          return
        when :autoload
          process_autoload(node)
          return
        when :attr_reader
          process_attr(node, reader: true, writer: false)
          return
        when :attr_writer
          process_attr(node, reader: false, writer: true)
          return
        when :attr_accessor
          process_attr(node, reader: true, writer: true)
          return
        when :include
          process_include(node)
        end
      end

      process(recv)
      process_all(args)
    end

    def process_require(node)
      _, _, *args = *node

      if args.count != 1
        raise Error, "require takes only one argument"
      end

      require_arg = args[0]

      if require_arg.type == :str
        file, = *require_arg

        @resolver.require_file(file: file, node: node)
      else
        UI.warn("dynamic require", node: node)
      end
    end

    def process_require_relative(node)
      _, _, *args = *node

      if args.count != 1
        raise Error, "require_relative takes only one argument"
      end

      require_arg = args[0]

      if require_arg.type == :str
        file, = *require_arg

        require_path = File.expand_path(file, File.dirname(node.location.expression.source_buffer.name))

        if File.file?(require_path)
          @resolver.process(require_path)
        elsif File.file?("#{require_path}.rb")
          @resolver.process("#{require_path}.rb")
        else
          UI.warn("could not resolve #{file.inspect} in require_relative", node: node)
        end
      else
        UI.warn("dynamic require", node: node)
      end
    end

    def process_autoload(node)
      _, _, *args = *node

      if args.count != 2
        raise Error, "autoload takes only two arguments"
      end

      if args[0].type != :sym
        raise Error, "first argument of autoload must be symbol literal"
      end

      if args[1].type != :str
        raise Error, "second argument of autoload must be string literal"
      end

      @scope.mod.autoload_const(env: @env, id: args[0].children[0], file: args[1].children[0], node: node)
    end

    def process_attr(node, reader:, writer:)
      _, _, *args = *node

      args.each do |arg|
        if arg.type != :sym
          UI.warn("dynamic attr definition", node: arg)
          next
        end

        attr_name, = *arg

        if reader
          @scope.mod.define_method(id: attr_name, method: RubyMethodStub.new(
            klass: @scope.mod,
            definition_node: node,
            prototype: Prototype.new(
              lead: [],
              opt: [],
              rest: nil,
              post: [],
              kwreq: [],
              kwopt: [],
              block: nil,
              return_type: Type::Any.new,
            ),
          ))
        end

        if writer
          @scope.mod.define_method(id: :"#{attr_name}=", method: RubyMethodStub.new(
            klass: @scope.mod,
            definition_node: node,
            prototype: Prototype.new(
              lead: [],
              opt: [],
              rest: nil,
              post: [],
              kwreq: [],
              kwopt: [],
              block: nil,
              return_type: Type::Any.new,
            ),
          ))
        end
      end
    end

    def process_include(node)
      _, _, *args = *node

      include_modules = args.map { |arg|
        begin
          mod = resolve_cpath(arg)
          raise Error, "not a module" if mod.class != RubyModule
          mod
        rescue Error => e
          UI.warn(e.message, node: arg)
          nil
        end
      }.compact

      include_modules.reverse_each do |mod|
        @scope.mod.include_module(mod)
      end
    end

    def on_str(node)
    end

    def on_sym(node)
    end

    def on_lvar(node)
    end

    def on_true(node)
    end

    def on_nil(node)
    end

    def on_false(node)
    end

    def on_self(node)
    end

    def on_ivar(node)
    end

    def on_regexp(node)
    end

    def on_int(node)
    end

    def on_float(node)
    end

    def on_nth_ref(node)
    end

    def on_back_ref(node)
    end

    def on_gvar(node)
    end

    def on_cvar(node)
    end

    def on_dstr(node)
      process_all(node.children)
    end

    def on_xstr(node)
      process_all(node.children)
    end

    def on_dsym(node)
      process_all(node.children)
    end

    def on_and(node)
      left, right = *node

      process(left)
      process(right)
    end

    def on_or(node)
      left, right = *node

      process(left)
      process(right)
    end

    def on_block(node)
      send, block_args, block_body = *node

      process(send)
      process(block_body)
    end

    def on_lvasgn(node)
      lv, expr = *node

      process(expr)
    end

    def on_match_with_lvasgn(node)
      regexp, expr = *node

      # TODO - implement lvasgn logic for regexp
      process(regexp)
      process(expr)
    end

    def on_const(node)
      resolve_cpath(node)
    rescue NoConstantError => e
      raise Error, e
    end

    def on_def(node)
      id, args, body = *node

      process_method_definition(
        target: @scope.mod,
        id: id,
        node: node,
      )
    end

    def on_defs(node)
      definee, id, args, body = *node

      if definee.type == :self
        resolved_definee = @scope.mod
      elsif definee.type == :const
        resolved_definee = resolve_cpath(definee)
      else
        UI.warn "cannot resolve method definee", node: node
        process(node)
        return
      end

      process_method_definition(
        target: resolved_definee.metaklass(env: @env),
        id: id,
        node: node,
      )
    end

    def process_method_definition(target:, id:, node:)
      method = RubyMethod.new(
        klass: target,
        scope: @scope,
        definition_node: node,
      )

      target.define_method(id: id, method: method)

      @resolver.pending_work << Checker::Typecheck.new(method: method)
    end

    def on_alias(node)
      left, right = *node

      if left.type == :dsym
        UI.warn "dynamic symbol in alias", node: left
        return process_unrecognised_alias(node)
      end

      if right.type == :dsym
        UI.warn "dynamic symbol in alias", node: right
        return process_unrecognised_alias(node)
      end

      if left.type == :sym
        if right.type != :sym
          raise Error, "unexpected right-hand side type in alias"
        end

        left_id, = *left
        right_id, = *right

        @scope.mod.alias_method(to_id: left_id, from_id: right_id)
        return
      end

      process_unrecognised_alias(node)
    end

    def process_unrecognised_alias(node)
      left, right = *node

      process(left)
      process(right)
    end

    def on_undef(node)
      node.children.each do |arg|
        if arg.type == :sym
          id, = *arg
          UI.warn "undefining #{@scope.mod.name}##{id}", node: arg
          @scope.mod.undefine_method(id: id)
        elsif arg.type == :dsym
          UI.warn "dynamic symbol in undef", node: arg
          process(arg)
        else
          raise Error, "unexpected #{arg.type} in undef argument list"
        end
      end
    end

    def on_arg(node)
    end

    def on_optarg(node)
    end

    def on_restarg(node)
    end

    def on_kwarg(node)
    end

    def on_kwoptarg(node)
    end

    def on_kwrestarg(node)
    end

    def on_blockarg(node)
    end

    def on_procarg0(node)
    end

    def on_mlhs(node)
      process_all(node.children)
    end

    def on_zsuper(node)
    end

    def on_super(node)
      process_all(node.children)
    end

    def on_yield(node)
      process_all(node.children)
    end

    def on_hash(node)
      process_all(node.children)
    end

    def on_pair(node)
      left, right = *node

      process(left)
      process(right)
    end

    def on_kwsplat(node)
      expr, = *node

      process(expr)
    end

    def on_ivasgn(node)
      id, expr = *node

      process(expr)
    end

    def on_cvasgn(node)
      id, expr = *node

      process(expr)
    end

    def on_gvasgn(node)
      id, expr = *node

      process(expr)
    end

    def process_lhs(lhs)
      case lhs.type
      when :ivasgn, :lvasgn, :gvasgn
        # pass
      when :send
        process(lhs)
      else
        raise Error, "dunno the left hand side of this assignment: #{lhs}"
      end
    end

    def on_or_asgn(node)
      left, right = *node

      process_lhs(left)
      process(right)
    end

    def on_and_asgn(node)
      left, right = *node

      process_lhs(left)
      process(right)
    end

    def on_op_asgn(node)
      left, op, right = *node

      process_lhs(left)
      process(right)
    end

    def on_masgn(node)
      mlhs, *rhs = *node

      if mlhs.type != :mlhs
        raise Error, "expected left hand side of masgn to be of type mlhs"
      end

      mlhs.children.each do |lhs|
        process_lhs(lhs)
      end

      process_all(rhs)
    end

    def on_if(node)
      cond_expr, then_expr, else_expr = *node

      process(cond_expr)
      process(then_expr)
      process(else_expr)
    end

    def on_while(node)
      condition, body = *node

      process(condition)
      process(body)
    end

    def on_while_post(node)
      condition, body = *node

      process(condition)
      process(body)
    end

    def on_return(node)
      expr, = *node

      process(expr)
    end

    def on_break(node)
      expr, = *node

      process(expr)
    end

    def on_next(node)
      expr, = *node

      process(expr)
    end

    def on_casgn(node)
      cbase, id, expr = *node

      cbase_ref = resolve_cbase(cbase)

      if expr.type == :const
        cbase_ref.set_const(id: id, value: resolve_cpath(expr))
        return
      end

      if expr.type == :send
        recv, mid, *args = *expr

        if recv && recv.type == :const
          recv_const = resolve_cpath(recv)

          if recv_const == @env.Class && mid == :new
            if args.count == 0
              superklass = @env.Object
            elsif args.count == 1 && args[0].type == :const
              superklass = resolve_cpath(args[0])
            else
              raise Error, "Class.new takes either 0 or 1 arguments"
            end

            cbase_ref.set_const(id: id, value: RubyClass.new(
              klass: @env.Class,
              name: id,
              superklass: superklass,
              type_parameters: [],
            ))

            return
          end
        end
      end

      if expr.type == :tr_cast
        expr, type = *expr

        process(expr)

        cbase_ref.set_const(id: id, value:
          RubyUnresolvedExpression.new(scope: @scope, node: expr, type:
            @env.resolve_type(node: type, scope: @scope)))

        return
      end

      cbase_ref.set_const(id: id, value:
        RubyUnresolvedExpression.new(scope: @scope, node: expr, type: Type::Any.new))
    end

    def resolve_casgn_expr(expr)
      if expr.type == :const
        return resolve_cpath(expr)
      end

      if expr.type == :send
      end
    end

    def on_module(node)
      mod_name, body = *node

      cbase, id = *mod_name

      m = resolve_cbase(cbase).define_module(
        env: @env,
        id: id,
        node: node,
      )

      enter_scope(m, node) do
        process(body)
      end
    end

    def on_class(node)
      cls_name, superclass, body = *node

      case cls_name.type
      when :tr_gendecl
        cls_name, *gen_args = *cls_name

        cbase, id = *cls_name
      when :const
        cbase, id = *cls_name
      else
        raise Error, "unknown node type for name in class declaration: #{cls_name.type}"
      end

      cbase, id = *cls_name

      c = resolve_cbase(cbase).define_class(
        env: @env,
        id: id,
        superklass: superclass ? resolve_cpath(superclass) : nil,
        node: node,
        type_parameters: gen_args,
      )

      enter_scope(c, node) do
        process(body)
      end
    end

    def on_sclass(node)
      singleton, body = *node

      case singleton.type
      when :self
        enter_scope(@scope.mod.metaklass(env: @env), node) do
          process(body)
        end
      when :const
        enter_scope(resolve_cpath(singleton).metaklass(env: @env), node) do
          process(body)
        end
      else
        UI.warn("opening singleton class for other than literal self", node: node)
        process(singleton)
        process(body)
      end
    end

    def on_array(node)
      process_all(node.children)
    end

    def on_case(node)
      cond, *whens, else_clause = *node

      process(cond)
      process_all(whens)
      process(else_clause)
    end

    def on_when(node)
      *conditions, body = *node

      process_all(conditions)
      process(body)
    end

    def on_kwbegin(node)
      process_all(node.children)
    end

    def on_rescue(node)
      process_all(node.children)
    end

    def on_resbody(node)
      classes, local, body = *node

      process(classes)
      process(body)
    end

    def on_ensure(node)
      body, ensure_clause = *node

      process(body)
      process(ensure_clause)
    end

    def on_irange(node)
      left, right = *node

      process(left)
      process(right)
    end

    def on_erange(node)
      left, right = *node

      process(left)
      process(right)
    end

    def on_defined?(node)
      expr, = *node

      process(expr)
    end

    def on_splat(node)
      expr, = *node

      process(expr)
    end

    def on_block_pass(node)
      expr, = *node

      process(expr)
    end

    def on_lambda(node)
    end

    def enter_scope(mod, node)
      previous_scope = @scope

      @scope = Scope.new(previous_scope, node, mod)

      yield
    ensure
      @scope = previous_scope
    end

    def resolve_cpath(node)
      env.resolve_cpath(node: node, scope: scope)
    end

    def resolve_cbase(cbase)
      if cbase
        resolve_cpath(cbase)
      else
        @scope.mod
      end
    end
  end
end
