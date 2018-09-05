local debug = require "debug"
local fs = require "fs"

local req = ctx.msg

debug.print_req_info(req)

local uuid_pattern = "%x%x%x%x%x%x%x%x%-%x%x%x%x%-%x%x%x%x%-%x%x%x%x%-%x%x%x%x%x%x%x%x%x%x%x%x"

local function split_document(document_text, id)
    local yaml_text, body = document_text:match("(.*)\n\n(.*)")
    local yaml = yaml.load(yaml_text)
    local processed_body = body:gsub("\n", "\\n")

    local params = {
        uuid = id,
        type = yaml.type,
        title = yaml.title,
        body = processed_body,
        created = yaml.created or "",
        updated = yaml.updated or "",
    }

    return params
end

if req.method == "POST" then
    -- POST /
    local post_uuid = uuid.v4()
    local file = io.open("content/" .. post_uuid, "w")
    local params = {
        title = req.body.title,
        type = req.body.type,
    }

    local yaml_string = yaml.dump(params)
    local document_text = yaml_string .. "\n\n" .. req.body.text
    local document_params = split_document(document_text, post_uuid)

    file:write(document_text)
    file:close()

    return {
        headers = {
            ["content-type"] = "application/json",
        },
        body = render("document.json", { document = document_params }),
    }
elseif req.path:match("^/%a+/" .. uuid_pattern .. "/?$") then
    -- GET /[type]/[uuid]
    local type, id = req.path:match("/(%a*)/(.*)")
    local file_content = fs.read_file("content/" .. id)
    local template_params = split_document(file_content, id, type)

    return {
        headers = {
            ["content-type"] = "application/json",
        },
        body = render("document.json", { document = template_params }),
    }
elseif req.path:match("^/%a+/?$") then
    -- GET /[type]
    local type = req.path:match("/(%a+)/?")
    local files = fs.get_all_files_in("content/")
    local documents = {}

    for _, file_name in ipairs(files) do
        local file_content = fs.read_file("content/" .. file_name)
        local template_params = split_document(file_content, file_name)

        if template_params.type == type then
            table.insert(documents, template_params)
        end
    end

    local body = "[]"
    if #documents > 0 then
        body = render("document-list.json", { documents = documents })
    end

    return {
        headers = {
            ["content-type"] = "application/json",
        },
        body = body,
    }
else
    return {
        status = 404,
    }
end
