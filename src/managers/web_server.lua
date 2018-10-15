-- declare request
local request = ctx.msg
local log = require "log"

require "package_loader"

-- try to trigger every rule. If the process failed give error 500
utils.try(function()
    events["incoming_request_received"]:trigger(request)
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

return response