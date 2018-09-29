local helpers = require "utils.helpers"
local fs = require "utils.fs"

local event = {"reqProcess_document_json"}
local priority = 1
-- GET /[type]/[uuid]
local function get_document(req)
    local type, id = req.path:match("/(%a*)/(.*)")
    local template_params
    local file_content = fs.read_file("content/" .. id)
    print ("[DEBUG] file path = " .. "content/" .. id)
    print ("[DEBUG] file content = " .. file_content)
    --file_content = fs.read_file("templates/index.html")

    if not file_content then
        print "empty file_content if"
        return {
            headers = {
                ["content-type"] = "application/json",
            },
            status = 404,
            body = '{"error": "Document not found"}',
        }
    end

    template_params = helpers.split_document(file_content, id)
    print("[DEBUG]" .. render("document.json", { document = template_params }))
    return {
        headers = {
            ["content-type"] = "application/json",
        },
        body = render("document.json", { document = template_params })
    }
end

return {
    get_document = get_document,
    action = get_document,
    event = event,
    priority = priority
}