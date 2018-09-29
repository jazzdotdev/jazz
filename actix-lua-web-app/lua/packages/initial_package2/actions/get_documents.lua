local helpers = require "utils.helpers"
local fs = require "utils.fs"

local event = {"reqProcess_documents"}
local priority = 1

-- GET /[type]
local function get_documents(req)
    local type = req.path:match("/(%a+)/?")
    local files = fs.get_all_files_in("content/")
    local documents = {}

    for _, file_name in ipairs(files) do
        local file_content = fs.read_file("content/" .. file_name)
        local template_params = helpers.split_document(file_content, file_name)

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
end

return {
    get_documents = get_documents,
    action = get_documents,
    event = event,
    priority = priority
}
