class BasicObject
  def initialize => :any; end

  def __id__ => Integer; end

  def ==(:any other) => Boolean; end
  def !=(:any other) => Boolean; end

  def ! => Boolean; end
end

module Kernel
  def puts(:any *) => nil; end

  def warn(:any *) => nil; end

  def nil? => Boolean; end

  def to_s => String; end

  def inspect => String; end

  def caller => [String]; end

  def rand => Float; end

  def __dir__ => String; end

  def proc[T](T &) => T; end

  def lambda[T](T &) => T; end

  # __method__ actually returns ~Symbol, but it will always return Symbol when
  # called from a method (TypedRuby only type-checks within methods)
  def __method__ => Symbol; end

  def loop({ || => :any } &) => nil; end

  def hash => Integer; end

  def tap({ |:self obj| => :any } &) => :self; end

  def block_given? => Boolean; end

  def respond_to?((String|Symbol) mid) => Boolean; end

  def nil? => Boolean; end

  def frozen? => Boolean; end

  def ===(:any other) => Boolean; end

  def object_id => Integer; end

  def eql?(Object other) => Boolean; end

  def <=>(Object other) => ~Integer; end
end

class ENVClass
  def [](String name) => ~String; end

  def fetch(String name) => String; end

  def []=(String name, String value) => String; end
end

class Object < BasicObject
  include Kernel
  STDOUT = (nil : IO)
  STDERR = (nil : IO)
  NIL = nil
  STDIN = (nil : IO)
  ARGF = nil
  TRUE = nil
  FALSE = nil
  ENV = (nil : ENVClass)
  RUBY_RELEASE_DATE = nil
  RUBY_PATCHLEVEL = nil
  RUBY_VERSION = nil
  RUBY_REVISION = nil
  RUBY_ENGINE = nil
  RUBY_ENGINE_VERSION = nil
  RUBY_COPYRIGHT = nil
  RUBY_PLATFORM = nil
  RUBY_DESCRIPTION = nil
  TOPLEVEL_BINDING = nil
  ARGV = nil

  def class => :class; end

  def dup => :self; end

  def freeze => :self; end

  def send(Symbol method_name, :any *args) => :any; end
end

module Enumerable
end

class IO < Object
  include Enumerable
  SEEK_SET = nil
  SEEK_CUR = nil
  SEEK_END = nil

  def self.read(
    String name,
    ~Integer length = nil,
    ~Integer offset = nil
    # :any encoding:, # TODO this should be String or Encoding
    # String mode:,
    # [String] open_args:
  ) => String
  end

  def write(String data) => Integer; end
end

class BasicSocket < IO
end

class IPSocket < BasicSocket
end

class File < IO
  Separator = nil
  SEPARATOR = nil
  ALT_SEPARATOR = nil
  PATH_SEPARATOR = nil

  # TODO - needs a stricter prototype
  def self.open(:any *, { |File f| => :any } &) => :any; end

  def self.join(String s, String *) => String; end

  def self.expand_path(String file, ~String dir = nil) => String; end

  def self.file?(String file) => Boolean; end

  def self.exist?(String file) => Boolean; end

  def self.rename(String old_name, String new_name) => Integer; end

  def self.dirname(String path) => String; end

  def self.basename(String path, ~String ext = nil) => String; end

  def flock(Integer operation) => (FalseClass | Integer); end
end

module File::Constants
  RDONLY = nil
  WRONLY = nil
  RDWR = (nil : Integer)
  APPEND = nil
  CREAT = (nil : Integer)
  EXCL = nil
  NONBLOCK = nil
  TRUNC = nil
  NOCTTY = nil
  BINARY = nil
  SHARE_DELETE = nil
  SYNC = nil
  DSYNC = nil
  NOFOLLOW = nil
  LOCK_SH = (nil : Integer)
  LOCK_EX = (nil : Integer)
  LOCK_UN = (nil : Integer)
  LOCK_NB = (nil : Integer)
  NULL = nil
  FNM_NOESCAPE = nil
  FNM_PATHNAME = nil
  FNM_DOTMATCH = nil
  FNM_CASEFOLD = nil
  FNM_EXTGLOB = nil
  FNM_SYSCASE = nil
  FNM_SHORTNAME = nil
end

class IO
  include File::Constants
end

module Comparable
end

class File::Stat < Object
  include Comparable
end

module IO::WaitReadable
end

module IO::WaitWritable
end

module Errno
end

class Exception < Object
  def initialize(String s) => nil; end

  def message => String; end

  def backtrace => [String]; end

  def set_backtrace([String] backtrace) => nil; end
end

class StandardError < Exception
end

class SystemCallError < StandardError
end

class Errno::EAGAIN < SystemCallError
  Errno = nil
end

class IO::EAGAINWaitReadable < Errno::EAGAIN
  include IO::WaitReadable
end

class Errno::NOERROR < SystemCallError
  Errno = nil
end

class Errno::EPERM < SystemCallError
  Errno = nil
end

class Errno::ENOENT < SystemCallError
  Errno = nil
end

class Errno::ESRCH < SystemCallError
  Errno = nil
end

class Errno::EINTR < SystemCallError
  Errno = nil
end

class Errno::EIO < SystemCallError
  Errno = nil
end

class Errno::ENXIO < SystemCallError
  Errno = nil
end

class Errno::E2BIG < SystemCallError
  Errno = nil
end

class Errno::ENOEXEC < SystemCallError
  Errno = nil
end

class Errno::EBADF < SystemCallError
  Errno = nil
end

class Errno::ECHILD < SystemCallError
  Errno = nil
end

class Errno::ENOMEM < SystemCallError
  Errno = nil
end

class Errno::EACCES < SystemCallError
  Errno = nil
end

class Errno::EFAULT < SystemCallError
  Errno = nil
end

class Errno::ENOTBLK < SystemCallError
  Errno = nil
end

class Errno::EBUSY < SystemCallError
  Errno = nil
end

class Errno::EEXIST < SystemCallError
  Errno = nil
end

class Errno::EXDEV < SystemCallError
  Errno = nil
end

class Errno::ENODEV < SystemCallError
  Errno = nil
end

class Errno::ENOTDIR < SystemCallError
  Errno = nil
end

class Errno::EISDIR < SystemCallError
  Errno = nil
end

class Errno::EINVAL < SystemCallError
  Errno = nil
end

class Errno::ENFILE < SystemCallError
  Errno = nil
end

class Errno::EMFILE < SystemCallError
  Errno = nil
end

class Errno::ENOTTY < SystemCallError
  Errno = nil
end

class Errno::ETXTBSY < SystemCallError
  Errno = nil
end

class Errno::EFBIG < SystemCallError
  Errno = nil
end

class Errno::ENOSPC < SystemCallError
  Errno = nil
end

class Errno::ESPIPE < SystemCallError
  Errno = nil
end

class Errno::EROFS < SystemCallError
  Errno = nil
end

class Errno::EMLINK < SystemCallError
  Errno = nil
end

class Errno::EPIPE < SystemCallError
  Errno = nil
end

class Errno::EDOM < SystemCallError
  Errno = nil
end

class Errno::ERANGE < SystemCallError
  Errno = nil
end

class Errno::EDEADLK < SystemCallError
  Errno = nil
end

class Errno::ENAMETOOLONG < SystemCallError
  Errno = nil
end

class Errno::ENOLCK < SystemCallError
  Errno = nil
end

class Errno::ENOSYS < SystemCallError
  Errno = nil
end

class Errno::ENOTEMPTY < SystemCallError
  Errno = nil
end

class Errno::ELOOP < SystemCallError
  Errno = nil
