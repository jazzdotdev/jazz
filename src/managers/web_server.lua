-- declare request and possible response
local request = ctx.msg
local possible_response
local log = require "log"

require "package_loader"

-- try to trigger every rule. If the process failed give error 500
utils.try(function()
    for k, v in pairs(rules) do
        -- v is rule require
        -- so we can create method in rule file that returns info about parameters/arguments
        -- with this info we could create object/table with all necessary parameters/arguments
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
--- 1. 1st line of rule file would be its weight in yaml
--- 2. Add this weight to rules table somehow
--- 3. Sort rules table from high to low
--- 4. Invoke it in proper order