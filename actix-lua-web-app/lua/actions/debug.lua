local debug = require "debug"

local function action (req)
    debug.print_req_info(req)
end

return{
    action = action
}