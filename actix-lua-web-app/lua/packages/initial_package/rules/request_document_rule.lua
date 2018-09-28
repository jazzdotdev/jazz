local utils = require "utils.utils"

function rule(req, events)
    
    if req.method == "GET" and req.path:match("^/%a+/" .. utils.uuid_pattern .. "/?$") then
        if string.match( req.headers["accept"], "json" ) then
            events["reqProcess_document_json"]:trigger(req) -- This is for json only
        elseif string.match( req.headers["accept"], "html" ) then
            events["reqProcess_document_html"]:trigger(req) -- This is for html only
        end
    end


end

return{
    rule = rule
}