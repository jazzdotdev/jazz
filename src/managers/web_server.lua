-- declare request and possible response
local request = ctx.msg
local possible_response

require "package_loader"

-- try to trigger every rule. If the process failed give error 500
utils.try(function()
    for k, v in pairs(rules) do 
        v.rule(request, events)
    end

end, function(err)
    log.err(err)
    response = { 
        status = 500,
         body = '{ "error": ' .. "try-catch error" .. ' }',
    }
end)

return returned_response