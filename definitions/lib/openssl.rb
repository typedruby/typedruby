module OpenSSL
  OPENSSL_VERSION_NUMBER = (nil : Integer)

  class HMAC
    # TODO - this doesn't work if we just say 'Digest digest'
    # constant scoping is obviously busted
    def self.hexdigest(OpenSSL::Digest digest, String secret, String message) => String; end
  end

  class Digest
    def initialize(String algorithm) => nil; end

    class MD5 < Digest
    end

    class SHA1 < Digest
    end

    class SHA256 < Digest
    end

    class SHA512 < Digest
    end

    class RIPEMD160 < Digest
    end
  end

  module PKey
    class PKey
    end

    class EC < PKey
    end
  end
end
