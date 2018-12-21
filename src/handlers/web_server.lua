-- Set the default response
torchbear.response = nil

-- Declare the request
local request = ctx.msg

xpcall(function ()

  -- Returned response
  local response = torchbear.handler(ctx.msg)

  -- The returned response from the handler takes precedence over whatever was set before
  if response then
    torchbear.response = response
  end
end, function (msg)
  msg = tostring(msg)
  
  local trace = debug.traceback(msg, 3)
  _log.error(trace)

  -- In case the handler errors, return the trace with http status 500 (Error)
  torchbear.response = { 
    status = 500,
    body = trace
  }
end)

-- The returned values from this handler is the response
return torchbear.response
