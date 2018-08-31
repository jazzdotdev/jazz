local debug = require "debug"

debug.print_req_info(ctx.msg)

if ctx.msg.path:match("/file/.*") then
    local file_path = string.gsub(ctx.msg.path, "^/file/", "")
    local file = io.open(file_path, "r")
    local file_content = file:read("*all")
    file:close()

    return file_content
else
    local yaml_str = "one: { two: 3 }"

    local doc = yaml.load(yaml_str)
    print("Ser: ", yaml.dump(doc))
    print("Nested: ", doc.one.two)

    -- If render fails, the thrown error will be pretty confusing since actix_lua doesn't handle lua errors yet.
    -- pcall or xpcall can be used to intercept errors if needed.
    return render("index.html", { host = ctx.msg.host or "0.0.0.0" })
end
