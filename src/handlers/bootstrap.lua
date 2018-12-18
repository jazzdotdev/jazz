return xpcall(function ()
  local fn, err = loadfile(torchbear.bootstrap)
  if not fn then error(err) end
  fn()
end, function (msg)
    msg = tostring(msg)
    local trace = debug.traceback(msg, 3)
    log.error(trace)
end)