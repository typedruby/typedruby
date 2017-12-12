class BigDecimal < Numeric
  def initialize((Integer|Float|Rational|BigDecimal|String) initial, Integer digits = 0) => nil; end

  def +(Numeric other) => BigDecimal; end
  def -(Numeric other) => BigDecimal; end
  def /(Numeric other) => BigDecimal; end
  def *(Numeric other) => BigDecimal; end

  def round(Integer n = 0, (Integer|Symbol) mode = 0) => BigDecimal; end

  def to_r => Rational; end
end
