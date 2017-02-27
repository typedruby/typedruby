module TypedRuby
  class AutoloadEntry
    attr_reader :file, :node

    def initialize(file:, node:)
      @file = file
      @node = node
    end

    def perform(env:)
      env.resolver.require_file(file: file, node: node)
    end
  end
end
