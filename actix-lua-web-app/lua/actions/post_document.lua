local create_document = require "documents.create_document"

function action(req)
    if req.method == "POST" and req.path == "/" then
        response = create_document(req)
    end

    return response
end

return{
    action = action
}