end

class Errno::ENOMSG < SystemCallError
  Errno = nil
end

class Errno::EIDRM < SystemCallError
  Errno = nil
end

class Errno::ENOSTR < SystemCallError
  Errno = nil
end

class Errno::ENODATA < SystemCallError
  Errno = nil
end

class Errno::ETIME < SystemCallError
  Errno = nil
end

class Errno::ENOSR < SystemCallError
  Errno = nil
end

class Errno::EREMOTE < SystemCallError
  Errno = nil
end

class Errno::ENOLINK < SystemCallError
  Errno = nil
end

class Errno::EPROTO < SystemCallError
  Errno = nil
end

class Errno::EMULTIHOP < SystemCallError
  Errno = nil
end

class Errno::EBADMSG < SystemCallError
  Errno = nil
end

class Errno::EOVERFLOW < SystemCallError
  Errno = nil
end

class Errno::EILSEQ < SystemCallError
  Errno = nil
end

class Errno::EUSERS < SystemCallError
  Errno = nil
end

class Errno::ENOTSOCK < SystemCallError
  Errno = nil
end

class Errno::EDESTADDRREQ < SystemCallError
  Errno = nil
end

class Errno::EMSGSIZE < SystemCallError
  Errno = nil
end

class Errno::EPROTOTYPE < SystemCallError
  Errno = nil
end

class Errno::ENOPROTOOPT < SystemCallError
  Errno = nil
end

class Errno::EPROTONOSUPPORT < SystemCallError
  Errno = nil
end

class Errno::ESOCKTNOSUPPORT < SystemCallError
  Errno = nil
end

class Errno::EOPNOTSUPP < SystemCallError
  Errno = nil
end

class Errno::EPFNOSUPPORT < SystemCallError
  Errno = nil
end

class Errno::EAFNOSUPPORT < SystemCallError
  Errno = nil
end

class Errno::EADDRINUSE < SystemCallError
  Errno = nil
end

class Errno::EADDRNOTAVAIL < SystemCallError
  Errno = nil
end

class Errno::ENETDOWN < SystemCallError
  Errno = nil
end

class Errno::ENETUNREACH < SystemCallError
  Errno = nil
end

class Errno::ENETRESET < SystemCallError
  Errno = nil
end

class Errno::ECONNABORTED < SystemCallError
  Errno = nil
end

class Errno::ECONNRESET < SystemCallError
  Errno = nil
end

class Errno::ENOBUFS < SystemCallError
  Errno = nil
end

class Errno::EISCONN < SystemCallError
  Errno = nil
end

class Errno::ENOTCONN < SystemCallError
  Errno = nil
end

class Errno::ESHUTDOWN < SystemCallError
  Errno = nil
end

class Errno::ETOOMANYREFS < SystemCallError
  Errno = nil
end

class Errno::ETIMEDOUT < SystemCallError
  Errno = nil
end

class Errno::ECONNREFUSED < SystemCallError
  Errno = nil
end

class Errno::EHOSTDOWN < SystemCallError
  Errno = nil
end

class Errno::EHOSTUNREACH < SystemCallError
  Errno = nil
end

class Errno::EALREADY < SystemCallError
  Errno = nil
end

class Errno::EINPROGRESS < SystemCallError
  Errno = nil
end

class Errno::ESTALE < SystemCallError
  Errno = nil
end

class Errno::EDQUOT < SystemCallError
  Errno = nil
end

class Errno::ECANCELED < SystemCallError
  Errno = nil
end

class Errno::ENOTRECOVERABLE < SystemCallError
  Errno = nil
end

class Errno::EOWNERDEAD < SystemCallError
  Errno = nil
end

class Errno::EAUTH < SystemCallError
  Errno = nil
end

class Errno::EBADRPC < SystemCallError
  Errno = nil
end

class Errno::EFTYPE < SystemCallError
  Errno = nil
end

class Errno::ENEEDAUTH < SystemCallError
  Errno = nil
end

class Errno::ENOATTR < SystemCallError
  Errno = nil
end

class Errno::ENOTSUP < SystemCallError
  Errno = nil
end

class Errno::EPROCLIM < SystemCallError
  Errno = nil
end

class Errno::EPROCUNAVAIL < SystemCallError
  Errno = nil
end

class Errno::EPROGMISMATCH < SystemCallError
  Errno = nil
end

class Errno::EPROGUNAVAIL < SystemCallError
  Errno = nil
end

class Errno::ERPCMISMATCH < SystemCallError
  Errno = nil
end

class IO::EAGAINWaitWritable < Errno::EAGAIN
  include IO::WaitWritable
end

class IO::EINPROGRESSWaitReadable < Errno::EINPROGRESS
  include IO::WaitReadable
end

class IO::EINPROGRESSWaitWritable < Errno::EINPROGRESS
  include IO::WaitWritable
end

class TCPSocket < IPSocket
end

class TCPServer < TCPSocket
end

class UNIXSocket < BasicSocket
end

class UNIXServer < UNIXSocket
end

class Method < Object
end

class Numeric < Object
  include Comparable

  def <(Numeric other) => Boolean; end
  def <=(Numeric other) => Boolean; end
  def >(Numeric other) => Boolean; end
  def >=(Numeric other) => Boolean; end
  def <=>(Numeric other) => Boolean; end
  def ==(Numeric other) => Boolean; end
  def !=(Numeric other) => Boolean; end
end

class Integer < Numeric
  GMP_VERSION = nil

  def |(Integer other) => Integer; end

  def +(Integer other) => Integer; end

  def -(Integer other) => Integer; end

  def *(Integer other) => Integer; end

  def /(Integer other) => Integer; end
end

class Float < Numeric
  ROUNDS = nil
  RADIX = nil
  MANT_DIG = nil
  DIG = nil
  MIN_EXP = nil
  MAX_EXP = nil
  MIN_10_EXP = nil
  MAX_10_EXP = nil
  MIN = nil
  MAX = nil
  EPSILON = nil
  INFINITY = nil
  NAN = nil
end

class String < Object
  include Comparable

  def +(String other) => String; end

  def <<(String other) => String; end

  def %(Object arg) => String; end

  def *(Integer times) => String; end

  def sub((String | Regexp) pattern, { |String s| => String } &) => String; end

  def gsub((String | Regexp) pattern, { |String s| => String } &) => String; end

  def sub!((String | Regexp) pattern, { |String s| => String } &) => ~String; end

  def gsub!((String | Regexp) pattern, { |String s| => String } &) => ~String; end

  def size => Integer; end

  def length => Integer; end

  def b => String; end

  def to_sym => Symbol; end

  def start_with?(String prefix) => Boolean; end

  def end_with?(String suffix) => Boolean; end

  def downcase => String; end

  def upcase => String; end

  def capitalize => String; end

  def strip => String; end

  def []((Integer | Range::[Integer, Integer]) idx) => String; end

  def []=((Integer | Range::[Integer, Integer]) idx, String replacement) => String; end

  def =~(Regexp pattern) => ~Integer; end

  def lines => [String]; end

  def chomp(String suffix = "") => String; end

  def chomp!(String suffix = "") => ~String; end

  def split(String delim) => [String]; end

  def unpack(String format) => [:any]; end

  def empty? => Boolean; end

  def to_sym => Symbol; end

  def intern => Symbol; end

  def to_i => Integer; end
end

