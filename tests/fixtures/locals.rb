def test_rebind => nil
  a = 123
  a = "foo"
  a
end

def test_pin => nil
  a = 123
  tap do |_| a end
  a = "foo"
  a
end

def test_union => nil
  if rand < 0.5
    a = 123
  else
    a = "foo"
  end

  a
end

def test_union2 => nil
  a = 123

  if rand < 0.5
    a = "foo"
  end

  a
end

def test_conditionally_pinned => nil
  if rand < 0.5
    a = 123
    tap do |_| a end
  end

  a
end

def test_conditionally_pinned2 => nil
  a = 123

  if rand < 0.5
    tap do |_| a end
  end

  a
end

def test_rescue_uncertainty => nil
  begin
    a = 123
    a = "foo"
  rescue
  end

  a
end

def test_rescue_refine => nil
  begin
    a = 123
    a = "foo"
  rescue
  end

  if a.is_a?(Integer)
    return a
  end
end

def test_pin_uncertain => nil
  begin
    a = 123
    a = "foo"
    if rand < 0.5
      tap do |_|
        a
      end
    end
  rescue
  end

  a
end

def test_pin_parent_through_merge => nil
  a = 123

  tap do |_|
    tap do |_|
      if rand < 0.5
        if rand < 0.5
          nil
        else
          a
        end
      else
        nil
      end
    end
  end

  a = "foo"

  nil
end
