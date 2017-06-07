module SecureRandom
  def self.bytes(Integer n) => String; end
end

module Random::Formatter
  def random_bytes(~Integer n = nil) => String; end
  def hex(~Integer n = nil) => String; end
  def base64(~Integer n = nil) => String; end
  def urlsafe_base64(~Integer n = nil, Boolean padding = false) => String; end
  def uuid => String; end
end

class << SecureRandom
  include Random::Formatter
end