class Array::[ElementType] < Object
  def each({ |ElementType element| => :any } &bk) => :self; end

  def each_with_index({ |ElementType element, Integer index| => :any } &) => :self; end

  def each_with_object[T](T object, { |ElementType element, T object| => :any } &) => :self; end

  include Enumerable

  def <<(ElementType item) => :self; end

  def map[ProjectedType]({ |ElementType element| => ProjectedType } &) => [ProjectedType]; end
  alias :collect :map

  def map!({ |ElementType element| => ElementType } &) => :self; end
  alias :collect! :map!

  def select({ |ElementType x| => Boolean } &) => [ElementType]; end

  def reject({ |ElementType x| => Boolean } &) => [ElementType]; end

  def reject!({ |ElementType element| => Boolean } &) => ~[ElementType]; end

  def include?(ElementType item) => Boolean; end

  def shift => ~ElementType; end

  def unshift(ElementType element) => :self; end

  def pop => ~ElementType; end

  def push(ElementType element) => :self; end

  def *(Integer times) => :self; end

  def join(String sep = "") => String; end

  def any?({ |ElementType element| => Boolean } &) => Boolean; end

  def all?({ |ElementType element| => Boolean } &) => Boolean; end

  def first => ~ElementType; end

  def last => ~ElementType; end

  def +([ElementType] other) => [ElementType]; end

  def -([ElementType] other) => [ElementType]; end

  def concat([ElementType] other) => :self; end

  def size => Integer; end

  def length => Integer; end

  def compact[NonNullType : ~NonNullType = ElementType] => [NonNullType]; end

  def empty? => Boolean; end

  def uniq(~{ |ElementType element| => :any } &) => [ElementType]; end

  def uniq!(~{ |ElementType element| => :any } &) => [ElementType]; end

  def find({ |ElementType element| => Boolean } &) => ~ElementType; end

  def drop(Integer n) => [ElementType]; end

  def [](Integer index) => ~ElementType; end

  def []=(Integer index, ElementType value) => ElementType; end

  def fetch(Integer index) => ElementType; end

  def to_h[K, V : ElementType = [K, V]] => { K => V }; end

  def group_by[GroupKey]({ |ElementType element| => GroupKey } &) => { GroupKey => [ElementType] }; end

  def delete_if({ |ElementType element| => Boolean } &) => :self; end

  # TODO enforce that SortKey must respond to <=>
  # or perhaps that it's comparable?
  def sort_by[SortKey]({ |ElementType element| => SortKey } &) => [ElementType]; end
  def sort_by![SortKey]({ |ElementType element| => SortKey } &) => :self; end

  def take(Integer count) => [ElementType]; end
end

class Hash::[KeyType, ValueType] < Object
  def each({ |KeyType k, ValueType v| => :any } &) => :self; end

  include Enumerable

  def merge(Hash::[KeyType, ValueType] other) => Hash::[KeyType, ValueType]; end

  def merge!(Hash::[KeyType, ValueType] other) => :self; end

  def select({ |KeyType k, ValueType v| => Boolean } &) => Hash::[KeyType, ValueType]; end

  def reject({ |KeyType k, ValueType v| => Boolean } &) => Hash::[KeyType, ValueType]; end

  def fetch(KeyType k, ~:any v = nil, ~{ || => ValueType } &) => ValueType; end

  def [](KeyType k) => ~ValueType; end

  def []=(KeyType k, ValueType v) => ValueType; end

  def key?(KeyType k) => Boolean; end
  alias :has_key? :key?

  def include?(KeyType k) => Boolean; end

  def empty? => Boolean; end

  def map[ProjectedType]({ |KeyType k, ValueType v| => ProjectedType } &) => [ProjectedType]; end
  alias :collect :map

  def keys => [KeyType]; end

  def values => [ValueType]; end

  def delete(KeyType k) => ~ValueType; end

  def any?({ |KeyType k, ValueType v| => Boolean } &) => Boolean; end

  def reduce[T](T initial, { |T acc, [KeyType, ValueType] kv| => T } &) => T; end
  alias :inject :reduce

  def delete_if({ |KeyType key, ValueType value| => Boolean } &) => :self; end
end

class NilClass < Object
  def nil? => TrueClass; end
end

class ArgumentError < StandardError
end

class UncaughtThrowError < ArgumentError
end

module FileTest
end

module GC
  INTERNAL_CONSTANTS = nil
  OPTS = nil
end

module GC::Profiler
end

class Fiber < Object
end

class FiberError < StandardError
end

class Rational < Numeric
end

module ObjectSpace
end

class ObjectSpace::WeakMap < Object
  include Enumerable
end

module DidYouMean
end

class Data < Object
end

class Boolean < Object
end

class TrueClass < Boolean
end

class FalseClass < Boolean
end

