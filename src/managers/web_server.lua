-- declare request
local request = ctx.msg
local log = require "log"

require "package_loader"

-- try to trigger every rule. If the process failed give error 500 and log the trace

local trace

local status = xpcall(function()
  events["incoming_request_received"]:trigger(request)
  for k, v in pairs(rules) do
    local rule_arguments = { }
    for k1, v1 in pairs(v.parameters) do
        if v1 == "events" then rule_arguments[v1] = events
        elseif v1 == "request" then rule_arguments[v1] = request
        -- elseif v1 == "parameter-name" then rule_arguments[v1] = parameter_value - this is how we add parameters to arugments table
        end
            
    end
    v.rule(rule_arguments)
  end
end, function (msg)
  local trace = debug.traceback(msg, 3)
  log.error(trace)
  response = { 
    status = 500,
    body = trace,
  }
end)

return response