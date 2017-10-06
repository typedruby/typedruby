def main => Boolean
  a = true

  tap do |_|
    if rand < 0.5
      a = false
    end
  end

  a
end