class Encoding < Object
  BINARY = nil
  ASCII_8BIT = nil
  UTF_8 = nil
  US_ASCII = nil
  Big5 = nil
  BIG5 = nil
  Big5_HKSCS = nil
  BIG5_HKSCS = nil
  Big5_UAO = nil
  BIG5_UAO = nil
  CP949 = nil
  Emacs_Mule = nil
  EMACS_MULE = nil
  EUC_JP = nil
  EUC_KR = nil
  EUC_TW = nil
  GB2312 = nil
  GB18030 = nil
  GBK = nil
  ISO_8859_1 = nil
  ISO_8859_2 = nil
  ISO_8859_3 = nil
  ISO_8859_4 = nil
  ISO_8859_5 = nil
  ISO_8859_6 = nil
  ISO_8859_7 = nil
  ISO_8859_8 = nil
  ISO_8859_9 = nil
  ISO_8859_10 = nil
  ISO_8859_11 = nil
  ISO_8859_13 = nil
  ISO_8859_14 = nil
  ISO_8859_15 = nil
  ISO_8859_16 = nil
  KOI8_R = nil
  KOI8_U = nil
  Shift_JIS = nil
  SHIFT_JIS = nil
  UTF_16BE = nil
  UTF_16LE = nil
  UTF_32BE = nil
  UTF_32LE = nil
  Windows_31J = nil
  WINDOWS_31J = nil
  Windows_1250 = nil
  WINDOWS_1250 = nil
  Windows_1251 = nil
  WINDOWS_1251 = nil
  Windows_1252 = nil
  WINDOWS_1252 = nil
  Windows_1253 = nil
  WINDOWS_1253 = nil
  Windows_1254 = nil
  WINDOWS_1254 = nil
  Windows_1257 = nil
  WINDOWS_1257 = nil
  IBM437 = nil
  CP437 = nil
  IBM737 = nil
  CP737 = nil
  IBM775 = nil
  CP775 = nil
  CP850 = nil
  IBM850 = nil
  IBM852 = nil
  CP852 = nil
  IBM855 = nil
  CP855 = nil
  IBM857 = nil
  CP857 = nil
  IBM860 = nil
  CP860 = nil
  IBM861 = nil
  CP861 = nil
  IBM862 = nil
  CP862 = nil
  IBM863 = nil
  CP863 = nil
  IBM864 = nil
  CP864 = nil
  IBM865 = nil
  CP865 = nil
  IBM866 = nil
  CP866 = nil
  IBM869 = nil
  CP869 = nil
  Windows_1258 = nil
  WINDOWS_1258 = nil
  CP1258 = nil
  GB1988 = nil
  MacCentEuro = nil
  MACCENTEURO = nil
  MacCroatian = nil
  MACCROATIAN = nil
  MacCyrillic = nil
  MACCYRILLIC = nil
  MacGreek = nil
  MACGREEK = nil
  MacIceland = nil
  MACICELAND = nil
  MacRoman = nil
  MACROMAN = nil
  MacRomania = nil
  MACROMANIA = nil
  MacThai = nil
  MACTHAI = nil
  MacTurkish = nil
  MACTURKISH = nil
  MacUkraine = nil
  MACUKRAINE = nil
  CP950 = nil
  Big5_HKSCS_2008 = nil
  BIG5_HKSCS_2008 = nil
  CP951 = nil
  IBM037 = nil
  EBCDIC_CP_US = nil
  Stateless_ISO_2022_JP = nil
  STATELESS_ISO_2022_JP = nil
  EucJP = nil
  EUCJP = nil
  EucJP_ms = nil
  EUCJP_MS = nil
  EUC_JP_MS = nil
  CP51932 = nil
  EUC_JIS_2004 = nil
  EUC_JISX0213 = nil
  EucKR = nil
  EUCKR = nil
  EucTW = nil
  EUCTW = nil
  EUC_CN = nil
  EucCN = nil
  EUCCN = nil
  GB12345 = nil
  CP936 = nil
  ISO_2022_JP = nil
  ISO2022_JP = nil
  ISO_2022_JP_2 = nil
  ISO2022_JP2 = nil
  CP50220 = nil
  CP50221 = nil
  ISO8859_1 = nil
  ISO8859_2 = nil
  ISO8859_3 = nil
  ISO8859_4 = nil
  ISO8859_5 = nil
  ISO8859_6 = nil
  Windows_1256 = nil
  WINDOWS_1256 = nil
  CP1256 = nil
  ISO8859_7 = nil
  ISO8859_8 = nil
  Windows_1255 = nil
  WINDOWS_1255 = nil
  CP1255 = nil
  ISO8859_9 = nil
  ISO8859_10 = nil
  ISO8859_11 = nil
  TIS_620 = nil
  Windows_874 = nil
  WINDOWS_874 = nil
  CP874 = nil
  ISO8859_13 = nil
  ISO8859_14 = nil
  ISO8859_15 = nil
  ISO8859_16 = nil
  CP878 = nil
  MacJapanese = nil
  MACJAPANESE = nil
  MacJapan = nil
  MACJAPAN = nil
  ASCII = nil
  ANSI_X3_4_1968 = nil
  UTF_7 = nil
  CP65000 = nil
  CP65001 = nil
  UTF8_MAC = nil
  UTF_8_MAC = nil
  UTF_8_HFS = nil
  UTF_16 = nil
  UTF_32 = nil
  UCS_2BE = nil
  UCS_4BE = nil
  UCS_4LE = nil
  CP932 = nil
  CsWindows31J = nil
  CSWINDOWS31J = nil
  SJIS = nil
  PCK = nil
  CP1250 = nil
  CP1251 = nil
  CP1252 = nil
  CP1253 = nil
  CP1254 = nil
  CP1257 = nil
  UTF8_DoCoMo = nil
  UTF8_DOCOMO = nil
  SJIS_DoCoMo = nil
  SJIS_DOCOMO = nil
  UTF8_KDDI = nil
  SJIS_KDDI = nil
  ISO_2022_JP_KDDI = nil
  Stateless_ISO_2022_JP_KDDI = nil
  STATELESS_ISO_2022_JP_KDDI = nil
  UTF8_SoftBank = nil
  UTF8_SOFTBANK = nil
  SJIS_SoftBank = nil
  SJIS_SOFTBANK = nil
end

class EncodingError < StandardError
end

class Encoding::UndefinedConversionError < EncodingError
end

class Encoding::InvalidByteSequenceError < EncodingError
end

class Encoding::ConverterNotFoundError < EncodingError
end

class Encoding::Converter < Data
  INVALID_MASK = nil
  INVALID_REPLACE = nil
  UNDEF_MASK = nil
  UNDEF_REPLACE = nil
  UNDEF_HEX_CHARREF = nil
  PARTIAL_INPUT = nil
  AFTER_OUTPUT = nil
  UNIVERSAL_NEWLINE_DECORATOR = nil
  CRLF_NEWLINE_DECORATOR = nil
  CR_NEWLINE_DECORATOR = nil
  XML_TEXT_DECORATOR = nil
  XML_ATTR_CONTENT_DECORATOR = nil
  XML_ATTR_QUOTE_DECORATOR = nil
end

class Encoding::CompatibilityError < EncodingError
end

class ZeroDivisionError < StandardError
end

class RangeError < StandardError
end

class FloatDomainError < RangeError
end

class Complex < Numeric
  I = nil
end

class Enumerator < Object
  include Enumerable
end

class Enumerator::Lazy < Enumerator
end

class Enumerator::Generator < Object
  include Enumerable
end

class Enumerator::Yielder < Object
end

class Struct < Object
  include Enumerable
end

module Process
  WNOHANG = nil
  WUNTRACED = nil
  PRIO_PROCESS = nil
  PRIO_PGRP = nil
  PRIO_USER = nil
  RLIM_SAVED_MAX = nil
  RLIM_INFINITY = nil
  RLIM_SAVED_CUR = nil
  RLIMIT_AS = nil
  RLIMIT_CORE = nil
  RLIMIT_CPU = nil
  RLIMIT_DATA = nil
  RLIMIT_FSIZE = nil
  RLIMIT_MEMLOCK = nil
  RLIMIT_NOFILE = nil
  RLIMIT_NPROC = nil
  RLIMIT_RSS = nil
  RLIMIT_STACK = nil
  CLOCK_REALTIME = nil
  CLOCK_MONOTONIC = nil
  CLOCK_PROCESS_CPUTIME_ID = nil
  CLOCK_THREAD_CPUTIME_ID = nil
  CLOCK_MONOTONIC_RAW = nil
  CLOCK_MONOTONIC_RAW_APPROX = nil
  CLOCK_UPTIME_RAW = nil
  CLOCK_UPTIME_RAW_APPROX = nil
end

class Process::Tms < Struct
end

class Thread < Object
end

class Process::Waiter < Thread
end

class Thread::Backtrace < Object
end

class Thread::Backtrace::Location < Object
end

class Thread::Mutex < Object
end

Mutex = Thread::Mutex

class Thread::Queue < Object
end

class Thread::SizedQueue < Thread::Queue
end

class Thread::ConditionVariable < Object
end

class Process::Status < Object
end

module Process::UID
end

module Process::GID
end

module Process::Sys
end

