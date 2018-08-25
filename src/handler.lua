host = "0.0.0.0"
if ctx.msg.host then
    host = ctx.msg.host
end

message = "Host: " .. host .. "\n" .. ctx.msg.req_line .. "\n\nHTTP headers:\n"

for k, v in pairs(ctx.msg.headers) do
    message = message .. k .. ": " .. v .. "\n"
end

message = message .. "\nRequest body:\n" .. ctx.msg.body

print(message)

return "<html><head><title>Hello " ..
    host ..
    "</title></head><body>Hello " ..
    host ..
    "</body></html>"
