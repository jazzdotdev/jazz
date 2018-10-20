-- declare request and possible response
local request = ctx.msg
local possible_response
local log = require "log"

require "package_loader"

-- try to trigger every rule. If the process failed give error 500
utils.try(function()
    for k, v in pairs(rules) do
        v.rule(request, events)
    end

end, function(err)
    log.error(err)
    response = { 
        status = 500,
         body = '{ "error": ' .. "try-catch error" .. ' }',
    }
end)

return returned_response

--- Rules priorities algorithm ---
--- 1. 1st line of rule file would be its priority in yaml
--- 2. Add this priority to rules table somehow
--- 3. Sort rules table from high to low
--- 4. Invoke it in proper order