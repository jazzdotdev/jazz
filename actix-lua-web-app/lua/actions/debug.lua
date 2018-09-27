local debug = require "utils.debug"

local event = {1, 2}
local priority = 1


local function action (req)
    debug.print_req_info(req)
end

return{
    event = event,
    action = action,
    priority = priority
}