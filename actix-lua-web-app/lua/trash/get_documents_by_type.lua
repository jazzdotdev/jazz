local get_documents = require "documents.get_documents"

local event = {1}

function action(req)
    if req.method == "GET" and req.path:match("^/%a+/?$" ) then
        response = get_documents(req)
        print("get_documents_by_type action")
    end

    return response
end

return{
    event = event,
    action = action
}