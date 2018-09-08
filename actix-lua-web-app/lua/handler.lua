local debug = require "debug"
local utils = require "utils"
local create_document = require "documents.create_document"
local get_document = require "documents.get_document"
local get_documents = require "documents.get_documents"

local req = ctx.msg

debug.print_req_info(req)

local response

utils.try(function()
    if req.method == "POST" and req.path == "/" then
        response = create_document(req)
    elseif req.method == "GET" and req.path:match("^/%a+/" .. utils.uuid_pattern .. "/?$") then
        response = get_document(req)
    elseif req.method == "GET" and req.path:match("^/%a+/?$") then
        response = get_documents(req)
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
end)

return response

