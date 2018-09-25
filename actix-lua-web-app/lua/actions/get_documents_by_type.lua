local get_documents = require "documents.get_documents"

function action(req)
    if req.method == "GET" and req.path:match("^/%a+/?$") then
        response = get_documents(req)
    end

    return response
end

return{
    action = action
}