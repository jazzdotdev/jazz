local get_document = require "documents.get_document"
local utils = require "utils"

function action(req)
    if req.method == "GET" and req.path:match("^/%a+/" .. utils.uuid_pattern .. "/?$") then
        response = get_document(req)
    end

    return response
end

return{
    action = action
}