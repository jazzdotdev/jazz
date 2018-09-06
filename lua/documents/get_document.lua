local helpers = require "documents.helpers"
local fs = require "fs"

-- GET /[type]/[uuid]
local function get_document(req)
    local type, id = req.path:match("/(%a*)/(.*)")
    local file_content, template_params
    file_content = fs.read_file("content/" .. id)
    template_params = helpers.split_document(file_content, id, type)

    return {
        headers = {
            ["content-type"] = "application/json",
        },
        body = render("document.json", { document = template_params }),
    }
end

return get_document
