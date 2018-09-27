local utils = require "utils.utils"
local luvent = require "utils.Luvent"
local test_client_action = require "actions.test-client"
local debug_action = require "actions.debug"
local post_document_action = require "actions.post_document"
local get_document_action = require "actions.get_document_by_type"
local get_documents_action = require "actions.get_documents_by_type"

local req = ctx.msg -- get the request
local response -- declare the response

-- Test the set module
local a = stringset.create()
a:insert("Colombia")
a:insert("Caracas")
a:insert("Curaçao")
local b = stringset.create()
b:insert("Venezuela")
b:insert("Colombia")
b:insert("Brazil")
print(a:union(b))

reqProcess = luvent.newEvent() -- create event for request processing

-- declare and add actions

reqProcess:addAction( 
    function(req)
        debug_action.action(req)
    end
)
reqProcess:addAction( -- 
    function(req)
       response = test_client_action.action(req)
    end
)
reqProcess:addAction(
    function(req)
        response = post_document_action.action(req)
    end
)
reqProcess:addAction(
    function(req)
        response = get_document_action.action(req)
    end
)
reqProcess:addAction(
    function(req)
        response = get_documents_action.action(req)
    end
)
-- end of declaring actions

-- try/catch in case of errors 

utils.try(function()
    
    reqProcess:trigger(req) -- try to process request and give response

end, function(err)
    response = { 
        status = 500,
        body = '{ "error": ' .. err .. ' }',  -- if something go wrong give error 500 in response
    }
end)


return response

