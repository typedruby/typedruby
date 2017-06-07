require "fileutils"

class Dir
  def self.tmpdir => String; end

  def self.mktmpdir(~String prefix_suffix = nil) => String; end
end
