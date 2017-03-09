module TypedRuby
  class AutoloadEntry
    include Task

    attr_reader :file, :node

    def @file : String
    def @node : Node

    def initialize(String file:, Node node:) => nil
      @file = file
      @node = node
      nil
    end

    def perform(Environment env:) => nil
      env.resolver.require_file(file: file, node: node)
    end
  end
end
