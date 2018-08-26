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

if ctx.msg.path:match("/file/.*") then
    file_path = string.gsub(ctx.msg.path, "^/file/", "")
    file = io.open(file_path, "r")
    file_content = file:read("*all")
    file:close()

    return file_content
else
    return "<html><head><title>Hello " ..
            host ..
            "</title></head><body>Hello " ..
            host ..
            "</body></html>"
end

