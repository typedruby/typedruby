class A
  def untyped_with_stub(x)
    x + 1
  end

  def typed(String y) => String
    untyped_with_stub(y)
  end
end
