-- The debug library is unpredictable in some cases,
-- so we only include the safe parts.

-- Modify the table itself instead of setting the
-- global field, because debug can also be required.

local to_remove = {}

for k, _ in pairs(debug) do
    if  k ~= "traceback"
    and k ~= "getinfo"
    then
        table.insert(to_remove, k)
    end
end

for _, k in ipairs(to_remove) do
    debug[k] = nil
end