local utils = require "utils.utils"

function rule(req, events)

    if req.method == "GET" and req.path:match("^/%a+/" .. utils.uuid_pattern .. "/?$") then
        if string.match( req.headers["accept"], "json" ) then
            events[3]:trigger(req) -- It is for json only
        elseif string.match( req.headers["accept"], "html" ) then
            events[6]:trigger(req) -- It is for html only
        end
    end


end

return{
    rule = rule
}