require "parser/typedruby24"
require "pry"

module TypedRuby
  autoload :AutoloadEntry, "typedruby/autoload_entry"
  autoload :Checker, "typedruby/checker"
  autoload :Environment, "typedruby/environment"
  autoload :Error, "typedruby/error"
  autoload :Location, "typedruby/location"
  autoload :NoConstantError, "typedruby/no_constant_error"
  autoload :ParserBuilder, "typedruby/parser_builder"
  autoload :Prototype, "typedruby/prototype"
  autoload :RubyClass, "typedruby/ruby_class"
  autoload :RubyModule, "typedruby/ruby_module"
  autoload :RubyObject, "typedruby/ruby_object"
  autoload :RubyMetaclass, "typedruby/ruby_metaclass"
  autoload :RubyMethod, "typedruby/ruby_method"
  autoload :RubyMethodStub, "typedruby/ruby_method_stub"
  autoload :RubyMethodEntry, "typedruby/ruby_method_entry"
  autoload :RubyUnresolvedExpression, "typedruby/ruby_unresolved_expression"
  autoload :Scope, "typedruby/scope"
  autoload :TopLevelEvaluator, "typedruby/top_level_evaluator"
  autoload :Type, "typedruby/type"
  autoload :UI, "typedruby/ui"

  class Resolver
    attr_reader \
      :load_paths,
      :autoload_paths,
      :ignore_errors_in,
      :autoloader,
      :env,
      :pending_work

    def initialize(load_paths:, autoload_paths:, ignore_errors_in:, autoloader:)
      @load_paths = load_paths
      @autoload_paths = autoload_paths
      @ignore_errors_in = ignore_errors_in
      @autoloader = autoloader

      @env = Environment.new(resolver: self)

      @loaded = {}

      @pending_work = []

      process("#{__dir__}/../definitions/stdlib.rb")
    end

    def evaluate(source)
      evaluate_ast(parse(source, file: "(eval)"))
    end

    def process(file)
      file = File.expand_path(file)

      if @loaded.key?(file)
        return @loaded[file]
      end

      @loaded[file] = nil

      begin
        evaluate_ast(parse(File.read(file), file: file))
      rescue TypedRuby::Error => e
        if ignore_errors_in.any? { |path| file.start_with?(path) }
          UI.warn(e.message, node: e.node_backtrace.first)
          return
        end
        raise e
      end

      @loaded[file] = true
    end

    def perform
      while task = pending_work.shift
        task.perform(env: env)
      end
    end

    def evaluate_ast(ast)
      evaluator = TopLevelEvaluator.new(
        env: env,
        resolver: self,
        scope: env.root_scope,
      )

      evaluator.process(ast)
    end

    def require_file(file:, node:)
      if require_path = search_file_for_require(file)
        process(require_path)
      elsif require_path = search_file_for_require("#{file}.rb")
        process(require_path)
      else
        UI.warn("could not resolve #{file.inspect} in require", node: node)
      end
    end

    def search_file_for_require(file)
      @load_paths.each do |path|
        absolute_path = "#{path}/#{file}"

        if File.file?(absolute_path)
          return absolute_path
        end
      end

      nil
    end

    def search_file_for_autoload(file)
      @autoload_paths.each do |path|
        absolute_path = "#{path}/#{file}"

        if File.file?(absolute_path)
          return absolute_path
        end
      end

      nil
    end

    def parse(source, file:)
      buffer = Parser::Source::Buffer.new(file)
      buffer.source = source

      parser = Parser::TypedRuby24.new(ParserBuilder.new)

      ast = parser.parse(buffer)

      if ast == false
        raise Error, "syntax error in #{file}"
      end

      ast
    end

    def autoload_const(mod:, id:)
      if autoloader
        autoloader.call(resolver: self, mod: mod, id: id)
      end
    end
  end
end
