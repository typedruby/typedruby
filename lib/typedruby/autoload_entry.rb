module TypedRuby
  class AutoloadEntry
    attr_reader :file, :node

    def initialize(String file:, Parser::AST::Node node:) => nil
      @file = file
      @node = node
      nil
    end

    def perform(Environment env:) => nil
      env.resolver.require_file(file: file, node: node)
    end
  end
end
