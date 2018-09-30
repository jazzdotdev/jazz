
_G.utils = require "utils"
_G.luvent = require "Luvent"
_G.fs = require "fs"

require "table_ext"
require "underscore_alias"

function string:split(sep)
    local sep, fields = sep or ":", {}
    local pattern = string.format("([^%s]+)", sep)
    self:gsub(pattern, function(c) fields[#fields+1] = c end)
    return fields
end

print("Hello Torchbear!")
