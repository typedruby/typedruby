Gem::Specification.new do |s|
  s.name = "typedruby"
  s.version = ENV.fetch("TYPEDRUBY_VERSION")
  s.summary = "Gradual static typing for Ruby"
  s.description = "Gradual static typing for Ruby."
  s.authors = ["Charlie Somerville"]
  s.email = ["charlie@github.com", "opensource+typedruby@github.com"]
  s.homepage = "https://github.com/typedruby/typedruby"
  s.license = "MIT"

  s.files = Dir["**/*"]

  s.executables << "typedruby"
end