module Etc
  SC_SAVED_IDS = nil
  SC_SEMAPHORES = nil
  SC_SHARED_MEMORY_OBJECTS = nil
  SC_SHELL = nil
  SC_SPAWN = nil
  SC_SPIN_LOCKS = nil
  SC_SPORADIC_SERVER = nil
  SC_SS_REPL_MAX = nil
  SC_SYNCHRONIZED_IO = nil
  SC_THREAD_ATTR_STACKADDR = nil
  SC_THREAD_ATTR_STACKSIZE = nil
  SC_THREAD_CPUTIME = nil
  SC_THREAD_PRIO_INHERIT = nil
  SC_THREAD_PRIO_PROTECT = nil
  SC_THREAD_PRIORITY_SCHEDULING = nil
  SC_THREAD_PROCESS_SHARED = nil
  SC_THREAD_SAFE_FUNCTIONS = nil
  SC_THREAD_SPORADIC_SERVER = nil
  SC_THREADS = nil
  SC_TIMEOUTS = nil
  SC_TIMERS = nil
  SC_TRACE = nil
  SC_TRACE_EVENT_FILTER = nil
  SC_TRACE_EVENT_NAME_MAX = nil
  SC_TRACE_INHERIT = nil
  SC_TRACE_LOG = nil
  SC_TRACE_NAME_MAX = nil
  SC_TRACE_SYS_MAX = nil
  SC_TRACE_USER_EVENT_MAX = nil
  SC_TYPED_MEMORY_OBJECTS = nil
  SC_VERSION = nil
  SC_V6_ILP32_OFF32 = nil
  SC_V6_ILP32_OFFBIG = nil
  SC_V6_LP64_OFF64 = nil
  SC_V6_LPBIG_OFFBIG = nil
  SC_2_C_BIND = nil
  SC_2_C_DEV = nil
  SC_2_CHAR_TERM = nil
  SC_2_FORT_DEV = nil
  SC_2_FORT_RUN = nil
  SC_2_LOCALEDEF = nil
  SC_2_PBS = nil
  SC_2_PBS_ACCOUNTING = nil
  SC_2_PBS_CHECKPOINT = nil
  SC_2_PBS_LOCATE = nil
  SC_2_PBS_MESSAGE = nil
  SC_2_PBS_TRACK = nil
  SC_2_SW_DEV = nil
  SC_2_UPE = nil
  SC_2_VERSION = nil
  SC_PAGE_SIZE = nil
  SC_PAGESIZE = nil
  SC_THREAD_DESTRUCTOR_ITERATIONS = nil
  SC_THREAD_KEYS_MAX = nil
  SC_THREAD_STACK_MIN = nil
  SC_THREAD_THREADS_MAX = nil
  SC_RE_DUP_MAX = nil
  SC_RTSIG_MAX = nil
  SC_SEM_NSEMS_MAX = nil
  SC_SEM_VALUE_MAX = nil
  SC_SIGQUEUE_MAX = nil
  SC_STREAM_MAX = nil
  SC_SYMLOOP_MAX = nil
  SC_TIMER_MAX = nil
  SC_TTY_NAME_MAX = nil
  SC_TZNAME_MAX = nil
  SC_XOPEN_CRYPT = nil
  SC_XOPEN_ENH_I18N = nil
  SC_XOPEN_REALTIME = nil
  SC_XOPEN_REALTIME_THREADS = nil
  SC_XOPEN_SHM = nil
  SC_XOPEN_STREAMS = nil
  SC_XOPEN_UNIX = nil
  SC_XOPEN_VERSION = nil
  SC_PHYS_PAGES = nil
  SC_NPROCESSORS_CONF = nil
  SC_NPROCESSORS_ONLN = nil
  CS_PATH = nil
  CS_POSIX_V6_ILP32_OFF32_CFLAGS = nil
  CS_POSIX_V6_ILP32_OFF32_LDFLAGS = nil
  CS_POSIX_V6_ILP32_OFF32_LIBS = nil
  CS_POSIX_V6_ILP32_OFFBIG_CFLAGS = nil
  CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS = nil
  CS_POSIX_V6_ILP32_OFFBIG_LIBS = nil
  CS_POSIX_V6_LP64_OFF64_CFLAGS = nil
  CS_POSIX_V6_LP64_OFF64_LDFLAGS = nil
  CS_POSIX_V6_LP64_OFF64_LIBS = nil
  CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS = nil
  CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS = nil
  CS_POSIX_V6_LPBIG_OFFBIG_LIBS = nil
  CS_POSIX_V6_WIDTH_RESTRICTED_ENVS = nil
  PC_FILESIZEBITS = nil
  PC_LINK_MAX = nil
  PC_MAX_CANON = nil
  PC_MAX_INPUT = nil
  PC_NAME_MAX = nil
  PC_PATH_MAX = nil
  PC_PIPE_BUF = nil
  PC_2_SYMLINKS = nil
  PC_ALLOC_SIZE_MIN = nil
  PC_REC_INCR_XFER_SIZE = nil
  PC_REC_MAX_XFER_SIZE = nil
  PC_REC_MIN_XFER_SIZE = nil
  PC_REC_XFER_ALIGN = nil
  PC_SYMLINK_MAX = nil
  PC_CHOWN_RESTRICTED = nil
  PC_NO_TRUNC = nil
  PC_VDISABLE = nil
  PC_ASYNC_IO = nil
  PC_PRIO_IO = nil
  PC_SYNC_IO = nil
  SC_AIO_LISTIO_MAX = nil
  SC_AIO_MAX = nil
  SC_AIO_PRIO_DELTA_MAX = nil
  SC_ARG_MAX = nil
  SC_ATEXIT_MAX = nil
  SC_BC_BASE_MAX = nil
  SC_BC_DIM_MAX = nil
  SC_BC_SCALE_MAX = nil
  SC_BC_STRING_MAX = nil
  SC_CHILD_MAX = nil
  SC_CLK_TCK = nil
  SC_COLL_WEIGHTS_MAX = nil
  SC_DELAYTIMER_MAX = nil
  SC_EXPR_NEST_MAX = nil
  SC_HOST_NAME_MAX = nil
  SC_IOV_MAX = nil
  SC_LINE_MAX = nil
  SC_LOGIN_NAME_MAX = nil
  SC_NGROUPS_MAX = nil
  SC_GETGR_R_SIZE_MAX = nil
  SC_GETPW_R_SIZE_MAX = nil
  SC_MQ_OPEN_MAX = nil
  SC_MQ_PRIO_MAX = nil
  SC_OPEN_MAX = nil
  SC_ADVISORY_INFO = nil
  SC_BARRIERS = nil
  SC_ASYNCHRONOUS_IO = nil
  SC_CLOCK_SELECTION = nil
  SC_CPUTIME = nil
  SC_FSYNC = nil
  SC_IPV6 = nil
  SC_JOB_CONTROL = nil
  SC_MAPPED_FILES = nil
  SC_MEMLOCK = nil
  SC_MEMLOCK_RANGE = nil
  SC_MEMORY_PROTECTION = nil
  SC_MESSAGE_PASSING = nil
  SC_MONOTONIC_CLOCK = nil
  SC_PRIORITIZED_IO = nil
  SC_PRIORITY_SCHEDULING = nil
  SC_RAW_SOCKETS = nil
  SC_READER_WRITER_LOCKS = nil
  SC_REALTIME_SIGNALS = nil
  SC_REGEXP = nil
end

class Etc::Passwd < Struct
end

class Etc::Group < Struct
end

class RegexpError < StandardError
end

class IndexError < StandardError
end

class StopIteration < IndexError
end

class RubyVM < Object
  OPTS = nil
  INSTRUCTION_NAMES = nil
  DEFAULT_PARAMS = nil
end

class RubyVM::InstructionSequence < Object
end

class Regexp < Object
  IGNORECASE = (nil : Integer)
  EXTENDED = (nil : Integer)
  MULTILINE = (nil : Integer)
  FIXEDENCODING = (nil : Integer)
  NOENCODING = (nil : Integer)

  def initialize(String pattern, (Integer | Boolean | nil) options = nil) => nil; end

  def match(String str, Integer pos = 0) => ~MatchData; end

  def ===(String str) => Boolean; end
end

class StdlibDumper < Object
  CYCLIC_DEPENDENCIES = nil
  DEFERRED_INCLUDES = nil
end

class TracePoint < Object
end

class MatchData < Object
end

class Addrinfo < Data
end

