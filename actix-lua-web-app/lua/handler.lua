local debug = require "debug"
local utils = require "utils"
local create_document = require "documents.create_document"
local get_document = require "documents.get_document"
local get_documents = require "documents.get_documents"
local inspect = require "inspect"
local luvent = require "Luvent"

local req = ctx.msg

printReqInfo = luvent.newEvent()
printReqInfo:addAction(
    function(req)
        debug.print_req_info(req)
    end
)

printReqInfo:trigger(req)

local response


createDocument = luvent.newEvent()
createDocument:addAction(
    function(req)
        return create_document(req)
    end
)

getDocument = luvent.newEvent()
getDocument:addAction(
    function(req)
        return get_document(req)
    end
)

getDocuments = luvent.newEvent()
getDocuments:addAction(
    function(req)
        return get_documents(req)
    end
)

inspectEvent = luvent.newEvent()
inspectEvent:addAction(
    function(new_todo)
        print(inspect(new_todo))
        response = {
            body = inspect(new_todo)
        }
    end
)

reqProcess = luvent.newEvent()
reqProcess:addAction(
    function(req)
        if req.method == "POST" and req.path == "/" then
            response = createDocument:trigger(req)
        elseif req.method == "GET" and req.path:match("^/%a+/" .. utils.uuid_pattern .. "/?$") then
            response = getDocument:trigger(req)
        elseif req.method == "GET" and req.path:match("^/%a+/?$") then
            response = getDocuments:trigger(req)
        elseif req.method == "GET" and req.path == "/test-client" then
            local new_todo = ClientRequest.build()
                :method("POST")
                :uri("http://jsonplaceholder.typicode.com/todos/")
                :headers({ ["content-type"] = "application/json" })
                :send()
                inspectEvent:trigger(new_todo)
        else
            response = {
                status = 404,
            }
        end
    end, function(err)
        response = {
            status = 500,
            body = '{ "error": ' .. err .. ' }',
        }
    end
)

utils.try(function()
    reqProcess:trigger(req)
end)

return response

