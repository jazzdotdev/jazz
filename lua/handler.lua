local debug = require "debug"
local fs = require "fs"

local req = ctx.msg

debug.print_req_info(req)

local uuid_pattern = "%x%x%x%x%x%x%x%x%-%x%x%x%x%-%x%x%x%x%-%x%x%x%x%-%x%x%x%x%x%x%x%x%x%x%x%x"

local function split_document(document_text, uuid, type)
    local yaml_text, body = document_text:match("(.*)\n\n(.*)")
    local yaml = yaml.load(yaml_text)
    local processed_body = body:gsub("\n", "\\n")

    local params = {
        uuid = uuid,
        type = type,
        title = yaml.title,
        body = processed_body,
        created = yaml.created or "",
        updated = yaml.updated or "",
    }

    return params
end

if req.path:match("/%a+/" .. uuid_pattern .. "/?") then
    -- /[type]/[uuid]
    local type, uuid = req.path:match("/(%a*)/(.*)")
    local file_content = fs.read_file("content/" .. uuid)
    local template_params = split_document(file_content, uuid, type)

    return {
        headers = {
            ["content-type"] = "application/json",
        },
        body = render("document.json", { document = template_params }),
    }
elseif req.path:match("/%a+/?") then
    -- /[type]
    local type = req.path:match("/(%a+)/?")
    local files = fs.get_all_files_in("content/")
    local documents = {}

    for _, file_name in ipairs(files) do
        local file_content = fs.read_file("content/" .. file_name)
        local template_params = split_document(file_content, file_name, type)

        if template_params.type == type then
            table.insert(documents, template_params)
        end
    end

    return {
        headers = {
            ["content-type"] = "application/json",
        },
        body = render("document-list.json", { documents = documents }),
    }
else
    local yaml_str = "one: { two: 3 }"

    local doc = yaml.load(yaml_str)
    print("Ser: ", yaml.dump(doc))
    print("Nested: ", doc.one.two)

    -- If render fails, the thrown error will be pretty confusing since actix_lua doesn't handle lua errors yet.
    -- pcall or xpcall can be used to intercept errors if needed.
    return {
        headers = {
            ["content-type"] = "text/html"
        },
        body = render("index.html", { host = req.host or "0.0.0.0" }),
    }
end
