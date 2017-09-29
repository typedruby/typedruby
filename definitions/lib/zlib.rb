require 'stringio'

module Zlib
  class GzipFile
  end

  class GzipWriter < GzipFile
    def initialize(
      (IO | StringIO) io,
      ~Integer level = nil,
      ~Integer strategy = nil,
      ~Encoding external_encoding: nil,
      ~Encoding internal_encoding: nil,
      ~Encoding encoding: nil
    ) => nil; end
  end
end
