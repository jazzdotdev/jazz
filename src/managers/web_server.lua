local req = ctx.msg -- get the request
local possibleResponse

require "package_loader"

utils.try(function()
    events["reqProcess"]:trigger(req) -- try to process request and give response
    for k, v in pairs(rules) do -- rule trigger
        v.rule(req, events)
    end

end, function(err)
    print("[ERROR]")
    print(err)
    response = { 
        status = 500,
         body = '{ "error": ' .. "try-catch error" .. ' }',  -- if something go wrong give error 500 in response
    }
end)

return torchbear_response