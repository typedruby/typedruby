module Timeout
  class Error < RuntimeError
  end

  def self.timeout[T](Numeric sec, ~Class::[Exception] klass = nil, ~String message = nil, { || => T } &) => T; end
end
