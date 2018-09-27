local utils = require "utils.utils"

function rule(req, events)
    if req.method == "GET" and string.match( req.headers["accept"], "html" ) and req.path:match("^/%a+/" .. utils.uuid_pattern .. "/?$") then
        events[3]:trigger(req)
    end

end

return{
    rule = rule
}