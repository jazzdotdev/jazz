local create_document = require "documents.create_document"

local event = {1}
local priority = 1

function action(req)
    if req.method == "POST" and req.path == "/" then
        response = create_document(req)
        print("post_document action")
    end

    return response
end

return{
    event = event,
    action = action,
    priority = priority
}