class Socket < BasicSocket
  SOCK_STREAM = nil
  SOCK_DGRAM = nil
  SOCK_RAW = nil
  SOCK_RDM = nil
  SOCK_SEQPACKET = nil
  AF_UNSPEC = nil
  PF_UNSPEC = nil
  PF_INET = nil
  PF_INET6 = nil
  PF_UNIX = nil
  AF_IPX = nil
  PF_IPX = nil
  AF_APPLETALK = nil
  PF_APPLETALK = nil
  AF_LOCAL = nil
  PF_LOCAL = nil
  AF_IMPLINK = nil
  PF_IMPLINK = nil
  AF_PUP = nil
  PF_PUP = nil
  AF_CHAOS = nil
  PF_CHAOS = nil
  AF_NS = nil
  PF_NS = nil
  AF_ISO = nil
  PF_ISO = nil
  AF_OSI = nil
  PF_OSI = nil
  AF_ECMA = nil
  PF_ECMA = nil
  PF_DATAKIT = nil
  AF_CCITT = nil
  AF_DATAKIT = nil
  PF_CCITT = nil
  PF_SNA = nil
  AF_DLI = nil
  AF_SNA = nil
  PF_DLI = nil
  PF_LAT = nil
  AF_HYLINK = nil
  AF_LAT = nil
  PF_HYLINK = nil
  PF_ROUTE = nil
  AF_LINK = nil
  AF_ROUTE = nil
  PF_LINK = nil
  PF_COIP = nil
  AF_CNT = nil
  AF_COIP = nil
  PF_CNT = nil
  PF_SIP = nil
  AF_NDRV = nil
  AF_SIP = nil
  PF_NDRV = nil
  PF_ISDN = nil
  AF_NATM = nil
  AF_ISDN = nil
  PF_NATM = nil
  PF_SYSTEM = nil
  AF_NETBIOS = nil
  AF_SYSTEM = nil
  PF_NETBIOS = nil
  PF_PPP = nil
  AF_MAX = nil
  AF_PPP = nil
  PF_MAX = nil
  PF_XTP = nil
  PF_RTIP = nil
  AF_E164 = nil
  PF_PIP = nil
  MSG_OOB = nil
  MSG_PEEK = nil
  MSG_DONTROUTE = nil
  MSG_EOR = nil
  MSG_TRUNC = nil
  MSG_CTRUNC = nil
  MSG_WAITALL = nil
  MSG_DONTWAIT = nil
  MSG_EOF = nil
  PF_KEY = nil
  MSG_FLUSH = nil
  MSG_HOLD = nil
  MSG_SEND = nil
  MSG_HAVEMORE = nil
  MSG_RCVMORE = nil
  SOL_SOCKET = nil
  IPPROTO_IP = nil
  IPPROTO_ICMP = nil
  IPPROTO_IGMP = nil
  IPPROTO_GGP = nil
  IPPROTO_TCP = nil
  IPPROTO_EGP = nil
  IPPROTO_PUP = nil
  IPPROTO_IDP = nil
  IPPROTO_HELLO = nil
  IPPROTO_UDP = nil
  IPPROTO_ND = nil
  IPPROTO_TP = nil
  IPPROTO_EON = nil
  IPPROTO_AH = nil
  IPPROTO_XTP = nil
  IPPROTO_ESP = nil
  IPPROTO_FRAGMENT = nil
  IPPROTO_DSTOPTS = nil
  IPPROTO_ICMPV6 = nil
  IPPROTO_IPV6 = nil
  IPPROTO_HOPOPTS = nil
  IPPROTO_ROUTING = nil
  IPPROTO_RAW = nil
  IPPROTO_NONE = nil
  IPPORT_RESERVED = nil
  IPPORT_USERRESERVED = nil
  IPPROTO_MAX = nil
  INADDR_BROADCAST = nil
  INADDR_LOOPBACK = nil
  INADDR_ANY = nil
  INADDR_ALLHOSTS_GROUP = nil
  INADDR_MAX_LOCAL_GROUP = nil
  INADDR_UNSPEC_GROUP = nil
  IP_OPTIONS = nil
  IP_HDRINCL = nil
  INADDR_NONE = nil
  IP_TTL = nil
  IP_RECVOPTS = nil
  IP_TOS = nil
  IP_RECVDSTADDR = nil
  IP_RETOPTS = nil
  IP_RECVRETOPTS = nil
  IP_RECVIF = nil
  IP_PORTRANGE = nil
  IP_RECVTTL = nil
  IP_MULTICAST_TTL = nil
  IP_MULTICAST_LOOP = nil
  IP_MULTICAST_IF = nil
  IP_DROP_MEMBERSHIP = nil
  IP_DEFAULT_MULTICAST_TTL = nil
  IP_ADD_MEMBERSHIP = nil
  IP_MAX_MEMBERSHIPS = nil
  IP_PKTINFO = nil
  IP_DEFAULT_MULTICAST_LOOP = nil
  IP_UNBLOCK_SOURCE = nil
  IP_BLOCK_SOURCE = nil
  IP_IPSEC_POLICY = nil
  IP_DROP_SOURCE_MEMBERSHIP = nil
  IP_MSFILTER = nil
  IP_ADD_SOURCE_MEMBERSHIP = nil
  MCAST_BLOCK_SOURCE = nil
  MCAST_UNBLOCK_SOURCE = nil
  MCAST_JOIN_GROUP = nil
  MCAST_JOIN_SOURCE_GROUP = nil
  MCAST_LEAVE_SOURCE_GROUP = nil
  MCAST_LEAVE_GROUP = nil
  MCAST_INCLUDE = nil
  SO_DEBUG = nil
  MCAST_EXCLUDE = nil
  SO_REUSEPORT = nil
  SO_TYPE = nil
  SO_REUSEADDR = nil
  SO_DONTROUTE = nil
  SO_BROADCAST = nil
  SO_ERROR = nil
  SO_RCVBUF = nil
  SO_KEEPALIVE = nil
  SO_SNDBUF = nil
  SO_OOBINLINE = nil
  SO_LINGER = nil
  SO_RCVLOWAT = nil
  SO_SNDLOWAT = nil
  SO_RCVTIMEO = nil
  SO_SNDTIMEO = nil
  SO_ACCEPTCONN = nil
  SO_USELOOPBACK = nil
  SO_DONTTRUNC = nil
  SO_WANTMORE = nil
  SO_WANTOOBFLAG = nil
  SO_NREAD = nil
  SO_NKE = nil
  SO_NOSIGPIPE = nil
  SO_TIMESTAMP = nil
  TCP_NODELAY = nil
  TCP_MAXSEG = nil
  TCP_KEEPCNT = nil
  TCP_KEEPINTVL = nil
  TCP_NOOPT = nil
  TCP_NOPUSH = nil
  TCP_FASTOPEN = nil
  EAI_ADDRFAMILY = nil
  EAI_AGAIN = nil
  EAI_BADFLAGS = nil
  EAI_FAIL = nil
  EAI_FAMILY = nil
  EAI_MEMORY = nil
  EAI_NODATA = nil
  EAI_NONAME = nil
  EAI_OVERFLOW = nil
  EAI_SERVICE = nil
  EAI_SOCKTYPE = nil
  EAI_SYSTEM = nil
  EAI_BADHINTS = nil
  EAI_PROTOCOL = nil
  EAI_MAX = nil
  AI_CANONNAME = nil
  AI_NUMERICHOST = nil
  AI_NUMERICSERV = nil
  AI_MASK = nil
  AI_ALL = nil
  AI_V4MAPPED_CFG = nil
  AI_ADDRCONFIG = nil
  AI_V4MAPPED = nil
  AI_DEFAULT = nil
  NI_MAXHOST = nil
  NI_MAXSERV = nil
  NI_NOFQDN = nil
  NI_NUMERICHOST = nil
  NI_NAMEREQD = nil
  NI_NUMERICSERV = nil
  NI_DGRAM = nil
  SHUT_RD = nil
  SHUT_WR = nil
  SHUT_RDWR = nil
  IPV6_JOIN_GROUP = nil
  IPV6_LEAVE_GROUP = nil
  IPV6_MULTICAST_HOPS = nil
  IPV6_MULTICAST_IF = nil
  IPV6_MULTICAST_LOOP = nil
  IPV6_UNICAST_HOPS = nil
  IPV6_CHECKSUM = nil
  IPV6_DONTFRAG = nil
  IPV6_DSTOPTS = nil
  IPV6_HOPLIMIT = nil
  IPV6_HOPOPTS = nil
  IPV6_NEXTHOP = nil
  IPV6_PATHMTU = nil
  IPV6_RECVDSTOPTS = nil
  IPV6_RECVHOPLIMIT = nil
  IPV6_RECVHOPOPTS = nil
  IPV6_RECVRTHDR = nil
  IPV6_RECVTCLASS = nil
  IPV6_RTHDR = nil
  IPV6_RTHDRDSTOPTS = nil
  IPV6_RTHDR_TYPE_0 = nil
  IPV6_RECVPATHMTU = nil
  IPV6_TCLASS = nil
  IPV6_USE_MIN_MTU = nil
  INET_ADDRSTRLEN = nil
  INET6_ADDRSTRLEN = nil
  IFNAMSIZ = nil
  IF_NAMESIZE = nil
  SCM_RIGHTS = nil
  SCM_TIMESTAMP = nil
  SCM_CREDS = nil
  LOCAL_PEERCRED = nil
  IFF_ALLMULTI = nil
  IFF_ALTPHYS = nil
  IFF_BROADCAST = nil
  IFF_DEBUG = nil
  IFF_LINK0 = nil
  IFF_LINK1 = nil
  IFF_LINK2 = nil
  IFF_LOOPBACK = nil
  IFF_MULTICAST = nil
  IFF_NOARP = nil
  IFF_NOTRAILERS = nil
  IFF_OACTIVE = nil
  IFF_POINTOPOINT = nil
  IFF_PROMISC = nil
  IFF_RUNNING = nil
  IFF_SIMPLEX = nil
  IFF_UP = nil
  SOMAXCONN = nil
  AF_INET = nil
  AF_INET6 = nil
  AF_UNIX = nil
  IPV6_V6ONLY = nil
  AI_PASSIVE = nil
  IPV6_RECVPKTINFO = nil
  IPV6_PKTINFO = nil
