# @typedruby

def test_csend(~String x, Object y) => nil
  # It works on normal nillable typed
  x&.length

  # nil-ability is propagated through &.
  x&.length + 1

  # Calling on known-non-nil types warns that the &. is unnecessary
  "hi"&.length

  # It works on Object, which may be nil, but is not an explicit union
  # including nil.
  y&.object_id

  nil
end
