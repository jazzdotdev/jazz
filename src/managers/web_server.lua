-- declare request
local request = ctx.msg
local log = require "log"

require "package_loader"

-- try to trigger every rule. If the process failed give error 500 and log the trace

local trace

local status = xpcall(function()
  events["incoming_request_received"]:trigger(request)
  for k, v in pairs(rules) do
    v.rule(request, events)
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