require "utils.underscore_alias"

_G.count = function (iter)
  return reduce(iter, 0, function (a, x) return a+1 end)
end

_G.sum = function (iter)
  return reduce(iter, 0, function (a, x) return a+x end)
end

-- Using sum+count would be two times slower and would consume the iterator
_G.mean = function (iter)
  local sum, count = 0, 0
  each(iter, function (x)
    sum = sum + x
    count = count + 1
  end)
  if count == 0 then
    return 0
  else
    return sum / count
  end
end

_G.count_pairs = function (iter)
  local n = 0
  for _ in pairs(iter) do
    n = n+1
  end
  return n
end