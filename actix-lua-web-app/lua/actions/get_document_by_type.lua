local get_document = require "documents.get_document"
local utils = require "utils.utils"
local debug = require "utils.debug"

function action(req)
    if req.method == "GET" and string.match( req.headers["accept"], "html" ) and req.path:match("^/%a+/" .. utils.uuid_pattern .. "/?$") then
        response = get_document(req)
        print("get_document_by_type action")
    end

    return response
end

return{
    action = action
}