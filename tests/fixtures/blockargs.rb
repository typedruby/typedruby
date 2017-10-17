
def timeout[T](Numeric sec, ~Class::[Exception] klass = nil, ~String message = nil, { || => T } &) => T; end

def timeout2(Numeric sec, Numeric klass, { || => nil } &) => nil; end
def timeout3(sec, klass, &); end
def timeout4(sec, klass, &blockname); end
def timeout5(Numeric sec, { || => nil} &blockname) => nil; end
def timeout6({ || => nil}  &) => nil; end
def timeout7(&); end
def timeout8({ || => nil } &blockname) => nil; end
def timeout9(&blockname); end