end

class Socket::Option < Object
end

class Socket::Ifaddr < Data
end

module Socket::Constants
  SOCK_STREAM = nil
  SOCK_DGRAM = nil
  SOCK_RAW = nil
  SOCK_RDM = nil
  SOCK_SEQPACKET = nil
  AF_UNSPEC = nil
  PF_UNSPEC = nil
  PF_INET = nil
  PF_INET6 = nil
  PF_UNIX = nil
  AF_IPX = nil
  PF_IPX = nil
  AF_APPLETALK = nil
  PF_APPLETALK = nil
  AF_LOCAL = nil
  PF_LOCAL = nil
  AF_IMPLINK = nil
  PF_IMPLINK = nil
  AF_PUP = nil
  PF_PUP = nil
  AF_CHAOS = nil
  PF_CHAOS = nil
  AF_NS = nil
  PF_NS = nil
  AF_ISO = nil
  PF_ISO = nil
  AF_OSI = nil
  PF_OSI = nil
  AF_ECMA = nil
  PF_ECMA = nil
  AF_DATAKIT = nil
  PF_DATAKIT = nil
  AF_CCITT = nil
  PF_CCITT = nil
  AF_SNA = nil
  PF_SNA = nil
  AF_DLI = nil
  PF_DLI = nil
  AF_LAT = nil
  PF_LAT = nil
  AF_HYLINK = nil
  PF_HYLINK = nil
  AF_ROUTE = nil
  PF_ROUTE = nil
  AF_LINK = nil
  PF_LINK = nil
  AF_COIP = nil
  PF_COIP = nil
  AF_CNT = nil
  PF_CNT = nil
  AF_SIP = nil
  PF_SIP = nil
  AF_NDRV = nil
  PF_NDRV = nil
  AF_ISDN = nil
  PF_ISDN = nil
  AF_NATM = nil
  PF_NATM = nil
  AF_SYSTEM = nil
  PF_SYSTEM = nil
  AF_NETBIOS = nil
  PF_NETBIOS = nil
  AF_PPP = nil
  PF_PPP = nil
  AF_MAX = nil
  PF_MAX = nil
  AF_E164 = nil
  PF_XTP = nil
  PF_RTIP = nil
  PF_PIP = nil
  PF_KEY = nil
  MSG_OOB = nil
  MSG_PEEK = nil
  MSG_DONTROUTE = nil
  MSG_EOR = nil
  MSG_TRUNC = nil
  MSG_CTRUNC = nil
  MSG_WAITALL = nil
  MSG_DONTWAIT = nil
  MSG_EOF = nil
  MSG_FLUSH = nil
  MSG_HOLD = nil
  MSG_SEND = nil
  MSG_HAVEMORE = nil
  MSG_RCVMORE = nil
  SOL_SOCKET = nil
  IPPROTO_IP = nil
  IPPROTO_ICMP = nil
  IPPROTO_IGMP = nil
  IPPROTO_GGP = nil
  IPPROTO_TCP = nil
  IPPROTO_EGP = nil
  IPPROTO_PUP = nil
  IPPROTO_UDP = nil
  IPPROTO_IDP = nil
  IPPROTO_HELLO = nil
  IPPROTO_ND = nil
  IPPROTO_TP = nil
  IPPROTO_XTP = nil
  IPPROTO_EON = nil
  IPPROTO_AH = nil
  IPPROTO_DSTOPTS = nil
  IPPROTO_ESP = nil
  IPPROTO_FRAGMENT = nil
  IPPROTO_HOPOPTS = nil
  IPPROTO_ICMPV6 = nil
  IPPROTO_IPV6 = nil
  IPPROTO_NONE = nil
  IPPROTO_ROUTING = nil
  IPPROTO_RAW = nil
  IPPROTO_MAX = nil
  IPPORT_RESERVED = nil
  IPPORT_USERRESERVED = nil
  INADDR_ANY = nil
  INADDR_BROADCAST = nil
  INADDR_LOOPBACK = nil
  INADDR_UNSPEC_GROUP = nil
  INADDR_ALLHOSTS_GROUP = nil
  INADDR_MAX_LOCAL_GROUP = nil
  INADDR_NONE = nil
  IP_OPTIONS = nil
  IP_HDRINCL = nil
  IP_TOS = nil
  IP_TTL = nil
  IP_RECVOPTS = nil
  IP_RECVRETOPTS = nil
  IP_RECVDSTADDR = nil
  IP_RETOPTS = nil
  IP_RECVTTL = nil
  IP_RECVIF = nil
  IP_PORTRANGE = nil
  IP_MULTICAST_IF = nil
  IP_MULTICAST_TTL = nil
  IP_MULTICAST_LOOP = nil
  IP_ADD_MEMBERSHIP = nil
  IP_DROP_MEMBERSHIP = nil
  IP_DEFAULT_MULTICAST_TTL = nil
  IP_DEFAULT_MULTICAST_LOOP = nil
  IP_MAX_MEMBERSHIPS = nil
  IP_PKTINFO = nil
  IP_IPSEC_POLICY = nil
  IP_UNBLOCK_SOURCE = nil
  IP_BLOCK_SOURCE = nil
  IP_ADD_SOURCE_MEMBERSHIP = nil
  IP_DROP_SOURCE_MEMBERSHIP = nil
  IP_MSFILTER = nil
  MCAST_JOIN_GROUP = nil
  MCAST_BLOCK_SOURCE = nil
  MCAST_UNBLOCK_SOURCE = nil
  MCAST_LEAVE_GROUP = nil
  MCAST_JOIN_SOURCE_GROUP = nil
  MCAST_LEAVE_SOURCE_GROUP = nil
  MCAST_EXCLUDE = nil
  MCAST_INCLUDE = nil
  SO_DEBUG = nil
  SO_REUSEADDR = nil
  SO_REUSEPORT = nil
  SO_TYPE = nil
  SO_ERROR = nil
  SO_DONTROUTE = nil
  SO_BROADCAST = nil
  SO_SNDBUF = nil
  SO_RCVBUF = nil
  SO_KEEPALIVE = nil
  SO_OOBINLINE = nil
  SO_LINGER = nil
  SO_RCVLOWAT = nil
  SO_SNDLOWAT = nil
  SO_RCVTIMEO = nil
  SO_SNDTIMEO = nil
  SO_ACCEPTCONN = nil
  SO_USELOOPBACK = nil
  SO_DONTTRUNC = nil
  SO_WANTMORE = nil
  SO_WANTOOBFLAG = nil
  SO_NREAD = nil
  SO_NKE = nil
  SO_NOSIGPIPE = nil
  SO_TIMESTAMP = nil
  TCP_NODELAY = nil
  TCP_MAXSEG = nil
  TCP_KEEPCNT = nil
  TCP_KEEPINTVL = nil
  TCP_NOOPT = nil
  TCP_NOPUSH = nil
  TCP_FASTOPEN = nil
  EAI_ADDRFAMILY = nil
  EAI_AGAIN = nil
  EAI_BADFLAGS = nil
  EAI_FAIL = nil
  EAI_FAMILY = nil
  EAI_MEMORY = nil
  EAI_NODATA = nil
  EAI_NONAME = nil
  EAI_OVERFLOW = nil
  EAI_SERVICE = nil
  EAI_SOCKTYPE = nil
  EAI_SYSTEM = nil
  EAI_BADHINTS = nil
  EAI_PROTOCOL = nil
  EAI_MAX = nil
  AI_CANONNAME = nil
  AI_NUMERICHOST = nil
  AI_NUMERICSERV = nil
  AI_MASK = nil
  AI_ALL = nil
  AI_V4MAPPED_CFG = nil
  AI_ADDRCONFIG = nil
  AI_V4MAPPED = nil
  AI_DEFAULT = nil
  NI_MAXHOST = nil
  NI_MAXSERV = nil
  NI_NOFQDN = nil
  NI_NUMERICHOST = nil
  NI_NAMEREQD = nil
  NI_NUMERICSERV = nil
  NI_DGRAM = nil
  SHUT_RD = nil
  SHUT_WR = nil
  SHUT_RDWR = nil
  IPV6_JOIN_GROUP = nil
  IPV6_LEAVE_GROUP = nil
  IPV6_MULTICAST_HOPS = nil
  IPV6_MULTICAST_IF = nil
  IPV6_MULTICAST_LOOP = nil
  IPV6_UNICAST_HOPS = nil
  IPV6_CHECKSUM = nil
  IPV6_DONTFRAG = nil
  IPV6_DSTOPTS = nil
  IPV6_HOPLIMIT = nil
  IPV6_HOPOPTS = nil
  IPV6_NEXTHOP = nil
  IPV6_PATHMTU = nil
  IPV6_RECVDSTOPTS = nil
  IPV6_RECVHOPLIMIT = nil
  IPV6_RECVHOPOPTS = nil
  IPV6_RECVRTHDR = nil
  IPV6_RECVTCLASS = nil
  IPV6_RTHDR = nil
  IPV6_RTHDRDSTOPTS = nil
  IPV6_RTHDR_TYPE_0 = nil
  IPV6_RECVPATHMTU = nil
  IPV6_TCLASS = nil
  IPV6_USE_MIN_MTU = nil
  INET_ADDRSTRLEN = nil
  INET6_ADDRSTRLEN = nil
  IFNAMSIZ = nil
  IF_NAMESIZE = nil
  SCM_RIGHTS = nil
  SCM_TIMESTAMP = nil
  SCM_CREDS = nil
  LOCAL_PEERCRED = nil
  IFF_ALLMULTI = nil
  IFF_ALTPHYS = nil
  IFF_BROADCAST = nil
  IFF_DEBUG = nil
  IFF_LINK0 = nil
  IFF_LINK1 = nil
  IFF_LINK2 = nil
  IFF_LOOPBACK = nil
  IFF_MULTICAST = nil
  IFF_NOARP = nil
  IFF_NOTRAILERS = nil
  IFF_OACTIVE = nil
  IFF_POINTOPOINT = nil
  IFF_PROMISC = nil
  IFF_RUNNING = nil
  IFF_SIMPLEX = nil
  IFF_UP = nil
  SOMAXCONN = nil
  AF_INET = nil
  AF_INET6 = nil
  AF_UNIX = nil
  IPV6_V6ONLY = nil
  AI_PASSIVE = nil
  IPV6_RECVPKTINFO = nil
  IPV6_PKTINFO = nil
