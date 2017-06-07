class ERB
  def initialize(String src, ~Integer safe_level = nil, ~String trim_mode = nil, String eoutvar = "_erbout") => nil; end

  def result(Binding binding = TOPLEVEL_BINDING) => String; end
end
