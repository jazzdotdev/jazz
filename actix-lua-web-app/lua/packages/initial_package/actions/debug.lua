local debug = require "utils.debug"

local event = {"reqProcess", "reqProcess2"}
local priority = 1


local function action (req)
    debug.print_req_info(req)
end

return{
    event = event,
    action = action,
    priority = priority
}