end

class Socket::AncillaryData < Object
end

class Socket::UDPSource < Object
end

class ThreadGroup < Object
  Default = nil
end

class Dir < Object
  include Enumerable

  def self.[](String pattern) => [String]; end
end

class ThreadError < StandardError
end

class ClosedQueueError < StopIteration
end

module Fcntl
  F_DUPFD = nil
  F_GETFD = nil
  F_GETLK = nil
  F_SETFD = nil
  F_GETFL = nil
  F_SETFL = nil
  F_SETLK = nil
  F_SETLKW = nil
  FD_CLOEXEC = nil
  F_RDLCK = nil
  F_UNLCK = nil
  F_WRLCK = nil
  O_CREAT = nil
  O_EXCL = nil
  O_NOCTTY = nil
  O_TRUNC = nil
  O_APPEND = nil
  O_NONBLOCK = nil
  O_NDELAY = nil
  O_RDONLY = nil
  O_RDWR = nil
  O_WRONLY = nil
  O_ACCMODE = nil
end

class SocketError < StandardError
end

class Time < Object
  include Comparable

  def self.now => :instance
  end
end

module Marshal
  MAJOR_VERSION = nil
  MINOR_VERSION = nil
end

class Range::[BeginType, EndType] < Object
  include Enumerable
end

class IOError < StandardError
end

class EOFError < IOError
end

class Random < Object
  DEFAULT = nil
end

module Random::Formatter
end

class Random
  include Random::Formatter
end

module Signal
end

class Symbol < Object
  include Comparable
end

class SystemExit < Exception
end

class Proc < Object
  alias :[] :call
end

class Module < Object
  def name => String
  end
end

class Class < Module
  def ===(Object instance) => Boolean; end

  def allocate => :instance; end
end

class LocalJumpError < StandardError
end

class SignalException < Exception
end

class Interrupt < SignalException
end

class TypeError < StandardError
end

class KeyError < IndexError
end

class ScriptError < Exception
end

class SyntaxError < ScriptError
end

class NotImplementedError < ScriptError
end

class NameError < StandardError
end

class SystemStackError < Exception
end

class NoMethodError < NameError
end

class RuntimeError < StandardError
end

class SecurityError < Exception
end

class NoMemoryError < Exception
end

class LoadError < ScriptError
end

class UnboundMethod < Object
end

module Warning
end

class Binding < Object
end

module Math
  PI = nil
  E = nil
end

class Math::DomainError < StandardError
end

class UDPSocket < IPSocket
end

