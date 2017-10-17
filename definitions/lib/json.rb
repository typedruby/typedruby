module JSON
  def self.dump(:any object) => String
  end

  def self.generate(:any object, (nil | JSON::State | Hash::[Symbol, :any]) options) => String
  end

  def self.parse(String json) => :any
  end

  class State
  end
end
