require 'uri/common'
require 'uri/generic'
require 'uri/ftp'
require 'uri/http'
require 'uri/https'
require 'uri/ldap'
require 'uri/ldaps'
require 'uri/mailto'

module URI
  def self.parse(String uri) => URI::Generic; end
end

class URI::Generic
  def @host : ~String
  def @port : ~Integer
end
