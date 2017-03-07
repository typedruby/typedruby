module TypedRuby
  module Task
    def perform(Environment env:) => nil
      raise NotImplementedError
    end
  end
end
