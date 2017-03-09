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
  autoload :RubyAttrReader, "typedruby/ruby_attr_reader"
  autoload :RubyAttrWriter, "typedruby/ruby_attr_writer"
  autoload :RubyClass, "typedruby/ruby_class"
  autoload :RubyIClass, "typedruby/ruby_iclass"
  autoload :RubyIVar, "typedruby/ruby_ivar"
  autoload :RubyModule, "typedruby/ruby_module"
  autoload :RubyObject, "typedruby/ruby_object"
  autoload :RubyMetaclass, "typedruby/ruby_metaclass"
  autoload :RubyMethod, "typedruby/ruby_method"
  autoload :RubyMethodEntry, "typedruby/ruby_method_entry"
  autoload :RubyUnresolvedExpression, "typedruby/ruby_unresolved_expression"
  autoload :Scope, "typedruby/scope"
  autoload :Task, "typedruby/task"
  autoload :TopLevelEvaluator, "typedruby/top_level_evaluator"
  autoload :Type, "typedruby/type"
  autoload :UI, "typedruby/ui"

  Node = Parser::AST::Node

  class Resolver
    attr_reader \
      :load_paths,
      :autoload_paths,
      :ignore_errors_in,
      :autoloader,
      :env,
      :pending_work

    def @load_paths : [String]
    def @autoload_paths : [String]
    def @ignore_errors_in : [String]
    def @autoloader : ~{ |Resolver resolver:, RubyModule mod:, Symbol id:| => ~RubyObject }
    def @env : Environment
    def @loaded : { String => ~Boolean }
    def @pending_work : [Task]

    def initialize(
      [String] load_paths:,
      [String] autoload_paths:,
      [String] ignore_errors_in:,
      ~{ |Resolver resolver:, RubyModule mod:, Symbol id:| => ~RubyObject } autoloader:
    ) => nil
      @load_paths = load_paths
      @autoload_paths = autoload_paths
      @ignore_errors_in = ignore_errors_in
      @autoloader = autoloader

      @env = Environment.new(resolver: self)

      @loaded = {}

      @pending_work = []

      process("#{__dir__}/../definitions/stdlib.rb")
    end

    def evaluate(String source) => nil
      evaluate_ast(parse(source, file: "(eval)"))
    end

    def process(String file) => nil
      file = File.expand_path(file)

      return if @loaded.key?(file)

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

      nil
    end

    def perform => nil
      while task = pending_work.shift
        task.perform(env: env)
      end
    end

    def evaluate_ast(Node ast) => nil
      evaluator = TopLevelEvaluator.new(
        env: env,
        resolver: self,
        scope: env.root_scope,
      )

      evaluator.process(ast)
    end

    def require_file(String file:, Node node:) => nil
      if require_path = search_file_for_require(file)
        process(require_path)
      elsif require_path = search_file_for_require("#{file}.rb")
        process(require_path)
      else
        UI.warn("could not resolve #{file.inspect} in require", node: node)
      end
    end

    def search_file_for_require(String file) => ~String
      @load_paths.each do |path|
        absolute_path = "#{path}/#{file}"

        if File.file?(absolute_path)
          return absolute_path
        end
      end

      nil
    end

    def search_file_for_autoload(String file) => ~String
      @autoload_paths.each do |path|
        absolute_path = "#{path}/#{file}"

        if File.file?(absolute_path)
          return absolute_path
        end
      end

      nil
    end

    def parse(String source, String file:) => Node
      buffer = Parser::Source::Buffer.new(file)
      buffer.source = source

      parser = Parser::TypedRuby24.new(ParserBuilder.new)

      ast = parser.parse(buffer)

      if ast == false
        raise Error, "syntax error in #{file}"
      end

      ast
    end

    def autoload_const(RubyModule mod:, Symbol id:) => ~RubyModule
      if autoloader
        autoloader.call(resolver: self, mod: mod, id: id)
      end
    end
  end
end
